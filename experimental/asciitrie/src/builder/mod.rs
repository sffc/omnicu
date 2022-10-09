// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! ```
//! use asciitrie::AsciiStr;
//! use asciitrie::AsciiTrie;
//!
//! let trie: AsciiTrie<Vec<u8>> = [
//!     ("foo", 1),
//!     ("bar", 2),
//!     ("bazzoo", 3),
//!     ("internationalization", 18),
//! ]
//! .into_iter()
//! .map(AsciiStr::try_from_str_with_value)
//! .collect::<Result<_, _>>()
//! .unwrap();
//!
//! assert_eq!(trie.get(b"foo"), Some(1));
//! assert_eq!(trie.get(b"bar"), Some(2));
//! assert_eq!(trie.get(b"bazzoo"), Some(3));
//! assert_eq!(trie.get(b"internationalization"), Some(18));
//! assert_eq!(trie.get(b"unknown"), None);
//! ```

mod asciistr;
mod builder;

pub use asciistr::AsciiStr;
pub use asciistr::NonAsciiError;

use super::AsciiTrie;
use alloc::vec::Vec;
use asciistr::AsciiByte;
use builder::AsciiTrieBuilder;
use litemap::LiteMap;

impl<'a> FromIterator<(&'a AsciiStr, usize)> for AsciiTrie<Vec<u8>> {
    /// **Enable this impl with the `"builder"` feature.**
    fn from_iter<T: IntoIterator<Item = (&'a AsciiStr, usize)>>(iter: T) -> Self {
        let items = LiteMap::<&AsciiStr, usize>::from_iter(iter);
        Self::from_litemap(&items)
    }
}

impl AsciiTrie<Vec<u8>> {
    /// Creates an [`AsciiTrie`] from a [`LiteMap`] mapping from [`AsciiStr`] to `usize`.
    ///
    /// **Requires the `"builder"` feature.**
    ///
    /// # Examples
    ///
    /// ```
    /// use asciitrie::{AsciiTrie, AsciiStr};
    /// use litemap::LiteMap;
    ///
    /// let mut map = LiteMap::new_vec();
    /// map.insert(AsciiStr::try_from_str("foo")?, 1);
    /// map.insert(AsciiStr::try_from_str("bar")?, 2);
    /// map.insert(AsciiStr::try_from_str("bazzoo")?, 3);
    ///
    /// let trie = AsciiTrie::from_litemap(&map);
    ///
    /// assert_eq!(trie.get(b"foo"), Some(1));
    /// assert_eq!(trie.get(b"bar"), Some(2));
    /// assert_eq!(trie.get(b"bazzoo"), Some(3));
    /// assert_eq!(trie.get(b"unknown"), None);
    ///
    /// # Ok::<_, asciitrie::NonAsciiError>(())
    /// ```
    pub fn from_litemap<'a, S>(items: &LiteMap<&'a AsciiStr, usize, S>) -> Self
    where
        S: litemap::store::StoreSlice<&'a AsciiStr, usize>,
        for<'l> &'l S::Slice: litemap::store::StoreSlice<&'a AsciiStr, usize, Slice = S::Slice>,
    {
        AsciiTrieBuilder::from_litemap(items.as_sliced())
            .to_ascii_trie()
            .to_owned()
    }
}
