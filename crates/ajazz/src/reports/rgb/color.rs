use strum::{Display, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter)]
pub enum Color {
    Rgb(u8, u8, u8),
    Rainbow,
}

impl Default for Color {
    fn default() -> Self {
        Color::Rgb(255, 255, 255)
    }
}
