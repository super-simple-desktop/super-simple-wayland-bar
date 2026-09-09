use gtk4::gdk::RGBA;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Orientation, Image, Label};
use gtk4_layer_shell::{Layer, Edge, LayerShell};
use std::time::{SystemTime, UNIX_EPOCH};

struct Config {
    colors: ConfigColors,
    blocks: std::collections::HashMap<String, BlockConfig>,
}

struct ConfigColors {
    background: String,
    launcher_left_bg: String,
    launcher_right_bg: String,
    user_left_bg: String,
    user_right_bg: String,
    date_left_bg: String,
    date_right_bg: String,
    text_fg: String,
}

struct BlockConfig {
    icon: Option<String>,
    text: Option<String>,
    left_click: String,
    right_click: String,
    middle_click: String,
    scroll_up: String,
    scroll_down: String,
}

fn load_config() -> Config {
    let config_dir = dirs::config_dir()
        .map(|d| d.join("super-simple-wayland-bar/config.toml"))
        .unwrap_or_else(|| std::path::PathBuf::from("config.toml"));

    let content = match std::fs::read_to_string(&config_dir) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Failed to read config from {:?}", config_dir);
            std::process::exit(1);
        }
    };

    let value: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Failed to parse config");
            std::process::exit(1);
        }
    };

    let colors = value.get("colors").and_then(|v| v.as_table()).cloned().unwrap_or_default();
    let colors = ConfigColors {
        background: colors.get("background").and_then(|v| v.as_str()).unwrap_or("#2b2b2b").to_string(),
        launcher_left_bg: colors.get("launcher_left_bg").and_then(|v| v.as_str()).unwrap_or("#3b3b3b").to_string(),
        launcher_right_bg: colors.get("launcher_right_bg").and_then(|v| v.as_str()).unwrap_or("#4b3b3b").to_string(),
        user_left_bg: colors.get("user_left_bg").and_then(|v| v.as_str()).unwrap_or("#3b3b3b").to_string(),
        user_right_bg: colors.get("user_right_bg").and_then(|v| v.as_str()).unwrap_or("#4b3b3b").to_string(),
        date_left_bg: colors.get("date_left_bg").and_then(|v| v.as_str()).unwrap_or("#3b3b3b").to_string(),
        date_right_bg: colors.get("date_right_bg").and_then(|v| v.as_str()).unwrap_or("#4b3b3b").to_string(),
        text_fg: colors.get("text_fg").and_then(|v| v.as_str()).unwrap_or("#e0e0e0").to_string(),
    };

    let blocks_table = value.get("blocks").and_then(|v| v.as_table()).cloned().unwrap_or_default();
    let mut blocks = std::collections::HashMap::new();

    for &name in &["launcher", "user", "date_time", "wifi", "network", "bluetooth", "battery", "power"] {
        let b = blocks_table.get(name).and_then(|v| v.as_table());
        blocks.insert(name.to_string(), BlockConfig {
            icon: b.and_then(|t| t.get("icon")).and_then(|v| v.as_str()).map(|s| s.to_string()),
            text: b.and_then(|t| t.get("text")).and_then(|v| v.as_str()).map(|s| s.to_string()),
            left_click: b.and_then(|t| t.get("left_click")).and_then(|v| v.as_str()).unwrap_or("").to_string(),
            right_click: b.and_then(|t| t.get("right_click")).and_then(|v| v.as_str()).unwrap_or("").to_string(),
            middle_click: b.and_then(|t| t.get("middle_click")).and_then(|v| v.as_str()).unwrap_or("").to_string(),
            scroll_up: b.and_then(|t| t.get("scroll_up")).and_then(|v| v.as_str()).unwrap_or("").to_string(),
            scroll_down: b.and_then(|t| t.get("scroll_down")).and_then(|v| v.as_str()).unwrap_or("").to_string(),
        });
    }

    Config { colors, blocks }
}

fn setup_layer_shell(window: &ApplicationWindow) {
    window.init_layer_shell();
    window.set_layer(Layer::Background);
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);
    window.set_exclusive_zone(32);
    window.set_height_request(32);
}

fn create_block_box(
    icon_name: &str,
    right_text: &str,
    left_bg_color: &str,
    right_bg_color: &str,
) -> (Image, Label, gtk4::Box) {
    let box_ = gtk4::Box::new(Orientation::Horizontal, 0);
    box_.set_hexpand(true);
    box_.set_vexpand(true);

    let image = Image::from_icon_name(icon_name);
    let label = Label::new(Some(right_text));

    box_.append(&image);
    box_.append(&label);

    let left_rgba = RGBA::parse(left_bg_color).unwrap_or_else(|_| RGBA::new(0.0, 0.0, 0.0, 1.0));
    let left_css = format!(
        r#"background: rgba({:.2}, {:.2}, {:.2}, {:.2});"#,
        left_rgba.red(), left_rgba.green(), left_rgba.blue(), left_rgba.alpha()
    );
    let css_provider = gtk4::CssProvider::new();
    css_provider.load_from_data(&left_css);
    image.style_context().add_provider(&css_provider, gtk4::STYLE_PROVIDER_PRIORITY_USER);

    let right_rgba = RGBA::parse(right_bg_color).unwrap_or_else(|_| RGBA::new(0.0, 0.0, 0.0, 1.0));
    let right_css = format!(
        r#"background: rgba({:.2}, {:.2}, {:.2}, {:.2});"#,
        right_rgba.red(), right_rgba.green(), right_rgba.blue(), right_rgba.alpha()
    );
    let label_css_provider = gtk4::CssProvider::new();
    label_css_provider.load_from_data(&right_css);
    label.style_context().add_provider(&label_css_provider, gtk4::STYLE_PROVIDER_PRIORITY_USER);

    (image, label, box_)
}

fn format_time() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let tm = unsafe { *libc::localtime(&now) };
    format!(
        "{:02}/{:02}/{:04} {:02}:{:02}",
        tm.tm_mday,
        tm.tm_mon + 1,
        tm.tm_year + 1900,
        tm.tm_hour,
        tm.tm_min
    )
}

fn main() {
    gtk4::init().expect("Failed to init GTK");

    let config = load_config();

    let app = Application::builder()
        .application_id("com.super.simple.wayland.bar")
        .build();

    app.connect_activate(move |app| {
        let mut window = ApplicationWindow::new(app);
        window.set_title(Some("super-simple-wayland-bar"));
        window.set_decorated(false);

        setup_layer_shell(&window);

        let css_provider = gtk4::CssProvider::new();
        let rgba = RGBA::parse(&config.colors.background).unwrap();
        css_provider.load_from_data(&format!(
            r#"background: rgba({:.2}, {:.2}, {:.2}, {:.2});"#,
            rgba.red(), rgba.green(), rgba.blue(), rgba.alpha()
        ));
        window.style_context().add_provider(&css_provider, gtk4::STYLE_PROVIDER_PRIORITY_USER);

        window.set_width_request(-1);
        window.set_height_request(32);

        let container = gtk4::Box::new(Orientation::Horizontal, 0);
        container.set_hexpand(true);
        container.set_vexpand(true);
        window.set_child(Some(&container));

        let (_, _, launcher_box) = create_block_box(
            &config.blocks.get("launcher").map(|b| b.icon.as_deref()).flatten()
                .unwrap_or("distributor-logo"),
            "",
            &config.colors.launcher_left_bg,
            &config.colors.launcher_right_bg,
        );
        container.append(&launcher_box);

        let username = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
        let (_, _, user_box) = create_block_box(
            "",
            &username,
            &config.colors.user_left_bg,
            &config.colors.user_right_bg,
        );
        container.append(&user_box);

        let (_, datetime_label, datetime_box) = create_block_box(
            "",
            "",
            &config.colors.date_left_bg,
            &config.colors.date_right_bg,
        );
        datetime_box.set_hexpand(true);
        container.append(&datetime_box);

        datetime_label.set_text(&format_time());

        let datetime_label_clone = datetime_label.clone();
        glib::timeout_add_seconds_local(1, move || {
            datetime_label_clone.set_text(&format_time());
            glib::ControlFlow::Continue
        });

        let (_, _, power_box) = create_block_box(
            "power-symbolic",
            "",
            &config.colors.launcher_left_bg,
            &config.colors.launcher_right_bg,
        );
        container.append(&power_box);

        let has_battery = std::fs::read_dir("/sys/class/power_supply")
            .ok()
            .map(|mut entries| {
                entries.any(|e| {
                    let name = e.ok()
                        .map(|f| f.file_name().to_string_lossy().to_string())
                        .unwrap_or_default();
                    name.contains("BAT") || name.contains("CYCLE")
                })
            })
            .unwrap_or(false);

        if has_battery {
            let (_, _, bat_box) = create_block_box(
                "battery-symbolic",
                "",
                &config.colors.launcher_left_bg,
                &config.colors.launcher_right_bg,
            );
            container.append(&bat_box);
        }

        let has_bluetooth = std::path::Path::new("/usr/bin/bluetoothctl").exists();
        if has_bluetooth {
            let (_, _, bt_box) = create_block_box(
                "bluetooth-symbolic",
                "",
                &config.colors.launcher_left_bg,
                &config.colors.launcher_right_bg,
            );
            container.append(&bt_box);
        }

        let has_network = std::path::Path::new("/usr/bin/nmcli").exists()
            || std::path::Path::new("/usr/sbin/nmcli").exists();
        if has_network {
            let (_, _, net_box) = create_block_box(
                "network-wireless-symbolic",
                "",
                &config.colors.launcher_left_bg,
                &config.colors.launcher_right_bg,
            );
            container.append(&net_box);
        }

        let has_wifi = std::fs::read_dir("/sys/class/net")
            .ok()
            .map(|mut entries| {
                entries.any(|e| {
                    let name = e.ok()
                        .map(|f| f.file_name().to_string_lossy().to_string())
                        .unwrap_or_default()
                        .to_lowercase();
                    name.contains("wlan") || name.contains("wl")
                })
            })
            .unwrap_or(false);

        if has_wifi {
            let (_, _, wifi_box) = create_block_box(
                "network-wireless-symbolic",
                "",
                &config.colors.launcher_left_bg,
                &config.colors.launcher_right_bg,
            );
            container.append(&wifi_box);
        }

        let (_, _, tray_box) = create_block_box(
            "",
            "",
            &config.colors.launcher_left_bg,
            &config.colors.launcher_right_bg,
        );
        container.append(&tray_box);

        window.show();
    });

    app.run();
}
