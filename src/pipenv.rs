use crate::block::Block;
use crate::source::Source;
use crate::style::{BaseColor, Brightness, Color};

use std::env;

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
    fn get_block(&self) -> Option<Block> {
        if self.is_pipenv() {
            Some(
                Block::new("🐍".to_string())
                    .with_bg(Color::new(BaseColor::BLACK, Brightness::BRIGHT))
                    .with_fg(Color::new(BaseColor::GREEN, Brightness::NORMAL))
                    .with_bold(),
            )
        } else {
            None
        }
    }
}
