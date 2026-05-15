use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::utils::colors::{Rgb, ThemePalette};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderFile {
    pub model: Option<String>,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeFile {
    pub error: Option<[u8; 3]>,
    pub success: Option<[u8; 3]>,
    pub warning: Option<[u8; 3]>,
    pub info: Option<[u8; 3]>,
    pub header: Option<[u8; 3]>,
    pub highlight: Option<[u8; 3]>,
    pub dim: Option<[u8; 3]>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConfigFile {
    #[serde(default)]
    pub provider: Option<String>,
    #[serde(default)]
    pub commit_format: Option<String>,
    #[serde(default)]
    pub theme: Option<String>,
    #[serde(default)]
    pub providers: HashMap<String, ProviderFile>,
    #[serde(default)]
    pub themes: HashMap<String, ThemeFile>,
}

impl ThemeFile {
    pub fn from_palette(palette: ThemePalette) -> Self {
        Self {
            error: Some(palette.error.to_array()),
            success: Some(palette.success.to_array()),
            warning: Some(palette.warning.to_array()),
            info: Some(palette.info.to_array()),
            header: Some(palette.header.to_array()),
            highlight: Some(palette.highlight.to_array()),
            dim: Some(palette.dim.to_array()),
        }
    }

    pub fn to_palette(&self, fallback: ThemePalette) -> ThemePalette {
        ThemePalette {
            error: self.error.map(Rgb::from_array).unwrap_or(fallback.error),
            success: self
                .success
                .map(Rgb::from_array)
                .unwrap_or(fallback.success),
            warning: self
                .warning
                .map(Rgb::from_array)
                .unwrap_or(fallback.warning),
            info: self.info.map(Rgb::from_array).unwrap_or(fallback.info),
            header: self.header.map(Rgb::from_array).unwrap_or(fallback.header),
            highlight: self
                .highlight
                .map(Rgb::from_array)
                .unwrap_or(fallback.highlight),
            dim: self.dim.map(Rgb::from_array).unwrap_or(fallback.dim),
        }
    }
}
