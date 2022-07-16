// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::block::Block;
use crate::source::{Context, Source};
use crate::style::{BaseColor, Brightness, Color, Style};

use libc::{geteuid, getpwuid, passwd};

use std::ffi::CStr;

#[derive(Default)]
pub struct User;

impl Source for User {
    fn get_block(&self, _: &Context) -> Option<Block> {
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
            BaseColor::Red
        } else {
            BaseColor::Blue
        };

        let style = Style::new()
            .with_fg(Color::new(BaseColor::Black, Brightness::Normal))
            .with_bg(Color::new(bg_color, Brightness::Normal));

        Some(Block::new(username).with_style(style))
    }
}
