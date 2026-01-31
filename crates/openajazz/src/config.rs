use crate::id::KeyboardId;
use jazztastic::reports::rgb::Rgb;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub keyboards: HashMap<KeyboardId, Rgb>,
}
