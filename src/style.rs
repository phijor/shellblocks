// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt;

#[allow(unused)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BaseColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Brightness {
    Normal,
    Bright,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub base: BaseColor,
    pub brightness: Brightness,
}

impl Color {
    pub const fn new(base: BaseColor, brightness: Brightness) -> Self {
        Self { base, brightness }
    }
}

#[derive(Default)]
pub struct Style {
    pub bold: bool,
    pub fg: Option<Color>,
    pub bg: Option<Color>,
}

impl Style {
    pub const fn new() -> Self {
        Self {
            bold: false,
            fg: None,
            bg: None,
        }
    }

    pub const fn with_fg(self, fg: Color) -> Self {
        Self {
            fg: Some(fg),
            ..self
        }
    }

    pub const fn with_bg(self, bg: Color) -> Self {
        Self {
            bg: Some(bg),
            ..self
        }
    }

    pub const fn with_bold(self) -> Self {
        Self { bold: true, ..self }
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color_val = |color: Option<Color>| match color {
            Some(c) => (c.base as i32) + 60 * (c.brightness as i32),
            None => 9,
        };
        let fg = 30 + color_val(self.fg);
        let bg = 40 + color_val(self.bg);

        write!(
            f,
            "%{{\x1b[{}{};{}m%}}",
            if self.bold { "1;" } else { "0;" },
            fg,
            bg
        )
    }
}
