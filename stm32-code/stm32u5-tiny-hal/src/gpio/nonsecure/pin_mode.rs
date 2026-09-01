use super::{PinInner, PinKind, private};

/// A Pin in Analog mode
#[derive(Debug)]
pub struct Analog(pub(crate) PinInner<false>);

impl PinKind<false> for Analog {
    fn degrade(self, _: private::Token) -> PinInner<false> {
        self.0
    }
}

/// A Pin in input mode
#[derive(Debug)]
pub struct Input(pub(crate) PinInner<false>);

impl Input {
    pub fn is_high(&self) -> bool {
        self.0.read_idr()
    }

    pub fn is_low(&self) -> bool {
        !self.is_high()
    }
}

impl PinKind<false> for Input {
    fn degrade(self, _: private::Token) -> PinInner<false> {
        self.0
    }
}

/// A Pin in output mode
#[derive(Debug)]
pub struct Output(pub(crate) PinInner<false>);

impl Output {
    /// Set pin high
    pub fn set_high(&self) {
        self.set(true);
    }

    /// Set pin low
    pub fn set_low(&self) {
        self.set(false);
    }

    /// Set pin high/low
    pub fn set(&self, high: bool) {
        self.0.write_odr(high);
    }
}

impl PinKind<false> for Output {
    fn degrade(self, _: private::Token) -> PinInner<false> {
        self.0
    }
}

/// A Pin in Alternate Function mode
#[derive(Debug)]
pub struct Af(pub(crate) PinInner<false>);

impl PinKind<false> for Af {
    fn degrade(self, _: private::Token) -> PinInner<false> {
        self.0
    }
}
