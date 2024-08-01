use crate::hotkey_widget::HotKey;
use crate::state::{State, StateCacheType};
use anyhow::Context;
use hudhook::imgui::Key;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

#[derive(Clone, Deserialize, Serialize)]
pub struct AppSettings {
    pub aim_key: Option<HotKey>,
    pub wallhack_key: Option<HotKey>,
    pub esp_key: Option<HotKey>,
    pub inf_nade: Option<HotKey>,
    pub no_reload: Option<HotKey>,
    pub invul: Option<HotKey>,
    pub inf_ammo: Option<HotKey>,
    pub no_recoil: Option<HotKey>,
    pub rapid_fire: Option<HotKey>,
    pub aimbot: Option<HotKey>,
    pub aim_draw_fov: Option<HotKey>,
    pub aim_smooth: Option<HotKey>,
    pub trigger_bot: Option<HotKey>,
    pub maphack: Option<HotKey>,
    pub fullbright: Option<HotKey>,
    pub theme_id: i32,
    pub language_id: i32
}

impl AppSettings {
    // Method to iterate over keys in AppSettings
    pub fn keys(&self) -> HashSet<&Key> {
        let mut keys = HashSet::new();

        if let Some(hotkey) = &self.aim_key {
            keys.insert(&hotkey.key);
        }
        if let Some(hotkey) = &self.wallhack_key {
            keys.insert(&hotkey.key);
        }
        if let Some(hotkey) = &self.esp_key {
            keys.insert(&hotkey.key);
        }
        if let Some(hotkey) = &self.inf_nade {
            keys.insert(&hotkey.key);
        }
        if let Some(hotkey) = &self.no_reload {
            keys.insert(&hotkey.key);
        }
        if let Some(hotkey) = &self.invul {
            keys.insert(&hotkey.key);
        }
        if let Some(hotkey) = &self.inf_ammo {
            keys.insert(&hotkey.key);
        }
        if let Some(hotkey) = &self.no_recoil {
            keys.insert(&hotkey.key);
        }
        if let Some(hotkey) = &self.rapid_fire {
            keys.insert(&hotkey.key);
        }
        if let Some(hotkey) = &self.aimbot {
            keys.insert(&hotkey.key);
        }
        if let Some(hotkey) = &self.aim_draw_fov {
            keys.insert(&hotkey.key);
        }
        if let Some(hotkey) = &self.aim_smooth {
            keys.insert(&hotkey.key);
        }
        if let Some(hotkey) = &self.trigger_bot {
            keys.insert(&hotkey.key);
        }
        if let Some(hotkey) = &self.maphack {
            keys.insert(&hotkey.key);
        }
        if let Some(hotkey) = &self.fullbright {
            keys.insert(&hotkey.key);
        }

        keys
    }

    // Method to check if a specific key is in AppSettings
    pub fn has_key(&self, key: &Key) -> bool {
        self.keys().contains(key)
    }
}

// Implement the Default trait for AppSettings
impl Default for AppSettings {
    fn default() -> Self {
        Self {
            aim_key: Some(HotKey { key: Key::C }), // Default to Key::C
            wallhack_key: Some(HotKey { key: Key::Home }),
            esp_key: Some(HotKey { key: Key::Delete }),
            inf_nade: Some(HotKey { key: Key::F1 }),
            no_reload: Some(HotKey { key: Key::F2 }),
            invul: Some(HotKey { key: Key::F3 }),
            inf_ammo: Some(HotKey { key: Key::F4 }),
            no_recoil: Some(HotKey { key: Key::F5 }),
            rapid_fire: Some(HotKey { key: Key::F6 }),
            aimbot: Some(HotKey { key: Key::F7 }),
            aim_draw_fov: Some(HotKey { key: Key::F8 }),
            aim_smooth: Some(HotKey { key: Key::F9 }),
            trigger_bot: Some(HotKey { key: Key::F10 }),
            maphack: Some(HotKey { key: Key::F11 }),
            fullbright: Some(HotKey { key: Key::F12 }),
            theme_id: 0i32,
            language_id: 0i32
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
        serde_yml::from_reader(&mut config).context("failed to parse app config")?;

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

    serde_yml::to_writer(&mut config, settings).context("failed to serialize config")?;

    log::debug!("Saved app config.");
    Ok(())
}
