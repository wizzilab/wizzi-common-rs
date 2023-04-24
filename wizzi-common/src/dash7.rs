pub type Vid = [u8; 2];
pub type Uid = [u8; 8];

pub mod fid {
    pub const UID: u8 = 0;
    pub const MODEM_VERSION: u8 = 2;
    pub const VID: u8 = 6;
    pub const DLL_CONFIG: u8 = 10;
    pub const RTC_TIME_STAMP: u8 = 29;
}

pub mod xcl {
    pub const DEVICE: u8 = 0x01;
    pub const GW: u8 = 0x21;
}
