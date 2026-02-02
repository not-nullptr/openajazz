use std::time::Duration;

use crate::{keyboards::Keyboard, reports::rgb::Rgb};
use hex_literal::hex;
use hidapi::{HidDevice, HidError};

#[derive(Debug)]
pub struct Ak35i {
    device: HidDevice,
}

impl Keyboard for Ak35i {
    const VENDOR_ID: u16 = 0x0c45;
    const PRODUCT_ID: u16 = 0x8009;
    const USAGE_PAGE: u16 = 0xff13;

    const MANUFACTURER: &str = "AJAZZ";
    const NAME: &str = "AK35I";

    fn new(device: HidDevice) -> Self {
        Self { device }
    }

    fn device(&self) -> &HidDevice {
        &self.device
    }

	fn read(&mut self, buf: &mut [u8]) -> Result<usize, HidError> {
		self.device.get_feature_report(buf)
	}

	fn write(&mut self, buf: &[u8]) -> Result<usize, HidError> {
		self.device.send_feature_report(buf)
			.map(|_| buf.len())
	}
}
