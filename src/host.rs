// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::block::Block;
use crate::source::{Context, Source};
use crate::style::{BaseColor, Brightness, Color, Style};

use nix::unistd::gethostname;

#[derive(Default)]
pub struct Host;

impl Source for Host {
    fn get_block(&self, _: &Context) -> Option<Block> {
        let hostname = gethostname().ok()?.into_string().ok()?;

        Some(
            Block::new(hostname).with_style(
                Style::new()
                    .with_fg(Color::new(BaseColor::Black, Brightness::Normal))
                    .with_bg(Color::new(BaseColor::Blue, Brightness::Normal)),
            ),
        )
    }
}
