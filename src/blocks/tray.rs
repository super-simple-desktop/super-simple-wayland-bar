use super::BlockWidget;
use gtk4::prelude::*;
use gtk4::{Box, Orientation, Image, Label, EventBox};
use gtk4::gdk::Texture;

pub struct SystemTray {
    base: super::blocks::BaseBlock,
    tray_container: gtk4::Box,
}

impl SystemTray {
    pub fn new(config: &super::config::BlockConfig, colors: &super::config::Colors) -> Self {
        // Note: system tray has NO custom actions - normal tray interaction
        let base = super::blocks::BaseBlock::new(
            "",
            "",
            &colors.launcher_left_bg,  // reuse launcher left bg
            &colors.launcher_right_bg, // reuse launcher right bg
        );

        let tray_container = gtk4::Box::new(Orientation::Horizontal, 0);
        tray_container.set_hexpand(true);
        tray_container.set_vexpand(true);

        // We'll be populated dynamically by the Wayland bar protocol
        // Tray icons are added by the compositor/wlr-layer-shell

        let box = base.as_widget().downcast::<gtk4::Box>().unwrap();

        // Connect clicks - but do nothing (normal tray interaction)
        // The base block's click handlers are intentionally left empty
        // for system tray to allow normal interaction

        SystemTray {
            base,
            tray_container,
        }
    }

    pub fn as_widget(&self) -> &gtk4::Widget {
        // Return a container that holds both the base block and tray icons
        // For now, return the base box - tray icons will be added separately
        self.base.as_widget()
    }

    pub fn add_tray_icon(&self, icon_name: &str) {
        let icon = gtk4::Image::from_icon_name(icon_name, gtk4::IconSize::Button);
        self.tray_container.pack_start(&icon, false, false, 0);
        self.tray_container.show_all();
    }
}