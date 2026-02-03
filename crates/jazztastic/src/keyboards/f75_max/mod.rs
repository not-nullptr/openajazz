use std::time::Duration;

use crate::{
    into_report::WriteInto,
    keyboards::{DynKeyboard, Keyboard, KeyboardFeatures},
    reports::{
        rgb::{Rgb, ToKeyboardFormat},
        time::TimeSync,
    },
};
use chrono::{Datelike, Timelike};
use hidapi::{HidDevice, HidError};

#[derive(Debug)]
pub struct F75Max {
    device: HidDevice,
}

impl Keyboard for F75Max {
    const VENDOR_ID: u16 = 0x0c45;
    const PRODUCT_ID: u16 = 0x800a;
    const USAGE_PAGE: u16 = 0xff13;

    const FEATURES: KeyboardFeatures = KeyboardFeatures::RGB.union(KeyboardFeatures::TIME_SYNC);

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

pub struct F75CommunicationGuard<'a, K: DynKeyboard + ?Sized> {
    keyboard: &'a mut K,
    temp: [u8; 65],
}

impl<'a, K: DynKeyboard + ?Sized> F75CommunicationGuard<'a, K> {
    const DELAY: Duration = Duration::from_millis(5);

    pub fn new(keyboard: &'a mut K) -> Result<Self, HidError> {
        let mut temp = [0; 65];

        keyboard.send_dyn(&F75ControlMessage::BeginCommunication)?;
        std::thread::sleep(Self::DELAY);
        keyboard.read_dyn(&mut temp)?;
        std::thread::sleep(Self::DELAY);

        Ok(Self { keyboard, temp })
    }

    pub fn send(&mut self, data: F75DataMessage) -> Result<(), HidError> {
        let mut buf = [0; 65];
        let kind = self.keyboard.keyboard_kind_dyn();

        let pre = data.pre();
        self.keyboard.write_dyn(&construct_pre_data_msg(pre))?;
        std::thread::sleep(Self::DELAY);
        self.keyboard.read_dyn(&mut self.temp)?;
        std::thread::sleep(Self::DELAY);

        match data {
            F75DataMessage::Rgb(rgb) => {
                buf[15] = 0xAA;
                buf[16] = 0x55;
                rgb.color.write_to_keyboard_format(kind, &mut buf);
                rgb.effect.write_to_keyboard_format(kind, &mut buf);
                rgb.brightness.write_to_keyboard_format(kind, &mut buf);
                rgb.speed.write_to_keyboard_format(kind, &mut buf);
                rgb.direction.write_to_keyboard_format(kind, &mut buf);
            }

            F75DataMessage::TimeSync(time_sync) => {
                buf[2] = 0x01;
                buf[3] = 0x5a;
                buf[4] = (time_sync.date_time.day() % 100) as u8; // year? (last 2 digits)
                buf[5] = time_sync.date_time.day() as u8; // day?
                buf[6] = time_sync.date_time.month() as u8; // month?
                buf[7] = time_sync.date_time.hour() as u8; // hour?
                buf[8] = time_sync.date_time.minute() as u8; // minute?
                buf[9] = time_sync.date_time.second() as u8; // second?

                // magic?
                buf[63] = 0xaa;
                buf[64] = 0x55;
            }
        }

        self.keyboard.write_dyn(&buf)?;
        std::thread::sleep(Self::DELAY);

        Ok(())
    }
}

impl<'a, K: DynKeyboard + ?Sized> Drop for F75CommunicationGuard<'a, K> {
    fn drop(&mut self) {
        self.keyboard
            .send_dyn(&F75ControlMessage::EndCommunication)
            .ok();
        std::thread::sleep(Self::DELAY);
        self.keyboard.read_dyn(&mut self.temp).ok();
    }
}

#[derive(Clone, Copy)]
enum F75ControlMessage {
    BeginCommunication = 0x18,
    EndCommunication = 0x02,
}

#[derive(Clone, Copy)]
pub enum F75DataMessage<'a> {
    Rgb(&'a Rgb),
    TimeSync(&'a TimeSync),
}

impl F75DataMessage<'_> {
    fn pre(&self) -> u8 {
        match self {
            Self::Rgb(_) => 0x13,
            Self::TimeSync(_) => 0x28,
        }
    }
}

impl WriteInto for F75ControlMessage {
    fn write_into<K: DynKeyboard + ?Sized>(&self, keyboard: &mut K) -> Result<(), HidError> {
        keyboard
            .write_dyn(&construct_control_msg(*self as u8))
            .map(|_| ())
    }
}

fn construct_control_msg(kind: u8) -> [u8; 65] {
    let mut buf = [0; 65];
    buf[1] = 0x04;
    buf[2] = kind;
    buf
}

fn construct_pre_data_msg(kind: u8) -> [u8; 65] {
    let mut buf = construct_control_msg(kind);
    buf[9] = 0x01;
    buf
}
