use std::fmt::Display;

use jazztastic::hidapi::DeviceInfo;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub struct KeyboardId(u64);

impl KeyboardId {
    pub fn new(device_info: &DeviceInfo) -> Self {
        let usage_page = device_info.usage_page() as u64;
        let vendor_id = device_info.vendor_id() as u64;
        let product_id = device_info.product_id() as u64;
        let release_number = device_info.release_number() as u64;
        KeyboardId((usage_page << 48) | (vendor_id << 32) | (product_id << 16) | release_number)
    }
}

impl Display for KeyboardId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
