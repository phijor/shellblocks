// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use lazycell::LazyCell;

use std::path::{Path, PathBuf};

use crate::block::Block;

pub trait Source {
    fn get_block(&self, context: &Context) -> Option<Block>;
}

#[derive(Debug, Default)]
pub struct Context {
    current_dir: LazyCell<PathBuf>,
}

fn get_current_dir() -> PathBuf {
    if let Some(dir) = std::env::var_os("SHELLBLOCKS_DIR") {
        PathBuf::from(dir)
    } else if let Ok(cwd) = std::env::current_dir() {
        cwd
    } else {
        ".".into()
    }
}

impl Context {
    pub fn current_dir(&self) -> &Path {
        self.current_dir.borrow_with(get_current_dir)
    }
}
