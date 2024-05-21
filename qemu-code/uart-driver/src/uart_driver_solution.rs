//! A working driver for the Arm CMSDK Uart
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2024

/// A driver for CMSDK Uart
pub struct Uart<const ADDR: usize>();

impl Uart<0xe7c0_0000> {
    /// Create a new UART object for UART0
    ///
    /// # Safety
    ///
    /// Only construct one object per UART at any given time.
    pub unsafe fn new_uart0() -> Self {
        Uart()
    }
}

impl<const ADDR: usize> Uart<ADDR> {
    /// Offset of the data register from the starting address, in u32 words
    const DATA_OFFSET: usize = 0;

    /// Offset of the status register from the starting address, in u32 words
    const STATUS_OFFSET: usize = 1;

    /// Offset of the control register from the starting address, in u32 words
    const CONTROL_OFFSET: usize = 2;

    /// Offset of the baud-rate-divider register from the starting address, in
    /// u32 words
    const BAUDDIV_OFFSET: usize = 4;

    /// Converting the const-param into a pointer
    const ADDR_PTR: *mut u32 = ADDR as *mut u32;

    /// The TX_FULL bit in the status register
    const STATUS_TX_FULL: u32 = 1 << 0;

    /// The TX_EN bit in the control register
    const CONTROL_TX_EN: u32 = 1 << 0;

    /// Turn on TX and RX
    pub fn enable(&mut self, baudrate: u32, system_clock: u32) {
        let divider = system_clock / baudrate;
        // Set the `bauddiv` register to the value `divider`
        self.set_bauddiv(divider);
        // Set the `control` register to `Self::CONTROL_TX_EN`
        self.set_control(Self::CONTROL_TX_EN);
    }

    /// Write a byte (blocking if there's no space)
    pub fn write(&mut self, byte: u8) {
        // Wait until the TX_FULL bit in the `status` register is not zero
        while (self.get_status() & Self::STATUS_TX_FULL) != 0 {}
        // Write the byte to the `data` register
        self.set_data(byte as u32);
    }

    /// Write to the data register
    fn set_data(&mut self, data: u32) {
        unsafe {
            let ptr = Self::ADDR_PTR.add(Self::DATA_OFFSET);
            ptr.write_volatile(data)
        }
    }

    /// Read from the status register
    fn get_status(&self) -> u32 {
        unsafe {
            let ptr = Self::ADDR_PTR.add(Self::STATUS_OFFSET);
            ptr.read_volatile()
        }
    }

    /// Write to the control register
    fn set_control(&mut self, data: u32) {
        unsafe {
            let ptr = Self::ADDR_PTR.add(Self::CONTROL_OFFSET);
            ptr.write_volatile(data)
        }
    }

    /// Write to the baud rate divider register
    fn set_bauddiv(&mut self, data: u32) {
        unsafe {
            let ptr = Self::ADDR_PTR.add(Self::BAUDDIV_OFFSET);
            ptr.write_volatile(data)
        }
    }
}

impl<const N: usize> core::fmt::Write for Uart<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            self.write(b);
        }
        Ok(())
    }
}

// End of file
