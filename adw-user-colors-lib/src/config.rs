// SPDX-License-Identifier: MPL-2.0-only

use crate::{colors::ColorOverrides, NAME};
use adw::StyleManager;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::prelude::*, path::PathBuf};

/// Cosmic Theme config
#[derive(Default, Debug, Deserialize, Serialize)]
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
        Self { light, dark }
    }

    /// save the cosmic theme config
    pub fn save(&self) -> Result<()> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(NAME)?;
        if let Ok(path) = xdg_dirs.place_config_file(PathBuf::from(format!("{CONFIG_NAME}.toml"))) {
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
        let xdg_dirs = xdg::BaseDirectories::with_prefix(NAME)?;
        let path = xdg_dirs.get_config_home();
        std::fs::create_dir_all(&path)?;
        let path = xdg_dirs.find_config_file(PathBuf::from(format!("{CONFIG_NAME}.toml")));
        if path.is_none() {
            let s = Self::default();
            s.save()?;
        }
        if let Some(path) = xdg_dirs.find_config_file(PathBuf::from(format!("{CONFIG_NAME}.toml")))
        {
            let mut f = File::open(&path)?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(toml::from_str(s.as_str())?)
        } else {
            anyhow::bail!("Failed to load config")
        }
    }

    /// get the name of the active theme
    pub fn active_name(&self) -> Option<String> {
        if !adw::is_initialized() {
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

    pub fn set_active_light(new: &str) -> Result<()> {
        let mut self_ = Self::load()?;
        self_.light = new.to_string();
        Ok(self_.save()?)
    }

    pub fn set_active_dark(new: &str) -> Result<()> {
        let mut self_ = Self::load()?;
        self_.dark = new.to_string();
        Ok(self_.save()?)
    }
}

impl From<(ColorOverrides, ColorOverrides)> for Config {
    fn from((light, dark): (ColorOverrides, ColorOverrides)) -> Self {
        Self {
            light: light.name,
            dark: dark.name,
        }
    }
}

impl From<ColorOverrides> for Config {
    fn from(t: ColorOverrides) -> Self {
        Self {
            light: t.clone().name,
            dark: t.name,
        }
    }
}
