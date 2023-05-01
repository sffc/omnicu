// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::builder::builder1b::AsciiTrieBuilder1b;
use crate::builder::builder2::AsciiTrieBuilder2;
use crate::builder::builder3::AsciiTrieBuilder3;
use crate::builder::builder4::AsciiTrieBuilder4;
use crate::builder::builder5::AsciiTrieBuilder5;
use crate::builder::builder6::AsciiTrieBuilder6;
use crate::builder::bytestr::ByteStr;
use crate::builder::AsciiTrieBuilder;
use crate::AsciiStr;
use crate::AsciiTrie;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::vec::Vec;
use litemap::LiteMap;

impl AsciiTrie<Vec<u8>> {
    /// Creates an [`AsciiTrie`] from a [`LiteMap`] mapping from [`AsciiStr`] to `usize`.
    ///
    /// ***Enable this function with the `"litemap"` feature.***
    ///
    /// # Examples
    ///
    /// ```
    /// use asciitrie::AsciiStr;
    /// use asciitrie::AsciiTrie;
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
        S: litemap::store::StoreSlice<&'a AsciiStr, usize, Slice = [(&'a AsciiStr, usize)]>,
    {
        /// TODO: Once const mut references are allowed, we can make this fully infallible by
        /// calculating the required length, heap-allocating the required capacity, and pointing
        /// ConstAsciiTrieBuilderStore to the heap buffer.
        /// ```compile_fail
        /// // error[E0658]: mutable references are not allowed in constant functions
        /// const fn write_to_mut_buffer(buf: &mut [u8]) { buf[0] = 0; }
        /// ```
        const _: () = ();

        AsciiTrieBuilder::<2048>::from_sorted_const_tuple_slice(items.as_slice().into())
            .to_ascii_trie()
            .to_owned()
    }
}

impl<S> AsciiTrie<S>
where
    S: AsRef<[u8]> + ?Sized,
{
    /// ***Enable this function with the `"litemap"` feature.***
    ///
    /// # Examples
    ///
    /// ```
    /// use asciitrie::AsciiStr;
    /// use asciitrie::AsciiTrie;
    /// use litemap::LiteMap;
    ///
    /// let trie = AsciiTrie::from_bytes(b"abc\x81def\x82");
    /// let items = trie.to_litemap();
    ///
    /// assert_eq!(items.len(), 2);
    /// assert_eq!(items.get("abc"), Some(&1));
    /// assert_eq!(items.get("abcdef"), Some(&2));
    ///
    /// let recovered_trie = AsciiTrie::from_litemap(
    ///     &items.to_borrowed_keys::<_, Vec<_>>()
    /// );
    /// assert_eq!(trie.as_bytes(), recovered_trie.as_bytes());
    /// ```
    pub fn to_litemap(&self) -> LiteMap<Box<AsciiStr>, usize> {
        self.iter().collect()
    }
}

impl<'a, S> From<LiteMap<&'a AsciiStr, usize, S>> for AsciiTrie<Vec<u8>>
where
    S: litemap::store::StoreSlice<&'a AsciiStr, usize, Slice = [(&'a AsciiStr, usize)]>,
{
    fn from(other: LiteMap<&'a AsciiStr, usize, S>) -> Self {
        Self::from_litemap(&other)
    }
}

impl<'a, S> From<&LiteMap<&'a AsciiStr, usize, S>> for AsciiTrie<Vec<u8>>
where
    S: litemap::store::StoreSlice<&'a AsciiStr, usize, Slice = [(&'a AsciiStr, usize)]>,
{
    fn from(other: &LiteMap<&'a AsciiStr, usize, S>) -> Self {
        Self::from_litemap(other)
    }
}

pub fn make2_litemap<'a, S>(items: &LiteMap<&'a AsciiStr, usize, S>) -> Vec<u8>
where
    S: litemap::store::StoreSlice<&'a AsciiStr, usize, Slice = [(&'a AsciiStr, usize)]>,
{
    AsciiTrieBuilder2::<10000>::from_sorted_const_tuple_slice(items.as_slice().into())
        .as_bytes()
        .to_owned()
}

pub fn make2_slice<'a>(items: &[(&'a AsciiStr, usize)]) -> Vec<u8> {
    AsciiTrieBuilder2::<10000>::from_tuple_slice(items.into())
        .as_bytes()
        .to_owned()
}

pub fn make3_litemap<'a, S>(items: &LiteMap<&'a AsciiStr, usize, S>) -> Vec<u8>
where
    S: litemap::store::StoreSlice<&'a AsciiStr, usize, Slice = [(&'a AsciiStr, usize)]>,
{
    AsciiTrieBuilder3::<10000>::from_sorted_const_tuple_slice(items.as_slice().into())
        .as_bytes()
        .to_owned()
}

pub fn make3_slice<'a>(items: &[(&'a AsciiStr, usize)]) -> Vec<u8> {
    AsciiTrieBuilder3::<10000>::from_tuple_slice(items.into())
        .as_bytes()
        .to_owned()
}

pub fn make1b_litemap<'a, S>(items: &LiteMap<&'a AsciiStr, usize, S>) -> Vec<u8>
where
    S: litemap::store::StoreSlice<&'a AsciiStr, usize, Slice = [(&'a AsciiStr, usize)]>,
{
    AsciiTrieBuilder1b::<10000>::from_sorted_const_tuple_slice(items.as_slice().into())
        .as_bytes()
        .to_owned()
}

pub fn make1b_slice<'a>(items: &[(&'a AsciiStr, usize)]) -> Vec<u8> {
    AsciiTrieBuilder1b::<10000>::from_tuple_slice(items.into())
        .as_bytes()
        .to_owned()
}

pub fn make4_litemap<'a, S>(items: &LiteMap<&'a AsciiStr, usize, S>) -> Vec<u8>
where
    S: litemap::store::StoreSlice<&'a AsciiStr, usize, Slice = [(&'a AsciiStr, usize)]>,
{
    AsciiTrieBuilder4::<15000>::from_sorted_const_tuple_slice(items.as_slice().into())
        .as_bytes()
        .to_owned()
}

pub fn make4_slice<'a>(items: &[(&'a AsciiStr, usize)]) -> Vec<u8> {
    AsciiTrieBuilder4::<15000>::from_tuple_slice(items.into())
        .as_bytes()
        .to_owned()
}

pub fn make5_litemap<'a, S>(items: &LiteMap<&'a AsciiStr, usize, S>) -> Vec<u8>
where
    S: litemap::store::StoreSlice<&'a AsciiStr, usize, Slice = [(&'a AsciiStr, usize)]>,
{
    AsciiTrieBuilder5::<15000>::from_sorted_const_tuple_slice(items.as_slice().into())
        .as_bytes()
        .to_owned()
}

pub fn make5_slice<'a>(items: &[(&'a AsciiStr, usize)]) -> Vec<u8> {
    AsciiTrieBuilder5::<15000>::from_tuple_slice(items.into())
        .as_bytes()
        .to_owned()
}

pub fn make6_litemap<'a, S>(items: &LiteMap<&'a AsciiStr, usize, S>) -> Vec<u8>
where
    S: litemap::store::StoreSlice<&'a AsciiStr, usize, Slice = [(&'a AsciiStr, usize)]>,
{
    let ascii_str_slice = items.as_slice();
    let byte_str_slice = ByteStr::from_ascii_str_slice_with_value(ascii_str_slice);
    AsciiTrieBuilder6::<15000>::from_sorted_const_tuple_slice(byte_str_slice.into())
        .as_bytes()
        .to_owned()
}

pub fn make6_slice<'a>(items: &[(&'a AsciiStr, usize)]) -> Vec<u8> {
    let byte_str_slice = ByteStr::from_ascii_str_slice_with_value(items);
    AsciiTrieBuilder6::<15000>::from_tuple_slice(byte_str_slice.into())
        .as_bytes()
        .to_owned()
}

pub fn make6_byte_litemap<'a, S>(items: &LiteMap<&'a [u8], usize, S>) -> Vec<u8>
where
    S: litemap::store::StoreSlice<&'a [u8], usize, Slice = [(&'a [u8], usize)]>,
{
    let byte_slice = items.as_slice();
    let byte_str_slice = ByteStr::from_byte_slice_with_value(byte_slice);
    AsciiTrieBuilder6::<15000>::from_sorted_const_tuple_slice(byte_str_slice.into())
        .as_bytes()
        .to_owned()
}

pub fn make6_byte_slice<'a>(items: &[(&'a [u8], usize)]) -> Vec<u8> {
    let byte_str_slice = ByteStr::from_byte_slice_with_value(items);
    AsciiTrieBuilder6::<15000>::from_tuple_slice(byte_str_slice.into())
        .as_bytes()
        .to_owned()
}
