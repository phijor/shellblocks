// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::block::Block;
use crate::source::{Context, Source};
use crate::style::{BaseColor, Brightness, Color, Style};

use nix::unistd::{self, geteuid};

#[derive(Default)]
pub struct User;

impl Source for User {
    fn get_block(&self, _: &Context) -> Option<Block> {
        let euid = geteuid();
        let user = unistd::User::from_uid(euid).ok()??;

        let bg_color = if euid.is_root() {
            BaseColor::Red
        } else {
            BaseColor::Blue
        };

        let style = Style::new()
            .with_fg(Color::new(BaseColor::Black, Brightness::Normal))
            .with_bg(Color::new(bg_color, Brightness::Normal));

        Some(Block::new(user.name).with_style(style))
    }
}
