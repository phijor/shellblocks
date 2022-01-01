use crate::block::Block;
use crate::source::Source;
use crate::style::{BaseColor, Brightness, Color, Style};

use std::ffi::{CStr, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

#[derive(Default)]
pub struct Tty;

impl Source for Tty {
    fn get_block(&self) -> Option<Block> {
        let tty = unsafe {
            use libc::{c_char, ttyname, STDOUT_FILENO};
            let tty: *mut c_char = ttyname(STDOUT_FILENO);
            CStr::from_ptr(tty)
        };
        let tty = PathBuf::from(OsStr::from_bytes(tty.to_bytes()));

        let ttyname = tty.file_name()?.to_str()?;

        if ttyname.starts_with("tty") {
            let style = Style::new()
                .with_fg(Color::new(BaseColor::BLACK, Brightness::NORMAL))
                .with_bg(Color::new(BaseColor::YELLOW, Brightness::NORMAL));
            Some(Block::new(ttyname.to_owned()).with_style(style))
        } else {
            None
        }
    }
}
