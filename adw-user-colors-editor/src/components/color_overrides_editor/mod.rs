// SPDX-License-Identifier: MPL-2.0-only

use crate::{
    components::{
        image_chooser_button::ImageChooserButton, theme_chooser_button::ThemeChooserButton,
    },
    util::{self, hex_from_rgba, SRGBA},
};

use adw::{builders::ExpanderRowBuilder, traits::ExpanderRowExt, ExpanderRow, StyleManager};
use cascade::cascade;
use gtk4::{
    gdk::{self, RGBA},
    gio::File,
    glib::{self, closure_local},
    prelude::*,
    subclass::prelude::*,
    Align, Box, Button, ColorButton, CssProvider, Entry, Label, MessageDialog, Orientation,
    ScrolledWindow, TextView, Window,
};
use relm4_macros::view;
use std::fmt::Display;
use user_colors::{colors::ColorOverrides, config::Config};
mod imp;

glib::wrapper! {
    pub struct ColorOverridesEditor(ObjectSubclass<imp::ColorOverridesEditor>)
        @extends gtk4::Box, gtk4::Widget,
    @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget, gtk4::Orientable;
}

impl ColorOverridesEditor {
    pub fn new(provider: CssProvider) -> Self {
        let self_: Self = glib::Object::new(&[]).expect("Failed to create Theme Editor Widget");

        let imp = imp::ColorOverridesEditor::from_instance(&self_);

        cascade! {
            &self_;
            ..set_orientation(Orientation::Vertical);
        };

        // TODO use i18n for labels
        let accent_section = ExpanderRow::builder()
            .name("Accent Colors")
            .expanded(true)
            .enable_expansion(true)
            .title("Accent Colors")
            .hexpand(true)
            .build();
        let (accent_bg_color, accent_bg_color_button) =
            Self::get_color_button(&self_, "accent_bg_color", "Accent Background Color");
        accent_section.add_row(&accent_bg_color);
        let (accent_fg_color, accent_fg_color_button) =
            Self::get_color_button(&self_, "accent_fg_color", "Accent Foreground Color");
        accent_section.add_row(&accent_fg_color);
        let (accent_color, accent_color_button) =
            Self::get_color_button(&self_, "accent_color", "Accent Color");
        accent_section.add_row(&accent_color);

        let destructive_section = ExpanderRow::builder()
            .name("Destructive Colors")
            .expanded(true)
            .enable_expansion(true)
            .title("Destructive Colors")
            .hexpand(true)
            .build();
        let (destructive_bg_color, destructive_bg_color_button) = Self::get_color_button(
            &self_,
            "destructive_bg_color",
            "Destructive Background Color",
        );
        destructive_section.add_row(&destructive_bg_color);
        let (destructive_fg_color, destructive_fg_color_button) = Self::get_color_button(
            &self_,
            "destructive_fg_color",
            "Destructive Foreground Color",
        );
        destructive_section.add_row(&destructive_fg_color);
        let (destructive_color, destructive_color_button) =
            Self::get_color_button(&self_, "destructive_color", "Destructive Color");
        destructive_section.add_row(&destructive_color);

        let status_section = ExpanderRow::builder()
            .name("Status Colors")
            .expanded(false)
            .enable_expansion(true)
            .title("Status Colors")
            .hexpand(true)
            .build();
        let (success_color, success_color_button) =
            Self::get_color_button(&self_, "success_color", "Success Color");
        status_section.add_row(&success_color);
        let (success_bg_color, success_bg_color_button) =
            Self::get_color_button(&self_, "success_bg_color", "Success Background Color");
        status_section.add_row(&success_bg_color);
        let (success_fg_color, success_fg_color_button) =
            Self::get_color_button(&self_, "success_fg_color", "Success Foreground Color");
        status_section.add_row(&success_fg_color);

        let (warning_color, warning_color_button) =
            Self::get_color_button(&self_, "warning_color", "Warning Color");
        status_section.add_row(&warning_color);
        let (warning_bg_color, warning_bg_color_button) =
            Self::get_color_button(&self_, "warning_bg_color", "Warning Background Color");
        status_section.add_row(&warning_bg_color);
        let (warning_fg_color, warning_fg_color_button) =
            Self::get_color_button(&self_, "warning_fg_color", "Warning Foreground Color");
        status_section.add_row(&warning_fg_color);

        let (error_color, error_color_button) =
            Self::get_color_button(&self_, "error_color", "Error Color");
        status_section.add_row(&error_color);
        let (error_bg_color, error_bg_color_button) =
            Self::get_color_button(&self_, "error_bg_color", "Error Background Color");
        status_section.add_row(&error_bg_color);
        let (error_fg_color, error_fg_color_button) =
            Self::get_color_button(&self_, "error_fg_color", "Error Foreground Color");
        status_section.add_row(&error_fg_color);

        let content_section = ExpanderRow::builder()
            .name("Content Colors")
            .expanded(false)
            .enable_expansion(true)
            .title("Content Colors")
            .hexpand(true)
            .build();
        let (view_bg_color, view_bg_color_button) =
            Self::get_color_button(&self_, "view_bg_color", "Widget Base Color");
        content_section.add_row(&view_bg_color);
        let (view_fg_color, view_fg_color_button) =
            Self::get_color_button(&self_, "view_fg_color", "Widget Text Color");
        content_section.add_row(&view_fg_color);

        let window_section = ExpanderRow::builder()
            .name("Window Colors")
            .expanded(false)
            .enable_expansion(true)
            .title("Window Colors")
            .hexpand(true)
            .build();
        let (window_bg_color, window_bg_color_button) =
            Self::get_color_button(&self_, "window_bg_color", "Window Background Color");
        window_section.add_row(&window_bg_color);
        let (window_fg_color, window_fg_color_button) =
            Self::get_color_button(&self_, "window_fg_color", "Window Foreground Color");
        window_section.add_row(&window_fg_color);

        let headerbar_section = ExpanderRow::builder()
            .name("Headerbar Colors")
            .expanded(false)
            .enable_expansion(true)
            .title("Headerbar Colors")
            .hexpand(true)
            .build();
        let (headerbar_bg_color, headerbar_bg_color_button) =
            Self::get_color_button(&self_, "headerbar_bg_color", "Headerbar Background Color");
        headerbar_section.add_row(&headerbar_bg_color);

        let (headerbar_fg_color, headerbar_fg_color_button) =
            Self::get_color_button(&self_, "headerbar_fg_color", "Headerbar Foreground Color");
        headerbar_section.add_row(&headerbar_fg_color);

        let (headerbar_border_color, headerbar_border_color_button) =
            Self::get_color_button(&self_, "headerbar_border_color", "Headerbar Border Color");
        headerbar_section.add_row(&headerbar_border_color);

        let (headerbar_backdrop_color, headerbar_backdrop_color_button) = Self::get_color_button(
            &self_,
            "headerbar_backdrop_color",
            "Headerbar Backdrop Color",
        );
        headerbar_section.add_row(&headerbar_backdrop_color);

        let (headerbar_shade_color, headerbar_shade_color_button) =
            Self::get_color_button(&self_, "headerbar_shade_color", "Shade Color");
        headerbar_section.add_row(&headerbar_shade_color);

        let card_section = ExpanderRow::builder()
            .name("Card Colors")
            .expanded(false)
            .enable_expansion(true)
            .title("Card Colors")
            .hexpand(true)
            .build();
        let (card_bg_color, card_bg_color_button) =
            Self::get_color_button(&self_, "card_bg_color", "Card Background Color");
        card_section.add_row(&card_bg_color);
        let (card_fg_color, card_fg_color_button) =
            Self::get_color_button(&self_, "card_fg_color", "Card Foreground Color");
        card_section.add_row(&card_fg_color);
        let (card_shade_color, card_shade_color_button) =
            Self::get_color_button(&self_, "card_shade_color", "Card Shade Color");
        card_section.add_row(&card_shade_color);

        let popover_section = ExpanderRow::builder()
            .name("Popover Colors")
            .expanded(false)
            .enable_expansion(true)
            .title("Popover Colors")
            .hexpand(true)
            .build();
        let (popover_bg_color, popover_bg_color_button) =
            Self::get_color_button(&self_, "popover_bg_color", "Popover Background Color");
        popover_section.add_row(&popover_bg_color);
        let (popover_fg_color, popover_fg_color_button) =
            Self::get_color_button(&self_, "popover_fg_color", "Popover Foreground Color");
        popover_section.add_row(&popover_fg_color);

        let misc_section = ExpanderRow::builder()
            .name("Miscellaneous Colors")
            .expanded(false)
            .enable_expansion(true)
            .title("Miscellaneous Colors")
            .hexpand(true)
            .build();
        let (scrollbar_outline_color, scrollbar_outline_color_button) =
            Self::get_color_button(&self_, "scrollbar_outline_color", "Scrollbar Outline Color");
        misc_section.add_row(&scrollbar_outline_color);
        let (shade_color, shade_color_button) =
            Self::get_color_button(&self_, "shade_color", "Shade Color");
        misc_section.add_row(&shade_color);

        let c = Config::load().unwrap_or_default();

        view! {
            inner = Box {
                set_orientation: Orientation::Vertical,
                set_spacing: 4,
                set_margin_top: 4,
                set_margin_bottom: 4,
                set_margin_start: 4,
                set_margin_end: 4,

                append: name = &Entry {
                    set_placeholder_text: Some("Theme Name"),
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,
                    set_width_request: 160,
                },

                append: color_box = &Box {
                    set_orientation: Orientation::Vertical,
                    set_spacing: 4,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,

                    append: &accent_section,
                    append: &destructive_section,
                    append: &status_section,
                    append: &content_section,
                    append: &window_section,
                    append: &headerbar_section,
                    append: &card_section,
                    append: &popover_section,
                    append: &misc_section,
                },


                // TODO add the rest label for each section

                append: control_button_box = &Box {
                    set_orientation: Orientation::Horizontal,
                    set_spacing: 4,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,

                    append: file_button = &ThemeChooserButton {},

                    append: preview_button = &Button {
                        set_margin_top: 4,
                        set_margin_bottom: 4,
                        set_margin_start: 4,
                        set_margin_end: 4,

                        set_child = Some(&Label) {
                            set_text: "Preview",
                        }
                    },

                    append: save_button = &Button {
                        set_margin_top: 4,
                        set_margin_bottom: 4,
                        set_margin_start: 4,
                        set_margin_end: 4,
                        add_css_class: "suggested-action",

                        set_child = Some(&Label) {
                            set_text: "Save",
                        }
                    },


                    // TODO load image as theme
                    // append: file_button = &ImageChooserButton {},
                },

                append = &Box {
                    set_orientation: Orientation::Horizontal,
                    set_spacing: 4,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,

                    append: light_theme_label = &Label {
                        set_text: &format!("Current Light Theme: {}", c.light),
                    },
                    append: light_button = &ThemeChooserButton {},

                },

                append = &Box {
                    set_orientation: Orientation::Horizontal,
                    set_spacing: 4,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,

                    append: dark_theme_label = &Label {
                        set_text: &format!("Current Dark Theme: {}", c.dark),
                    },
                    append: dark_button = &ThemeChooserButton {},

                },
            }
        };

        light_button.connect_closure(
            "file-selected",
            false,
            closure_local!(@weak-allow-none light_theme_label, => move |_file_button: ThemeChooserButton, f: File| {
                if let (Some(label), Some(name)) = (light_theme_label, f.basename()) {
                    let name = name.file_stem().unwrap().to_string_lossy();
                    label.set_text(&format!("Current Light Theme: {}", name));
                    user_colors::config::Config::set_active_light(&name).unwrap();
                }
            }),
        );

        dark_button.connect_closure(
            "file-selected",
            false,
            closure_local!(@weak-allow-none dark_theme_label, => move |_file_button: ThemeChooserButton, f: File| {
                if let (Some(label), Some(name)) = (dark_theme_label, f.basename()) {
                    let name = name.file_stem().unwrap().to_string_lossy();
                    label.set_text(&format!("Current Dark Theme: {}", name));
                    user_colors::config::Config::set_active_dark(&name).unwrap();
                }
            }),
        );

        let scroll_window = ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .child(&inner)
            .build();

        self_.append(&scroll_window);

        imp.css_provider.set(provider).unwrap();

        // set widget state
        imp.name.set(name).unwrap();
        imp.save.set(save_button).unwrap();
        imp.preview.set(preview_button).unwrap();
        imp.file_button.set(file_button).unwrap();
        imp.color_editor.set(color_box).unwrap();

        self_.connect_name();
        self_.connect_control_buttons();
        self_.connect_file_button();

        self_
    }

    fn connect_name(&self) {
        let imp = imp::ColorOverridesEditor::from_instance(&self);
        imp.name.get().unwrap().connect_changed(
            glib::clone!(@weak imp.theme as theme => move |name| {
                let name = name.text();
                theme.borrow_mut().name = String::from(name.as_str());
            }),
        );
    }

    fn set_buttons(container: &Box, theme: &ColorOverrides) {}

    fn connect_file_button(&self) {
        let imp = imp::ColorOverridesEditor::from_instance(&self);
        imp.file_button.get().unwrap().connect_closure(
            "file-selected",
            false,
            closure_local!(@weak-allow-none imp.name as name, @weak-allow-none imp.theme as theme, @weak-allow-none imp.color_editor as color_editor, @weak-allow-none self as self_ => move |_file_button: ThemeChooserButton, f: File| {
                if let (Some(theme), Some(name), Some(Ok(t)), Some(color_editor)) = (theme, name, f.path().as_ref().map(|p| ColorOverrides::load(p)), color_editor) {
                    let name = name.get().unwrap();
                    name.set_text(&t.name);
                    theme.replace(t);

                    let color_editor = color_editor.get().unwrap();
                    let mut c = color_editor.first_child();
                    while let Some(child) = c  {
                        color_editor.remove(&child);
                        c = color_editor.first_child();
                    }
                    let accent_section = ExpanderRow::builder()
                    .name("Accent Colors")
                    .expanded(true)
                    .enable_expansion(true)
                    .title("Accent Colors")
                    .hexpand(true)
                    .build();
                let self_ = self_.unwrap();
                let (accent_bg_color, accent_bg_color_button) =
                    Self::get_color_button(&self_, "accent_bg_color", "Accent Background Color");
                accent_section.add_row(&accent_bg_color);
                let (accent_fg_color, accent_fg_color_button) =
                    Self::get_color_button(&self_, "accent_fg_color", "Accent Foreground Color");
                accent_section.add_row(&accent_fg_color);
                let (accent_color, accent_color_button) =
                    Self::get_color_button(&self_, "accent_color", "Accent Color");
                accent_section.add_row(&accent_color);
        
                let destructive_section = ExpanderRow::builder()
                    .name("Destructive Colors")
                    .expanded(true)
                    .enable_expansion(true)
                    .title("Destructive Colors")
                    .hexpand(true)
                    .build();
                let (destructive_bg_color, destructive_bg_color_button) = Self::get_color_button(
                    &self_,
                    "destructive_bg_color",
                    "Destructive Background Color",
                );
                destructive_section.add_row(&destructive_bg_color);
                let (destructive_fg_color, destructive_fg_color_button) = Self::get_color_button(
                    &self_,
                    "destructive_fg_color",
                    "Destructive Foreground Color",
                );
                destructive_section.add_row(&destructive_fg_color);
                let (destructive_color, destructive_color_button) =
                    Self::get_color_button(&self_, "destructive_color", "Destructive Color");
                destructive_section.add_row(&destructive_color);
        
                let status_section = ExpanderRow::builder()
                    .name("Status Colors")
                    .expanded(false)
                    .enable_expansion(true)
                    .title("Status Colors")
                    .hexpand(true)
                    .build();
                let (success_color, success_color_button) =
                    Self::get_color_button(&self_, "success_color", "Success Color");
                status_section.add_row(&success_color);
                let (success_bg_color, success_bg_color_button) =
                    Self::get_color_button(&self_, "success_bg_color", "Success Background Color");
                status_section.add_row(&success_bg_color);
                let (success_fg_color, success_fg_color_button) =
                    Self::get_color_button(&self_, "success_fg_color", "Success Foreground Color");
                status_section.add_row(&success_fg_color);
        
                let (warning_color, warning_color_button) =
                    Self::get_color_button(&self_, "warning_color", "Warning Color");
                status_section.add_row(&warning_color);
                let (warning_bg_color, warning_bg_color_button) =
                    Self::get_color_button(&self_, "warning_bg_color", "Warning Background Color");
                status_section.add_row(&warning_bg_color);
                let (warning_fg_color, warning_fg_color_button) =
                    Self::get_color_button(&self_, "warning_fg_color", "Warning Foreground Color");
                status_section.add_row(&warning_fg_color);
        
                let (error_color, error_color_button) =
                    Self::get_color_button(&self_, "error_color", "Error Color");
                status_section.add_row(&error_color);
                let (error_bg_color, error_bg_color_button) =
                    Self::get_color_button(&self_, "error_bg_color", "Error Background Color");
                status_section.add_row(&error_bg_color);
                let (error_fg_color, error_fg_color_button) =
                    Self::get_color_button(&self_, "error_fg_color", "Error Foreground Color");
                status_section.add_row(&error_fg_color);
        
                let content_section = ExpanderRow::builder()
                    .name("Content Colors")
                    .expanded(false)
                    .enable_expansion(true)
                    .title("Content Colors")
                    .hexpand(true)
                    .build();
                let (view_bg_color, view_bg_color_button) =
                    Self::get_color_button(&self_, "view_bg_color", "Widget Base Color");
                content_section.add_row(&view_bg_color);
                let (view_fg_color, view_fg_color_button) =
                    Self::get_color_button(&self_, "view_fg_color", "Widget Text Color");
                content_section.add_row(&view_fg_color);
        
                let window_section = ExpanderRow::builder()
                    .name("Window Colors")
                    .expanded(false)
                    .enable_expansion(true)
                    .title("Window Colors")
                    .hexpand(true)
                    .build();
                let (window_bg_color, window_bg_color_button) =
                    Self::get_color_button(&self_, "window_bg_color", "Window Background Color");
                window_section.add_row(&window_bg_color);
                let (window_fg_color, window_fg_color_button) =
                    Self::get_color_button(&self_, "window_fg_color", "Window Foreground Color");
                window_section.add_row(&window_fg_color);
        
                let headerbar_section = ExpanderRow::builder()
                    .name("Headerbar Colors")
                    .expanded(false)
                    .enable_expansion(true)
                    .title("Headerbar Colors")
                    .hexpand(true)
                    .build();
                let (headerbar_bg_color, headerbar_bg_color_button) =
                    Self::get_color_button(&self_, "headerbar_bg_color", "Headerbar Background Color");
                headerbar_section.add_row(&headerbar_bg_color);
        
                let (headerbar_fg_color, headerbar_fg_color_button) =
                    Self::get_color_button(&self_, "headerbar_fg_color", "Headerbar Foreground Color");
                headerbar_section.add_row(&headerbar_fg_color);
        
                let (headerbar_border_color, headerbar_border_color_button) =
                    Self::get_color_button(&self_, "headerbar_border_color", "Headerbar Border Color");
                headerbar_section.add_row(&headerbar_border_color);
        
                let (headerbar_backdrop_color, headerbar_backdrop_color_button) = Self::get_color_button(
                    &self_,
                    "headerbar_backdrop_color",
                    "Headerbar Backdrop Color",
                );
                headerbar_section.add_row(&headerbar_backdrop_color);
        
                let (headerbar_shade_color, headerbar_shade_color_button) =
                    Self::get_color_button(&self_, "headerbar_shade_color", "Shade Color");
                headerbar_section.add_row(&headerbar_shade_color);
        
                let card_section = ExpanderRow::builder()
                    .name("Card Colors")
                    .expanded(false)
                    .enable_expansion(true)
                    .title("Card Colors")
                    .hexpand(true)
                    .build();
                let (card_bg_color, card_bg_color_button) =
                    Self::get_color_button(&self_, "card_bg_color", "Card Background Color");
                card_section.add_row(&card_bg_color);
                let (card_fg_color, card_fg_color_button) =
                    Self::get_color_button(&self_, "card_fg_color", "Card Foreground Color");
                card_section.add_row(&card_fg_color);
                let (card_shade_color, card_shade_color_button) =
                    Self::get_color_button(&self_, "card_shade_color", "Card Shade Color");
                card_section.add_row(&card_shade_color);
        
                let popover_section = ExpanderRow::builder()
                    .name("Popover Colors")
                    .expanded(false)
                    .enable_expansion(true)
                    .title("Popover Colors")
                    .hexpand(true)
                    .build();
                let (popover_bg_color, popover_bg_color_button) =
                    Self::get_color_button(&self_, "popover_bg_color", "Popover Background Color");
                popover_section.add_row(&popover_bg_color);
                let (popover_fg_color, popover_fg_color_button) =
                    Self::get_color_button(&self_, "popover_fg_color", "Popover Foreground Color");
                popover_section.add_row(&popover_fg_color);
        
                let misc_section = ExpanderRow::builder()
                    .name("Miscellaneous Colors")
                    .expanded(false)
                    .enable_expansion(true)
                    .title("Miscellaneous Colors")
                    .hexpand(true)
                    .build();
                let (scrollbar_outline_color, scrollbar_outline_color_button) =
                    Self::get_color_button(&self_, "scrollbar_outline_color", "Scrollbar Outline Color");
                misc_section.add_row(&scrollbar_outline_color);
                let (shade_color, shade_color_button) =
                    Self::get_color_button(&self_, "shade_color", "Shade Color");
                misc_section.add_row(&shade_color);
        
                    color_editor.append(&accent_section);
                    color_editor.append(&destructive_section);
                    color_editor.append(&status_section);
                    color_editor.append(&content_section);
                    color_editor.append(&window_section);
                    color_editor.append(&headerbar_section);
                    color_editor.append(&card_section);
                    color_editor.append(&popover_section);
                    color_editor.append(&misc_section);
                }
            }),
        );
    }

    fn update_color_buttons(&self) {
        let imp = imp::ColorOverridesEditor::from_instance(&self);
        // TODO update alll buttons to match colors with current theme
    }

    fn get_color_button(&self, id: &str, label: &str) -> (Box, ColorButton) {
        // TODO add button for clearing color
        let imp = imp::ColorOverridesEditor::from_instance(&self);

        let rgba = SRGBA::default().into();
        let color_button = cascade! {
            ColorButton::with_rgba(&rgba);
            ..set_title(label);
            ..set_use_alpha(true);
        };
        if let Some(Ok(c)) = imp.theme.borrow().get_key(id).map(|c| RGBA::parse(&c)) {
            color_button.set_rgba(&c);
        } else {
            color_button.set_rgba(&RGBA::new(0.0, 0.0, 0.0, 0.0));
        }
        let id_clone = id.to_string();
        color_button
        .connect_rgba_notify(glib::clone!(@weak imp.theme as theme => move |self_| {
            let mut t = theme.borrow_mut();
            t.set_key(&id_clone, Some(hex_from_rgba(self_.rgba()))).expect(&format!("Failed to set {id_clone}"));
        }));
        let clear_button = Button::with_label("Clear");
        clear_button.add_css_class("destructive-action");
        clear_button.set_halign(Align::End);
        let id_clone = id.to_string();
        clear_button.connect_clicked(
            glib::clone!(@weak color_button, @weak imp.theme as theme => move |_| {
                let mut t = theme.borrow_mut();
                t.set_key(&id_clone, None).expect(&format!("Failed to set {id_clone}"));
                drop(t);
                color_button.set_rgba(&RGBA::new(0.0, 0.0, 0.0, 0.0));
            }),
        );
        view! {
            color_box = Box {
                set_orientation: Orientation::Horizontal,
                set_spacing: 4,
                set_margin_top: 4,
                set_margin_bottom: 4,
                set_margin_start: 4,
                set_margin_end: 4,
                set_hexpand: true,

                append: &color_button,

                append: accent_color_label = &Label {
                    set_text: label,
                },
                append = &Box {
                    set_orientation: Orientation::Horizontal,
                    set_hexpand: true,
                    set_halign: Align::End,
                    append: &clear_button,
                },
            }
        };
        (color_box, color_button)
    }

    fn connect_control_buttons(&self) {
        let imp = imp::ColorOverridesEditor::from_instance(&self);
        let theme = &imp.theme;
        let css_provider = &imp.css_provider;

        imp.save.get().unwrap().connect_clicked(
            glib::clone!(@weak theme, @weak self as self_ => move |save| {
                if &theme.borrow().name != "" {
                    // TODO toast if fails
                    theme.borrow().save().unwrap();
                } else {
                    // todo replace with toast
                    let window = self_.root().map(|root| {
                        if let Ok(w) = root.downcast::<Window>() {
                            Some(w)
                        } else {
                            None
                        }
                    }).unwrap_or_default();
                    if let Some(window) = window {
                        glib::MainContext::default().spawn_local(Self::dialog(window, "Please enter a name"));
                    }
                }
            }),
        );

        imp.preview.get().unwrap().connect_clicked(
            glib::clone!(@weak theme, @weak css_provider => move |_| {
                let manager = StyleManager::default();
                let default_theme  = if manager.is_dark() {
                    ColorOverrides::dark_default()
                } else {
                    ColorOverrides::light_default()
                };
                let preview_css = &mut default_theme.as_css();
                preview_css.push_str(&theme.borrow().as_css());
                css_provider.get().unwrap().load_from_data(preview_css.as_bytes());
            }),
        );
    }

    async fn dialog<T: Display>(window: Window, msg: T) {
        let msg_dialog = MessageDialog::builder()
            .transient_for(&window)
            .modal(true)
            .buttons(gtk4::ButtonsType::Close)
            .text(&format!("{}", msg))
            .build();
        cascade! {
            msg_dialog.message_area();
            ..set_margin_top(8);
            ..set_margin_bottom(8);
            ..set_margin_start(8);
            ..set_margin_end(8);
        };
        let _ = msg_dialog.run_future().await;
        msg_dialog.close();
    }
}
