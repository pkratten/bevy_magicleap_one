use crate::magicleap_one_plugin::MagicLeapOneAppState;
use crate::magicleap_one_plugin::APP_STATE;
use libc::c_void;
use tracing::info;

#[no_mangle]
pub unsafe extern "C" fn on_stopped(_void: *mut c_void) {
    info!("STOPPED! ######################################################################################################################################################################################################################################");
    *APP_STATE.lock().unwrap() = MagicLeapOneAppState::Stopped;
}
#[no_mangle]
pub unsafe extern "C" fn on_paused(_void: *mut c_void) {
    info!("PAUSED! ######################################################################################################################################################################################################################################");
    *APP_STATE.lock().unwrap() = MagicLeapOneAppState::Paused;
}
#[no_mangle]
pub unsafe extern "C" fn on_resumed(_void: *mut c_void) {
    info!("RESUMEND! ######################################################################################################################################################################################################################################");
    *APP_STATE.lock().unwrap() = MagicLeapOneAppState::Running;
}
#[no_mangle]
pub unsafe extern "C" fn on_unload_resources(_void: *mut c_void) {
    info!("ON UNLOAD RESOURCES! ######################################################################################################################################################################################################################################");
    *APP_STATE.lock().unwrap() = MagicLeapOneAppState::Running;
}
#[no_mangle]
pub unsafe extern "C" fn on_new_initarg(_void: *mut c_void) {
    info!("on_new_initarg! ######################################################################################################################################################################################################################################");
    *APP_STATE.lock().unwrap() = MagicLeapOneAppState::Running;
}
#[no_mangle]
pub unsafe extern "C" fn on_device_active(_void: *mut c_void) {
    info!("on_device_active! ######################################################################################################################################################################################################################################");
    *APP_STATE.lock().unwrap() = MagicLeapOneAppState::Running;
}
#[no_mangle]
pub unsafe extern "C" fn on_device_reality(_void: *mut c_void) {
    info!("on_device_reality! ######################################################################################################################################################################################################################################");
    *APP_STATE.lock().unwrap() = MagicLeapOneAppState::Running;
}
#[no_mangle]
pub unsafe extern "C" fn on_device_standby(_void: *mut c_void) {
    info!("on_device_standby! #####################################################################################################################################################################################################################################");
    *APP_STATE.lock().unwrap() = MagicLeapOneAppState::Standby;
}
#[no_mangle]
pub unsafe extern "C" fn on_focus_lost(
    _void: *mut c_void,
    reason: magicleap_one_lumin_sdk_sys::magicleap_c_api::MLLifecycleFocusLostReason,
) {
    info!("on_focus_lost! {:?} ######################################################################################################################################################################################################################################", reason);
    *APP_STATE.lock().unwrap() = MagicLeapOneAppState::LostFocus;
}
#[no_mangle]
pub unsafe extern "C" fn on_focus_gained(_void: *mut c_void) {
    info!("on_focus_gained! ######################################################################################################################################################################################################################################");
    *APP_STATE.lock().unwrap() = MagicLeapOneAppState::Running;
}
