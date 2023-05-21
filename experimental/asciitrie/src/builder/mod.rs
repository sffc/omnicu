// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod asciistr;
mod builder;
mod builder1b;
mod builder2;
mod builder3;
mod builder4;
mod builder5;
mod builder6;
mod bytestr;
pub(crate) mod const_util;
#[cfg(feature = "litemap")]
mod litemap;
mod store;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "alloc")]
pub mod tstore;

#[cfg(feature = "litemap")]
pub use self::litemap::{
    make1b_litemap, make1b_slice, make2_litemap, make2_slice, make3_litemap, make3_slice,
    make4_litemap, make4_slice, make5_litemap, make5_slice, make6_byte_litemap, make6_byte_slice,
    make6_litemap, make6_slice,
};
pub(crate) use asciistr::AsciiByte;
pub use asciistr::AsciiStr;
pub use asciistr::NonAsciiError;

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
        AsciiTrieBuilder::<N>::from_tuple_slice(items).into_ascii_trie_or_panic()
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

#[cfg(feature = "alloc")]
impl<'a> FromIterator<(&'a AsciiStr, usize)> for AsciiTrie<Vec<u8>> {
    /// ***Enable this function with the `"alloc"` feature.***
    ///
    /// ```
    /// use asciitrie::AsciiStr;
    /// use asciitrie::AsciiTrie;
    ///
    /// let trie: AsciiTrie<Vec<u8>> = [
    ///     ("foo", 1),
    ///     ("bar", 2),
    ///     ("bazzoo", 3),
    ///     ("internationalization", 18),
    /// ]
    /// .into_iter()
    /// .map(AsciiStr::try_from_str_with_value)
    /// .collect::<Result<_, _>>()
    /// .unwrap();
    ///
    /// assert_eq!(trie.get(b"foo"), Some(1));
    /// assert_eq!(trie.get(b"bar"), Some(2));
    /// assert_eq!(trie.get(b"bazzoo"), Some(3));
    /// assert_eq!(trie.get(b"internationalization"), Some(18));
    /// assert_eq!(trie.get(b"unknown"), None);
    /// ```
    fn from_iter<T: IntoIterator<Item = (&'a AsciiStr, usize)>>(iter: T) -> Self {
        let mut items = Vec::<(&AsciiStr, usize)>::from_iter(iter);
        items.sort();
        AsciiTrieBuilder::<2048>::from_sorted_const_tuple_slice(items.as_slice().into())
            .to_ascii_trie()
            .to_owned()
    }
}
