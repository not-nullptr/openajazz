# openajazz

a small service to enable RGB effects on AJAZZ keyboards in a cross-platform manner.

## installation

[install](https://rustup.rs/) the Rust toolchain, and then run `cargo build --release openajazz`. the binary will be located in `target/release` as either `openajazz.exe` or just `openajazz`, depending on your platform.

you can use task scheduler, systemctl or your event scheduler of choice to run at login/startup, it only matters that the program has permission to write to usb ports + access `config.toml` (which is expected to be next to the program).

## config

check [config.example.toml](config.example.toml). tl;dr there is a `keyboards` map, where the key is your keyboard's ID. you can find your keyboard's ID by running `openajazz` and then taking note of the ID logged by the program (it'll be a very long number). you can configure the colour, effect, speed, brightness and direction of the keyboard's RGB.

## support

- AJAZZ AK820
- AJAZZ AK35i (RGB only)