use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, EnumIter, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Effect {
    #[default]
    Static,

    // AK850
    Corrugated,
    Cloud,
    Serpentine,
    Spectrum,
    Breath,
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

    // AK35i
    // Static,
    Glittering,
    Falling,
    Colourful,
    // Breath,
    // Spectrum,
    Outward,
    Scrolling,
    Rolling,
    Rotating,
    Explode,
    Launch,
    // Ripples,
    Flowing,
    Pulsating,
    Tilt,
    Shuttle,
}

use crate::{
    keyboards::{Keyboard, KeyboardKind, ak35i::Ak35i, ak820::Ak820},
    reports::rgb::ToKeyboardFormat,
};

impl ToKeyboardFormat for Effect {
    fn write_to_keyboard_format(&self, keyboard_kind: KeyboardKind, buf: &mut [u8]) {
        let Some(effect_u8) = self.to_u8(keyboard_kind) else {
            log::warn!("unsupported effect {:?} for keyboard kind", self);
            return;
        };

        match keyboard_kind {
            k if k == Ak820::keyboard_kind() => {
                buf[9] = effect_u8;
            }

            k if k == Ak35i::keyboard_kind() => {
                buf[1] = effect_u8;
            }

            _ => unimplemented!("unsupported keyboard kind for effect"),
        }
    }
}

impl Effect {
    pub fn to_u8(&self, keyboard_kind: KeyboardKind) -> Option<u8> {
        match keyboard_kind {
            k if k == Ak820::keyboard_kind() => match self {
                Effect::Static => Some(5),
                Effect::Corrugated => Some(0),
                Effect::Cloud => Some(1),
                Effect::Serpentine => Some(2),
                Effect::Spectrum => Some(3),
                Effect::Breath => Some(4),
                Effect::Reaction => Some(6),
                Effect::Ripples => Some(7),
                Effect::Traverse => Some(8),
                Effect::Stars => Some(9),
                Effect::Flowers => Some(10),
                Effect::Roll => Some(11),
                Effect::Wave => Some(12),
                Effect::Cartoon => Some(13),
                Effect::Rain => Some(14),
                Effect::Scan => Some(15),
                Effect::Surmount => Some(16),
                Effect::Speed => Some(17),
                _ => None,
            },

            k if k == Ak35i::keyboard_kind() => match self {
                Effect::Static => Some(1),
                Effect::Glittering => Some(4),
                Effect::Falling => Some(5),
                Effect::Colourful => Some(6),
                Effect::Breath => Some(7),
                Effect::Spectrum => Some(8),
                Effect::Outward => Some(9),
                Effect::Scrolling => Some(10),
                Effect::Rolling => Some(11),
                Effect::Rotating => Some(12),
                Effect::Explode => Some(13),
                Effect::Launch => Some(14),
                Effect::Ripples => Some(15),
                Effect::Flowing => Some(16),
                Effect::Pulsating => Some(17),
                Effect::Tilt => Some(18),
                Effect::Shuttle => Some(19),
                _ => None,
            },

            _ => unimplemented!("unsupported keyboard kind for effect"),
        }
    }
}
