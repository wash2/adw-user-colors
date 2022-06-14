// SPDX-License-Identifier: MPL-2.0-only

use std::{fs::File, io::BufReader, path::PathBuf};

use adw::{
    glib::{MainContext, MainLoop, Priority},
    StyleManager,
};

pub mod colors;
pub mod config;

pub const NAME: &'static str = "adwaita-user-colors";
pub const THEME_DIR: &'static str = "color-overrides";
use crate::config::CONFIG_NAME;
use futures::{channel::mpsc::channel, SinkExt, StreamExt};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

enum Event {
    UpdateColors,
}

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

    let active_theme_path = if let Some(p) = css_dirs.find_data_file(format!("{active}.ron")) {
        p
    } else {
        anyhow::bail!("Failed to find theme");
    };

    let active_theme_file = File::open(active_theme_path)?;
    let reader = BufReader::new(active_theme_file);
    let overrides: colors::ColorOverrides = ron::de::from_reader(reader)?;

    let mut user_color_css = String::new();
    user_color_css.push_str(&overrides.as_css());
    user_color_css.push_str(&format!("\n@import url(\"custom.css\");\n"));

    let xdg_dirs = xdg::BaseDirectories::with_prefix("gtk-4.0")?;
    let mut path = xdg_dirs.place_config_file(PathBuf::from("gtk.css"))?;

    std::fs::write(&path, &user_color_css)?;

    // FIXME
    let main_context = MainContext::default();
    let (tx, rx) = MainContext::channel(Priority::default());
    let tx_clone = tx.clone();
    main_context.spawn_local(async move {
        let style_manager = StyleManager::default();
        style_manager.connect_color_scheme_notify(move |_| {
            let _ = tx_clone.send(Event::UpdateColors);
        });
    });

    let tx_clone = tx.clone();
    main_context.spawn_local(async move {
        let (mut tx, mut rx) = channel(1);

        // Automatically select the best implementation for your platform.
        // You can also access each implementation directly e.g. INotifyWatcher.
        let mut watcher = RecommendedWatcher::new(move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        })
        .unwrap();

        let mut config_path = Default::default();
        if let Ok(xdg_dirs) = xdg::BaseDirectories::with_prefix(NAME) {
            if let Some(path) =
                xdg_dirs.find_config_file(PathBuf::from(format!("{CONFIG_NAME}.toml")))
            {
                let _ = watcher
                    .watch(&path, RecursiveMode::NonRecursive)
                    .unwrap();
                config_path = path;
            }
        }
        if let Some(p) = css_dirs.find_data_file(format!("{active}.ron")) {
            path = p;
            let _ = watcher.watch(path.as_ref(), RecursiveMode::NonRecursive);
        }

        while let Some(res) = rx.next().await {
            match res {
                Ok(e) => match e.kind {
                    notify::EventKind::Create(_) | notify::EventKind::Modify(_) => {
                        let _ = tx_clone.send(Event::UpdateColors);
                        if e.paths.contains(&config_path) {
                            let _ = watcher.unwatch(&path);
                            if let Ok(theme) = config::Config::load() {
                                let active = theme.active_name();
    
                                let css_path: PathBuf = [NAME, THEME_DIR].iter().collect();
                                let css_dirs = xdg::BaseDirectories::with_prefix(css_path);
    
                                if let (Some(active), Ok(css_dirs)) = (active, css_dirs) {
                                    if let Some(p) = css_dirs.find_data_file(format!("{active}.ron")) {
                                        dbg!(&p);
                                        path = p;
                                        let _ = watcher.watch(&path, RecursiveMode::NonRecursive);
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                },
                Err(e) => eprintln!("watch error: {:?}", e),
            }
        }
    });

    rx.attach(Some(&main_context), move |_| {
        let mut user_color_css = String::new();
        let active = theme.active_name().unwrap();
        let css_path: PathBuf = [NAME, THEME_DIR].iter().collect();
        let css_dirs = xdg::BaseDirectories::with_prefix(css_path).unwrap();
        let active_theme_path = css_dirs.find_data_file(format!("{active}.ron")).unwrap();
        let active_theme_file = File::open(active_theme_path).unwrap();
        let reader = BufReader::new(active_theme_file);
        let overrides: colors::ColorOverrides = ron::de::from_reader(reader).unwrap();

        user_color_css.push_str(&overrides.as_css());
        user_color_css.push_str(&format!("\n@import url(\"custom.css\");\n"));
        if let Ok(xdg_dirs) = xdg::BaseDirectories::with_prefix("gtk-4.0") {
            if let Ok(path) = xdg_dirs.place_config_file(PathBuf::from("gtk.css")) {
                let _ = std::fs::write(&path, &user_color_css);
            }
        }
        adw::prelude::Continue(true)
    });
    let main_loop = MainLoop::new(Some(&main_context), true);
    main_loop.run();
    Ok(())
}

pub fn unload() -> anyhow::Result<()> {
    todo!();
}
