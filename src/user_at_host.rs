use crate::block::Block;
use crate::source::Source;
use crate::style::{BaseColor, Brightness, Color};

use gethostname::gethostname;
use libc::{geteuid, getpwuid, passwd};

use std::ffi::CStr;

#[derive(Default)]
pub struct UserAtHost;

impl Source for UserAtHost {
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
        let hostname = gethostname();

        let bg_color = if euid == 0 {
            BaseColor::RED
        } else {
            BaseColor::BLUE
        };

        Some(
            Block::new(format!("{}@{}", username, hostname.to_str()?))
                .with_fg(Color::new(BaseColor::BLACK, Brightness::NORMAL))
                .with_bg(Color::new(bg_color, Brightness::NORMAL)),
        )
    }
}
