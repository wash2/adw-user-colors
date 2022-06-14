// SPDX-License-Identifier: MPL-2.0-only

use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

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
    pub success_bg_color: Option<String>,
    pub success_fg_color: Option<String>,

    pub warning_color: Option<String>,
    pub warning_bg_color: Option<String>,
    pub warning_fg_color: Option<String>,

    pub error_color: Option<String>,
    pub error_bg_color: Option<String>,
    pub error_fg_color: Option<String>,

    // Main window background
    pub window_bg_color: Option<String>,
    pub window_fg_color: Option<String>,

    // Content areas, e.g. text views
    pub view_bg_color: Option<String>,
    pub view_fg_color: Option<String>,

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
    pub shade_color: Option<String>,
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

            "success_color" => self.success_color = value,
            "success_bg_color" => self.success_color = value,
            "success_fg_color" => self.success_color = value,
            
            "warning_color" => self.warning_color = value,
            "warning_bg_color" => self.warning_color = value,
            "warning_fg_color" => self.warning_color = value,

            "error_color" => self.error_color = value,
            "error_bg_color" => self.error_color = value,
            "error_fg_color" => self.error_color = value,

            // Content areas, e.g. text views
            "view_bg_color" => self.view_bg_color = value,
            "view_fg_color" => self.view_fg_color = value,

            // Main window background
            "window_bg_color" => self.window_bg_color = value,
            "window_fg_color" => self.window_fg_color = value,

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
            "shade_color" => self.shade_color = value,
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

            "success_color" => self.success_color.clone(),
            "success_bg_color" => self.success_color.clone(),
            "success_fg_color" => self.success_color.clone(),
            
            "warning_color" => self.warning_color.clone(),
            "warning_bg_color" => self.warning_color.clone(),
            "warning_fg_color" => self.warning_color.clone(),

            "error_color" => self.error_color.clone(),
            "error_bg_color" => self.error_color.clone(),
            "error_fg_color" => self.error_color.clone(),

            // Content areas.clone(), e.g. text views
            "view_bg_color" => self.view_bg_color.clone(),
            "view_fg_color" => self.view_fg_color.clone(),

            // Main window background
            "window_bg_color" => self.window_bg_color.clone(),
            "window_fg_color" => self.window_fg_color.clone(),

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
            "shade_color" => self.shade_color.clone(),
            _ => None,
        }
    }

    pub fn as_css(&self) -> String {
        let mut user_color_css = String::new();
        if let Some(accent_bg_color) = self.accent_bg_color.as_ref() {
            user_color_css.push_str(&format!(
                "@define-color accent_bg_color {};\n",
                &accent_bg_color
            ));
        }
        if let Some(accent_fg_color) = self.accent_fg_color.as_ref() {
            user_color_css.push_str(&format!(
                "@define-color accent_fg_color {};\n",
                &accent_fg_color
            ));
        }
        if let Some(accent_color) = self.accent_color.as_ref() {
            user_color_css.push_str(&format!("@define-color accent_color {};\n", &accent_color));
        }

        if let Some(destructive_bg_color) = self.destructive_bg_color.as_ref() {
            user_color_css.push_str(&format!(
                "@define-color destructive_bg_color {};\n",
                &destructive_bg_color
            ));
        }
        if let Some(destructive_fg_color) = self.destructive_fg_color.as_ref() {
            user_color_css.push_str(&format!(
                "@define-color destructive_fg_color {};\n",
                &destructive_fg_color
            ));
        }
        if let Some(destructive_color) = self.destructive_color.as_ref() {
            user_color_css.push_str(&format!(
                "@define-color destructive_color {};\n",
                &destructive_color
            ));
        }

        if let Some(success_color) = self.success_color.as_ref() {
            user_color_css.push_str(&format!(
                "@define-color success_color {};\n",
                &success_color
            ));
        }
        if let Some(success_bg_color) = self.success_bg_color.as_ref() {
            user_color_css.push_str(&format!(
                "@define-color success_bg_color {};\n",
                &success_bg_color
            ));
        }
        if let Some(success_fg_color) = self.success_fg_color.as_ref() {
            user_color_css.push_str(&format!(
                "@define-color success_fg_color {};\n",
                &success_fg_color
            ));
        }
        if let Some(warning_color) = self.warning_color.as_ref() {
            user_color_css.push_str(&format!(
                "@define-color warning_color {};\n",
                &warning_color
            ));
        }
        if let Some(warning_bg_color) = self.warning_bg_color.as_ref() {
            user_color_css.push_str(&format!(
                "@define-color warning_bg_color {};\n",
                &warning_bg_color
            ));
        }
        if let Some(warning_fg_color) = self.warning_fg_color.as_ref() {
            user_color_css.push_str(&format!(
                "@define-color warning_fg_color {};\n",
                &warning_fg_color
            ));
        }
        if let Some(error_color) = self.error_color.as_ref() {
            user_color_css.push_str(&format!("@define-color error_color {};\n", &error_color));
        }
        if let Some(error_bg_color) = self.error_bg_color.as_ref() {
            user_color_css.push_str(&format!("@define-color error_bg_color {};\n", &error_bg_color));
        }
        if let Some(error_fg_color) = self.error_fg_color.as_ref() {
            user_color_css.push_str(&format!("@define-color error_fg_color {};\n", &error_fg_color));
        }

        if let Some(window_bg_color) = self.window_bg_color.as_ref() {
            user_color_css.push_str(&format!("@define-color window_bg_color {};\n", &window_bg_color));
        }
        if let Some(window_fg_color) = self.window_fg_color.as_ref() {
            user_color_css.push_str(&format!("@define-color window_fg_color {};\n", &window_fg_color));
        }

        if let Some(view_bg_color) = self.view_bg_color.as_ref() {
            user_color_css.push_str(&format!("@define-color view_bg_color {};\n", &view_bg_color));
        }
        if let Some(view_fg_color) = self.view_fg_color.as_ref() {
            user_color_css.push_str(&format!("@define-color view_fg_color {};\n", &view_fg_color));
        }
        if let Some(shade_color) = self.shade_color.as_ref() {
            user_color_css.push_str(&format!("@define-color shade_color {};\n", &shade_color));
        }

        if let Some(headerbar_bg_color) = self.headerbar_bg_color.as_ref() {
            user_color_css.push_str(&format!(
                "@define-color headerbar_bg_color {};\n",
                &headerbar_bg_color
            ));
        }
        if let Some(headerbar_fg_color) = self.headerbar_fg_color.as_ref() {
            user_color_css.push_str(&format!(
                "@define-color headerbar_fg_color {};\n",
                &headerbar_fg_color
            ));
        }
        if let Some(headerbar_border_color) = self.headerbar_border_color.as_ref() {
            user_color_css.push_str(&format!(
                "@define-color headerbar_border_color {};\n",
                &headerbar_border_color
            ));
        }
        if let Some(headerbar_backdrop_color) = self.headerbar_backdrop_color.as_ref() {
            user_color_css.push_str(&format!(
                "@define-color headerbar_backdrop_color {};\n",
                &headerbar_backdrop_color
            ));
        }
        if let Some(headerbar_shade_color) = self.headerbar_shade_color.as_ref() {
            user_color_css.push_str(&format!(
                "@define-color headerbar_shade_color {};\n",
                &headerbar_shade_color
            ));
        }

        if let Some(card_bg_color) = self.card_bg_color.as_ref() {
            user_color_css.push_str(&format!(
                "@define-color card_bg_color {};\n",
                &card_bg_color
            ));
        }
        if let Some(card_fg_color) = self.card_fg_color.as_ref() {
            user_color_css.push_str(&format!(
                "@define-color card_fg_color {};\n",
                &card_fg_color
            ));
        }
        if let Some(card_border_color) = self.card_border_color.as_ref() {
            user_color_css.push_str(&format!(
                "@define-color card_border_color {};\n",
                &card_border_color
            ));
        }

        if let Some(popover_bg_color) = self.popover_bg_color.as_ref() {
            user_color_css.push_str(&format!(
                "@define-color popover_bg_color {};\n",
                &popover_bg_color
            ));
        }
        if let Some(popover_fg_color) = self.popover_fg_color.as_ref() {
            user_color_css.push_str(&format!(
                "@define-color popover_fg_color {};\n",
                &popover_fg_color
            ));
        }

        if let Some(scrollbar_outline_color) = self.scrollbar_outline_color.as_ref() {
            user_color_css.push_str(&format!(
                "@define-color scrollbar_outline_color {};\n",
                &scrollbar_outline_color
            ));
        }
        if let Some(window_outline_color) = self.window_outline_color.as_ref() {
            user_color_css.push_str(&format!(
                "@define-color window_outline_color {};\n",
                &window_outline_color
            ));
        }
        user_color_css
    }
}
