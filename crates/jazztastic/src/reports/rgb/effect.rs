use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, EnumIter, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Effect {
    Corrugated = 0x01,
    Cloud,
    Serpentine,
    Spectrum,
    Breath,
    #[default]
    Normal,
    Reaction,
    Ripples,
    Traverse,
    Stars,
    Flowers,
    Roll,
    Wave,
    Cartoon,
    Rain,
    Scan,
    Surmount,
    Speed,
}
