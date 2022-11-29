use xdotool::{command::options::KeyboardOption, keyboard};

use super::models::{KeyPress, TextTyping};

pub fn press_key(press: KeyPress) {
    let mut options = vec![];

    if let Some(true) = press.clear_modifiers {
        options.push(KeyboardOption::ClearModifiers);
    }

    if let Some(delay) = press.delay {
        options.push(KeyboardOption::Delay(delay));
    }

    if let Some(window) = press.window {
        options.push(KeyboardOption::Window(window));
    }

    keyboard::send_key(&press.keystroke, xdotool::OptionVec(options));
}

pub fn type_text(typing: TextTyping) {
    let mut options = vec![];

    if let Some(true) = typing.clear_modifiers {
        options.push(KeyboardOption::ClearModifiers);
    }

    if let Some(delay) = typing.delay {
        options.push(KeyboardOption::Delay(delay));
    }

    if let Some(window) = typing.window {
        options.push(KeyboardOption::Window(window));
    }

    keyboard::type_text(&typing.text, xdotool::OptionVec(options));
}
