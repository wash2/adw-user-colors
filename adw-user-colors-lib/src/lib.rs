// SPDX-License-Identifier: MPL-2.0-only

use std::{fs::File, io::BufReader, path::PathBuf};

use adw::{
    glib::{MainContext, MainLoop, Priority},
    StyleManager,
};
use colors::ColorOverrides;
use config::Config;

pub mod colors;
pub mod config;

pub const NAME: &'static str = "adwaita-user-colors";
pub const THEME_DIR: &'static str = "color-overrides";
use futures::{channel::mpsc::channel, SinkExt, StreamExt};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

enum Event {
    UpdateColors,
}

// load selected cosmic-theme
pub fn load() -> anyhow::Result<()> {
    adw::gtk::init()?;
    adw::init();
    let config_dir_path = Config::init()?;
    let color_dir_path = ColorOverrides::init()?;

    let theme = config::Config::load()?;
    let active = theme.active_name();
    if active.is_none() {
        anyhow::bail!("no configured theme");
    }
    let active = active.unwrap();

    let css_path: PathBuf = [NAME, THEME_DIR].iter().collect();
    let css_dirs = xdg::BaseDirectories::with_prefix(css_path)?;

    if let Some(active_theme_path) = css_dirs.find_data_file(format!("{active}.ron")) {
        let active_theme_file = File::open(active_theme_path)?;
        let reader = BufReader::new(active_theme_file);
        let overrides: colors::ColorOverrides = ron::de::from_reader(reader)?;

        let mut user_color_css = String::new();
        user_color_css.push_str(&overrides.as_css());
        user_color_css.push_str(&format!("\n@import url(\"custom.css\");\n"));

        let xdg_dirs = xdg::BaseDirectories::with_prefix("gtk-4.0")?;
        let path = xdg_dirs.place_config_file(PathBuf::from("gtk.css"))?;

        std::fs::write(&path, &user_color_css)?;
    }

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
        let _ = watcher
            .watch(&config_dir_path, RecursiveMode::Recursive)
            .unwrap();
        let _ = watcher.watch(&color_dir_path.as_ref(), RecursiveMode::Recursive);

        while let Some(res) = rx.next().await {
            match res {
                Ok(e) => match e.kind {
                    // TODO only notify for changed data file if it is the active file
                    notify::EventKind::Create(_) | notify::EventKind::Modify(_) => {
                        let _ = tx_clone.send(Event::UpdateColors);
                    }
                    _ => {}
                },
                Err(e) => eprintln!("watch error: {:?}", e),
            }
        }
    });

    rx.attach(Some(&main_context), move |_| {
        if let Ok(overrides) = ColorOverrides::load_active() {
            let user_color_css = &mut overrides.as_css().to_string();
            user_color_css.push_str(&format!("\n@import url(\"custom.css\");\n"));
            if let Ok(xdg_dirs) = xdg::BaseDirectories::with_prefix("gtk-4.0") {
                if let Ok(path) = xdg_dirs.place_config_file(PathBuf::from("gtk.css")) {
                    let _ = std::fs::write(&path, &user_color_css);
                }
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
