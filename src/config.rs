use std::path::Path;
use std::collections::HashMap;

pub struct Colors {
    pub background: String,
    pub launcher_left_bg: String,
    pub launcher_right_bg: String,
    pub user_left_bg: String,
    pub user_right_bg: String,
    pub date_left_bg: String,
    pub date_right_bg: String,
    pub text_fg: String,
}

pub struct BlockActions {
    pub left_click: String,
    pub right_click: String,
    pub middle_click: String,
    pub scroll_up: String,
    pub scroll_down: String,
}

pub struct BlockConfig {
    pub icon: Option<String>,
    pub text: Option<String>,
    pub actions: BlockActions,
    pub has_right_part: bool,
}

pub struct Config {
    pub colors: Colors,
    pub blocks: HashMap<String, BlockConfig>,
}

impl Config {
    pub fn load(config_path: &Path) -> Self {
        let content = std::fs::read_to_string(config_path).unwrap_or_else(|e| {
            eprintln!("Failed to read config: {}", e);
            std::process::exit(1);
        });
        let value: toml::Value = content.parse().unwrap_or_else(|e| {
            eprintln!("Failed to parse config: {}", e);
            std::process::exit(1);
        });

        let colors = value.get("colors").map(|v| {
            let c = v.as_table().unwrap();
            Colors {
                background: c.get("background").and_then(|v| v.as_str()).unwrap_or("#2b2b2b").to_string(),
                launcher_left_bg: c.get("launcher_left_bg").and_then(|v| v.as_str()).unwrap_or("#3b3b3b").to_string(),
                launcher_right_bg: c.get("launcher_right_bg").and_then(|v| v.as_str()).unwrap_or("#4b3b3b").to_string(),
                user_left_bg: c.get("user_left_bg").and_then(|v| v.as_str()).unwrap_or("#3b3b3b").to_string(),
                user_right_bg: c.get("user_right_bg").and_then(|v| v.as_str()).unwrap_or("#4b3b3b").to_string(),
                date_left_bg: c.get("date_left_bg").and_then(|v| v.as_str()).unwrap_or("#3b3b3b").to_string(),
                date_right_bg: c.get("date_right_bg").and_then(|v| v.as_str()).unwrap_or("#4b3b3b").to_string(),
                text_fg: c.get("text_fg").and_then(|v| v.as_str()).unwrap_or("#e0e0e0").to_string(),
            }
        }).unwrap_or_else(|| Colors {
            background: "#2b2b2b".to_string(),
            launcher_left_bg: "#3b3b3b".to_string(),
            launcher_right_bg: "#4b3b3b".to_string(),
            user_left_bg: "#3b3b3b".to_string(),
            user_right_bg: "#4b3b3b".to_string(),
            date_left_bg: "#3b3b3b".to_string(),
            date_right_bg: "#4b3b3b".to_string(),
            text_fg: "#e0e0e0".to_string(),
        });

        let blocks = value.get("blocks").and_then(|v| v.as_table()).cloned().unwrap_or_default();
        let mut block_configs = HashMap::new();

        // launcher
        block_configs.insert("launcher".to_string(), Self::parse_block("launcher", &blocks));

        // user
        block_configs.insert("user".to_string(), Self::parse_block("user", &blocks));

        // date_time
        block_configs.insert("date_time".to_string(), Self::parse_block("date_time", &blocks));

        // wifi
        block_configs.insert("wifi".to_string(), Self::parse_block("wifi", &blocks));

        // network
        block_configs.insert("network".to_string(), Self::parse_block("network", &blocks));

        // bluetooth
        block_configs.insert("bluetooth".to_string(), Self::parse_block("bluetooth", &blocks));

        // battery
        block_configs.insert("battery".to_string(), Self::parse_block("battery", &blocks));

        // power
        block_configs.insert("power".to_string(), Self::parse_block("power", &blocks));

        // system_tray (special - no actions)
        block_configs.insert("system_tray".to_string(), BlockConfig {
            icon: None,
            text: None,
            actions: BlockActions {
                left_click: "".to_string(),
                right_click: "".to_string(),
                middle_click: "".to_string(),
                scroll_up: "".to_string(),
                scroll_down: "".to_string(),
            },
            has_right_part: false,
        });

        Config { colors, blocks: block_configs }
    }

    fn parse_block(name: &str, blocks: &toml::Value) -> BlockConfig {
        let block = blocks.get(name);
        let (icon, text, has_right_part) = if let Some(b) = block {
            let b = b.as_table().unwrap();
            (
                b.get("icon").and_then(|v| v.as_str()).map(|s| s.to_string()),
                b.get("text").and_then(|v| v.as_str()).map(|s| s.to_string()),
                b.get("has_right_part").and_then(|v| v.as_bool()).unwrap_or(true),
            )
        } else {
            (None, None, true)
        };

        let actions = BlockActions {
            left_click: block
                .and_then(|b| b.get("left_click").and_then(|v| v.as_str()))
                .unwrap_or("")
                .to_string(),
            right_click: block
                .and_then(|b| b.get("right_click").and_then(|v| v.as_str()))
                .unwrap_or("")
                .to_string(),
            middle_click: block
                .and_then(|b| b.get("middle_click").and_then(|v| v.as_str()))
                .unwrap_or("")
                .to_string(),
            scroll_up: block
                .and_then(|b| b.get("scroll_up").and_then(|v| v.as_str()))
                .unwrap_or("")
                .to_string(),
            scroll_down: block
                .and_then(|b| b.get("scroll_down").and_then(|v| v.as_str()))
                .unwrap_or("")
                .to_string(),
        };

        BlockConfig {
            icon,
            text,
            actions,
            has_right_part,
        }
    }
}