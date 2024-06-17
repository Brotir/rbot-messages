#![allow(non_snake_case)]
// DO NOT EDIT MANUALLY
// AUTOMATICALLY GENERATED
use crate::messages::messages as msg;

macro_rules! dec {
    ($bytes: expr => $msg: ty) => {
        match serde_json::from_slice::<$msg>($bytes) {
            Err(_) => <$msg>::default(),
            Ok(m) => m,
        }
    };
}

#[no_mangle]
pub extern "C" fn decode_MsgError(bytes_ptr: *const u8, len: u32) -> msg::MsgError {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::MsgError)
}

#[no_mangle]
pub extern "C" fn decode_MsgEmpty(bytes_ptr: *const u8, len: u32) -> msg::MsgEmpty {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::MsgEmpty)
}

#[no_mangle]
pub extern "C" fn decode_MsgUse(bytes_ptr: *const u8, len: u32) -> msg::MsgUse {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::MsgUse)
}

#[no_mangle]
pub extern "C" fn decode_MsgVelocity(bytes_ptr: *const u8, len: u32) -> msg::MsgVelocity {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::MsgVelocity)
}

#[no_mangle]
pub extern "C" fn decode_MsgAngle(bytes_ptr: *const u8, len: u32) -> msg::MsgAngle {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::MsgAngle)
}

#[no_mangle]
pub extern "C" fn decode_MsgComponentStatusQuery(
    bytes_ptr: *const u8,
    len: u32,
) -> msg::MsgComponentStatusQuery {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::MsgComponentStatusQuery)
}

#[no_mangle]
pub extern "C" fn decode_RMsgComponentStatus(
    bytes_ptr: *const u8,
    len: u32,
) -> msg::RMsgComponentStatus {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::RMsgComponentStatus)
}

#[no_mangle]
pub extern "C" fn decode_MsgState(bytes_ptr: *const u8, len: u32) -> msg::MsgState {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::MsgState)
}

#[no_mangle]
pub extern "C" fn decode_RMsgState(bytes_ptr: *const u8, len: u32) -> msg::RMsgState {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::RMsgState)
}

#[no_mangle]
pub extern "C" fn decode_MsgModuleStatusQuery(
    bytes_ptr: *const u8,
    len: u32,
) -> msg::MsgModuleStatusQuery {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::MsgModuleStatusQuery)
}

#[no_mangle]
pub extern "C" fn decode_RMsgModuleStatus(bytes_ptr: *const u8, len: u32) -> msg::RMsgModuleStatus {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::RMsgModuleStatus)
}

#[no_mangle]
pub extern "C" fn decode_MsgTeleport(bytes_ptr: *const u8, len: u32) -> msg::MsgTeleport {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::MsgTeleport)
}

#[no_mangle]
pub extern "C" fn decode_MsgRadar(bytes_ptr: *const u8, len: u32) -> msg::MsgRadar {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::MsgRadar)
}

#[no_mangle]
pub extern "C" fn decode_RMsgRadar(bytes_ptr: *const u8, len: u32) -> msg::RMsgRadar {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::RMsgRadar)
}

#[no_mangle]
pub extern "C" fn decode_MsgLaser(bytes_ptr: *const u8, len: u32) -> msg::MsgLaser {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::MsgLaser)
}

#[no_mangle]
pub extern "C" fn decode_RMsgLaser(bytes_ptr: *const u8, len: u32) -> msg::RMsgLaser {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::RMsgLaser)
}

#[no_mangle]
pub extern "C" fn decode_MsgForceField(bytes_ptr: *const u8, len: u32) -> msg::MsgForceField {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::MsgForceField)
}

#[no_mangle]
pub extern "C" fn decode_MsgMine(bytes_ptr: *const u8, len: u32) -> msg::MsgMine {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::MsgMine)
}

#[no_mangle]
pub extern "C" fn decode_MsgTime(bytes_ptr: *const u8, len: u32) -> msg::MsgTime {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::MsgTime)
}

#[no_mangle]
pub extern "C" fn decode_RMsgTime(bytes_ptr: *const u8, len: u32) -> msg::RMsgTime {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::RMsgTime)
}

#[no_mangle]
pub extern "C" fn decode_MsgRepair(bytes_ptr: *const u8, len: u32) -> msg::MsgRepair {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::MsgRepair)
}

#[no_mangle]
pub extern "C" fn decode_RMsgRepair(bytes_ptr: *const u8, len: u32) -> msg::RMsgRepair {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::RMsgRepair)
}

#[no_mangle]
pub extern "C" fn decode_MsgThrust(bytes_ptr: *const u8, len: u32) -> msg::MsgThrust {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::MsgThrust)
}

#[no_mangle]
pub extern "C" fn decode_MsgScan(bytes_ptr: *const u8, len: u32) -> msg::MsgScan {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::MsgScan)
}

#[no_mangle]
pub extern "C" fn decode_RMsgScanObject(bytes_ptr: *const u8, len: u32) -> msg::RMsgScanObject {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::RMsgScanObject)
}

#[no_mangle]
pub extern "C" fn decode_RMsgScan(bytes_ptr: *const u8, len: u32) -> msg::RMsgScan {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::RMsgScan)
}

#[no_mangle]
pub extern "C" fn decode_MsgGPS(bytes_ptr: *const u8, len: u32) -> msg::MsgGPS {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::MsgGPS)
}

#[no_mangle]
pub extern "C" fn decode_RMsgGPS(bytes_ptr: *const u8, len: u32) -> msg::RMsgGPS {
    let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, len as usize) };
    dec!(bytes => msg::RMsgGPS)
}
