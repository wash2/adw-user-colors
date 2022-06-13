// SPDX-License-Identifier: MPL-2.0-only

use std::{path::{PathBuf, Path}, fs::File, io::Write};

use serde::{Deserialize, Serialize};

use crate::{NAME, THEME_DIR};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct ColorOverrides {
    /// name
    pub name: String,
    pub accent_bg_color: Option<String>,
    pub accent_fg_color: Option<String>,
    pub accent_color: Option<String>,
    
    // destructive-action buttons
    pub destructive_bg_color: Option<String>,
    pub destructive_fg_color: Option<String>,
    pub destructive_color: Option<String>,
    
    // Levelbars, entries, labels and infobars. These don't need text colors
    pub success_color: Option<String>,
    pub warning_color: Option<String>,
    pub error_color: Option<String>,
    
    // Content areas, e.g. text views
    pub base_color: Option<String>,
    pub text_color: Option<String>,
    
    // Main window background
    pub bg_color: Option<String>,
    pub fg_color: Option<String>,
    pub shade_color: Option<String>,
    
    // Header bar, search bar, tab bar
    pub headerbar_bg_color: Option<String>,
    pub headerbar_fg_color: Option<String>,
    pub headerbar_border_color: Option<String>,
    pub headerbar_backdrop_color: Option<String>,
    pub headerbar_shade_color: Option<String>,
    
    // Cards, boxed lists
    pub card_bg_color: Option<String>,
    pub card_fg_color: Option<String>,
    pub card_border_color: Option<String>,
    
    // Popovers
    pub popover_bg_color: Option<String>,
    pub popover_fg_color: Option<String>,
    
    // Miscellaneous
    pub scrollbar_outline_color: Option<String>,
    pub window_outline_color: Option<String>,
    pub window_border_color: Option<String>,
    pub window_border_backdrop_color: Option<String>,
}

impl ColorOverrides {
    pub fn save(&self) -> anyhow::Result<()> {
        let ron_path: PathBuf = [NAME, THEME_DIR].iter().collect();
        let ron_dirs = xdg::BaseDirectories::with_prefix(ron_path)?;
        let ron_name = format!("{}.ron", &self.name);

        if let Ok(p) = ron_dirs.place_data_file(ron_name) {
            let mut f = File::create(p)?;
            f.write_all(ron::ser::to_string(self)?.as_bytes())?;
        } else {
            anyhow::bail!("Failed to write RON theme.");
        }
        Ok(())
    }

    pub fn load_from_name(name: &str) -> anyhow::Result<Self> {
        let ron_path: PathBuf = [NAME, THEME_DIR].iter().collect();
        let ron_dirs = xdg::BaseDirectories::with_prefix(ron_path)?;
        let ron_name = format!("{}.ron", name);
        if let Some(p) = ron_dirs.find_data_file(ron_name) {
            let f = File::open(p)?;
            Ok(ron::de::from_reader(f)?)
        } else {
            anyhow::bail!("Failed to write RON theme.");
        }
    }

    pub fn load(p: &dyn AsRef<Path>) -> anyhow::Result<Self> {
        let f = File::open(p)?;
        Ok(ron::de::from_reader(f)?)
    }

    pub fn set_key(&mut self, key: &str, value: Option<String>) -> anyhow::Result<()> {
        match key {
            "accent_bg_color" => self.accent_bg_color = value,
            "accent_fg_color" => self.accent_fg_color = value,
            "accent_color" => self.accent_color = value,
            
            // destructive-action buttons
            "destructive_bg_color" => self.destructive_bg_color = value,
            "destructive_fg_color" => self.destructive_fg_color = value,

            "destructive_color" => self.destructive_color = value,
            
            // Levelbars, entries, labels and infobars. These don't need text colors
            "success_color" => self.success_color = value,
            "warning_color" => self.warning_color = value,
            "error_color" => self.error_color = value,
            
            // Content areas, e.g. text views
            "base_color" => self.base_color = value,
            "text_color" => self.text_color = value,
            
            // Main window background
            "bg_color" => self.bg_color = value,
            "fg_color" => self.fg_color = value,
            "shade_color" => self.shade_color = value,
            
            // Header bar, search bar, tab bar
            "headerbar_bg_color" => self.headerbar_bg_color = value,
            "headerbar_fg_color" => self.headerbar_fg_color = value,
            "headerbar_border_color" => self.headerbar_border_color = value,
            "headerbar_backdrop_color" => self.headerbar_backdrop_color = value,
            "headerbar_shade_color" => self.headerbar_shade_color = value,
            
            // Cards, boxed lists
            "card_bg_color" => self.card_bg_color = value,
            "card_fg_color" => self.card_fg_color = value,
            "card_border_color" => self.card_border_color = value,
            
            // Popovers
            "popover_bg_color" => self.popover_bg_color = value,
            "popover_fg_color" => self.popover_fg_color = value,
            
            // Miscellaneous
            "scrollbar_outline_color" => self.scrollbar_outline_color = value,
            "window_outline_color" => self.window_outline_color = value,
            "window_border_color" => self.window_border_color = value,
            "window_border_backdrop_color" => self.window_border_backdrop_color = value,
            _ => anyhow::bail!("Invalid key"),
        }
        Ok(())
    }

    
    pub fn get_key(&self, key: &str) -> Option<String> {
        match key {
            "accent_bg_color" => self.accent_bg_color.clone(),
            "accent_fg_color" => self.accent_fg_color.clone(),
            "accent_color" => self.accent_color.clone(),
            
            // destructive-action buttons
            "destructive_bg_color" => self.destructive_bg_color.clone(),
            "destructive_fg_color" => self.destructive_fg_color.clone(),

            "destructive_color" => self.destructive_color.clone(),
            
            // Levelbars.clone(), entries.clone(), labels and infobars. These don't need text colors
            "success_color" => self.success_color.clone(),
            "warning_color" => self.warning_color.clone(),
            "error_color" => self.error_color.clone(),
            
            // Content areas.clone(), e.g. text views
            "base_color" => self.base_color.clone(),
            "text_color" => self.text_color.clone(),
            
            // Main window background
            "bg_color" => self.bg_color.clone(),
            "fg_color" => self.fg_color.clone(),
            "shade_color" => self.shade_color.clone(),
            
            // Header bar.clone(), search bar.clone(), tab bar
            "headerbar_bg_color" => self.headerbar_bg_color.clone(),
            "headerbar_fg_color" => self.headerbar_fg_color.clone(),
            "headerbar_border_color" => self.headerbar_border_color.clone(),
            "headerbar_backdrop_color" => self.headerbar_backdrop_color.clone(),
            "headerbar_shade_color" => self.headerbar_shade_color.clone(),
            
            // Cards.clone(), boxed lists
            "card_bg_color" => self.card_bg_color.clone(),
            "card_fg_color" => self.card_fg_color.clone(),
            "card_border_color" => self.card_border_color.clone(),
            
            // Popovers
            "popover_bg_color" => self.popover_bg_color.clone(),
            "popover_fg_color" => self.popover_fg_color.clone(),
            
            // Miscellaneous
            "scrollbar_outline_color" => self.scrollbar_outline_color.clone(),
            "window_outline_color" => self.window_outline_color.clone(),
            "window_border_color" => self.window_border_color.clone(),
            "window_border_backdrop_color" => self.window_border_backdrop_color.clone(),
            _ => None,
        }
    }
}