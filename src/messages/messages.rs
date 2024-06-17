use serde::{Deserialize, Serialize};
use std::{any::Any, fmt::Debug};

// -----------------------------------------------------------------
//                            Traits
pub trait Message: Debug {
    fn as_any(&self) -> &dyn Any;
}

pub trait MessageIdentity {
    fn id() -> i32;
}

// -----------------------------------------------------------------
//              Error Message (if things went wrong)
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub struct MsgError {
    pub error_code: i32,
}

// -----------------------------------------------------------------
//                            Core
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub struct MsgEmpty {
    pub value: i32, // Dummy value, extern C do not allow empty types
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default, Clone, Copy)]
pub struct MsgUse {
    pub component_id: i32,
    pub sticky: i32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub struct MsgVelocity {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub struct MsgAngle {
    pub angle: f32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub struct MsgComponentStatusQuery {
    pub component_id: i32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub struct RMsgComponentStatus {
    pub component_id: i32,
    pub health: f32,
    pub cooldown: f32,
    pub is_activated: bool,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub struct MsgState {
    pub value: i32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RMsgState {
    pub angle: f32,
    pub vel_x: f32,
    pub vel_y: f32,
    pub health: f32,
    pub buffs: Vec<String>,
}

// -----------------------------------------------------------------
//                           Modules
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub struct MsgModuleStatusQuery {
    pub module_id: i32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub struct RMsgModuleStatus {
    pub cooldown: f32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub struct MsgTeleport {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct MsgRadar {
    pub value: i32, // A dummy value to satisfy extern "C"
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct RMsgRadar {
    pub x: f32,
    pub y: f32,
    pub distance: f32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct MsgLaser {
    pub angle: f32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RMsgLaser {
    pub tag: String,
    pub kind: String,
    pub distance: f32,
    pub angle: f32,
    pub buffs: Vec<String>,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct MsgForceField {
    pub value: i32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct MsgMine {
    pub value: i32,
}

// -----------------------------------------------------------------
//                        Messages Added Ad-Hoc
// After this point, all of the messages are added ad-hoc This is because the
// message number cannot continue to change and it is dependent on the struct
// definition order.
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct MsgTime {
    pub value: i32, // Dummy value (for C FFI)
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct RMsgTime {
    pub timestamp: f32, // time in seconds
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct MsgRepair {
    pub component_id: i32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct RMsgRepair {
    pub healed_amount: f32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct MsgThrust {
    pub angle: f32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct MsgScan {
    pub value: i32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RMsgScanObject {
    pub tag: String,
    pub kind: String,
    pub x: f32,
    pub y: f32,
    pub buffs: Vec<String>,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RMsgScan {
    pub objects: Vec<RMsgScanObject>,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct MsgGPS {
    pub value: i32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RMsgGPS {
    pub x: f32,
    pub y: f32,
}
