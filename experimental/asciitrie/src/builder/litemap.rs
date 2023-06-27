// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::konst::*;
use crate::builder::bytestr::ByteStr;
use crate::error::Error;
use crate::zerotrie::ZeroTrieExtendedCapacity;
use crate::zerotrie::ZeroTriePerfectHash;
use crate::zerotrie::ZeroTrieSimpleAscii;
use crate::AsciiStr;
use crate::ZeroTrie;
use alloc::borrow::Borrow;
use alloc::vec::Vec;
use litemap::LiteMap;

impl ZeroTrieSimpleAscii<Vec<u8>> {
    #[doc(hidden)]
    pub fn try_from_litemap_with_const_builder<'a, S>(
        items: &LiteMap<&'a AsciiStr, usize, S>,
    ) -> Result<Self, Error>
    where
        S: litemap::store::StoreSlice<&'a AsciiStr, usize, Slice = [(&'a AsciiStr, usize)]>,
    {
        let ascii_str_slice = items.as_slice();
        ZeroTrieBuilderConst::<10000>::from_sorted_const_tuple_slice::<100>(ascii_str_slice.into())
            .map(|s| Self {
                store: s.as_bytes().to_vec(),
            })
    }
}

impl ZeroTrie<Vec<u8>> {
    /// Creates an [`ZeroTrie`] from a [`LiteMap`] mapping from [`[u8]`] to `usize`.
    ///
    /// This will select the most appropriate ZeroTrie variant based on the input data.
    ///
    /// ***Enable this function with the `"litemap"` feature.***
    ///
    /// # Examples
    ///
    /// ```
    /// use asciitrie::ZeroTrie;
    /// use litemap::LiteMap;
    ///
    /// let mut map = LiteMap::<&[u8], usize>::new_vec();
    /// map.insert("foo".as_bytes(), 1);
    /// map.insert("bar".as_bytes(), 2);
    /// map.insert("bazzoo".as_bytes(), 3);
    ///
    /// let trie = ZeroTrie::try_from_litemap(&map).unwrap();
    ///
    /// assert_eq!(trie.get("foo".as_bytes()), Some(1));
    /// assert_eq!(trie.get("bar".as_bytes()), Some(2));
    /// assert_eq!(trie.get("bazzoo".as_bytes()), Some(3));
    /// assert_eq!(trie.get("unknown".as_bytes()), None);
    ///
    /// # Ok::<_, asciitrie::NonAsciiError>(())
    /// ```
    pub fn try_from_litemap<'a, K, S>(items: &LiteMap<K, usize, S>) -> Result<Self, Error>
    where
        // Borrow, not AsRef, because we rely on Ord being the same. Unfortunately
        // this means `LiteMap<&str, usize>` does not work.
        K: Borrow<[u8]>,
        S: litemap::store::StoreSlice<K, usize, Slice = [(K, usize)]>,
    {
        let byte_litemap = items.to_borrowed_keys::<[u8], Vec<_>>();
        let byte_slice = byte_litemap.as_slice();
        let byte_str_slice = ByteStr::from_byte_slice_with_value(byte_slice);
        Self::try_from_tuple_slice(byte_str_slice)
    }

    /// Creates an [`ZeroTrie`] from a [`LiteMap`] mapping from [`str`] to `usize`.
    ///
    /// This will select the most appropriate ZeroTrie variant based on the input data.
    ///
    /// ***Enable this function with the `"litemap"` feature.***
    ///
    /// # Examples
    ///
    /// ```
    /// use asciitrie::ZeroTrie;
    /// use litemap::LiteMap;
    ///
    /// let mut map = LiteMap::<&str, usize>::new_vec();
    /// map.insert("foo", 1);
    /// map.insert("bar", 2);
    /// map.insert("bazzoo", 3);
    ///
    /// let trie = ZeroTrie::try_from_str_litemap(&map).unwrap();
    ///
    /// assert_eq!(trie.get("foo"), Some(1));
    /// assert_eq!(trie.get("bar"), Some(2));
    /// assert_eq!(trie.get("bazzoo"), Some(3));
    /// assert_eq!(trie.get("unknown"), None);
    ///
    /// # Ok::<_, asciitrie::NonAsciiError>(())
    /// ```
    pub fn try_from_str_litemap<'a, K, S>(items: &LiteMap<K, usize, S>) -> Result<Self, Error>
    where
        K: Borrow<str>,
        S: litemap::store::StoreSlice<K, usize, Slice = [(K, usize)]>,
    {
        let str_litemap = items.to_borrowed_keys::<str, Vec<_>>();
        let str_slice = str_litemap.as_slice();
        let byte_str_slice = ByteStr::from_str_slice_with_value(str_slice);
        Self::try_from_tuple_slice(byte_str_slice)
    }
}

/// TODO: Once const mut references are allowed, we can make this more infallible by
/// calculating the required length, heap-allocating the required capacity, and pointing
/// ConstAsciiTrieBuilderStore to the heap buffer.
/// ```compile_fail
/// // error[E0658]: mutable references are not allowed in constant functions
/// const fn write_to_mut_buffer(buf: &mut [u8]) { buf[0] = 0; }
/// ```
const _: () = ();
