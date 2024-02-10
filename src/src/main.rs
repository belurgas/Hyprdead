use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use toml::Value;

use hyprland::data::Client;
use hyprland::dispatch::*;

use hyprland::prelude::*;

fn main() -> hyprland::Result<()> {
    // hyprland::dispatch!(Exec, "alacritty").expect("Idk");
    Dispatch::call(DispatchType::ToggleFullscreen(FullscreenType::Real))?;

    let win = Client::get_active().unwrap().unwrap();
    let config_path = Path::new("/home/beluga/.config/alacritty.toml");

    let mut config_toml = String::new();
    File::open(&config_path)
        .expect("failed to open config file")
        .read_to_string(&mut config_toml)
        .expect("Failed to read config file");

    let mut alacritty_config: Value = config_toml.parse().expect("Пизда, нихуя не работае");

    // Вносим изменения

    // записываем конфиг обратно на базу

    if win.fullscreen == true && win.initial_class == "Alacritty" {
        alacritty_config["window"]["opacity"] = Value::try_from(1.00).unwrap();

        let mut file = File::create(&config_path).expect("Failed to create config file");
        let toml_string = toml::to_string_pretty(&alacritty_config).expect("ебло сломаю");
        file.write_all(toml_string.as_bytes())
            .expect("ЭРОР ПИЗДА НАХУЙ")
    } else if win.fullscreen == false && win.initial_class == "Alacritty" {
        alacritty_config["window"]["opacity"] = Value::try_from(0.95).unwrap();

        let mut file = File::create(&config_path).expect("Failed to create config file");
        let toml_string = toml::to_string_pretty(&alacritty_config).expect("ебло сломаю");
        file.write_all(toml_string.as_bytes())
            .expect("ЭРОР ПИЗДА НАХУЙ")
    }
    Ok(())
}
