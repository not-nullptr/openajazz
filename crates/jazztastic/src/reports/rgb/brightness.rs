use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, EnumIter, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Brightness {
    Lowest,
    Low,
    Medium,
    High,
    #[default]
    Highest,
}

use crate::{keyboards::{Keyboard, KeyboardKind, ak35i::Ak35i, ak820::Ak820}, reports::rgb::ToKeyboardFormat};
 
impl ToKeyboardFormat for Brightness {
	fn write_to_keyboard_format(&self, keyboard_kind: KeyboardKind, buf: &mut [u8]) {
		match keyboard_kind {
			k if k == Ak820::keyboard_kind() || k == Ak35i::keyboard_kind() => {
				buf[10] = *self as u8;
			}

			_ => unimplemented!("unsupported keyboard kind for brightness"),
		}
	}
}