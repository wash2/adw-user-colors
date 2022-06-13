// SPDX-License-Identifier: MPL-2.0-only

use std::{path::PathBuf, fs::File, io::BufReader};

pub mod colors;
pub mod config;

pub const NAME: &'static str = "adwaita-user-colors";
pub const THEME_DIR: &'static str = "color-overrides";

// load selected cosmic-theme
pub fn load() -> anyhow::Result<()> {
    adw::gtk::init()?;
    adw::init();
    let theme = config::Config::load()?;
    let active = theme.active_name();
    if active.is_none() {
        anyhow::bail!("no configured theme");
    }
    let active = active.unwrap();

    let css_path: PathBuf = [NAME, THEME_DIR].iter().collect();
    let css_dirs = xdg::BaseDirectories::with_prefix(css_path)?;
    dbg!(&active);
    let active_theme_path = if let Some(p) = css_dirs.find_data_file(format!("{active}.ron")) {
        p
    } else {
        anyhow::bail!("Failed to find theme");
    };
    dbg!(&active_theme_path);
    let active_theme_file = File::open(active_theme_path)?;
    let reader = BufReader::new(active_theme_file);
    let overrides: colors::ColorOverrides = ron::de::from_reader(reader)?;

    let mut user_color_css = String::new();
    if let Some(accent_bg_color) = overrides.accent_bg_color.as_ref() {
        user_color_css.push_str(&format!("@define-color accent_bg_color {};", &accent_bg_color));
    }
    if let Some(accent_fg_color) = overrides.accent_fg_color.as_ref() {
        user_color_css.push_str(&format!("@define-color accent_fg_color {};", &accent_fg_color));
    }
    if let Some(accent_color) = overrides.accent_color.as_ref() {
        user_color_css.push_str(&format!("@define-color accent_color {};", &accent_color));
    }

    if let Some(destructive_bg_color) = overrides.destructive_bg_color.as_ref() {
        user_color_css.push_str(&format!("@define-color destructive_bg_color {};", &destructive_bg_color));
    }
    if let Some(destructive_fg_color) = overrides.destructive_fg_color.as_ref() {
        user_color_css.push_str(&format!("@define-color destructive_fg_color {};", &destructive_fg_color));
    }
    if let Some(destructive_color) = overrides.destructive_color.as_ref() {
        user_color_css.push_str(&format!("@define-color destructive_color {};", &destructive_color));
    }

    if let Some(success_color) = overrides.success_color.as_ref() {
        user_color_css.push_str(&format!("@define-color success_color {};", &success_color));
    }
    if let Some(warning_color) = overrides.warning_color.as_ref() {
        user_color_css.push_str(&format!("@define-color warning_color {};", &warning_color));
    }
    if let Some(error_color) = overrides.error_color.as_ref() {
        user_color_css.push_str(&format!("@define-color error_color {};", &error_color));
    }

    if let Some(base_color) = overrides.base_color.as_ref() {
        user_color_css.push_str(&format!("@define-color base_color {};", &base_color));
    }
    if let Some(text_color) = overrides.text_color.as_ref() {
        user_color_css.push_str(&format!("@define-color text_color {};", &text_color));
    }

    
    if let Some(bg_color) = overrides.bg_color.as_ref() {
        user_color_css.push_str(&format!("@define-color bg_color {};", &bg_color));
    }
    if let Some(fg_color) = overrides.fg_color.as_ref() {
        user_color_css.push_str(&format!("@define-color fg_color {};", &fg_color));
    }
    if let Some(shade_color) = overrides.shade_color.as_ref() {
        user_color_css.push_str(&format!("@define-color shade_color {};", &shade_color));
    }

    if let Some(headerbar_bg_color) = overrides.headerbar_bg_color.as_ref() {
        user_color_css.push_str(&format!("@define-color headerbar_bg_color {};", &headerbar_bg_color));
    }
    if let Some(headerbar_fg_color) = overrides.headerbar_fg_color.as_ref() {
        user_color_css.push_str(&format!("@define-color headerbar_fg_color {};", &headerbar_fg_color));
    }
    if let Some(headerbar_border_color) = overrides.headerbar_border_color.as_ref() {
        user_color_css.push_str(&format!("@define-color headerbar_border_color {};", &headerbar_border_color));
    }
    if let Some(headerbar_backdrop_color) = overrides.headerbar_backdrop_color.as_ref() {
        user_color_css.push_str(&format!("@define-color headerbar_backdrop_color {};", &headerbar_backdrop_color));
    }
    if let Some(headerbar_shade_color) = overrides.headerbar_shade_color.as_ref() {
        user_color_css.push_str(&format!("@define-color headerbar_shade_color {};", &headerbar_shade_color));
    }

    if let Some(card_bg_color) = overrides.card_bg_color.as_ref() {
        user_color_css.push_str(&format!("@define-color card_bg_color {};", &card_bg_color));
    }
    if let Some(card_fg_color) = overrides.card_fg_color.as_ref() {
        user_color_css.push_str(&format!("@define-color card_fg_color {};", &card_fg_color));
    }
    if let Some(card_border_color) = overrides.card_border_color.as_ref() {
        user_color_css.push_str(&format!("@define-color card_border_color {};", &card_border_color));
    }

    if let Some(popover_bg_color) = overrides.popover_bg_color.as_ref() {
        user_color_css.push_str(&format!("@define-color popover_bg_color {};", &popover_bg_color));
    }
    if let Some(popover_fg_color) = overrides.popover_fg_color.as_ref() {
        user_color_css.push_str(&format!("@define-color popover_fg_color {};", &popover_fg_color));
    }


    if let Some(scrollbar_outline_color) = overrides.scrollbar_outline_color.as_ref() {
        user_color_css.push_str(&format!("@define-color scrollbar_outline_color {};", &scrollbar_outline_color));
    }
    if let Some(window_outline_color) = overrides.window_outline_color.as_ref() {
        user_color_css.push_str(&format!("@define-color window_outline_color {};", &window_outline_color));
    }
    if let Some(window_border_color) = overrides.window_border_color.as_ref() {
        user_color_css.push_str(&format!("@define-color window_border_color {};", &window_border_color));
    }
    if let Some(window_border_backdrop_color) = overrides.window_border_backdrop_color.as_ref() {
        user_color_css.push_str(&format!("@define-color window_border_backdrop_color {};", &window_border_backdrop_color));
    }

    user_color_css.push_str(&format!("\n@import url(\"custom.css\");\n"));

    let xdg_dirs = xdg::BaseDirectories::with_prefix("gtk-4.0")?;
    let path = xdg_dirs.place_config_file(PathBuf::from("gtk.css"))?;
    dbg!(&path);
    std::fs::write(path, &user_color_css)?;

    println!("{user_color_css}");

    Ok(())
}

pub fn unload() -> anyhow::Result<()> {
    Ok(())
}