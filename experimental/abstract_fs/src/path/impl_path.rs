// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use std::path::{Path, PathBuf, Iter, MAIN_SEPARATOR};

pub struct StdPathIterator<'a, P> {
    raw_path: &'a P,
    iter: Option<Iter<'a>>,
}

impl<'a, P: AsRef<Path>> PathIterator for StdPathIterator<'a, P> {
    fn reset(&mut self) {
        self.iter.replace(self.raw_path.as_ref().iter());
    }

    fn next(&mut self) -> Option<&str> {
        let iter = self.iter.get_or_insert(self.raw_path.as_ref().iter());
        let mut result = iter.next().map(|s| s.to_str()).flatten();
        let mut tmp = [0; 4];
        if result == Some(MAIN_SEPARATOR.encode_utf8(&mut tmp)) {
            result = iter.next().map(|s| s.to_str()).flatten();
        }
        result
    }

    fn is_absolute(&self) -> bool {
        self.raw_path.as_ref().is_absolute()
    }
}

impl<'a, P: AsRef<Path>> From<&'a P> for StdPathIterator<'a, P> {
    fn from(path: &'a P) -> Self {
        Self {
            raw_path: path,
            iter: None,
        }
    }
}

#[test]
fn test_relative() {
    let path = PathBuf::from("foo/bar/baz.txt");
    let mut iter: StdPathIterator<PathBuf> = (&path).into();

    assert!(!iter.is_absolute());
    assert_eq!(iter.next(), Some("foo"));
    assert_eq!(iter.next(), Some("bar"));
    assert_eq!(iter.next(), Some("baz.txt"));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    iter.reset();
    assert!(!iter.is_absolute());
    assert_eq!(iter.next(), Some("foo"));
    assert_eq!(iter.next(), Some("bar"));
    assert_eq!(iter.next(), Some("baz.txt"));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_absolute() {
    let path = PathBuf::from("/foo/bar/baz.txt");
    let mut iter: StdPathIterator<PathBuf> = (&path).into();

    assert!(iter.is_absolute());
    assert_eq!(iter.next(), Some("foo"));
    assert_eq!(iter.next(), Some("bar"));
    assert_eq!(iter.next(), Some("baz.txt"));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    iter.reset();
    assert!(iter.is_absolute());
    assert_eq!(iter.next(), Some("foo"));
    assert_eq!(iter.next(), Some("bar"));
    assert_eq!(iter.next(), Some("baz.txt"));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}
