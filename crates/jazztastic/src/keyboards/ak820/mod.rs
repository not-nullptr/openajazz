use crate::{keyboards::Keyboard, reports::rgb::Rgb};
use hidapi::{HidDevice, HidError};

#[derive(Debug)]
pub struct Ak820 {
    device: HidDevice,
}

impl Keyboard for Ak820 {
    const VENDOR_ID: u16 = 0x320F;
    const PRODUCT_ID: u16 = 0x505B;
    const USAGE_PAGE: u16 = 0xFF1C;

    fn new(device: HidDevice) -> Self {
        Self { device }
    }

    fn device(&self) -> &HidDevice {
        &self.device
    }
}

impl Ak820 {
    pub fn set_rgb(&self, rgb: &Rgb) -> Result<(), HidError> {
        self.send(rgb)
    }
}
