#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;
mod qbittorrent;
mod ui;

use std::{fs, path::PathBuf};

use ini::Ini;
use nwg::NativeUi;
use qbittorrent::*;

fn load_config(path: PathBuf) -> Vec<QbtHost> {
    let mut hosts = Vec::<QbtHost>::new();
    if !path.is_file() {
        // Load the config file
        let mut conf = Ini::new();
        conf.with_section(Some("Host1"))
            .set("username", "USERNAME")
            .set("password", "PASSWORD")
            .set("url", "WEB_API_URL");
        conf.write_to_file(&path)
            .expect("Failed to write default config");
    }
    let config: Ini = Ini::load_from_file(path).expect("Unable to read config");

    for (sec, prop) in &config {
        let name = match sec {
            None => continue,
            Some(x) => x.to_string(),
        };

        let host: QbtHost;
        let url = prop
            .get("url")
            .expect("Missing required property 'url'")
            .to_string();

        let username = prop.get("username").map(str::to_string);
        let password = prop.get("password").map(str::to_string);

        host = QbtHost::new(name, url, username, password);
        hosts.push(host);
    }

    return hosts;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load config from disk
    // let home_dir = home::home_dir().unwrap_or(PathBuf::from("."));
    const CONF_FILENAME: &str = "settings.ini";
    let conf_file_path = match home::home_dir() {
        Some(mut x) => {
            x.push("AppData");
            x.push("Roaming");
            x.push("qbt-controller");
            x.push(CONF_FILENAME);
            x
        }
        None => PathBuf::from(CONF_FILENAME),
    };

    // This will never panic, the case is handled above.
    let conf_parent_dir = conf_file_path.parent().unwrap();
    if !conf_parent_dir.is_dir() {
        fs::create_dir(conf_parent_dir).expect("Error creating default configuration");
    }

    let hosts = load_config(conf_file_path);

    let mut tray_info: ui::SystemTray = Default::default();
    tray_info.hosts = hosts;

    nwg::init().expect("Failed to init Native Windows GUI");
    let _ui = ui::SystemTray::build_ui(tray_info).expect("Failed to build UI");
    nwg::dispatch_thread_events();

    Ok(())
}
