use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, EnumIter, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Speed {
    Fastest,
    Fast,
    #[default]
    Medium,
    Slow,
    Slowest,
}

use crate::{
    keyboards::{Keyboard, KeyboardKind, ak35i::Ak35i, ak820::Ak820},
    reports::rgb::ToKeyboardFormat,
};
impl ToKeyboardFormat for Speed {
    fn write_to_keyboard_format(&self, keyboard_kind: KeyboardKind, buf: &mut [u8]) {
        match keyboard_kind {
            k if k == Ak820::keyboard_kind() => {
                buf[11] = *self as u8;
            }

            k if k == Ak35i::keyboard_kind() => {
                buf[11] = 4 - (*self as u8);
            }

            _ => unimplemented!("unsupported keyboard kind for speed"),
        }
    }
}
