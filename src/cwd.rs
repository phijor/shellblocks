use crate::block::Block;
use crate::source::Source;
use crate::style::{BaseColor, Brightness, Color};

use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use dirs::home_dir;

pub struct Cwd(PathBuf);

impl Cwd {
    pub fn new<P: AsRef<Path>>(path: &P) -> Self {
        Cwd(path.as_ref().to_path_buf())
    }
}

fn first_char(s: &str) -> &str {
    let end = s
        .char_indices()
        .nth(1)
        .map(|(end, _)| end)
        .unwrap_or(s.len());
    &s[0..end]
}

impl Cwd {
    fn shortened(&self) -> Option<PathBuf> {
        let home = home_dir()?;
        let path = match self.0.strip_prefix(home) {
            Ok(ref path) => Cow::Owned(PathBuf::from("~").join(path)),
            Err(_) => Cow::Borrowed(&self.0),
        };

        {
            let mut compoments: Vec<&str> = path
                .iter()
                .map(|c: &OsStr| c.to_str().unwrap_or(&"\u{fffd}"))
                .collect();
            match compoments.len() {
                0 => None,
                len => {
                    let mut shortened = PathBuf::new();
                    if len > 4 {
                        compoments.splice(2..len - 2, Some("…"));
                    }

                    let (initials, last) = compoments.split_at(len - 1);
                    for compoment in initials {
                        shortened.push(first_char(compoment));
                    }
                    shortened.push(last[0]);

                    Some(shortened)
                }
            }
        }
    }
}

impl Source for Cwd {
    fn get_block(&self) -> Option<Block> {
        self.shortened().map(|path: PathBuf| {
            Block::new(path.display().to_string())
                .with_bg(Color::new(BaseColor::BLUE, Brightness::BRIGHT))
                .with_bold()
        })
    }
}
