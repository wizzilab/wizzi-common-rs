pub const TI_DIVIDER: u64 = 1024;
pub const XTI_PER_TI: u64 = 16;
pub const XTI_DIVIDER: u64 = XTI_PER_TI * TI_DIVIDER;

pub const TI_DURATION: std::time::Duration =
    std::time::Duration::from_micros(1_000_000 / TI_DIVIDER);
pub const XTI_DURATION: std::time::Duration =
    std::time::Duration::from_micros(1_000_000 / XTI_DIVIDER);
