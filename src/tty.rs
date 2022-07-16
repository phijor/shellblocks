// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::block::Block;
use crate::source::{Context, Source};
use crate::style::{BaseColor, Brightness, Color, Style};

use std::env;
use std::ffi::{CStr, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

const STYLE: Style = Style::new()
    .with_fg(Color::new(BaseColor::Black, Brightness::Normal))
    .with_bg(Color::new(BaseColor::Yellow, Brightness::Normal));

#[derive(Default)]
pub struct Tty;

impl Tty {
    pub fn from_env() -> Option<PathBuf> {
        env::var_os("TTY").map(PathBuf::from)
    }

    pub fn from_system() -> Option<PathBuf> {
        let path = unsafe {
            use libc::{c_char, ttyname, STDIN_FILENO};
            let path: *mut c_char = ttyname(STDIN_FILENO);

            if path.is_null() {
                return None;
            } else {
                let path = CStr::from_ptr(path);
                PathBuf::from(OsStr::from_bytes(path.to_bytes()))
            }
        };

        Some(path)
    }
}

impl Source for Tty {
    fn get_block(&self, _: &Context) -> Option<Block> {
        let path = Self::from_system().or_else(Self::from_env)?;
        let ispts = path
            .iter()
            .any(|component| component.to_str().map(|c| c == "pts").unwrap_or(false));

        if !ispts {
            Some(Block::new(path.to_string_lossy().into()).with_style(STYLE))
        } else {
            None
        }
    }
}
