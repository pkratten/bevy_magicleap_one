use libc::dup2;
use libc::pipe;
use libc::read;
use magicleap_one_lumin_sdk_sys::magicleap_c_api;
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::str;
use std::thread;
use std::{ffi::CString, io::Write};
use tracing::{error, info};
use tracing_core::Level;
use tracing_subscriber::fmt::MakeWriter;

pub fn setup_magicleap_one_tracing() {
    let logger = MagicLeapOneLogger {};
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_max_level(Level::DEBUG)
        .with_writer(logger)
        .init();
    redirect_stdout_to_log();
}

pub enum LogLevelWriter {
    #[allow(dead_code)]
    Fatal(String),
    Error(String),
    Warning(String),
    Info(String),
    Debug(String),
    Verbose(String),
}

impl Write for LogLevelWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let buf = buf.to_owned();
        let log_level;
        let tag;
        match self {
            LogLevelWriter::Fatal(string) => {
                log_level = magicleap_c_api::MLLogLevel_MLLogLevel_Fatal;
                tag = string;
            }
            LogLevelWriter::Error(string) => {
                log_level = magicleap_c_api::MLLogLevel_MLLogLevel_Error;
                tag = string;
            }
            LogLevelWriter::Warning(string) => {
                log_level = magicleap_c_api::MLLogLevel_MLLogLevel_Warning;
                tag = string;
            }
            LogLevelWriter::Info(string) => {
                log_level = magicleap_c_api::MLLogLevel_MLLogLevel_Info;
                tag = string;
            }
            LogLevelWriter::Debug(string) => {
                log_level = magicleap_c_api::MLLogLevel_MLLogLevel_Debug;
                tag = string;
            }
            LogLevelWriter::Verbose(string) => {
                log_level = magicleap_c_api::MLLogLevel_MLLogLevel_Verbose;
                tag = string;
            }
        };

        let message = (*buf).as_ptr();

        let c_str = CString::new(tag.to_owned()).unwrap();
        let tag = c_str.as_ptr();

        unsafe {
            magicleap_c_api::MLLoggingLog(log_level, tag, message);
        };

        return Ok(buf.len());
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub struct MagicLeapOneLogger {}

impl<'a> MakeWriter<'a> for MagicLeapOneLogger {
    type Writer = LogLevelWriter;

    fn make_writer(&'a self) -> Self::Writer {
        LogLevelWriter::Info("stdout".to_string())
    }

    fn make_writer_for(&'a self, meta: &tracing::Metadata<'_>) -> Self::Writer {
        let tag = meta.target().to_string();
        match *meta.level() {
            Level::DEBUG => LogLevelWriter::Debug(tag),
            Level::ERROR => LogLevelWriter::Error(tag),
            Level::INFO => LogLevelWriter::Info(tag),
            Level::TRACE => LogLevelWriter::Verbose(tag),
            Level::WARN => LogLevelWriter::Warning(tag),
        }
    }
}

//Derived from: github.com/servo/webxr
/* Below Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
fn redirect_stdout_to_log() {
    // The first step is to redirect stdout and stderr to the logs.
    // We redirect stdout and stderr to a custom descriptor.
    let mut pfd: [c_int; 2] = [0, 0];
    unsafe {
        pipe(pfd.as_mut_ptr());
        dup2(pfd[1], 1);
        dup2(pfd[1], 2);
    }

    let descriptor = pfd[0];

    // Then we spawn a thread whose only job is to read from the other side of the
    // pipe and redirect to the logs.
    let _detached = thread::spawn(move || {
        const BUF_LENGTH: usize = 512;
        let mut buf = vec![b'\0' as c_char; BUF_LENGTH];

        // Always keep at least one null terminator
        const BUF_AVAILABLE: usize = BUF_LENGTH - 1;
        let buf = &mut buf[..BUF_AVAILABLE];

        let mut cursor = 0_usize;

        loop {
            let result = {
                let read_into = &mut buf[cursor..];
                unsafe {
                    read(
                        descriptor,
                        read_into.as_mut_ptr() as *mut _,
                        read_into.len(),
                    )
                }
            };

            let end = if result == 0 {
                return;
            } else if result < 0 {
                error!("Error occured in log thread! Closing!");
                return;
            } else {
                result as usize + cursor
            };

            // Only modify the portion of the buffer that contains real data.
            let buf = &mut buf[0..end];

            if let Some(last_newline_pos) = buf.iter().rposition(|&c| c == b'\n' as c_char) {
                buf[last_newline_pos] = b'\0' as c_char;

                if let Ok(string) = str::from_utf8(buf) {
                    info!(string);
                } else {
                    info!("Unreadable log buffer!");
                }

                if last_newline_pos < buf.len() - 1 {
                    let pos_after_newline = last_newline_pos + 1;
                    let len_not_logged_yet = buf[pos_after_newline..].len();
                    for j in 0..len_not_logged_yet as usize {
                        buf[j] = buf[pos_after_newline + j];
                    }
                    cursor = len_not_logged_yet;
                } else {
                    cursor = 0;
                }
            } else if end == BUF_AVAILABLE {
                // No newline found but the buffer is full, flush it anyway.
                // `buf.as_ptr()` is null-terminated by BUF_LENGTH being 1 less than BUF_AVAILABLE.

                if let Ok(string) = str::from_utf8(buf) {
                    info!(string);
                } else {
                    info!("Unreadable log buffer!");
                }

                cursor = 0;
            } else {
                cursor = end;
            }
        }
    });
}
