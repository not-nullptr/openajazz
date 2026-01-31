use crate::{error::DetectError, into_report::IntoReport};
use hidapi::{HidApi, HidDevice, HidError};

pub trait Keyboard: Sized {
    const VENDOR_ID: u16;
    const PRODUCT_ID: u16;
    const USAGE_PAGE: u16;

    fn new(device: HidDevice) -> Self;
    fn device(&self) -> &HidDevice;

    fn detect(api: &HidApi) -> Result<Self, DetectError> {
        let device_info = api
            .device_list()
            .find(|d| {
                d.vendor_id() == Self::VENDOR_ID
                    && d.product_id() == Self::PRODUCT_ID
                    && d.usage_page() == Self::USAGE_PAGE
            })
            .ok_or(DetectError::KeyboardNotFound)?;

        let device = device_info.open_device(api)?;
        Ok(Self::new(device))
    }

    fn send<R: IntoReport>(&self, message: &R) -> Result<(), HidError> {
        let mut buf = [0u8; 64];
        buf[0] = 0x04;

        let buf_slice: &mut [u8; 63] = (&mut buf[1..]).try_into().unwrap();
        message.write_into(buf_slice);

        self.device().write(&buf)?;

        Ok(())
    }

    fn as_dyn(&self) -> &dyn DynKeyboard {
        self
    }
}

pub trait DynKeyboard {
    fn device_dyn(&self) -> &HidDevice;
    fn send_dyn(&self, message: &dyn IntoReport) -> Result<(), HidError> {
        let mut buf = [0u8; 64];
        buf[0] = 0x04;

        let buf_slice: &mut [u8; 63] = (&mut buf[1..]).try_into().unwrap();
        message.write_into(buf_slice);

        self.device_dyn().write(&buf)?;

        Ok(())
    }
}

impl<K: Keyboard> DynKeyboard for K {
    fn device_dyn(&self) -> &HidDevice {
        self.device()
    }
}
