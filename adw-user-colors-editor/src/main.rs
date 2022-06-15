// SPDX-License-Identifier: MPL-2.0-only

use adw::{ActionRow, Application, ApplicationWindow, HeaderBar};
use components::ColorOverridesEditor;
use gtk4::{gdk::Display, glib, prelude::*, CssProvider, StyleContext, gio};

mod components;
mod util;

const APP_STRING: &'static str = "gay.ash.AdwaitaUserColorsEditor";
const APP_TITLE: &'static str = "Adwaita User Colors Editor";

fn setup_shortcuts(app: &Application) {
    //quit shortcut
    app.set_accels_for_action("win.quit", &["<primary>W", "Escape"]);
}

fn load_css() -> CssProvider {
    let provider = CssProvider::new();
    StyleContext::add_provider_for_display(
        &Display::default().expect("Error initializing GTK CSS provider."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_USER,
    );
    provider
}

fn main() {
    let app = Application::builder().application_id(APP_STRING).build();
    gio::resources_register_include!("compiled.gresource").unwrap();

    app.connect_startup(|app| {
        setup_shortcuts(app);
    });
    app.connect_activate(move |app| {
        let provider = load_css();
        let theme_editor = ColorOverridesEditor::new(provider);

        let window = ApplicationWindow::builder()
            .application(app)
            .title(APP_TITLE)
            .default_width(350)
            // add content to window
            .content(&theme_editor)
            .build();

        window.show();
    });
    app.run();
}
