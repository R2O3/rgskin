#![allow(unused)]

use include_dir::{include_dir, Dir};

use crate::utils::io::get_filename;

static CURSORS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/resources/cursors");

pub struct Resources;

impl Resources {
    pub fn cursor(name: &str) -> Option<std::borrow::Cow<'static, [u8]>> {
        CURSORS.get_file(name).map(|f| std::borrow::Cow::Borrowed(f.contents()))
    }

    pub fn get_cursor_names() -> Vec<String> {
        CURSORS
            .files()
            .map(|f| get_filename(f.path().to_str().unwrap_or_default()))
            .collect()
    }
}