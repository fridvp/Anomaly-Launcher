use std::fs;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Renderer {
    DX8,
    DX9,
    DX10,
    DX11,
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ShadowMapSize {
    Size1536,
    Size2048,
    Size2560,
    Size3072,
    Size4096,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowMode {
    Default,
    Fullscreen,
    Windowed,
    BorderlessWindowed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub renderer: Renderer,
    pub use_avx: bool,
    pub shadow_map: ShadowMapSize,
    pub debug: bool,
    pub prefetch_sounds: bool,
    pub cpuaffinity: bool,
    pub sndfix: bool,
    pub window_mode: WindowMode,
    pub custom_args: bool,          // New field
    pub custom_args_text: String,   // New field
}

pub enum AppConfigError {
    ReadFailed,
    BadStructure,
    WriteFailed,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            renderer: Renderer::DX11,
            shadow_map: ShadowMapSize::Size1536,
            debug: false,
            use_avx: false,
            prefetch_sounds: false,
            cpuaffinity: true,
            sndfix: false,
            window_mode: WindowMode::Default,
            custom_args: false,      // Default to false
            custom_args_text: String::new(), // Empty string by default
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, AppConfigError> {
        if let Ok(file_data) = fs::read_to_string("launcherconfig.toml") {
            if let Ok(config) = toml::from_str::<AppConfig>(&file_data) {
                Ok(config)
            } else {
                Err(AppConfigError::BadStructure)
            }
        } else {
            Err(AppConfigError::ReadFailed)
        }
    }

    pub fn write(&self) -> Result<(), AppConfigError> {
        let string_config = toml::to_string(self).unwrap();
        if fs::write("launcherconfig.toml", string_config).is_err() {
            return Err(AppConfigError::WriteFailed);
        }
        Ok(())
    }
}
