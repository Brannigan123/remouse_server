use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct KeyPress {
    pub keystroke: String,
    pub delay: Option<u32>,
    pub window: Option<String>,
    pub clear_modifiers: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TextTyping {
    pub text: String,
    pub delay: Option<u32>,
    pub window: Option<String>,
    pub clear_modifiers: Option<bool>,
}
