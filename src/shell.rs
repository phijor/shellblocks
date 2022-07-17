use crate::block::Block;
use crate::source::{Context, Source};
use crate::style::{BaseColor, Brightness, Color, Style};

const STYLE: Style = Style::new()
    .with_bg(Color::new(BaseColor::Black, Brightness::Bright))
    .with_fg(Color::new(BaseColor::Yellow, Brightness::Normal));

const SUBSCRIPTS: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];

#[derive(Default)]
pub struct ShellLevel;

impl Source for ShellLevel {
    fn get_block(&self, _: &Context) -> Option<Block> {
        let level: usize = std::env::var("SHLVL").ok()?.parse().ok()?;

        if level > 2 {
            let digit = SUBSCRIPTS.get(level).unwrap_or(&'₊');
            Some(Block::new(format!("🗗 {digit}")).with_style(STYLE))
        } else {
            None
        }
    }
}
