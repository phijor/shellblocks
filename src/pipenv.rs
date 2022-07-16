use crate::block::Block;
use crate::source::{Context, Source};
use crate::style::{BaseColor, Brightness, Color, Style};

use std::env;

const STYLE: Style = Style::new()
    .with_bg(Color::new(BaseColor::Black, Brightness::Bright))
    .with_fg(Color::new(BaseColor::Green, Brightness::Normal))
    .with_bold();

#[derive(Default)]
pub struct Pipenv;

impl Pipenv {
    pub fn is_pipenv(&self) -> bool {
        match env::var_os("PIPENV_ACTIVE") {
            Some(ref value) => value == "1",
            _ => false,
        }
    }
}

impl Source for Pipenv {
    fn get_block(&self, _: &Context) -> Option<Block> {
        if self.is_pipenv() {
            Some(Block::new("🐍".to_string()).with_style(STYLE))
        } else {
            None
        }
    }
}
