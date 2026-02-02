use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, EnumIter, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    #[default]
    LeftToRight = 0,
    RightToLeft,
}

use crate::{
    keyboards::{Keyboard, KeyboardKind, ak35i::Ak35i, ak820::Ak820},
    reports::rgb::ToKeyboardFormat,
};

impl ToKeyboardFormat for Direction {
    fn write_to_keyboard_format(&self, keyboard_kind: KeyboardKind, buf: &mut [u8]) {
        match keyboard_kind {
            k if k == Ak820::keyboard_kind() || k == Ak35i::keyboard_kind() => {
                buf[12] = *self as u8;
            }

            _ => unimplemented!("unsupported keyboard kind for direction"),
        }
    }
}
