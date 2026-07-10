//! Memory Addresses for nonsecure access to memory

pub const SRAM1_START: u32 = 0x2000_0000;
pub const SRAM2_START: u32 = 0x200C_0000;
pub const SRAM3_START: u32 = 0x200D_0000;
pub const SRAM4_START: u32 = 0x2800_0000;
pub const SRAM5_START: u32 = 0x201A_0000;

pub const SRAM1_LEN: u32 = 0xC_0000;
pub const SRAM2_LEN: u32 = 0x1_0000;
pub const SRAM3_LEN: u32 = 0xD_0000;
pub const SRAM4_LEN: u32 = 0x0_4000;
pub const SRAM5_LEN: u32 = 0xD_0000;

pub const SRAM1_END: u32 = (SRAM1_START + SRAM1_LEN) - 1;
pub const SRAM2_END: u32 = (SRAM2_START + SRAM2_LEN) - 1;
pub const SRAM3_END: u32 = (SRAM3_START + SRAM3_LEN) - 1;
pub const SRAM4_END: u32 = (SRAM4_START + SRAM4_LEN) - 1;
pub const SRAM5_END: u32 = (SRAM5_START + SRAM5_LEN) - 1;

pub const FLASH1_START: u32 = 0x0800_0000;
pub const FLASH2_START: u32 = 0x0820_0000;

pub const FLASH1_END: u32 = (FLASH1_START + FLASH1_LEN) - 1;
pub const FLASH2_END: u32 = (FLASH2_START + FLASH2_LEN) - 1;

pub const FLASH1_LEN: u32 = 0x20_0000;
pub const FLASH2_LEN: u32 = 0x20_0000;

pub const PERIPH_START: u32 = 0x4000_0000;
pub const PERIPH_LEN: u32 = 0x1000_0000;
pub const PERIPH_END: u32 = (PERIPH_START + PERIPH_LEN) - 1;
