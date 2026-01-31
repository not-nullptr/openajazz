mod brightness;
mod builder;
mod color;
mod direction;
mod effect;
mod flags;
mod speed;

pub use brightness::Brightness;
pub use builder::RgbBuilder;
pub use color::Color;
pub use direction::Direction;
pub use effect::Effect;
pub use speed::Speed;

use crate::into_report::IntoReport;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub color: Color,
    pub effect: Effect,
    pub speed: Speed,
    pub brightness: Brightness,
    pub direction: Direction,
}

impl Rgb {
    pub fn builder() -> RgbBuilder {
        RgbBuilder::new()
    }
}

impl IntoReport for Rgb {
    fn write_into(&self, buf: &mut [u8; 63]) {
        buf[0] = 0x28;
        buf[1] = 0x03;
        buf[2] = 0x06;
        buf[3] = 0x1d;

        buf[8] = self.effect as u8;
        buf[9] = self.brightness as u8;
        buf[10] = self.speed as u8;
        buf[11] = self.direction as u8;

        match self.color {
            Color::Rgb(r, g, b) => {
                buf[13] = r;
                buf[14] = g;
                buf[15] = b;
            }
            Color::Rainbow => {
                buf[12] = 0x01;
            }
        }
    }
}
