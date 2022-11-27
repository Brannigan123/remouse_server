use std::string::FromUtf8Error;
use xdotool::{
    command::options::{ClickOption, MouseMoveRelativeOption},
    mouse,
};

use super::models::{CursorMove, MouseAction};

pub fn get_cursor_position() -> Result<Vec<u32>, FromUtf8Error> {
    let output = mouse::get_mouse_location();

    String::from_utf8(output.stdout).map(|s| {
        println!("{}", &s);
        s.trim()
            .split(|c| c == ' ' || c == ':')
            .map(|p| p.parse::<u32>().unwrap_or_default())
            .collect::<Vec<u32>>()
    })
}

pub fn move_cursor(movement: CursorMove) {
    let mut options = vec![];

    let dx = movement.dx.unwrap_or_default();
    let dy = movement.dy.unwrap_or_default();

    if let Some(clear_modifiers) = movement.clear_modifiers && clear_modifiers {
        options.push(MouseMoveRelativeOption::ClearModifiers);
    }

    mouse::move_mouse_relative(dx, dy, xdotool::OptionVec(options));
}

pub fn click(click: MouseAction) {
    let mut options = vec![];

    let button = match click.button {
        0 => mouse::Button::Left,
        1 => mouse::Button::Middle,
        2 => mouse::Button::Right,
        3 => mouse::Button::ScrollUp,
        4 => mouse::Button::ScrollDown,
        _ => mouse::Button::Left,
    };

    if let Some(delay) = click.delay {
        options.push(ClickOption::Delay(delay));
    }

    if let Some(count) = click.count {
        options.push(ClickOption::Repeat(count));
    }

    mouse::click(button, xdotool::OptionVec(options));
}
