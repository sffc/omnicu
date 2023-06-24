// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::builder::builder6::AsciiMode;
use crate::builder::builder6::AsciiTrieBuilder6;
use crate::builder::builder6::CapacityMode;
use crate::builder::builder6::PhfMode;
use crate::builder::builder6::ZeroTrieBuilderOptions;
use crate::builder::builder7b::AsciiTrieBuilder7b;
use crate::builder::bytestr::ByteStr;
use crate::error::Error;
use crate::zerotrie::ZeroTrieSimpleAscii;
use crate::zerotrie::ZeroTriePerfectHash;
use crate::zerotrie::ZeroTrieExtendedCapacity;
use crate::AsciiStr;
use alloc::borrow::ToOwned;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use litemap::LiteMap;

impl ZeroTrieSimpleAscii<Vec<u8>> {
    /// Creates an [`ZeroTrieSimpleAscii`] from a [`LiteMap`] mapping from [`AsciiStr`] to `usize`.
    ///
    /// ***Enable this function with the `"litemap"` feature.***
    ///
    /// # Examples
    ///
    /// ```
    /// use asciitrie::AsciiStr;
    /// use asciitrie::ZeroTrieSimpleAscii;
    /// use litemap::LiteMap;
    ///
    /// let mut map = LiteMap::new_vec();
    /// map.insert(AsciiStr::try_from_str("foo")?, 1);
    /// map.insert(AsciiStr::try_from_str("bar")?, 2);
    /// map.insert(AsciiStr::try_from_str("bazzoo")?, 3);
    ///
    /// let trie = ZeroTrieSimpleAscii::try_from_litemap(&map).unwrap();
    ///
    /// assert_eq!(trie.get(b"foo"), Some(1));
    /// assert_eq!(trie.get(b"bar"), Some(2));
    /// assert_eq!(trie.get(b"bazzoo"), Some(3));
    /// assert_eq!(trie.get(b"unknown"), None);
    ///
    /// # Ok::<_, asciitrie::NonAsciiError>(())
    /// ```
    pub fn try_from_litemap<'a, S>(items: &LiteMap<&'a AsciiStr, usize, S>) -> Result<Self, Error>
    where
        S: litemap::store::StoreSlice<&'a AsciiStr, usize, Slice = [(&'a AsciiStr, usize)]>,
    {
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
    }
}

impl ZeroTriePerfectHash<Vec<u8>> {
    /// Creates an [`ZeroTrieSimpleAscii`] from a [`LiteMap`] mapping from [`[u8]`] to `usize`.
    ///
    /// ***Enable this function with the `"litemap"` feature.***
    ///
    /// # Examples
    ///
    /// ```
    /// use asciitrie::ZeroTriePerfectHash;
    /// use litemap::LiteMap;
    ///
    /// let mut map = LiteMap::<&[u8], usize>::new_vec();
    /// map.insert("foo".as_bytes(), 1);
    /// map.insert("bår".as_bytes(), 2);
    /// map.insert("båzzøø".as_bytes(), 3);
    ///
    /// let trie = ZeroTriePerfectHash::try_from_litemap(&map).unwrap();
    ///
    /// assert_eq!(trie.get("foo".as_bytes()), Some(1));
    /// assert_eq!(trie.get("bår".as_bytes()), Some(2));
    /// assert_eq!(trie.get("båzzøø".as_bytes()), Some(3));
    /// assert_eq!(trie.get("bazzoo".as_bytes()), None);
    ///
    /// # Ok::<_, asciitrie::NonAsciiError>(())
    /// ```
    pub fn try_from_litemap<'a, S>(items: &LiteMap<&'a [u8], usize, S>) -> Result<Self, Error>
    where
        S: litemap::store::StoreSlice<&'a [u8], usize, Slice = [(&'a [u8], usize)]>,
    {
        let byte_slice = items.as_slice();
        let byte_str_slice = ByteStr::from_byte_slice_with_value(byte_slice);
        AsciiTrieBuilder6::<VecDeque<u8>>::from_sorted_const_tuple_slice(
            byte_str_slice.into(),
            ZeroTrieBuilderOptions {
                phf_mode: PhfMode::UsePhf,
                ascii_mode: AsciiMode::BinarySpans,
                capacity_mode: CapacityMode::Normal,
            },
        )
        .map(|s| Self {
            store: s.to_bytes(),
        })
    }
}

impl ZeroTrieExtendedCapacity<Vec<u8>> {
    /// Creates an [`ZeroTrieSimpleAscii`] from a [`LiteMap`] mapping from [`[u8]`] to `usize`.
    ///
    /// ***Enable this function with the `"litemap"` feature.***
    ///
    /// # Examples
    ///
    /// ```
    /// use asciitrie::ZeroTriePerfectHash;
    /// use litemap::LiteMap;
    ///
    /// let mut map = LiteMap::<&[u8], usize>::new_vec();
    /// map.insert("foo".as_bytes(), 1);
    /// map.insert("bår".as_bytes(), 2);
    /// map.insert("båzzøø".as_bytes(), 3);
    ///
    /// let trie = ZeroTriePerfectHash::try_from_litemap(&map).unwrap();
    ///
    /// assert_eq!(trie.get("foo".as_bytes()), Some(1));
    /// assert_eq!(trie.get("bår".as_bytes()), Some(2));
    /// assert_eq!(trie.get("båzzøø".as_bytes()), Some(3));
    /// assert_eq!(trie.get("bazzoo".as_bytes()), None);
    ///
    /// # Ok::<_, asciitrie::NonAsciiError>(())
    /// ```
    pub fn try_from_litemap<'a, S>(items: &LiteMap<&'a [u8], usize, S>) -> Result<Self, Error>
    where
        S: litemap::store::StoreSlice<&'a [u8], usize, Slice = [(&'a [u8], usize)]>,
    {
        let byte_slice = items.as_slice();
        let byte_str_slice = ByteStr::from_byte_slice_with_value(byte_slice);
        AsciiTrieBuilder6::<VecDeque<u8>>::from_sorted_const_tuple_slice(
            byte_str_slice.into(),
            ZeroTrieBuilderOptions {
                phf_mode: PhfMode::UsePhf,
                ascii_mode: AsciiMode::BinarySpans,
                capacity_mode: CapacityMode::Extended,
            },
        )
        .map(|s| Self {
            store: s.to_bytes(),
        })
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

impl<'a, S> From<LiteMap<&'a AsciiStr, usize, S>> for ZeroTrieSimpleAscii<Vec<u8>>
where
    S: litemap::store::StoreSlice<&'a AsciiStr, usize, Slice = [(&'a AsciiStr, usize)]>,
{
    fn from(other: LiteMap<&'a AsciiStr, usize, S>) -> Self {
        Self::try_from_litemap(&other).unwrap()
    }
}

impl<'a, S> From<&LiteMap<&'a AsciiStr, usize, S>> for ZeroTrieSimpleAscii<Vec<u8>>
where
    S: litemap::store::StoreSlice<&'a AsciiStr, usize, Slice = [(&'a AsciiStr, usize)]>,
{
    fn from(other: &LiteMap<&'a AsciiStr, usize, S>) -> Self {
        Self::try_from_litemap(other).unwrap()
    }
}

pub fn make7b_litemap<'a, S>(items: &LiteMap<&'a AsciiStr, usize, S>) -> Result<Vec<u8>, Error>
where
    S: litemap::store::StoreSlice<&'a AsciiStr, usize, Slice = [(&'a AsciiStr, usize)]>,
{
    AsciiTrieBuilder7b::<10000>::from_sorted_const_tuple_slice::<100>(items.as_slice().into())
        .map(|x| x.as_bytes().to_owned())
}

pub fn make7b_slice<'a>(items: &[(&'a AsciiStr, usize)]) -> Result<Vec<u8>, Error> {
    AsciiTrieBuilder7b::<10000>::from_tuple_slice::<100>(items.into())
        .map(|x| x.as_bytes().to_owned())
}
