// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod asciistr;
mod builder4;
mod builder5;
mod builder6;
mod builder7b;
mod bytestr;
pub(crate) mod const_util;
#[cfg(feature = "litemap")]
mod litemap;
mod store;
#[cfg(feature = "alloc")]
use alloc::{collections::VecDeque, vec::Vec};
#[cfg(feature = "alloc")]
pub mod tstore;

#[cfg(feature = "litemap")]
pub use self::litemap::{
    make4_litemap, make4_slice, make5_litemap, make5_slice, make7b_slice, make7b_litemap
};
pub(crate) use asciistr::AsciiByte;
pub use asciistr::AsciiStr;
pub use asciistr::NonAsciiError;
pub(crate) use bytestr::ByteStr;

use super::ZeroTrieSimpleAscii;
use builder7b::AsciiTrieBuilder7b;

impl<const N: usize> ZeroTrieSimpleAscii<[u8; N]> {
    /// **Const Constructor:** Creates an [`ZeroTrieSimpleAscii`] from a sorted slice of keys and values.
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
    /// Create a `const` ZeroTrieSimpleAscii at compile time:
    ///
    /// ```
    /// use asciitrie::{ZeroTrieSimpleAscii, AsciiStr};
    ///
    /// // The required capacity for this trie happens to be 17 bytes
    /// const TRIE: ZeroTrieSimpleAscii<[u8; 17]> = ZeroTrieSimpleAscii::from_asciistr_value_slice(&[
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
    /// # use asciitrie::{ZeroTrieSimpleAscii, AsciiStr};
    /// const TRIE: ZeroTrieSimpleAscii<[u8; 17]> = ZeroTrieSimpleAscii::from_asciistr_value_slice(&[
    ///     (AsciiStr::from_str_or_panic("foo"), 1),
    ///     (AsciiStr::from_str_or_panic("bar"), 2),
    ///     (AsciiStr::from_str_or_panic("bazzoo"), 3),
    /// ]);
    /// ```
    ///
    /// Panics if capacity is too small:
    ///
    /// ```compile_fail
    /// # use asciitrie::{ZeroTrieSimpleAscii, AsciiStr};
    /// const TRIE: ZeroTrieSimpleAscii<[u8; 15]> = ZeroTrieSimpleAscii::from_asciistr_value_slice(&[
    ///     (AsciiStr::from_str_or_panic("bar"), 2),
    ///     (AsciiStr::from_str_or_panic("bazzoo"), 3),
    ///     (AsciiStr::from_str_or_panic("foo"), 1),
    /// ]);
    /// ```
    ///
    /// Panics if capacity is too large:
    ///
    /// ```compile_fail
    /// # use asciitrie::{ZeroTrieSimpleAscii, AsciiStr};
    /// const TRIE: ZeroTrieSimpleAscii<[u8; 20]> = ZeroTrieSimpleAscii::from_asciistr_value_slice(&[
    ///     (AsciiStr::from_str_or_panic("bar"), 2),
    ///     (AsciiStr::from_str_or_panic("bazzoo"), 3),
    ///     (AsciiStr::from_str_or_panic("foo"), 1),
    /// ]);
    /// ```
    pub const fn from_asciistr_value_slice(items: &[(&AsciiStr, usize)]) -> Self {
        let result = AsciiTrieBuilder7b::<N>::from_tuple_slice::<100>(items);
        match result {
            Ok(s) => Self::from_store(s.take_or_panic()),
            Err(_) => panic!("Failed to build ZeroTrie"),
        }
    }

    /// **Const Constructor:** Creates an [`ZeroTrieSimpleAscii`] from a sorted slice of keys and values.
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
    /// Create a `const` ZeroTrieSimpleAscii at compile time:
    ///
    /// ```
    /// use asciitrie::{ZeroTrieSimpleAscii, AsciiStr};
    ///
    /// // The required capacity for this trie happens to be 17 bytes
    /// const TRIE: ZeroTrieSimpleAscii<[u8; 17]> = ZeroTrieSimpleAscii::from_str_value_array([
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
    /// # use asciitrie::{ZeroTrieSimpleAscii, AsciiStr};
    /// const TRIE: ZeroTrieSimpleAscii<[u8; 17]> = ZeroTrieSimpleAscii::from_str_value_array([
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
impl<'a> FromIterator<(&'a AsciiStr, usize)> for ZeroTrieSimpleAscii<Vec<u8>> {
    /// ***Enable this function with the `"alloc"` feature.***
    ///
    /// ```
    /// use asciitrie::AsciiStr;
    /// use asciitrie::ZeroTrieSimpleAscii;
    ///
    /// let trie: ZeroTrieSimpleAscii<Vec<u8>> = [
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
        use builder6::*;
        let mut items = Vec::<(&AsciiStr, usize)>::from_iter(iter);
        items.sort();
        let ascii_str_slice = items.as_slice();
        let byte_str_slice = ByteStr::from_ascii_str_slice_with_value(ascii_str_slice);
        AsciiTrieBuilder6::<VecDeque<u8>>::from_sorted_const_tuple_slice(
            byte_str_slice.into(),
            ZeroTrieBuilderOptions {
                phf_mode: PhfMode::BinaryOnly,
                ascii_mode: AsciiMode::AsciiOnly,
                capacity_mode: CapacityMode::Normal,
            },
        )
        .map(|s| Self {
            store: s.to_bytes(),
        })
        .unwrap()
    }
}
