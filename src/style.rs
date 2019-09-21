use std::fmt;

#[allow(unused)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BaseColor {
    BLACK,
    RED,
    GREEN,
    YELLOW,
    BLUE,
    MAGENTA,
    CYAN,
    WHITE,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Brightness {
    NORMAL,
    BRIGHT,
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
            "\x1b[{}{};{}m",
            if self.bold { "1;" } else { "0;" },
            fg,
            bg
        )
    }
}
