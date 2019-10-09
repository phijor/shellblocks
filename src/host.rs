use crate::block::Block;
use crate::source::Source;
use crate::style::{BaseColor, Brightness, Color, Style};

use gethostname::gethostname;

#[derive(Default)]
pub struct Host;

impl Source for Host {
    fn get_block(&self) -> Option<Block> {
        Some(
            Block::new(gethostname().to_str()?.to_string()).with_style(
                Style::new()
                    .with_fg(Color::new(BaseColor::BLACK, Brightness::NORMAL))
                    .with_bg(Color::new(BaseColor::BLUE, Brightness::NORMAL)),
            ),
        )
    }
}
