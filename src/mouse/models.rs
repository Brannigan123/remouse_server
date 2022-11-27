use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CursorPosition {
    pub x: u32,
    pub y: u32,
    pub screen: u16,
    pub window: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CursorMove {
    pub dx: Option<i16>,
    pub dy: Option<i16>,
    pub clear_modifiers: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MouseAction {
    pub button: i8,
    pub count: Option<u32>,
    pub delay: Option<u32>,
}
