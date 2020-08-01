#![no_std]

pub mod clock;

pub trait RegIO {
    const REG_BASE: u32;
    #[inline(always)]
    unsafe fn write(&self, reg: u32, data: u32) {
        ((reg + Self::REG_BASE) as *mut u32).write_volatile(data);
    }

    #[inline(always)]
    unsafe fn read(&self, reg: u32) -> u32 {
        return ((reg + Self::REG_BASE) as *mut u32).read_volatile();
    }
}

pub const HOST1X_BASE:u32 = 0x50000000;
pub const DISPLAY_A_BASE:u32 = 0x54200000;
pub const DSI_BASE:u32 = 0x54300000;
pub const VIC_BASE:u32 = 0x54340000;
pub const TSEC_BASE:u32 = 0x54500000;
pub const SOR1_BASE:u32 = 0x54580000;
pub const PG_UP_BASE:u32 = 0x60000000;
pub const TMR_BASE:u32 = 0x60005000;
pub const CLOCK_BASE:u32 = 0x60006000;
pub const FLOW_CTLR_BASE:u32 = 0x60007000;
pub const SYSREG_BASE:u32 = 0x6000C000;
pub const SB_BASE:u32 = SYSREG_BASE + 0x200;
pub const GPIO_BASE:u32 = 0x6000D000;
pub const GPIO_1_BASE:u32 = GPIO_BASE;
pub const GPIO_2_BASE:u32 = GPIO_BASE + 0x100;
pub const GPIO_3_BASE:u32 = GPIO_BASE + 0x200;
pub const GPIO_4_BASE:u32 = GPIO_BASE + 0x300;
pub const GPIO_5_BASE:u32 = GPIO_BASE + 0x400;
pub const GPIO_6_BASE:u32 = GPIO_BASE + 0x500;
pub const GPIO_7_BASE:u32 = GPIO_BASE + 0x600;
pub const GPIO_8_BASE:u32 = GPIO_BASE + 0x700;
pub const EXCP_VEC_BASE:u32 = 0x6000F000;
pub const APB_MISC_BASE:u32 = 0x70000000;
pub const PINMUX_AUX_BASE:u32 = 0x70003000;
pub const UART_BASE:u32 = 0x70006000;
pub const PMC_BASE:u32 = 0x7000E400;
pub const SYSCTR0_BASE:u32 = 0x7000F000;
pub const FUSE_BASE:u32 = 0x7000F800;
pub const KFUSE_BASE:u32 = 0x7000FC00;
pub const SE_BASE:u32 = 0x70012000;
pub const MC_BASE:u32 = 0x70019000;
pub const EMC_BASE:u32 = 0x7001B000;
pub const MIPI_CAL_BASE:u32 = 0x700E3000;
pub const I2S_BASE:u32 = 0x702D1000;

pub static CLOCK: clock::Clock = clock::Clock {
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
