mod app;
mod manager;

use ajazz::hidapi::HidApi;

use crate::app::App;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    env_logger::init_from_env(
        env_logger::Env::default().default_filter_or("bin=debug,ajazz=debug"),
    );

    iced::application(App::new, App::update, App::view)
        .subscription(App::subscription)
        .run()?;

    Ok(())
}
