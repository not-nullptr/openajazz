use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, EnumIter, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Speed {
    Highest,
    High,
    #[default]
    Medium,
    Low,
    Lowest,
}
