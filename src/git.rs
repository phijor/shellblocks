use crate::block::Block;
use crate::source::{Context, Source};
use crate::style::{BaseColor, Brightness, Color, Style};

use std::fs::read_to_string;
use std::path::Path;

const STYLE: Style = Style::new()
    .with_fg(Color::new(BaseColor::WHITE, Brightness::NORMAL))
    .with_bg(Color::new(BaseColor::BLACK, Brightness::BRIGHT))
    .with_bold();

pub struct GitDir<'r> {
    root: &'r Path,
}

#[derive(Default)]
pub struct Git;

#[derive(Debug, Clone, Copy)]
enum CurrentState {
    Normal,
    Rebasing,
    Merging,
}

impl<'r> GitDir<'r> {
    fn find(path: &'r Path) -> Option<Self> {
        let root = path
            .ancestors()
            .find(|candidate| candidate.join(".git").exists())?;
        Some(Self { root })
    }

    pub fn current_branch(&self) -> Option<String> {
        let head = read_to_string(self.root.join(".git/HEAD")).ok()?;
        Some(Self::shorten(head.trim_end().trim_start_matches("ref: refs/heads/")).to_string())
    }

    fn shorten(ref_spec: &str) -> &str {
        if ref_spec.chars().all(|c| c.is_ascii_hexdigit()) {
            &ref_spec[..7]
        } else {
            ref_spec
        }
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
            CurrentState::Normal => ("", BaseColor::WHITE),
            CurrentState::Rebasing => ("↥", BaseColor::RED),
            CurrentState::Merging => ("⥇", BaseColor::YELLOW),
        };

        Some(
            Block::new(format!("{} {}", indicator, branch))
                .with_style(STYLE.with_fg(Color::new(fg, Brightness::NORMAL))),
        )
    }
}
