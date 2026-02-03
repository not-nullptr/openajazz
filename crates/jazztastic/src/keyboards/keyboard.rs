use crate::{
    error::DetectError,
    into_report::{IntoReport, OneOrMany},
};
use hidapi::{HidApi, HidDevice, HidError};
use std::any::TypeId;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyboardKind(TypeId);

pub trait Keyboard: Sized + 'static {
    const VENDOR_ID: u16;
    const PRODUCT_ID: u16;
    const USAGE_PAGE: u16;

    const MANUFACTURER: &str;
    const NAME: &str;

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

    fn send<R: IntoReport>(&mut self, message: &R) -> Result<(), HidError> {
        let buf = message.report(Self::keyboard_kind());

        match buf {
            OneOrMany::One(instruction) => instruction.execute(self),

            OneOrMany::Many(instructions) => {
                for instruction in instructions {
                    instruction.execute(self)?;
                }

                Ok(())
            }
        }
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, HidError> {
        self.device().read(buf)
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize, HidError> {
        self.device().write(buf)
    }

    fn as_dyn(&self) -> &dyn DynKeyboard {
        self
    }

    fn keyboard_kind() -> KeyboardKind {
        KeyboardKind(TypeId::of::<Self>())
    }
}

pub trait DynKeyboard {
    fn device_dyn(&self) -> &HidDevice;
    // fn send_dyn(&self, message: &dyn IntoReport) -> Result<(), HidError> {
    //     let mut buf = [0u8; 64];
    //     buf[0] = 0x04;

    //     let buf_slice: &mut [u8; 63] = (&mut buf[1..]).try_into().unwrap();
    //     message.write_into(buf_slice);

    //     self.device_dyn().write(&buf)?;

    //     Ok(())
    // }

    fn send_dyn(&mut self, message: &dyn IntoReport) -> Result<(), HidError>;

    fn name_dyn(&self) -> &'static str;
    fn manufacturer_dyn(&self) -> &'static str;

    fn keyboard_kind_dyn(&self) -> KeyboardKind;
}

impl<K: Keyboard> DynKeyboard for K {
    fn device_dyn(&self) -> &HidDevice {
        self.device()
    }

    fn send_dyn(&mut self, message: &dyn IntoReport) -> Result<(), HidError> {
        K::send(self, &message)
    }

    fn name_dyn(&self) -> &'static str {
        K::NAME
    }

    fn manufacturer_dyn(&self) -> &'static str {
        K::MANUFACTURER
    }

    fn keyboard_kind_dyn(&self) -> KeyboardKind {
        K::keyboard_kind()
    }
}
