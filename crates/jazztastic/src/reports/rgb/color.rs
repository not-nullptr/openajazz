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
