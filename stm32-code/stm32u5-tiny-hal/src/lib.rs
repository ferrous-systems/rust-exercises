//! # stm32u5-tiny-hal
//!
//! This small HAL only supports:
//!
//! * RCC
//! * USART1
//! * GPIO Inputs and Outputs
//! * Global TrustZone Controller

#![no_std]

// Ensure we pick up the critical-section implementation
use cortex_m as _;

pub mod gpio;
pub mod gtzc;
pub mod ns_addr;
pub mod pwr;
pub mod rcc;
pub mod usart;
