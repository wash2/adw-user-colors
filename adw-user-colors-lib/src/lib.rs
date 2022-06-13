// SPDX-License-Identifier: MPL-2.0-only

use std::{path::PathBuf, fs::File, io::BufReader};

pub mod colors;
pub mod config;

pub const NAME: &'static str = "adwaita-user-colors";
// load selected cosmic-theme
pub fn load() -> anyhow::Result<()> {
    let theme = cosmic_theme::Config::load()?;
    let active = theme.active_name();

    let css_path: PathBuf = ["cosmic-theme", "themes"].iter().collect();
    let css_dirs = xdg::BaseDirectories::with_prefix(css_path)?;
    let active_theme_path = if let Some(p) = css_dirs.find_data_file(format!("{active}.ron")) {
        p
    } else {
        anyhow::bail!("Failed to find theme");
    };
    dbg!(&active_theme_path);
    let active_theme_file = File::open(active_theme_path)?;
    let reader = BufReader::new(active_theme_file);
    let theme: colors::ColorOverrides = ron::de::from_reader(reader)?;
    
    let accent = theme.accent_bg_color.unwrap();
    let destructive = theme.destructive_bg_color.unwrap();
    // TODO optionally overwrite all the colors depending on the configuration.
    let user_color_css = format!(r#"
@define-color accent_color {accent};
@define-color accent_bg_color {accent};
@define-color destructive_color {destructive};
@define-color destructive_bg_color {destructive};

@import url("custom.css");
"#);
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