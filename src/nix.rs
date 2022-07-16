// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::block::Block;
use crate::source::{Context, Source};
use crate::style::{BaseColor, Brightness, Color, Style};

use std::env;

const STYLE: Style = Style::new()
    .with_bg(Color::new(BaseColor::Black, Brightness::Bright))
    .with_fg(Color::new(BaseColor::Blue, Brightness::Normal))
    .with_bold();

#[derive(Default)]
pub struct NixShell;

impl NixShell {
    pub fn is_in_nixshell(&self) -> bool {
        env::var_os("IN_NIX_SHELL").is_some()
    }
}

impl Source for NixShell {
    fn get_block(&self, _: &Context) -> Option<Block> {
        if self.is_in_nixshell() {
            Some(Block::new("λ".to_string()).with_style(STYLE))
        } else {
            None
        }
    }
}
