use crate::{config::Config, id::KeyboardId};
use chrono::{Local};
use jazztastic::{
    hidapi::HidApi,
    keyboards::{DynKeyboard, Keyboard, KeyboardFeatures, ak35i::Ak35i, ak820::Ak820, f75_max::F75Max},
    reports::{rgb::Rgb, time::TimeSync},
};
use notify::RecursiveMode;
use notify_debouncer_full::new_debouncer;
use std::{collections::HashMap, path::Path, sync::mpsc, time::Duration};

struct KeyboardEntry {
    device: Box<dyn DynKeyboard + Send>,
    current_config: Option<Rgb>,
}

pub struct KeyboardTask {
    api: HidApi,
    config: Config,
    keyboards: HashMap<KeyboardId, KeyboardEntry>,
    interruptor: mpsc::Receiver<bool>,
}

impl KeyboardTask {
    pub fn new(config: Config, api: HidApi) -> Self {
        let (tx, interruptor) = mpsc::channel();

        {
            let tx = tx.clone();
            std::thread::spawn(move || {
                if let Err(e) = interrupt_config(tx) {
                    log::error!("failed to watch config.toml: {e}");
                }
            });
        }

        std::thread::spawn(move || {
            if let Err(e) = interrupt_usb(tx) {
                log::error!("failed to watch USB devices: {e}");
            }
        });

        Self {
            api,
            config,
            keyboards: HashMap::new(),
            interruptor,
        }
    }

    pub fn run(&mut self) -> color_eyre::Result<()> {
        while let Ok(reload_config) = self.interruptor.recv() {
            if reload_config {
                let new_config =
                    match toml::from_str::<Config>(&std::fs::read_to_string("config.toml")?) {
                        Ok(cfg) => cfg,
                        Err(e) => {
                            log::error!(
                                "failed to parse config.toml: {e}, keeping existing configuration"
                            );
                            continue;
                        }
                    };

                self.config = new_config;
            }

            self.refresh_devices()?;
        }

        Ok(())
    }

    fn refresh_devices(&mut self) -> color_eyre::Result<()> {
        self.api.refresh_devices()?;

        let mut found_ids = Vec::with_capacity(self.keyboards.len());

        for device_info in self.api.device_list() {
            let id = KeyboardId::new(device_info);
            if self.keyboards.contains_key(&id) {
                found_ids.push(id);
                continue;
            }

            macro_rules! match_keyboards {
				(
					$device_info:expr,
					$( $kbd_type:ty ),* $(,)?
				) => {
					(match (
						$device_info.vendor_id(),
						$device_info.product_id(),
						$device_info.usage_page(),
					) {
						$(
							(<$kbd_type>::VENDOR_ID, <$kbd_type>::PRODUCT_ID, <$kbd_type>::USAGE_PAGE) => {
								let device = $device_info.open_device(&self.api)?;
								Some(Box::new(<$kbd_type>::new(device)) as Box<dyn DynKeyboard + Send>)
							}
						)*
						_ => None,
					})
				};
			}

            #[rustfmt::skip]
            let Some(mut keyboard) = match_keyboards![
                device_info, 
                Ak820, 
                Ak35i, 
                F75Max
            ] else {
                continue;
            };

            log::info!(
                "new keyboard connected: {} {} ('{}', id: {id})",
                keyboard.manufacturer(),
                keyboard.name(),
                device_info.product_string().unwrap_or("Unknown Keyboard"),
            );

            if keyboard.features().contains(KeyboardFeatures::TIME_SYNC) {
                keyboard.send_dyn(&TimeSync {
                    date_time: Local::now(),
                })?;
                log::info!("synced time with keyboard!");
            }

            self.keyboards.insert(
                id,
                KeyboardEntry {
                    device: keyboard,
                    current_config: None,
                },
            );
            found_ids.push(id);
        }

        let existing_ids: Vec<KeyboardId> = self.keyboards.keys().cloned().collect();
        for id in existing_ids {
            if !found_ids.contains(&id) {
                log::info!("disconnected keyboard with id: {:?}", id);
                self.keyboards.remove(&id);
            }
        }

        self.update_rgb()?;

        Ok(())
    }

    fn update_rgb(&mut self) -> color_eyre::Result<()> {
        for (id, keyboard) in self.keyboards.iter_mut().filter(|(_, k)| k.device.features().contains(KeyboardFeatures::RGB)) {
            if let Some(rgb) = self.config.keyboards.get(id) {
                if keyboard.current_config.as_ref() == Some(rgb) {
                    continue;
                }

                keyboard.current_config = Some(*rgb);

                keyboard.device.send_dyn(rgb)?;
            }
        }

        Ok(())
    }
}

fn interrupt_config(tx: mpsc::Sender<bool>) -> color_eyre::Result<()> {
    let (notify_tx, notify_rx) = std::sync::mpsc::channel();
    let mut watcher = new_debouncer(Duration::from_millis(50), None, notify_tx)?;

    watcher.watch(Path::new("config.toml"), RecursiveMode::NonRecursive)?;

    loop {
        match notify_rx.recv() {
            Ok(Ok(_)) => {
                log::info!("config.toml changed, updating keyboards");
                tx.send(true)?;
            }

            Err(e) => {
                log::error!("watch error: {:?}", e);
            }

            _ => { /* ignore other events */ }
        }
    }
}

// TODO: actually watch USB events instead of polling
fn interrupt_usb(tx: mpsc::Sender<bool>) -> color_eyre::Result<()> {
    loop {
        tx.send(false)?;
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
