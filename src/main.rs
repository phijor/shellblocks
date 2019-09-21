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
use std::ops::Deref;

fn main() {
    struct Blocks(Vec<Block>);

    impl Blocks {
        pub fn add_from<S: Source>(&mut self, source: S) {
            source.get_block().map(|block| self.0.push(block));
        }
    }

    impl Deref for Blocks {
        type Target = Vec<Block>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let mut blocks = Blocks(Vec::new());

    let cwd = env::current_dir().unwrap_or_else(|_| ".".into());

    blocks.add_from(UserAtHost::default());
    blocks.add_from(Cwd::new(&cwd));
    blocks.add_from(Git::new(&cwd));
    blocks.add_from(Pipenv::default());

    let prompt = format(blocks.iter());
    // println!("{:?}", prompt);
    print!("{}", prompt);
}
