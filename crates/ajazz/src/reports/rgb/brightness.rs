use strum::{Display, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, EnumIter)]
pub enum Brightness {
    Lowest,
    Low,
    Medium,
    High,
    #[default]
    Highest,
}
