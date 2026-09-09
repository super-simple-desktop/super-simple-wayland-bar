use gtk4::prelude::*;
use gtk4::{Box, Orientation, Image, Label};
use glib::clone;

pub trait BlockWidget: Send + Sync + 'static {
    fn as_widget(&self) -> &gtk4::Widget;
    fn set_left_bg_color(&self, color: &gdk::rgba::RGBA);
    fn set_right_bg_color(&self, color: &gdk::rgba::RGBA);
    fn update_icon(&self, icon_name: &str);
    fn update_text(&self, text: &str);
    fn connect_left_click(&self, f: impl Fn() + Send + 'static);
    fn connect_right_click(&self, f: impl Fn() + Send + 'static);
    fn connect_middle_click(&self, f: impl Fn() + Send + 'static);
    fn connect_scroll_up(&self, f: impl Fn() + Send + 'static);
    fn connect_scroll_down(&self, f: impl Fn() + Send + 'static);
}

pub struct BaseBlock {
    box: gtk4::Box,
    icon: gtk4::Image,
    label: gtk4::Label,
    left_bg: gdk::rgba::RGBA,
    right_bg: gdk::rgba::RGBA,
}

impl BaseBlock {
    pub fn new(icon_name: &str, right_text: &str, left_bg: &str, right_bg: &str) -> Self {
        let box_ = Box::new(Orientation::Horizontal, 0);
        box_.set_hexpand(true);
        box_.set_vexpand(true);

        let icon = Image::from_icon_name(icon_name, gtk4::IconSize::Button);
        let label = Label::new(Some(right_text));

        box_.pack_start(&icon, false, false, 0);
        box_.pack_start(&label, true, true, 0);

        let left_rgba = gdk::rgba::RGBA::parse(left_bg).unwrap_or_else(|| gdk::rgba::RGBA::new(0.0, 0.0, 0.0, 1.0));
        let right_rgba = gdk::rgba::RGBA::parse(right_bg).unwrap_or_else(|| gdk::rgba::RGBA::new(0.0, 0.0, 0.0, 1.0));

        icon.set_halign(gtk4::Align::Start);
        label.set_halign(gtk4::Align::Start);

        BaseBlock {
            box_,
            icon,
            label,
            left_bg: left_rgba,
            right_bg: right_rgba,
        }
    }

    pub fn as_widget(&self) -> &gtk4::Widget {
        &self.box_ as &gtk4::Widget
    }

    pub fn set_left_bg_color(&self, color: &gdk::rgba::RGBA) {
        self.left_bg = *color;
        self.apply_bg_color(&self.box_, &self.left_bg);
    }

    pub fn set_right_bg_color(&self, color: &gdk::rgba::RGBA) {
        self.right_bg = *color;
        self.apply_bg_color(&self.label_, &self.right_bg);
    }

    pub fn update_icon(&self, icon_name: &str) {
        self.icon.set_from_icon_name(icon_name, gtk4::IconSize::Button);
    }

    pub fn update_text(&self, text: &str) {
        self.label.set_text(text);
    }

    fn apply_bg_color(widget: &impl IsA<gtk4::Widget>, color: &gdk::rgba::RGBA) {
        let css = format!("background-color: rgba({}, {}, {}, {});", 
            color.red_f(), color.green_f(), color.blue_f(), color.alpha());
        let provider = CssProvider::new();
        provider.load_from_string(&css).ok();
        let screen = widget.screen();
        if let Some(screen) = screen {
            provider.attach_to_screen(&screen);
        }
        widget.style_context()->add_provider(&provider, gtk4::STYLE_PROVIDER_PRIORITY_USER);
    }
}

impl BlockWidget for BaseBlock {
    fn as_widget(&self) -> &gtk4::Widget {
        &self.box_ as &gtk4::Widget
    }

    fn set_left_bg_color(&self, color: &gdk::rgba::RGBA) {
        self.set_left_bg_color(color);
    }

    fn set_right_bg_color(&self, color: &gdk::rgba::RGBA) {
        self.set_right_bg_color(color);
    }

    fn update_icon(&self, icon_name: &str) {
        self.update_icon(icon_name);
    }

    fn update_text(&self, text: &str) {
        self.update_text(text);
    }

    fn connect_left_click(&self, f: impl Fn() + Send + 'static) {
        let f = Box::new(f);
        let box_clone = self.box_.clone();
        self.box_.connect_button_press_event(move |_, event| {
            if let gtk4::gdk::EventButton::LeftPress(_) = event {
                (*f)();
            }
            gtk4::Propagation::hide()
        });
    }

    fn connect_right_click(&self, f: impl Fn() + Send + 'static) {
        let f = Box::new(f);
        let box_clone = self.box_.clone();
        self.box_.connect_button_press_event(move |_, event| {
            if let gtk4::gdk::EventButton::RightPress(_) = event {
                (*f)();
            }
            gtk4::Propagation::hide()
        });
    }

    fn connect_middle_click(&self, f: impl Fn() + Send + 'static) {
        let f = Box::new(f);
        let box_clone = self.box_.clone();
        self.box_.connect_button_press_event(move |_, event| {
            if let gtk4::gdk::EventButton::MiddlePress(_) = event {
                (*f)();
            }
            gtk4::Propagation::hide()
        });
    }

    fn connect_scroll_up(&self, f: impl Fn() + Send + 'static) {
        let f = Box::new(f);
        let box_clone = self.box_.clone();
        self.box_.connect_scroll_event(move |_, event| {
            if let Some(scroll) = event.downcast_ref::<gtk4::gdk::EventScroll>() {
                if scroll.direction() == gtk4::gdk::ScrollDirection::Up {
                    (*f)();
                }
            }
            gtk4::Propagation::hide()
        });
    }

    fn connect_scroll_down(&self, f: impl Fn() + Send + 'static) {
        let f = Box::new(f);
        let box_clone = self.box_.clone();
        self.box_.connect_scroll_event(move |_, event| {
            if let Some(scroll) = event.downcast_ref::<gtk4::gdk::EventScroll>() {
                if scroll.direction() == gtk4::gdk::ScrollDirection::Down {
                    (*f)();
                }
            }
            gtk4::Propagation::hide()
        });
    }
}