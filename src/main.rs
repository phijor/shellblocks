mod block;
mod cwd;
mod git;
mod pipenv;
mod source;
mod style;
mod user_at_host;

use crate::block::{format, Block};
use crate::cwd::Cwd;
use crate::git::Git;
use crate::pipenv::Pipenv;
use crate::source::Source;
use crate::user_at_host::UserAtHost;

use std::env;
use std::io::{self, Write};
use std::ops::Deref;

struct Blocks(Vec<Block>);

impl Blocks {
    pub fn from_names<I: Iterator<Item = String>>(names: I) -> Self {
        let mut blocks = Vec::new();
        let cwd = env::current_dir().unwrap_or_else(|_| ".".into());

        for name in names {
            let block: Option<Block> = match name.as_str() {
                "user-at-host" => UserAtHost::default().get_block(),
                "cwd" => Cwd::new(&cwd).get_block(),
                "git" => Git::new(&cwd).get_block(),
                "pipenv" => Pipenv::default().get_block(),
                _ => {
                    eprintln!("Unknown source {}", name);
                    continue;
                }
            };

            match block {
                Some(block) => blocks.push(block),
                None => {}
            }
        }

        Self(blocks)
    }
}

impl Deref for Blocks {
    type Target = Vec<Block>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() -> io::Result<()> {
    let blocks = Blocks::from_names(env::args().skip(1));
    let prompt = format(blocks.iter());
    io::stdout().write(prompt.as_bytes())?;

    Ok(())
}
