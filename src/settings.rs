use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use crate::hotkey_widget::HotKey;
use hudhook::imgui::Key;
use crate::state::{State, StateCacheType};

#[derive(Clone, Deserialize, Serialize)]
pub struct AppSettings {
    pub AIM_KEY: Option<HotKey>,
    pub ESP_KEY: Option<HotKey>,
    pub INF_NADE: Option<HotKey>,
    pub NO_RELOAD: Option<HotKey>,
    pub INVUL: Option<HotKey>,
    pub INF_AMMO: Option<HotKey>,
    pub NO_RECOIL: Option<HotKey>,
    pub RAPID_FIRE: Option<HotKey>,
    pub AIMBOT: Option<HotKey>,
    pub AIM_DRAW_FOV: Option<HotKey>,
    pub AIM_SMOOTH: Option<HotKey>,
    pub TRIGGER_BOT: Option<HotKey>,
    pub MAPHACK: Option<HotKey>,
    pub FULLBRIGHT: Option<HotKey>
}

// Implement the Default trait for AppSettings
impl Default for AppSettings {
    fn default() -> Self {
        Self {
            AIM_KEY: Some(HotKey { key: Key::C }), // Default to Key::C
            ESP_KEY: Some(HotKey { key: Key::Delete}),
            INF_NADE: Some(HotKey { key: Key::F1}),
            NO_RELOAD: Some(HotKey { key: Key::F2}),
            INVUL: Some(HotKey {key: Key::F3}),
            INF_AMMO: Some(HotKey {key: Key::F4}),
            NO_RECOIL: Some(HotKey { key: Key::F5}),
            RAPID_FIRE: Some(HotKey { key: Key::F6}),
            AIMBOT: Some(HotKey { key: Key::F7 }),
            AIM_DRAW_FOV: Some(HotKey { key: Key::F8 }),
            AIM_SMOOTH:  Some(HotKey { key: Key::F9 }),
            TRIGGER_BOT: Some(HotKey { key: Key::F10}),
            MAPHACK: Some(HotKey { key: Key::F11}),
            FULLBRIGHT: Some(HotKey { key: Key::F12})
        }
    }
}


impl State for AppSettings {
    type Parameter = ();

    fn cache_type() -> StateCacheType {
        StateCacheType::Persistent
    }
}

pub fn get_settings_path() -> anyhow::Result<PathBuf> {
    let exe_file = std::env::current_exe().context("missing current exe path")?;
    let base_dir = exe_file.parent().context("could not get exe directory")?;

    Ok(base_dir.join("config.yaml"))
}

pub fn load_app_settings() -> anyhow::Result<AppSettings> {
    let config_path = get_settings_path()?;
    if !config_path.is_file() {
        log::info!(
            "App config file {} does not exist.",
            config_path.to_string_lossy()
        );
        log::info!("Using default config.");
        return Ok(AppSettings::default());
    }

    let config = File::open(&config_path).with_context(|| {
        format!(
            "failed to open app config at {}",
            config_path.to_string_lossy()
        )
    })?;
    let mut config = BufReader::new(config);

    let config: AppSettings =
        serde_yaml::from_reader(&mut config).context("failed to parse app config")?;

    log::info!("Loaded app config from {}", config_path.to_string_lossy());
    Ok(config)
}

pub fn save_app_settings(settings: &AppSettings) -> anyhow::Result<()> {
    let config_path = get_settings_path()?;
    let config = File::options()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&config_path)
        .with_context(|| {
            format!(
                "failed to open app config at {}",
                config_path.to_string_lossy()
            )
        })?;
    let mut config = BufWriter::new(config);

    serde_yaml::to_writer(&mut config, settings).context("failed to serialize config")?;

    log::debug!("Saved app config.");
    Ok(())
}