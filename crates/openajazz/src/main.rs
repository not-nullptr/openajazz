use crate::collection::KeyboardTask;
use jazztastic::hidapi::HidApi;

mod collection;
mod config;
mod id;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    env_logger::init_from_env(
        env_logger::Env::default().default_filter_or("bin=debug,openajazz=debug,jazztastic=debug"),
    );

    let config = toml::from_str(&std::fs::read_to_string("config.toml")?)?;

    let api = HidApi::new()?;
    let mut task = KeyboardTask::new(config, api);
    task.run()?;

    Ok(())
}
