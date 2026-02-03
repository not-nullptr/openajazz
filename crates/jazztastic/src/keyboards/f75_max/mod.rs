use crate::keyboards::Keyboard;
use hidapi::{HidDevice, HidError};

#[derive(Debug)]
pub struct F75Max {
    device: HidDevice,
}

impl Keyboard for F75Max {
    const VENDOR_ID: u16 = 0x0c45;
    const PRODUCT_ID: u16 = 0x800a;
    const USAGE_PAGE: u16 = 0xff13;

    const MANUFACTURER: &str = "Aula";
    const NAME: &str = "F75 Max";

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
        self.device.send_feature_report(buf).map(|_| buf.len())
    }
}
