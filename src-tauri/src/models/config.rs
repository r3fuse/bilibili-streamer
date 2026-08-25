use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FloatWindowState {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Default for FloatWindowState {
    fn default() -> Self {
        Self {
            x: -1.0,
            y: -1.0,
            width: 320.0,
            height: 450.0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub current_uid: Option<u64>,
    pub users: HashMap<String, UserConfig>,
    #[serde(default = "default_min_to_tray")]
    pub min_to_tray: bool,
    #[serde(default)]
    pub float_window: Option<FloatWindowState>,
    #[serde(default = "default_disable_dmabuf_renderer")]
    pub disable_dmabuf_renderer:bool
}

impl Default for AppConfig {
    fn default() -> Self {
         Self {
            current_uid: None,
            users: HashMap::new(),
            min_to_tray: false,
            float_window: None,
            disable_dmabuf_renderer: true,
        }
    }
}

fn default_min_to_tray() -> bool {
    false
}

fn default_disable_dmabuf_renderer() -> bool {
    true
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserConfig {
    pub uid: u64,
    pub uname: String,
    pub face: String,
    pub cookie: String,
    pub room_id: String,
    pub csrf: String,
    #[serde(default)]
    pub last_title: String,
    #[serde(default)]
    pub last_area_id: u64,
    #[serde(default)]
    pub last_area_name: Vec<String>,
    #[serde(default)]
    pub level: u32,
    #[serde(default)]
    pub follower: u32,
    #[serde(default)]
    pub following: u32,
    #[serde(default)]
    pub dynamic_count: u32,
}
