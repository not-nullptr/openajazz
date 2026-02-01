use crate::{keyboards::Keyboard, reports::rgb::Rgb};
use hidapi::{HidDevice, HidError};

#[derive(Debug)]
pub struct Ak35i {
    device: HidDevice,
}

impl Keyboard for Ak35i {
    const VENDOR_ID: u16 = 0x0c45;
    const PRODUCT_ID: u16 = 0x8009;
    const USAGE_PAGE: u16 = 0xFF1C;

	const MANUFACTURER: &str = "AJAZZ";
	const NAME: &str = "AK35I";

    fn new(device: HidDevice) -> Self {
        Self { device }
    }

    fn device(&self) -> &HidDevice {
        &self.device
    }
}

impl Ak35i {
    pub fn set_rgb(&self, rgb: &Rgb) -> Result<(), HidError> {
        self.send(rgb)
    }
}
