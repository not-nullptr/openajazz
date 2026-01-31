use crate::reports::rgb::{Brightness, Color, Effect, Rgb, Speed, direction::Direction};

#[derive(Debug, Clone, Default)]
pub struct RgbBuilder {
    color: Color,
    effect: Effect,
    speed: Speed,
    brightness: Brightness,
    direction: Direction,
}

impl RgbBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.color = Color::Rgb(r, g, b);
        self
    }

    pub fn rainbow(mut self) -> Self {
        self.color = Color::Rainbow;
        self
    }

    pub fn effect(mut self, effect: Effect) -> Self {
        self.effect = effect;
        self
    }

    pub fn speed(mut self, speed: Speed) -> Self {
        self.speed = speed;
        self
    }

    pub fn brightness(mut self, brightness: Brightness) -> Self {
        self.brightness = brightness;
        self
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn build(self) -> Rgb {
        Rgb {
            color: self.color,
            effect: self.effect,
            speed: self.speed,
            brightness: self.brightness,
            direction: self.direction,
        }
    }
}
