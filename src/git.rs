use crate::block::Block;
use crate::source::Source;
use crate::style::{BaseColor, Brightness, Color};

use std::fs::read_to_string;
use std::path::{Path, PathBuf};

pub struct Git {
    path: PathBuf,
}

impl Git {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn current_brach(&self) -> Option<String> {
        for candidate in self.path.ancestors() {
            match read_to_string(candidate.join(".git/HEAD")) {
                Err(_) => continue,
                Ok(head) => {
                    return Some(
                        head.trim_end()
                            .trim_start_matches("ref: refs/heads/")
                            .to_string(),
                    )
                }
            }
        }

        None
    }
}

impl Source for Git {
    fn get_block(&self) -> Option<Block> {
        self.current_brach().map(|branch| {
            Block::new(format!(" {}", branch))
                .with_fg(Color::new(BaseColor::WHITE, Brightness::NORMAL))
                .with_bg(Color::new(BaseColor::BLACK, Brightness::BRIGHT))
                .with_bold()
        })
    }
}
