use std::path::{Path, PathBuf};

use failure::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::manifest::ManifestReferenceInfo;

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct ThemeConfig {
    pub theme: ThemeInfo,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub manifests: Vec<ManifestReferenceInfo>,
}

impl ThemeConfig {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let string = std::fs::read_to_string(path)?;
        let config = toml::from_str(&string)?;

        Ok(config)
    }

    pub fn from_json(json: Value) -> Result<Self, Error> {
        let config = serde_json::from_value(json)?;

        Ok(config)
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            theme: ThemeInfo::default(),
            manifests: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct ThemeInfo {
    pub name: String,
    pub label: String,
    pub description: String,
}

impl Default for ThemeInfo {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            label: "Default".to_string(),
            description: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ThemeReferenceInfo {
    pub name: Option<String>,
    pub path: PathBuf,
}