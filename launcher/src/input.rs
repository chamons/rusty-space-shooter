use macroquad::prelude::*;

#[cfg(feature = "hotreload")]
use crate::hotreload::binding::example::game::types::{
    ClickInfo, Key, KeyboardInfo, MouseInfo, Position,
};

#[cfg(not(feature = "hotreload"))]
pub use game::example::game::types::{ClickInfo, Key, KeyboardInfo, MouseInfo, Position};

pub fn get_key_info() -> KeyboardInfo {
    KeyboardInfo {
        pressed: get_keys_pressed().into_iter().map(map_key).collect(),
        released: get_keys_released().into_iter().map(map_key).collect(),
        down: get_keys_down().into_iter().map(map_key).collect(),
    }
}

pub fn map_key(key: KeyCode) -> Key {
    match key {
        KeyCode::Space => Key::Space,
        KeyCode::Apostrophe => Key::Apostrophe,
        KeyCode::Comma => Key::Comma,
        KeyCode::Minus => Key::Minus,
        KeyCode::Period => Key::Period,
        KeyCode::Slash => Key::Slash,
        KeyCode::Key0 => Key::KeyZero,
        KeyCode::Key1 => Key::KeyOne,
        KeyCode::Key2 => Key::KeyTwo,
        KeyCode::Key3 => Key::KeyThree,
        KeyCode::Key4 => Key::KeyFour,
        KeyCode::Key5 => Key::KeyFive,
        KeyCode::Key6 => Key::KeySix,
        KeyCode::Key7 => Key::KeySeven,
        KeyCode::Key8 => Key::KeyEight,
        KeyCode::Key9 => Key::KeyNine,
        KeyCode::Semicolon => Key::Semicolon,
        KeyCode::Equal => Key::Equal,
        KeyCode::A => Key::A,
        KeyCode::B => Key::B,
        KeyCode::C => Key::C,
        KeyCode::D => Key::D,
        KeyCode::E => Key::E,
        KeyCode::F => Key::F,
        KeyCode::G => Key::G,
        KeyCode::H => Key::H,
        KeyCode::I => Key::I,
        KeyCode::J => Key::J,
        KeyCode::K => Key::K,
        KeyCode::L => Key::L,
        KeyCode::M => Key::M,
        KeyCode::N => Key::N,
        KeyCode::O => Key::O,
        KeyCode::P => Key::P,
        KeyCode::Q => Key::Q,
        KeyCode::R => Key::R,
        KeyCode::S => Key::S,
        KeyCode::T => Key::T,
        KeyCode::U => Key::U,
        KeyCode::V => Key::V,
        KeyCode::W => Key::W,
        KeyCode::X => Key::X,
        KeyCode::Y => Key::Y,
        KeyCode::Z => Key::Z,
        KeyCode::LeftBracket => Key::LeftBracket,
        KeyCode::Backslash => Key::Backslash,
        KeyCode::RightBracket => Key::RightBracket,
        KeyCode::GraveAccent => Key::GraveAccent,
        KeyCode::World1 => Key::WorldOne,
        KeyCode::World2 => Key::WorldTwo,
        KeyCode::Escape => Key::Escape,
        KeyCode::Enter => Key::Enter,
        KeyCode::Tab => Key::Tab,
        KeyCode::Backspace => Key::Backspace,
        KeyCode::Insert => Key::Insert,
        KeyCode::Delete => Key::Delete,
        KeyCode::Right => Key::Right,
        KeyCode::Left => Key::Left,
        KeyCode::Down => Key::Down,
        KeyCode::Up => Key::Up,
        KeyCode::PageUp => Key::PageUp,
        KeyCode::PageDown => Key::PageDown,
        KeyCode::Home => Key::Home,
        KeyCode::End => Key::End,
        KeyCode::CapsLock => Key::CapsLock,
        KeyCode::ScrollLock => Key::ScrollLock,
        KeyCode::NumLock => Key::NumLock,
        KeyCode::PrintScreen => Key::PrintScreen,
        KeyCode::Pause => Key::Pause,
        KeyCode::F1 => Key::F1,
        KeyCode::F2 => Key::F2,
        KeyCode::F3 => Key::F3,
        KeyCode::F4 => Key::F4,
        KeyCode::F5 => Key::F5,
        KeyCode::F6 => Key::F6,
        KeyCode::F7 => Key::F7,
        KeyCode::F8 => Key::F8,
        KeyCode::F9 => Key::F9,
        KeyCode::F10 => Key::F10,
        KeyCode::F11 => Key::F11,
        KeyCode::F12 => Key::F12,
        KeyCode::F13 => Key::F13,
        KeyCode::F14 => Key::F14,
        KeyCode::F15 => Key::F15,
        KeyCode::F16 => Key::F16,
        KeyCode::F17 => Key::F17,
        KeyCode::F18 => Key::F18,
        KeyCode::F19 => Key::F19,
        KeyCode::F20 => Key::F20,
        KeyCode::F21 => Key::F21,
        KeyCode::F22 => Key::F22,
        KeyCode::F23 => Key::F23,
        KeyCode::F24 => Key::F24,
        KeyCode::F25 => Key::F25,
        KeyCode::Kp0 => Key::Kp0,
        KeyCode::Kp1 => Key::Kp1,
        KeyCode::Kp2 => Key::Kp2,
        KeyCode::Kp3 => Key::Kp3,
        KeyCode::Kp4 => Key::Kp4,
        KeyCode::Kp5 => Key::Kp5,
        KeyCode::Kp6 => Key::Kp6,
        KeyCode::Kp7 => Key::Kp7,
        KeyCode::Kp8 => Key::Kp8,
        KeyCode::Kp9 => Key::Kp9,
        KeyCode::KpDecimal => Key::KpDecimal,
        KeyCode::KpDivide => Key::KpDivide,
        KeyCode::KpMultiply => Key::KpMultiply,
        KeyCode::KpSubtract => Key::KpSubtract,
        KeyCode::KpAdd => Key::KpAdd,
        KeyCode::KpEnter => Key::KpEnter,
        KeyCode::KpEqual => Key::KpEqual,
        KeyCode::LeftShift => Key::LeftShift,
        KeyCode::LeftControl => Key::LeftControl,
        KeyCode::LeftAlt => Key::LeftAlt,
        KeyCode::LeftSuper => Key::LeftSuper,
        KeyCode::RightShift => Key::RightShift,
        KeyCode::RightControl => Key::RightControl,
        KeyCode::RightAlt => Key::RightAlt,
        KeyCode::RightSuper => Key::RightSuper,
        KeyCode::Menu => Key::Menu,
        KeyCode::Unknown => Key::Unknown,
    }
}

pub fn get_mouse_state() -> MouseInfo {
    let mouse_position = mouse_position();
    MouseInfo {
        position: Position {
            x: mouse_position.0,
            y: mouse_position.1,
        },
        left: ClickInfo {
            pressed: is_mouse_button_pressed(MouseButton::Left),
            released: is_mouse_button_released(MouseButton::Left),
            down: is_mouse_button_down(MouseButton::Left),
        },
        right: ClickInfo {
            pressed: is_mouse_button_pressed(MouseButton::Right),
            released: is_mouse_button_released(MouseButton::Right),
            down: is_mouse_button_down(MouseButton::Right),
        },
        middle: ClickInfo {
            pressed: is_mouse_button_pressed(MouseButton::Middle),
            released: is_mouse_button_released(MouseButton::Middle),
            down: is_mouse_button_down(MouseButton::Middle),
        },
    }
}