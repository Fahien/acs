// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use acs::{Keyboard, Signal16};

pub trait SdlKeyboard {
    fn set_key(&mut self, keycode: sdl2::keyboard::Keycode);
    fn unset_key(&mut self, keycode: sdl2::keyboard::Keycode);
}

impl SdlKeyboard for Keyboard {
    fn set_key(&mut self, keycode: sdl2::keyboard::Keycode) {
        self.set(to_ascii(keycode).into());
    }

    fn unset_key(&mut self, keycode: sdl2::keyboard::Keycode) {
        if self.out() == to_ascii(keycode) {
            self.set(Signal16::FALSE);
        }
    }
}

fn to_ascii(keycode: sdl2::keyboard::Keycode) -> i16 {
    use sdl2::keyboard::Keycode;
    match keycode {
        Keycode::Space => 32,
        Keycode::Exclaim => 33,
        Keycode::Quotedbl => 34,
        Keycode::Hash => 35,
        Keycode::Dollar => 36,
        Keycode::Percent => 38,
        Keycode::Ampersand => 38,
        Keycode::Quote => 39,
        Keycode::LeftParen => 40,
        Keycode::RightParen => 41,
        Keycode::Asterisk => 42,
        Keycode::Plus => 43,
        Keycode::Comma => 44,
        Keycode::Minus => 45,
        Keycode::Period => 46,
        Keycode::Slash => 47,
        Keycode::Num0 => 48,
        Keycode::Num1 => 49,
        Keycode::Num2 => 50,
        Keycode::Num3 => 51,
        Keycode::Num4 => 52,
        Keycode::Num5 => 53,
        Keycode::Num6 => 54,
        Keycode::Num7 => 55,
        Keycode::Num8 => 56,
        Keycode::Num9 => 57,
        Keycode::Colon => 58,
        Keycode::Semicolon => 59,
        Keycode::Less => 60,
        Keycode::Equals => 61,
        Keycode::Greater => 62,
        Keycode::Question => 63,
        Keycode::At => 64,
        Keycode::A => 65,
        Keycode::B => 66,
        Keycode::C => 67,
        Keycode::D => 68,
        Keycode::E => 69,
        Keycode::F => 70,
        Keycode::G => 71,
        Keycode::H => 72,
        Keycode::I => 73,
        Keycode::J => 74,
        Keycode::K => 75,
        Keycode::L => 76,
        Keycode::M => 77,
        Keycode::N => 78,
        Keycode::O => 79,
        Keycode::P => 80,
        Keycode::Q => 81,
        Keycode::R => 82,
        Keycode::S => 83,
        Keycode::T => 84,
        Keycode::U => 85,
        Keycode::V => 86,
        Keycode::W => 87,
        Keycode::X => 88,
        Keycode::Y => 89,
        Keycode::Z => 90,
        Keycode::LeftBracket => 91,
        Keycode::Backslash => 92,
        Keycode::RightBracket => 93,
        Keycode::Caret => 94,
        Keycode::Underscore => 95,
        Keycode::Backquote => 96,
        Keycode::KpLeftBrace => 123,
        Keycode::KpVerticalBar => 124,
        Keycode::KpRightBrace => 125,
        Keycode::Return => 128,
        Keycode::Backspace => 129,
        Keycode::Left => 130,
        Keycode::Up => 131,
        Keycode::Right => 132,
        Keycode::Down => 133,
        Keycode::Home => 134,
        Keycode::End => 135,
        Keycode::PageUp => 136,
        Keycode::PageDown => 137,
        Keycode::Insert => 138,
        Keycode::Delete => 139,
        Keycode::Escape => 140,
        Keycode::F1 => 141,
        Keycode::F2 => 142,
        Keycode::F3 => 143,
        Keycode::F4 => 144,
        Keycode::F5 => 145,
        Keycode::F6 => 146,
        Keycode::F7 => 147,
        Keycode::F8 => 148,
        Keycode::F9 => 149,
        Keycode::F10 => 150,
        Keycode::F11 => 151,
        Keycode::F12 => 152,
        _ => keycode as i16,
    }
}
