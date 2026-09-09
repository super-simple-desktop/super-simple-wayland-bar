use super::BlockWidget;
use gtk4::prelude::*;

pub struct WiFiBlock {
    base: super::blocks::BaseBlock,
    connected: bool,
}

impl WiFiBlock {
    pub fn new(config: &super::config::BlockConfig, colors: &super::config::Colors, connected: bool) -> Self {
        let icon_name = if connected {
            "network-wireless-symbolic"
        } else {
            "network-off-symbolic"
        };

        let base = super::blocks::BaseBlock::new(
            &icon_name,
            "",
            &colors.launcher_left_bg,
            &colors.launcher_right_bg,
        );

        let connected_ = connected;

        base.connect_left_click(|| {});
        base.connect_right_click(|| {});
        base.connect_middle_click(|| {});
        base.connect_scroll_up(|| {});
        base.connect_scroll_down(|| {});

        WiFiBlock {
            base,
            connected,
        }
    }

    pub fn as_widget(&self) -> &gtk4::Widget {
        self.base.as_widget()
    }

    pub fn set_connected(&mut self, connected: bool) {
        self.connected = connected;
        let icon_name = if connected {
            "network-wireless-symbolic"
        } else {
            "network-off-symbolic"
        };
        self.base.update_icon(icon_name);
    }
}