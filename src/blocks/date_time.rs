use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt::Write;
use glib::timeout;
use gtk4::prelude::*;
use gtk4::Box;
use gtk4::Label;

pub struct DateTimeBlock {
    base: super::blocks::BaseBlock,
    box: gtk4::Box,
    label: gtk4::Label,
}

impl DateTimeBlock {
    pub fn new(config: &super::config::BlockConfig, colors: &super::config::Colors) -> Self {
        let base = super::blocks::BaseBlock::new(
            "",
            "",
            &colors.date_left_bg,
            &colors.date_right_bg,
        );

        let box_ = base.as_widget().downcast::<gtk4::Box>().unwrap();
        let label_ = gtk4::Label::new(None);
        label_.set_label("00/00/0000 00:00");
        label_.set_halign(gtk4::Align::Start);

        box_.pack_start(&label_, true, true, 0);
        box_.show_all();

        let base_ = base; // Clone for closure

        let _id = timeout::every(std::time::Duration::from_secs(1), move || {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default();

            let secs = now.as_secs();
            let tm = *localtime(&secs);

            let mut buf = String::new();
            let _ = write!(
                buf,
                "{:02}/{:02}/{} {:02}:{:02}",
                tm.tm_mday,
                tm.tm_mon + 1,
                tm.tm_year + 1900,
                tm.tm_hour,
                tm.tm_min
            );

            label_.set_text(&buf);
            glib::ControlFlow::Continue
        });

        DateTimeBlock {
            base,
            box: box_,
            label: label_,
        }
    }

    pub fn as_widget(&self) -> &gtk4::Widget {
        &self.box as &gtk4::Widget
    }
}