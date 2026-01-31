use strum::{Display, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, EnumIter)]
pub enum Speed {
    Highest,
    High,
    #[default]
    Medium,
    Low,
    Lowest,
}
