// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::reader::read_range;
use crate::reader::debug_get;
use crate::reader::debug_split_at;
use crate::varint::read_varint;
use crate::reader::{byte_type, ByteType};
use alloc::boxed::Box;
use crate::AsciiStr;
use crate::AsciiTrie;
use crate::builder::AsciiByte;
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

    pub fn iter(&self) -> impl Iterator<Item = (Box<AsciiStr>, usize)> + '_ {
        AsciiTrieIterator {
            state: alloc::vec![(self.as_bytes(), alloc::vec![], 0)],
        }
    }
}

struct AsciiTrieIterator<'a> {
    state: Vec<(&'a [u8], Vec<AsciiByte>, usize)>,
}

impl<'a> AsciiTrieIterator<'a> {
}

impl<'a> Iterator for AsciiTrieIterator<'a> {
    type Item = (Box<AsciiStr>, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let (mut trie, mut string, mut dial);
        (trie, string, dial) = self.state.pop()?;
        loop {
            let (b, x, search);
            let old_trie = trie;
            (b, trie) = match trie.split_first() {
                Some(tpl) => tpl,
                None => {
                    (trie, string, dial) = self.state.pop()?;
                    continue;
                }
            };
            let byte_type = byte_type(*b);
            if matches!(byte_type, ByteType::Ascii) {
                string.push(AsciiByte::debug_from_u8(*b));
                continue;
            }
            (x, trie) = read_varint(*b, trie)?;
            if matches!(byte_type, ByteType::Value) {
                let retval = AsciiStr::from_boxed_ascii_slice(string.clone().into_boxed_slice());
                self.state.push((trie, string, 0));
                return Some((retval, x))
            }
            if dial + 1 < x {
                // Return to this branch node at the next index
                self.state.push((old_trie, string.clone(), dial + 1));
            }
            (search, trie) = debug_split_at(trie, x)?;
            let ascii = debug_get(search, dial)?;
            string.push(AsciiByte::debug_from_u8(ascii));
            trie = read_range(trie, dial, x)?;
            dial = 0;
        }
    }
}
