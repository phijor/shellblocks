use crate::block::Block;
use crate::source::Source;
use crate::style::{BaseColor, Brightness, Color, Style};

use std::env;

const STYLE: Style = Style::new()
    .with_bg(Color::new(BaseColor::BLACK, Brightness::BRIGHT))
    .with_fg(Color::new(BaseColor::BLUE, Brightness::NORMAL))
    .with_bold();

#[derive(Default)]
pub struct NixShell;

impl NixShell {
    pub fn is_in_nixshell(&self) -> bool {
        env::var_os("IN_NIX_SHELL").is_some()
    }
}

impl Source for NixShell {
    fn get_block(&self) -> Option<Block> {
        if self.is_in_nixshell() {
            Some(Block::new("λ".to_string()).with_style(STYLE))
        } else {
            None
        }
    }
}
