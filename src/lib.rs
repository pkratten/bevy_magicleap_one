#[cfg(target_arch = "aarch64")]
mod callbacks;
#[cfg(target_arch = "aarch64")]
mod graphics;
#[cfg(target_arch = "aarch64")]
mod log;
#[cfg(target_arch = "aarch64")]
mod magicleap_one_plugin;

mod test_systems;

#[cfg(target_arch = "aarch64")]
use std::os::raw::c_int;

#[cfg(target_arch = "aarch64")]
#[no_mangle]
pub unsafe extern "C" fn run_bevy_magicleap_one() -> c_int {
    magicleap_one_plugin::main();
    return 0;
}
