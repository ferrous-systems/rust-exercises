//! Global TrustZone Controller driver for STM32U5

use stm32u5::stm32u5a5 as pac;

/// Describes one of our banks of RAM
///
/// Only banks the GTZC1 can control are listed. We don't have a GTZC2 driver.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SramBank {
    SRAM1,
    SRAM2,
    SRAM3,
    SRAM5,
}

/// A basic Global TrustZone Controller driver for GTZC1
pub struct Driver {
    mem_protect_ctrl1: pac::SEC_GTZC1_MPCBB1,
    mem_protect_ctrl2: pac::SEC_GTZC1_MPCBB2,
    mem_protect_ctrl3: pac::SEC_GTZC1_MPCBB3,
    mem_protect_ctrl5: pac::SEC_GTZC1_MPCBB5,
}

impl Driver {
    /// Create a new Global TrustZone Controller driver for GTZC1
    pub fn new(
        mem_protect_ctrl1: pac::SEC_GTZC1_MPCBB1,
        mem_protect_ctrl2: pac::SEC_GTZC1_MPCBB2,
        mem_protect_ctrl3: pac::SEC_GTZC1_MPCBB3,
        mem_protect_ctrl5: pac::SEC_GTZC1_MPCBB5,
    ) -> Self {
        Self {
            mem_protect_ctrl1,
            mem_protect_ctrl2,
            mem_protect_ctrl3,
            mem_protect_ctrl5,
        }
    }

    /// Give secure mode read/write access to a memory bank even when it's set to nonsecure mode
    pub fn allow_secure_read_write(&mut self, sram_bank: SramBank) -> Result<(), Error> {
        match sram_bank {
            SramBank::SRAM1 => {
                self.mem_protect_ctrl1.cr().modify(|_r, w| {
                    w.srwiladis().set_bit();
                    w
                });
                Ok(())
            }
            SramBank::SRAM2 => {
                self.mem_protect_ctrl2.cr().modify(|_r, w| {
                    w.srwiladis().set_bit();
                    w
                });
                Ok(())
            }
            SramBank::SRAM3 => {
                self.mem_protect_ctrl3.cr().modify(|_r, w| {
                    w.srwiladis().set_bit();
                    w
                });
                Ok(())
            }
            SramBank::SRAM5 => {
                self.mem_protect_ctrl5.cr().modify(|_r, w| {
                    w.srwiladis().set_bit();
                    w
                });
                Ok(())
            }
        }
    }

    /// Map some addresses as nonsecure
    ///
    /// * The start and end address must be relative to the start of the given SRAM Bank.
    /// * The start and end address must be a multiple of the superblock size (16K).
    /// * The end address must be one beyond the end of the range you wish to
    /// map.
    ///
    /// To map all of SRAM3, call `map_addresses_nonsecure(SramBank::SRAM3, 0x00_0000, 0x0D_0000)`
    pub fn map_addresses_nonsecure(
        &mut self,
        sram_bank: SramBank,
        address_range: core::ops::Range<u32>,
    ) -> Result<(), Error> {
        const SUPERBLOCK_SIZE: u32 = 32 * 512;
        let start = address_range.start;
        let end = address_range.end;
        if !start.is_multiple_of(SUPERBLOCK_SIZE) {
            return Err(Error::InvalidStart(start));
        }
        if !end.is_multiple_of(SUPERBLOCK_SIZE) {
            return Err(Error::InvalidEnd(end));
        }
        let start_superblock = (start / SUPERBLOCK_SIZE) as usize;
        let end_superblock = ((end - 1) / SUPERBLOCK_SIZE) as usize;
        for super_block in start_superblock..=end_superblock {
            match sram_bank {
                SramBank::SRAM1 => {
                    self.mem_protect_ctrl1.seccfgr(super_block).write(|w| {
                        // mark all 32 blocks in the superblock as nonsecure
                        unsafe {
                            w.bits(0x0000_0000);
                        }
                        w
                    });
                }
                SramBank::SRAM2 => {
                    self.mem_protect_ctrl2.seccfgr(super_block).write(|w| {
                        unsafe {
                            w.bits(0x0000_0000);
                        }
                        w
                    });
                }
                SramBank::SRAM3 => {
                    self.mem_protect_ctrl3.seccfgr(super_block).write(|w| {
                        unsafe {
                            w.bits(0x0000_0000);
                        }
                        w
                    });
                }
                SramBank::SRAM5 => {
                    self.mem_protect_ctrl5.seccfgr(super_block).write(|w| {
                        unsafe {
                            w.bits(0x0000_0000);
                        }
                        w
                    });
                }
            }
        }

        Ok(())
    }
}

/// An error from the Global TrustZone Controller driver
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidStart(u32),
    InvalidEnd(u32),
}
