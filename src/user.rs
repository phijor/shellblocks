use crate::block::Block;
use crate::source::Source;
use crate::style::{BaseColor, Brightness, Color, Style};

use libc::{geteuid, getpwuid, passwd};

use std::ffi::CStr;

#[derive(Default)]
pub struct User;

impl Source for User {
    fn get_block(&self) -> Option<Block> {
        let euid = unsafe { geteuid() };

        let username: String = unsafe {
            let pw: *mut passwd = getpwuid(euid);
            if pw.is_null() {
                panic!("Cannot find pw entry for EUID {}", euid)
            } else {
                CStr::from_ptr((*pw).pw_name).to_string_lossy().into_owned()
            }
        };

        let bg_color = if euid == 0 {
            BaseColor::RED
        } else {
            BaseColor::BLUE
        };

        let style = Style::new()
            .with_fg(Color::new(BaseColor::BLACK, Brightness::NORMAL))
            .with_bg(Color::new(bg_color, Brightness::NORMAL));

        Some(Block::new(username).with_style(style))
    }
}
