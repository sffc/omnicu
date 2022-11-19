// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::builder::builder2::AsciiTrieBuilder2;
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
