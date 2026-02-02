use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter)]
pub enum Color {
    Rgb(u8, u8, u8),
    Rainbow,
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Color::Rgb(r, g, b) => {
                let s = format!("#{:02X}{:02X}{:02X}", r, g, b);
                serializer.serialize_str(&s)
            }

            Color::Rainbow => serializer.serialize_str("rainbow"),
        }
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = serde::Deserialize::deserialize(deserializer)?;

        if s.eq_ignore_ascii_case("rainbow") {
            return Ok(Color::Rainbow);
        }

        if let Some(stripped) = s.strip_prefix('#')
            && stripped.len() == 6
            && let Ok(r) = u8::from_str_radix(&stripped[0..2], 16)
            && let Ok(g) = u8::from_str_radix(&stripped[2..4], 16)
            && let Ok(b) = u8::from_str_radix(&stripped[4..6], 16)
        {
            return Ok(Color::Rgb(r, g, b));
        }

        Err(serde::de::Error::custom(format!(
            "invalid color string: {}",
            s
        )))
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::Rgb(255, 255, 255)
    }
}

use crate::{keyboards::{Keyboard, KeyboardKind, ak35i::Ak35i, ak820::Ak820}, reports::rgb::ToKeyboardFormat};
 
impl ToKeyboardFormat for Color {
	fn write_to_keyboard_format(&self, keyboard_kind: KeyboardKind, buf: &mut [u8]) {
		match keyboard_kind {
			k if k == Ak820::keyboard_kind() => {
				match self {
				    Color::Rgb(r, g, b) => {
				        buf[14] = *r;
				        buf[15] = *g;
				        buf[16] = *b;
				    }
				    Color::Rainbow => {
				        buf[13] = 0x01;
				    }
				}
			}

			k if k == Ak35i::keyboard_kind() => {
				match self {
				    Color::Rgb(r, g, b) => {
				        buf[2] = *r;
				        buf[3] = *g;
				        buf[4] = *b;
				    }
				    Color::Rainbow => {
				        unimplemented!("rainbow color not supported for Ak35i");
				    }
				}
			}

			_ => unimplemented!("unsupported keyboard kind for brightness"),
		}
	}
}