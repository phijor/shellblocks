// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::block::Block;
use crate::source::{Context, Source};
use crate::style::{BaseColor, Brightness, Color, Style};

use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;
use std::path::{Path, PathBuf};
use std::process::Command;

const STYLE: Style = Style::new()
    .with_fg(Color::new(BaseColor::White, Brightness::Normal))
    .with_bg(Color::new(BaseColor::Black, Brightness::Bright))
    .with_bold();

pub struct GitDir {
    root: PathBuf,
}

#[derive(Default)]
pub struct Git;

#[derive(Debug, Clone, Copy)]
enum CurrentState {
    Normal,
    Rebasing,
    Merging,
}

trait CommandReadStdout {
    fn read_stdout(self) -> Option<Vec<u8>>;
}

impl CommandReadStdout for &mut Command {
    fn read_stdout(self) -> Option<Vec<u8>> {
        let output = self.output().ok()?;
        if output.status.success() {
            Some(output.stdout)
        } else {
            None
        }
    }
}

impl GitDir {
    fn find(path: &Path) -> Option<Self> {
        let output = Command::new("git")
            .arg("rev-parse")
            .arg("--show-toplevel")
            .current_dir(path)
            .read_stdout()?;

        let root = PathBuf::from(OsString::from_vec(output));
        Some(Self { root })
    }

    pub fn current_branch(&self) -> Option<String> {
        fn git(args: &[&'static str]) -> Command {
            let mut cmd = Command::new("git");
            cmd.args(args);
            cmd
        }

        [
            git(&["symbolic-ref", "--short", "HEAD"]),
            git(&["rev-parse", "--short", "HEAD"]),
        ]
        .into_iter()
        .find_map(|mut cmd| {
            let output = cmd.read_stdout()?;
            let branch = String::from_utf8(output).ok()?;
            Some(branch.trim().into())
        })
    }

    fn current_state(&self) -> CurrentState {
        for (indicator_file, state) in &[
            (".git/rebase-merge", CurrentState::Rebasing),
            (".git/MERGE_HEAD", CurrentState::Merging),
        ] {
            if self.root.join(indicator_file).exists() {
                return *state;
            }
        }

        CurrentState::Normal
    }
}

impl Source for Git {
    fn get_block(&self, context: &Context) -> Option<Block> {
        let gitdir = GitDir::find(context.current_dir())?;

        let branch = gitdir.current_branch()?;
        let (indicator, fg) = match gitdir.current_state() {
            CurrentState::Normal => ("", BaseColor::White),
            CurrentState::Rebasing => ("↥", BaseColor::Red),
            CurrentState::Merging => ("⥇", BaseColor::Yellow),
        };

        Some(
            Block::new(format!("{} {}", indicator, branch))
                .with_style(STYLE.with_fg(Color::new(fg, Brightness::Normal))),
        )
    }
}
