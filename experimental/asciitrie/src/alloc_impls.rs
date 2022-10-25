// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::AsciiTrie;
use alloc::borrow::ToOwned;
use alloc::vec::Vec;
use core::borrow::Borrow;

// Note: Can't generalize this impl due to the `core::borrow::Borrow` blanket impl.
impl Borrow<AsciiTrie<[u8]>> for AsciiTrie<Vec<u8>> {
    fn borrow(&self) -> &AsciiTrie<[u8]> {
        self.as_borrowed()
    }
}

impl ToOwned for AsciiTrie<[u8]> {
    type Owned = AsciiTrie<Vec<u8>>;
    /// This impl allows [`AsciiTrie`] to be used inside of a [`Cow`](std::borrow::Cow).
    ///
    /// Note that it is also possible to use `AsciiTrie<ZeroVec<u8>>` for a similar result.
    ///
    /// ***Enable this impl with the `"alloc"` feature.***
    ///
    /// # Examples
    ///
    /// ```
    /// use std::borrow::Cow;
    /// use asciitrie::AsciiTrie;
    ///
    /// let trie: Cow<AsciiTrie<[u8]>> = Cow::Borrowed(AsciiTrie::from_bytes(b"abc\x85"));
    /// assert_eq!(trie.get(b"abc"), Some(5));
    /// ```
    fn to_owned(&self) -> Self::Owned {
        AsciiTrie {
            0: Vec::from(self.0.as_ref()),
        }
    }
}

impl<S> AsciiTrie<S>
where
    S: AsRef<[u8]> + ?Sized,
{
    /// Converts a possibly-borrowed AsciiTrie to an owned one.
    ///
    /// ***Enable this impl with the `"alloc"` feature.***
    ///
    /// # Examples
    ///
    /// ```
    /// use std::borrow::Cow;
    /// use asciitrie::AsciiTrie;
    ///
    /// let trie: AsciiTrie<&[u8]> = AsciiTrie(b"abc\x85");
    /// let owned: AsciiTrie<Vec<u8>> = trie.to_owned();
    ///
    /// assert_eq!(trie.get(b"abc"), Some(5));
    /// assert_eq!(owned.get(b"abc"), Some(5));
    /// ```
    pub fn to_owned(&self) -> AsciiTrie<Vec<u8>> {
        AsciiTrie {
            0: Vec::from(self.0.as_ref()),
        }
    }
}
