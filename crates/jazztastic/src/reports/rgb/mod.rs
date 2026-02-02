mod brightness;
mod builder;
mod color;
mod direction;
mod effect;
mod flags;
mod speed;

use std::time::Duration;

pub use brightness::Brightness;
pub use builder::RgbBuilder;
pub use color::Color;
pub use direction::Direction;
pub use effect::Effect;
use hex_literal::hex;
use serde::{Deserialize, Serialize};
pub use speed::Speed;

use crate::{
    into_report::{Bytes, Instruction, IntoReport, OneOrMany},
    keyboards::{Keyboard, KeyboardKind, ak35i::Ak35i, ak820::Ak820},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rgb {
    #[serde(default)]
    pub color: Color,

    #[serde(default)]
    pub effect: Effect,

    #[serde(default)]
    pub speed: Speed,

    #[serde(default)]
    pub brightness: Brightness,

    #[serde(default)]
    pub direction: Direction,
}

impl Rgb {
    pub fn builder() -> RgbBuilder {
        RgbBuilder::new()
    }
}

impl IntoReport for Rgb {
    fn report(&self, keyboard_kind: KeyboardKind) -> OneOrMany<Instruction> {
        let mut buf = [0u8; 65];

        self.color.write_to_keyboard_format(keyboard_kind, &mut buf);
        self.effect
            .write_to_keyboard_format(keyboard_kind, &mut buf);
        self.brightness
            .write_to_keyboard_format(keyboard_kind, &mut buf);
        self.speed.write_to_keyboard_format(keyboard_kind, &mut buf);
        self.direction
            .write_to_keyboard_format(keyboard_kind, &mut buf);

        match keyboard_kind {
            k if k == Ak820::keyboard_kind() => {
                buf[0] = 0x04;

                buf[1] = 0x2A;
                buf[2] = 0x3D;
                buf[3] = 0x06;
                buf[4] = 0x1d;

                OneOrMany::One(Instruction::Write(buf))
            }

            k if k == Ak35i::keyboard_kind() => {
                const DELAY: Duration = Duration::from_millis(5);

                buf[15] = 0xAA;
                buf[16] = 0x55;

                OneOrMany::Many(vec![
                    Instruction::Write(hex!(
                        "00 04 02 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"
                    )),
                    Instruction::Delay(DELAY),
                    Instruction::Read(Bytes::SixtyFive),
                    Instruction::Delay(DELAY),
                    Instruction::Write(hex!(
                        "00 04 18 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"
                    )),
                    Instruction::Delay(DELAY),
                    Instruction::Read(Bytes::SixtyFive),
                    Instruction::Delay(DELAY),
                    Instruction::Write(hex!(
                        "00 04 13 00 00 00 00 00 00 01 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"
                    )),
                    Instruction::Delay(DELAY),
                    Instruction::Write(buf),
                ])
            }

            _ => {
                panic!("unknown keyboard kind for RGB report");
            }
        }
    }
}

pub trait ToKeyboardFormat {
    fn write_to_keyboard_format(&self, keyboard_kind: KeyboardKind, buf: &mut [u8]);
}
