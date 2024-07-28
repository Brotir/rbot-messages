#![allow(non_snake_case)]
// DO NOT EDIT MANUALLY
// AUTOMATICALLY GENERATED

use crate::messages as msg;
use crate::messaging::serialize_message;
use std::ffi::{c_char, CString};

#[no_mangle]
pub extern "C" fn encode_MsgError(message: msg::MsgError) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_MsgEmpty(message: msg::MsgEmpty) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_MsgUse(message: msg::MsgUse) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_MsgVelocity(message: msg::MsgVelocity) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_MsgAngle(message: msg::MsgAngle) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_MsgComponentStatusQuery(
    message: msg::MsgComponentStatusQuery,
) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_RMsgComponentStatus(message: msg::RMsgComponentStatus) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_MsgState(message: msg::MsgState) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_RMsgState(message: msg::RMsgState) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_MsgModuleStatusQuery(message: msg::MsgModuleStatusQuery) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_RMsgModuleStatus(message: msg::RMsgModuleStatus) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_MsgTeleport(message: msg::MsgTeleport) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_MsgRadar(message: msg::MsgRadar) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_RMsgRadar(message: msg::RMsgRadar) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_MsgLaser(message: msg::MsgLaser) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_RMsgLaser(message: msg::RMsgLaser) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_MsgForceField(message: msg::MsgForceField) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_MsgMine(message: msg::MsgMine) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_MsgTime(message: msg::MsgTime) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_RMsgTime(message: msg::RMsgTime) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_MsgRepair(message: msg::MsgRepair) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_RMsgRepair(message: msg::RMsgRepair) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_MsgThrust(message: msg::MsgThrust) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_MsgScan(message: msg::MsgScan) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_RMsgScanObject(message: msg::RMsgScanObject) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_RMsgScan(message: msg::RMsgScan) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_MsgGPS(message: msg::MsgGPS) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_RMsgGPS(message: msg::RMsgGPS) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn encode_MsgAwaitAction(message: msg::MsgAwaitAction) -> *mut c_char {
    let bytes = serialize_message(&message).unwrap();
    let c_string = unsafe { CString::from_vec_unchecked(bytes) };
    c_string.into_raw()
}
