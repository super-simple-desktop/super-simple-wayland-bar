use super::BlockWidget;
use gtk4::prelude::*;

pub struct LauncherBlock {
    base: super::blocks::BaseBlock,
    icon_name: String,
}

impl LauncherBlock {
    pub fn new(config: &super::config::BlockConfig, colors: &super::config::Colors) -> Self {
        let base = super::blocks::BaseBlock::new(
            &config.icon.unwrap_or("distributor-logo"),
            "",
            &colors.launcher_left_bg,
            &colors.launcher_right_bg,
        );

        let box = base.as_widget().downcast::<gtk4::Box>().unwrap();
        let icon_name = config.icon.clone().unwrap_or_else(|| "distributor-logo".to_string());

        // Connect clicks
        base.connect_left_click(|| {
            // Action defined in config - would be passed as closure
        });
        base.connect_right_click(|| {
            // Action defined in config
        });
        base.connect_middle_click(|| {
            // Action defined in config
        });
        base.connect_scroll_up(|| {
            // Action defined in config
        });
        base.connect_scroll_down(|| {
            // Action defined in config
        });

        LauncherBlock { base, icon_name }
    }

    pub fn as_widget(&self) -> &gtk4::Widget {
        self.base.as_widget()
    }

    pub fn update_connection(&mut self, connected: bool) {
        let icon = if connected {
            "network-wireless-symbolic"
        } else {
            "network-off-symbolic"
        };
        self.base.update_icon(icon);
    }
}