// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::style::Style;

use std::fmt;

pub struct Block {
    value: String,
    style: Style,
}

impl Block {
    pub fn new(value: String) -> Self {
        Block {
            value,
            style: Style::default(),
        }
    }

    pub fn with_style(self, style: Style) -> Self {
        Block { style, ..self }
    }

    pub fn style(&self) -> &Style {
        &self.style
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} ", self.style, self.value)
    }
}

pub fn format<'a, I: Iterator<Item = &'a Block>>(blocks: I) -> String {
    fn separator(current_style: &Style, next_style: Option<&Style>) -> String {
        match next_style {
            None => format!(
                "{} %{{\x1b[0m%}}",
                Style {
                    bold: false,
                    fg: current_style.bg,
                    bg: None,
                }
            ),
            Some(next_style) => {
                if current_style.bg != next_style.bg {
                    format!(
                        "{}",
                        Style {
                            fg: current_style.bg,
                            bg: next_style.bg,
                            bold: false
                        }
                    )
                } else {
                    format!("{}", current_style)
                }
            }
        }
    }

    let mut blocks = blocks.peekable();
    let mut output = String::new();
    while let Some(block) = blocks.next() {
        output.push_str(&block.to_string());

        let current_style: &Style = block.style();
        let next_style: Option<&Style> = blocks.peek().map(|next_block| next_block.style());

        output.push_str(&separator(current_style, next_style))
    }

    output
}
