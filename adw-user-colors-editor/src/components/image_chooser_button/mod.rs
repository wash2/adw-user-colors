// SPDX-License-Identifier: MPL-2.0-only

mod imp;

use cascade::cascade;
use gtk4::{glib, prelude::*, subclass::prelude::*, Button, FileChooserNative, Window};

glib::wrapper! {
    pub struct ImageChooserButton(ObjectSubclass<imp::ImageChooserButton>)
        @extends gtk4::Box, gtk4::Widget,
    @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget, gtk4::Orientable;
}

impl Default for ImageChooserButton {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageChooserButton {
    pub fn new() -> Self {
        let button = cascade! {
            Button::with_label("Load from Image");
            ..add_css_class("background-component");
            ..add_css_class("padding-medium");
            ..add_css_class("border-radius-medium");
        };

        let self_: Self = glib::Object::new(&[]).expect("Failed to create `ImageChooserButton`.");
        cascade! {
            &self_;
            ..append(&button);
            ..add_css_class("background");
            ..set_margin_top(4);
            ..set_margin_bottom(4);
            ..set_margin_start(4);
            ..set_margin_end(4);

        };
        let imp = imp::ImageChooserButton::from_instance(&self_);

        let window = self_
            .root()
            .map(|root| {
                if let Ok(w) = root.downcast::<Window>() {
                    Some(w)
                } else {
                    None
                }
            })
            .unwrap_or_default();

        let file_chooser = FileChooserNative::new(
            Some("Select Image"),
            window.as_ref(),
            gtk4::FileChooserAction::Open,
            None,
            None,
        );
        let image_filter = gtk4::FileFilter::new();
        image_filter.add_pixbuf_formats();
        file_chooser.add_filter(&image_filter);

        imp.button.replace(button);
        imp.file_chooser.replace(file_chooser);

        self_.connect_button_to_chooser_dialog();
        self_.connect_file_chooser();

        self_
    }

    fn connect_button_to_chooser_dialog(&self) {
        let imp = imp::ImageChooserButton::from_instance(&self);
        imp.button.borrow().connect_clicked(
            glib::clone!(@weak imp.file_chooser as file_chooser, @weak self as self_ => move |_| {
                file_chooser.borrow().show();
            }),
        );
    }

    fn connect_file_chooser(&self) {
        let imp = imp::ImageChooserButton::from_instance(&self);
        imp.file_chooser.borrow().connect_response(
            glib::clone!(@weak self as self_ => move |file_chooser, response| {
                if response != gtk4::ResponseType::Accept {return};
                if let Some(f) = file_chooser.file() {
                    self_.emit_by_name::<()>("image-selected", &[&f]);
                }
            }),
        );
    }
}
