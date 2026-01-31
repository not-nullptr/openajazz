use strum::{Display, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, EnumIter)]
pub enum Direction {
    #[default]
    LeftToRight = 0,
    RightToLeft,
}
