
pub mod module_id
{
    pub type ID = u32;
    pub const APP_CONFIG: u32 = 0x00000001;
    pub const LOGGING: u32 = 0x00000002;
        
}
pub mod event_id
{
    pub const USER_EVENT : u32 = 0x00010000;
}

pub mod logging_level
{    
    pub const NONE: u16 = 0x0000;
    pub const ERROR: u16 = 0x0001;
    pub const WARN: u16 = 0x0010;
    pub const INFO: u16 = 0x0100;
    pub const DEBUG: u16 = 0x1000;
}
pub mod env_os
{
    pub type OS = u8;
    pub const UNKNOWN: OS = 0;
    pub const LINUX: OS = 1;
    pub const WINDOW: OS = 2;
}
pub mod env_val
{
    use crate::env_os;
#[cfg(target_os = "linux")]
    pub const OS: env_os::OS = env_os::LINUX;
#[cfg(target_os = "windows")]
    pub const OS: env_os::OS = env_os::WINDOW;
}

pub mod sock_val
{
    pub const PACKET_HEAD_SIZE: usize = 4;
    pub const PACKET_CHECK: u16 = 50000;
    pub const SERVER: u32 = 1;
    pub const CLIENT: u32 = 2;
}