use crate::block::Block;
use crate::source::Source;
use crate::style::{BaseColor, Brightness, Color, Style};

use std::fs::read_to_string;
use std::path::{Path, PathBuf};

const STYLE: Style = Style::new()
    .with_fg(Color::new(BaseColor::WHITE, Brightness::NORMAL))
    .with_bg(Color::new(BaseColor::BLACK, Brightness::BRIGHT))
    .with_bold();

pub struct Git {
    repo_root: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy)]
enum CurrentState {
    Normal,
    Rebasing,
    Merging,
}

impl Git {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            repo_root: Self::find_root(path.as_ref()),
        }
    }

    fn find_root(path: &Path) -> Option<PathBuf> {
        path.ancestors()
            .find(|candidate| candidate.join(".git").exists())
            .map(Path::to_path_buf)
    }

    fn root(&self) -> Option<&Path> {
        self.repo_root.as_ref().map(|root| root.as_ref())
    }

    pub fn current_brach(&self) -> Option<String> {
        let head = read_to_string(self.root()?.join(".git/HEAD")).ok()?;
        Some(Self::shorten(head.trim_end().trim_start_matches("ref: refs/heads/")).to_string())
    }

    fn shorten(ref_spec: &str) -> &str {
        if ref_spec.chars().all(|c| c.is_ascii_hexdigit()) {
            &ref_spec[..7]
        } else {
            ref_spec
        }
    }

    fn current_state(&self) -> Option<CurrentState> {
        self.root().map(|root| {
            for (indicator_file, state) in &[
                (".git/rebase-merge", CurrentState::Rebasing),
                (".git/MERGE_HEAD", CurrentState::Merging),
            ] {
                if root.join(indicator_file).exists() {
                    return *state;
                }
            }

            CurrentState::Normal
        })
    }
}

impl Source for Git {
    fn get_block(&self) -> Option<Block> {
        let branch = self.current_brach()?;
        let (indicator, fg) = match self.current_state()? {
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
