pub mod task;

use ajazz::{
    hidapi::{DeviceInfo, HidApi},
    reports::rgb::Rgb,
};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::manager::task::KeyboardTask;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug)]
pub struct ManagerEntry {
    pub id: KeyboardId,
    pub name: String,
}

#[derive(Debug)]
pub struct KeyboardManager {
    tx: Sender<ToKeyboardTask>,
    rx: Option<Receiver<FromKeyboardTask>>,
}

impl Clone for KeyboardManager {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
            rx: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ToKeyboardTask {
    SetRgb(KeyboardId, Rgb),
    RefreshDevices,
}

#[derive(Debug, Clone)]
pub enum FromKeyboardTask {
    Connected(KeyboardId, String),
    Disconnected(KeyboardId),
}

impl KeyboardManager {
    pub fn new(api: HidApi) -> Self {
        let (tx, rx) = KeyboardTask::spawn(api);
        Self { tx, rx: Some(rx) }
    }

    pub fn from_channels(tx: Sender<ToKeyboardTask>, rx: Receiver<FromKeyboardTask>) -> Self {
        Self { tx, rx: Some(rx) }
    }

    pub async fn recv(&mut self) -> Option<FromKeyboardTask> {
        self.rx.as_mut()?.recv().await
    }

    pub async fn set_rgb(&self, id: KeyboardId, rgb: Rgb) -> color_eyre::Result<()> {
        self.tx.send(ToKeyboardTask::SetRgb(id, rgb)).await?;
        Ok(())
    }

    pub async fn refresh_devices(&self) -> color_eyre::Result<()> {
        self.tx.send(ToKeyboardTask::RefreshDevices).await?;
        Ok(())
    }
}
