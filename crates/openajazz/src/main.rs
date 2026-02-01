use crate::{collection::KeyboardTask, config::Config};
use jazztastic::hidapi::HidApi;

mod collection;
mod config;
mod id;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    env_logger::init_from_env(
        env_logger::Env::default().default_filter_or("bin=debug,openajazz=debug,jazztastic=debug"),
    );

	let config = match std::fs::read_to_string("config.toml") {
		Ok(cfg) => cfg,
		Err(e) => {
			log::error!("failed to read config.toml: {e}");
			return Ok(());
		}
	};

    let config: Config = toml::from_str(&config)?;

    let api = HidApi::new()?;

	if config.debug.log_usb_devices {
		for device in api.device_list() {
			log::debug!(
				"Discovered '{} {}' (VID: 0x{:04x}, PID: 0x{:04x}, Usage Page: 0x{:04x})",
				device.manufacturer_string().unwrap_or("Unknown Manufacturer"),
				device.product_string().unwrap_or("Unknown Product"),
				device.vendor_id(),
				device.product_id(),
				device.usage_page()
			);
		}
	}

	log::info!("starting openajazz...");

    let mut task = KeyboardTask::new(config, api);
    task.run()?;

    Ok(())
}
