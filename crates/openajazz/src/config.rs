use crate::id::KeyboardId;
use jazztastic::reports::rgb::Rgb;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub keyboards: HashMap<KeyboardId, Rgb>,

    #[serde(default)]
    pub debug: DebugConfig,

    #[serde(default)]
    pub hacks: HacksConfig,
}

#[derive(Debug, Deserialize)]
pub struct DebugConfig {
    #[serde(default = "default_log_level")]
    pub log_level: String,

    #[serde(default)]
    pub log_usb_devices: bool,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            log_level: default_log_level(),
            log_usb_devices: false,
        }
    }
}

pub fn default_log_level() -> String {
    "openajazz=debug,jazztastic=debug".to_string()
}

#[derive(Debug, Deserialize, Default)]
pub struct HacksConfig {
    #[serde(default)]
    pub delay_secs: u64,
}
