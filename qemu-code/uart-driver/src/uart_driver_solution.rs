//! A working driver for the Arm CMSDK Uart
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2025

/// A CMSDK UART Driver
pub struct Uart {
    registers: MmioRegisters,
}

impl Uart {
    /// Create a new UART handle at the MPS3-AN536 default address for UART 0
    ///
    /// # Safety
    ///
    /// Only call this function once. Creating two handles to the same UART
    /// is Undefined Behaviour.
    pub const unsafe fn new_uart0() -> Uart {
        Uart {
            registers: Registers::new_mmio_at(0xe7c0_0000),
        }
    }

    /// Turn on TX and RX
    pub fn enable(&mut self, baudrate: u32, system_clock: u32) {
        let divider = system_clock / baudrate;
        // Set the `bauddiv` register to the value `divider`
        self.registers.write_baud_div(divider);
        // Set the `tx_en` and `rx_en` bits in the `control` register
        self.registers.modify_control(|mut r| {
            r.set_tx_en(true);
            r
        });
    }

    /// Write a byte (blocking if there's no space)
    pub fn write(&mut self, byte: u8) {
        // Wait while the `tx_full` bit in the `state` register is set
        while self.registers.read_state().tx_full() {
            core::hint::spin_loop();
        }
        // Write the byte to the `data` register
        self.registers.write_data(byte as u32);
    }
}

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            self.write(b);
        }
        Ok(())
    }
}

unsafe impl Send for Uart {}

#[bitbybit::bitfield(u32)]
struct State {
    /// Did RX overflow?
    #[bits(3..=3, rw)]
    rx_ovf: bool,
    /// Did TX overflow?
    #[bits(2..=2, rw)]
    tx_ovf: bool,
    /// Is RX buffer full?
    #[bits(1..=1, rw)]
    rx_full: bool,
    /// Is TX buffer full?
    #[bits(0..=0, rw)]
    tx_full: bool,
}

#[bitbybit::bitfield(u32)]
struct Control {
    /// UART received enabled
    #[bits(1..=1, rw)]
    rx_en: bool,
    /// UART transmit enabled
    #[bits(0..=0, rw)]
    tx_en: bool,
}

#[bitbybit::bitfield(u32)]
struct IntStatus {
    /// RX overflow interrupt.
    #[bits(3..=3, rw)]
    rx_ovf_int: bool,
    /// TX overflow interrupt.
    #[bits(2..=2, rw)]
    tx_ovf_int: bool,
    /// RX interrupt.
    #[bits(1..=1, rw)]
    rx_int: bool,
    /// TX interrupt.
    #[bits(0..=0, rw)]
    tx_int: bool,
}

/// The registers in a CMSDK UART Peripheral
///
/// Running this derive-macro on some type `X` will produce a new struct called
/// `MmioX`.
#[derive(derive_mmio::Mmio)]
#[repr(C)]
struct Registers {
    /// UART TX/RX buffer
    #[mmio(RW)]
    data: u32,
    /// UART State
    #[mmio(RW)]
    state: State,
    /// UART Configuration
    #[mmio(RW)]
    control: Control,
    /// Interrupt Status/Clear
    #[mmio(RW)]
    int_status: IntStatus,
    /// Baud Rate Divisor
    #[mmio(RW)]
    baud_div: u32,
}

// End of file
