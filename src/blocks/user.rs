use super::BlockWidget;
use gtk4::gdk::Display;
use std::env;

pub struct UserBlock {
    base: super::blocks::BaseBlock,
}

impl UserBlock {
    pub fn new(config: &super::config::BlockConfig, colors: &super::config::Colors) -> Self {
        let username = env::var("USER")
            .unwrap_or_else(|_| "user".to_string());

        let base = super::blocks::BaseBlock::new(
            "",
            &username,
            &colors.user_left_bg,
            &colors.user_right_bg,
        );

        let box = base.as_widget().downcast::<gtk4::Box>().unwrap();

        // Connect clicks
        base.connect_left_click(|| {});
        base.connect_right_click(|| {});
        base.connect_middle_click(|| {});
        base.connect_scroll_up(|| {});
        base.connect_scroll_down(|| {});

        UserBlock { base }
    }

    pub fn as_widget(&self) -> &gtk4::Widget {
        self.base.as_widget()
    }
}