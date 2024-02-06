//! Some USB 2.0 data types
// NOTE this is a partial solution to exercise `usb-4`

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

use core::num::NonZeroU8;

/// Device address assigned by the host; will be in the range 1..=127
pub type Address = NonZeroU8;

/// The ways this module can fail
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Recognised the request type but the parameters were bad
    BadRequest,
    /// Don't support that request type
    UnknownRequest,
}

/// Standard USB request
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Request {
    /// GET_DESCRIPTOR
    // see section 9.4.3 of the USB specification
    GetDescriptor {
        /// Requested descriptor
        descriptor: Descriptor,
        /// How many bytes of data to return
        length: u16,
    },

    /// SET_ADDRESS
    // see section 9.4.6 of the USB specification
    SetAddress {
        /// New device address, in the range `1..=127`
        address: Option<Address>,
    },

    /// SET_CONFIGURATION
    // see section 9.4.7 of the USB specification
    #[cfg(TODO)]
    SetConfiguration {
        /// bConfigurationValue to change the device to
        value: Option<NonZeroU8>,
    },
    // there are even more standard requests but we don't need to support them
}

impl Request {
    /// Parses SETUP packet data into a USB request
    ///
    /// Returns `Err` if the SETUP data doesn't match a supported standard request
    // see section 9.4 of the USB specification; in particular tables 9-3, 9-4 and 9-5
    pub fn parse(
        bmrequesttype: u8,
        brequest: u8,
        wvalue: u16,
        windex: u16,
        wlength: u16,
    ) -> Result<Self, Error> {
        // see table 9-4 (USB specification)
        const SET_ADDRESS: u8 = 5;
        const GET_DESCRIPTOR: u8 = 6;

        if bmrequesttype == 0b10000000 && brequest == GET_DESCRIPTOR {
            // see table 9-5
            const DEVICE: u8 = 1;
            const CONFIGURATION: u8 = 2;

            let desc_ty = (wvalue >> 8) as u8;
            let desc_index = wvalue as u8;
            let langid = windex;

            if desc_ty == DEVICE && desc_index == 0 && langid == 0 {
                Ok(Request::GetDescriptor {
                    descriptor: Descriptor::Device,
                    length: wlength,
                })
            } else if desc_ty == CONFIGURATION && langid == 0 {
                Ok(Request::GetDescriptor {
                    descriptor: Descriptor::Configuration { index: desc_index },
                    length: wlength,
                })
            } else {
                Err(Error::BadRequest)
            }
        } else if bmrequesttype == 0b00000000 && brequest == SET_ADDRESS {
            // Set the device address for all future accesses.
            // (Needed to successfully init when conected to Apple devices)
            // Section 9.4.6 Set Address of the USB specification explains which values for wvalue,
            // windex and wlength are valid.
            if wvalue < 128 && windex == 0 && wlength == 0 {
                Ok(Request::SetAddress {
                    address: NonZeroU8::new(wvalue as u8),
                })
            } else {
                Err(Error::BadRequest)
            }
        } else {
            Err(Error::UnknownRequest)
        }
    }
}

/// Descriptor types that appear in GET_DESCRIPTOR requests
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Descriptor {
    /// Device descriptor
    Device,

    /// Configuration descriptor
    Configuration {
        /// Index of the descriptor
        index: u8,
    },
    // there are even more descriptor types but we don't need to support them
}

#[cfg(test)]
mod tests {
    use core::num::NonZeroU8;

    use crate::{Descriptor, Request};

    #[test]
    fn get_descriptor_device() {
        // OK: GET_DESCRIPTOR Device [length=18]
        assert_eq!(
            Request::parse(0b1000_0000, 0x06, 0x0100, 0, 18),
            Ok(Request::GetDescriptor {
                descriptor: Descriptor::Device,
                length: 18
            })
        );

        // wrong descriptor index
        assert!(Request::parse(0b1000_0000, 0x06, 0x01_01, 0, 18).is_err());
        //                                             ^^

        // has language ID but shouldn't
        assert!(Request::parse(0b1000_0000, 0x06, 0x01_00, 1033, 18).is_err());
        //                                                 ^^^^
    }

    #[test]
    fn set_address() {
        // OK: SET_ADDRESS 16
        assert_eq!(
            Request::parse(0b0000_0000, 0x05, 0x00_10, 0, 0),
            Ok(Request::SetAddress {
                address: NonZeroU8::new(0x10)
            })
        );

        // OK: SET_ADDRESS 0
        assert_eq!(
            Request::parse(0b0000_0000, 0x05, 0x00_00, 0, 0),
            Ok(Request::SetAddress { address: None })
        );

        // address is outside the valid range
        assert!(Request::parse(0b0000_0000, 0x05, 0x00_ff, 1033, 0).is_err());
        //                                             ^^

        // has language id but shouldn't
        assert!(Request::parse(0b0000_0000, 0x05, 0x00_10, 1033, 0).is_err());
        //                                                 ^^^^

        // length should be zero
        assert!(Request::parse(0b0000_0000, 0x05, 0x00_10, 0, 1).is_err());
        //                                                    ^
    }

    #[test]
    fn get_descriptor_configuration() {
        // OK: GET_DESCRIPTOR Configuration 0 [length=9]
        assert_eq!(
            Request::parse(0b1000_0000, 0x06, 0x02_00, 0, 9),
            Ok(Request::GetDescriptor {
                descriptor: Descriptor::Configuration { index: 0 },
                length: 9
            })
        );

        // has language ID but shouldn't
        assert!(Request::parse(0b1000_0000, 0x06, 0x02_00, 1033, 9).is_err());
        //                                                 ^^^^
    }

    #[cfg(TODO)]
    #[test]
    fn set_configuration() {
        // OK: SET_CONFIGURATION 1
        assert_eq!(
            Request::parse(0b0000_0000, 0x09, 0x00_01, 0, 0),
            Ok(Request::SetConfiguration {
                value: NonZeroU8::new(1)
            })
        );

        // OK: SET_CONFIGURATION 0
        assert_eq!(
            Request::parse(0b0000_0000, 0x09, 0x00_00, 0, 0),
            Ok(Request::SetConfiguration { value: None })
        );

        // has language id but shouldn't
        assert!(Request::parse(0b0000_0000, 0x09, 0x00_01, 1033, 0).is_err());
        //                                                 ^^^^

        // length should be zero
        assert!(Request::parse(0b0000_0000, 0x09, 0x00_01, 0, 1).is_err());
        //                                                    ^
    }
}
