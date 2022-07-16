use crate::block::Block;
use crate::source::{Context, Source};
use crate::style::{BaseColor, Brightness, Color, Style};

use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use dirs::home_dir;

const STYLE: Style = Style::new()
    .with_bg(Color::new(BaseColor::BLUE, Brightness::BRIGHT))
    .with_bold();

#[derive(Default)]
pub struct Cwd;

fn first_char(s: &str) -> &str {
    match s.char_indices().nth(1) {
        Some((end, _)) => &s[0..end],
        None => s,
    }
}

fn abbreviate_home<'p>(home: &Path, path: &'p Path) -> Cow<'p, Path> {
    match path.strip_prefix(home) {
        Ok(abbreviated) => Cow::Owned(PathBuf::from("~").join(abbreviated)),
        Err(_) => Cow::Borrowed(path),
    }
}

fn shorten(path: &Path) -> Option<PathBuf> {
    let mut components: Vec<&str> = path
        .iter()
        .map(|c: &OsStr| c.to_str().unwrap_or("\u{fffd}"))
        .collect();

    match components.len() {
        0 => None,
        len => {
            let mut shortened = PathBuf::new();
            if len > 4 {
                components.splice(2..len - 2, Some("…"));
            }

            let len = components.len();
            let (initials, last) = components.split_at(len.saturating_sub(1));
            for compoment in initials {
                shortened.push(first_char(compoment));
            }

            if let Some(component) = last.first() {
                shortened.push(component)
            }

            Some(shortened)
        }
    }
}

impl Cwd {
    fn shortened(&self, current_dir: &Path) -> Option<PathBuf> {
        let home = home_dir()?;
        let abbreviated = abbreviate_home(&home, current_dir);
        shorten(abbreviated.as_ref())
    }
}

impl Source for Cwd {
    fn get_block(&self, context: &Context) -> Option<Block> {
        self.shortened(context.current_dir())
            .map(|path: PathBuf| Block::new(path.display().to_string()).with_style(STYLE))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_abbreviated {
        (home = $home: expr, path = $path: expr, expected = $expected: expr $(,)?) => {
            let home = Path::new($home);
            let path = Path::new($path);
            let expected = Path::new($expected);

            assert_eq!(abbreviate_home(home, path), expected)
        };
    }

    #[test]
    fn abbreviate_home_self() {
        assert_abbreviated! {
            home = "/home/alice/",
            path = "/home/alice/",
            expected = "~",
        }
    }

    #[test]
    fn abbreviate_home_some_dir() {
        assert_abbreviated! {
            home = "/home/alice/",
            path = "/home/alice/foo/bar",
            expected = "~/foo/bar",
        }
    }

    #[test]
    fn abbreviate_home_no_subdir() {
        assert_abbreviated! {
            home = "/home/alice/",
            path = "/path/to/somewhere/else",
            expected = "/path/to/somewhere/else",
        }
    }

    macro_rules! assert_shortened {
        (path = $path: expr, expected = $expected: expr $(,)?) => {
            let path = Path::new($path);
            let expected = Some(Path::new($expected));

            assert_eq!(shorten(path).as_deref(), expected)
        };
    }

    #[test]
    fn shorten_empty() {
        assert_eq!(shorten(Path::new("")), None)
    }

    #[test]
    fn shorten_home() {
        assert_shortened! {
            path = "~",
            expected = "~",
        }
    }

    #[test]
    fn shorten_root() {
        assert_shortened! {
            path = "/",
            expected = "/",
        }
    }

    #[test]
    fn shorten_none() {
        assert_shortened! {
            path = "~/foo",
            expected = "~/foo",
        }
    }

    #[test]
    fn shorten_components_only() {
        assert_shortened! {
            path = "/path/to/foo",
            expected = "/p/t/foo",
        }
    }

    #[test]
    fn shorten_components_and_omit() {
        assert_shortened! {
            path = "/path/to/deply/nested/foo",
            expected = "/p/…/n/foo",
        }
    }

    #[test]
    fn abbreviate_and_shorten() {
        let home = Path::new("/home/alice");
        let path = Path::new("/home/alice/foo/bar");
        let expected = Path::new("~/f/bar");

        let abbreviated = abbreviate_home(home, path);
        let shortened = shorten(&abbreviated);

        assert_eq!(shortened.as_deref(), Some(expected));
    }

    #[test]
    fn abbreviate_and_shorten_with_omission() {
        let home = Path::new("/home/alice");
        let path = Path::new("/home/alice/deeply/nested/foo/bar");
        let expected = Path::new("~/d/…/f/bar");

        let abbreviated = abbreviate_home(home, path);
        let shortened = shorten(&abbreviated);

        assert_eq!(shortened.as_deref(), Some(expected));
    }
}
