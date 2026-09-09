use super::BlockWidget;
use gtk4::prelude::*;

pub struct BatteryBlock {
    base: super::blocks::BaseBlock,
}

impl BatteryBlock {
    pub fn new(config: &super::config::BlockConfig, colors: &super::config::Colors) -> Self {
        let base = super::blocks::BaseBlock::new(
            "battery-symbolic",
            "",
            &colors.launcher_left_bg,
            &colors.launcher_right_bg,
        );

        base.connect_left_click(|| {});
        base.connect_right_click(|| {});
        base.connect_middle_click(|| {});
        base.connect_scroll_up(|| {});
        base.connect_scroll_down(|| {});

        BatteryBlock { base }
    }

    pub fn as_widget(&self) -> &gtk4::Widget {
        self.base.as_widget()
    }
}