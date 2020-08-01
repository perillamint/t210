use super::RegIO;

pub const PWRGATE_TOGGLE: u32 = 0x30;
pub const PWRGATE_STATUS: u32 = 0x38;
pub const NO_IOPOWER: u32 = 0x44;
pub const SCRATCH0: u32 = 0x50;
pub const SCRATCH1: u32 = 0x54;
pub const SCRATCH20: u32 = 0xA0;
pub const PWR_DET_VAL: u32 = 0xE4;
pub const DDR_PWR: u32 = 0xE8;
pub const CRYPTO_OP: u32 = 0xF4;
pub const OSC_EDPD_OVER: u32 = 0x1A4;
pub const IO_DPD_REQ: u32 = 0x1B8;
pub const IO_DPD2_REQ: u32 = 0x1C0;
pub const VDDP_SEL: u32 = 0x1CC;
pub const TSC_MULT: u32 = 0x2B4;
pub const REG_SHORT: u32 = 0x2CC;
pub const WEAK_BIAS: u32 = 0x2C8;
pub const SECURE_SCRATCH21: u32 = 0x334;
pub const CNTRL2: u32 = 0x440;
pub const IO_DPD4_REQ: u32 = 0x464;
pub const DDR_CNTRL: u32 = 0x4E4;
pub const SCRATCH188: u32 = 0x810;
pub const SCRATCH190: u32 = 0x818;
pub const SCRATCH200: u32 = 0x840;

pub struct PMC {
}

impl RegIO for PMC {
    const REG_BASE: u32 = super::PMC_BASE;
}