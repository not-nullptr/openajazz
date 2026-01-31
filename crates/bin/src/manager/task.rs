use ajazz::{
    hidapi::HidApi,
    keyboards::{DynKeyboard, Keyboard, ak820::Ak820},
};

use crate::manager::{FromKeyboardTask, KeyboardId, ToKeyboardTask};
use std::collections::HashMap;
use tokio::sync::mpsc::{self, Receiver, Sender};

pub struct KeyboardTask {
    tx: Sender<FromKeyboardTask>,
    rx: Receiver<ToKeyboardTask>,
    api: HidApi,
    keyboards: HashMap<KeyboardId, Box<dyn DynKeyboard + Send>>,
}

impl KeyboardTask {
    pub fn spawn(api: HidApi) -> (Sender<ToKeyboardTask>, Receiver<FromKeyboardTask>) {
        let (their_tx, our_rx) = mpsc::channel(128);
        let (our_tx, their_rx) = mpsc::channel(128);

        let their_tx_clone = their_tx.clone();
        tokio::spawn(async move {
            Self::refresh_devices_interval(their_tx_clone).await;
        });

        let mut task = KeyboardTask {
            tx: our_tx,
            rx: our_rx,
            api,
            keyboards: HashMap::new(),
        };

        tokio::spawn(async move {
            loop {
                match task.run().await {
                    Ok(_) => {
                        break;
                    }
                    Err(e) => {
                        log::error!("keyboard task error: {}", e);
                    }
                }
            }
        });

        (their_tx, their_rx)
    }

    async fn run(&mut self) -> color_eyre::Result<()> {
        self.refresh_devices().await?;
        while let Some(message) = self.rx.recv().await {
            match message {
                ToKeyboardTask::SetRgb(id, rgb) => {
                    if let Some(keyboard) = self.keyboards.get(&id) {
                        keyboard.send_dyn(&rgb)?;
                    } else {
                        log::warn!("keyboard with id {:?} not found", id);
                    }
                }

                ToKeyboardTask::RefreshDevices => {
                    self.refresh_devices().await?;
                }
            }
        }

        Ok(())
    }

    async fn refresh_devices(&mut self) -> color_eyre::Result<()> {
        self.api.refresh_devices()?;

        let mut found_ids = Vec::with_capacity(self.keyboards.len());

        for device_info in self.api.device_list() {
            let id = KeyboardId::new(device_info);
            if self.keyboards.contains_key(&id) {
                found_ids.push(id);
                continue;
            }

            let Some(keyboard) = (match (
                device_info.vendor_id(),
                device_info.product_id(),
                device_info.usage_page(),
            ) {
                (Ak820::VENDOR_ID, Ak820::PRODUCT_ID, Ak820::USAGE_PAGE) => {
                    let device = device_info.open_device(&self.api)?;
                    Some(Box::new(Ak820::new(device)) as Box<dyn DynKeyboard + Send>)
                }
                _ => None,
            }) else {
                continue;
            };

            log::info!(
                "connected keyboard: {:?} (id: {:?})",
                device_info.product_string(),
                id
            );

            self.tx
                .send(FromKeyboardTask::Connected(
                    id,
                    format!(
                        "{} {}",
                        device_info
                            .manufacturer_string()
                            .unwrap_or("Unknown Manufacturer"),
                        device_info.product_string().unwrap_or("Unknown Keyboard")
                    ),
                ))
                .await?;

            self.keyboards.insert(id, keyboard);
            found_ids.push(id);
        }

        let existing_ids: Vec<KeyboardId> = self.keyboards.keys().cloned().collect();
        for id in existing_ids {
            if !found_ids.contains(&id) {
                log::info!("disconnected keyboard with id: {:?}", id);
                self.keyboards.remove(&id);
                self.tx.send(FromKeyboardTask::Disconnected(id)).await?;
            }
        }

        Ok(())
    }

    async fn refresh_devices_interval(tx: Sender<ToKeyboardTask>) {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            if tx.send(ToKeyboardTask::RefreshDevices).await.is_err() {
                log::warn!("keyboard task refresh sender closed, stopping interval");
                break;
            }
        }
    }
}
