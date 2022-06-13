// SPDX-License-Identifier: MPL-2.0-only

use crate::{NAME, colors::ColorOverrides};
use adw::StyleManager;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::prelude::*, path::PathBuf};

/// Cosmic Theme config
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// Selected light theme name
    pub light: String,
    /// Selected dark theme name
    pub dark: String,
}

pub const CONFIG_NAME: &'static str = "config.ron";

impl Config {
    /// create a new cosmic theme config
    pub fn new(light: String, dark: String) -> Self {
        Self {
            light,
            dark,
        }
    }

    /// save the cosmic theme config
    pub fn save(&self) -> Result<()> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(NAME)?;
        if let Ok(path) = xdg_dirs.place_config_file(PathBuf::from(CONFIG_NAME)) {
            let mut f = File::create(path)?;
            let toml = toml::ser::to_string_pretty(&self)?;
            f.write_all(toml.as_bytes())?;
            Ok(())
        } else {
            bail!("failed to save theme config")
        }
    }

    /// load the cosmic theme config
    pub fn load() -> Result<Self> {
        let p = Self::config_path()?;
        let mut f = File::open(p)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(toml::from_str(s.as_str())?)
    }

    /// get the path of the cosmic theme config
    pub fn config_path() -> Result<PathBuf> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(NAME)?;
        if let Some(path) = xdg_dirs.find_config_file(PathBuf::from(CONFIG_NAME)) {
            Ok(path)
        } else {
            dbg!(xdg_dirs.get_config_home());
            dbg!(xdg_dirs.get_config_dirs());
            bail!("no theme config");
        }
    }

    /// get the name of the active theme
    pub fn active_name(&self) -> Option<String> {
        if adw::is_initialized() {
            None
        } else {
            let manager = StyleManager::default();
            if manager.is_dark() {
                Some(self.dark.clone())
            } else {
                Some(self.light.clone())
            }
        }
    }
}

impl From<(ColorOverrides, ColorOverrides)> for Config
{
    fn from((light, dark): (ColorOverrides, ColorOverrides)) -> Self {
        Self {
            light: light.name,
            dark: dark.name,
        }
    }
}

impl From<ColorOverrides> for Config
{
    fn from(t: ColorOverrides) -> Self {
        Self {
            light: t.clone().name,
            dark: t.name,
        }
    }
}
