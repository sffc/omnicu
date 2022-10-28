// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod asciistr;
mod builder;
pub(crate) mod const_util;
#[cfg(feature = "litemap")]
mod litemap;
mod store;

pub use asciistr::AsciiStr;
pub use asciistr::NonAsciiError;
pub(crate) use asciistr::AsciiByte;

use super::AsciiTrie;
use builder::AsciiTrieBuilder;

impl<const N: usize> AsciiTrie<[u8; N]> {
    /// **Const Constructor:** Creates an [`AsciiTrie`] from a sorted slice of keys and values.
    ///
    /// This function needs to know the exact length of the resulting trie at compile time.
    ///
    /// Also see [`Self::from_str_value_array`].
    ///
    /// # Panics
    ///
    /// Panics if `items` is not sorted or if `N` is not correct.
    ///
    /// # Examples
    ///
    /// Create a `const` AsciiTrie at compile time:
    ///
    /// ```
    /// use asciitrie::{AsciiTrie, AsciiStr};
    ///
    /// // The required capacity for this trie happens to be 19 bytes
    /// const TRIE: AsciiTrie<[u8; 19]> = AsciiTrie::from_asciistr_value_slice(&[
    ///     (AsciiStr::from_str_or_panic("bar"), 2),
    ///     (AsciiStr::from_str_or_panic("bazzoo"), 3),
    ///     (AsciiStr::from_str_or_panic("foo"), 1),
    /// ]);
    ///
    /// assert_eq!(TRIE.get(b"foo"), Some(1));
    /// assert_eq!(TRIE.get(b"bar"), Some(2));
    /// assert_eq!(TRIE.get(b"bazzoo"), Some(3));
    /// assert_eq!(TRIE.get(b"unknown"), None);
    /// ```
    ///
    /// Panics if strings are not sorted:
    ///
    /// ```compile_fail
    /// # use asciitrie::{AsciiTrie, AsciiStr};
    /// const TRIE: AsciiTrie<[u8; 19]> = AsciiTrie::from_asciistr_value_slice(&[
    ///     (AsciiStr::from_str_or_panic("foo"), 1),
    ///     (AsciiStr::from_str_or_panic("bar"), 2),
    ///     (AsciiStr::from_str_or_panic("bazzoo"), 3),
    /// ]);
    /// ```
    ///
    /// Panics if capacity is too small:
    ///
    /// ```compile_fail
    /// # use asciitrie::{AsciiTrie, AsciiStr};
    /// const TRIE: AsciiTrie<[u8; 15]> = AsciiTrie::from_asciistr_value_slice(&[
    ///     (AsciiStr::from_str_or_panic("bar"), 2),
    ///     (AsciiStr::from_str_or_panic("bazzoo"), 3),
    ///     (AsciiStr::from_str_or_panic("foo"), 1),
    /// ]);
    /// ```
    ///
    /// Panics if capacity is too large:
    ///
    /// ```compile_fail
    /// # use asciitrie::{AsciiTrie, AsciiStr};
    /// const TRIE: AsciiTrie<[u8; 20]> = AsciiTrie::from_asciistr_value_slice(&[
    ///     (AsciiStr::from_str_or_panic("bar"), 2),
    ///     (AsciiStr::from_str_or_panic("bazzoo"), 3),
    ///     (AsciiStr::from_str_or_panic("foo"), 1),
    /// ]);
    /// ```
    pub const fn from_asciistr_value_slice(items: &[(&AsciiStr, usize)]) -> Self {
        AsciiTrieBuilder::<N>::from_tuple_vec(items).into_ascii_trie_or_panic()
    }

    /// **Const Constructor:** Creates an [`AsciiTrie`] from a sorted slice of keys and values.
    ///
    /// This function needs to know the exact length of the resulting trie at compile time.
    ///
    /// Also see [`Self::from_asciistr_value_slice`].
    ///
    /// # Panics
    ///
    /// Panics if `items` is not sorted, if `N` is not correct, or if any of the strings contain
    /// non-ASCII characters.
    ///
    /// # Examples
    ///
    /// Create a `const` AsciiTrie at compile time:
    ///
    /// ```
    /// use asciitrie::{AsciiTrie, AsciiStr};
    ///
    /// // The required capacity for this trie happens to be 19 bytes
    /// const TRIE: AsciiTrie<[u8; 19]> = AsciiTrie::from_str_value_array([
    ///     ("bar", 2),
    ///     ("bazzoo", 3),
    ///     ("foo", 1),
    /// ]);
    ///
    /// assert_eq!(TRIE.get(b"foo"), Some(1));
    /// assert_eq!(TRIE.get(b"bar"), Some(2));
    /// assert_eq!(TRIE.get(b"bazzoo"), Some(3));
    /// assert_eq!(TRIE.get(b"unknown"), None);
    /// ```
    ///
    /// Panics if the strings are not ASCII:
    ///
    /// ```compile_fail
    /// # use asciitrie::{AsciiTrie, AsciiStr};
    /// const TRIE: AsciiTrie<[u8; 19]> = AsciiTrie::from_str_value_array([
    ///     ("bár", 2),
    ///     ("båzzöo", 3),
    ///     ("foo", 1),
    /// ]);
    /// ```
    pub const fn from_str_value_array<const M: usize>(items: [(&str, usize); M]) -> Self {
        let mut asciistr_array = [(AsciiStr::empty(), 0); M];
        let mut i = 0;
        while i < items.len() {
            asciistr_array[i].0 = AsciiStr::from_str_or_panic(items[i].0);
            asciistr_array[i].1 = items[i].1;
            i += 1;
        }
        Self::from_asciistr_value_slice(&asciistr_array)
    }
}
