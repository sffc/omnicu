// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::reader::AsciiTrieIterator;
use crate::AsciiStr;
use crate::ZeroTrieSimpleAscii;
use alloc::borrow::Cow;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::borrow::Borrow;

// Note: Can't generalize this impl due to the `core::borrow::Borrow` blanket impl.
impl Borrow<ZeroTrieSimpleAscii<[u8]>> for ZeroTrieSimpleAscii<Box<[u8]>> {
    fn borrow(&self) -> &ZeroTrieSimpleAscii<[u8]> {
        self.as_borrowed()
    }
}

// Note: Can't generalize this impl due to the `core::borrow::Borrow` blanket impl.
impl Borrow<ZeroTrieSimpleAscii<[u8]>> for ZeroTrieSimpleAscii<Vec<u8>> {
    fn borrow(&self) -> &ZeroTrieSimpleAscii<[u8]> {
        self.as_borrowed()
    }
}

impl ToOwned for ZeroTrieSimpleAscii<[u8]> {
    type Owned = ZeroTrieSimpleAscii<Box<[u8]>>;
    /// This impl allows [`ZeroTrieSimpleAscii`] to be used inside of a [`Cow`](std::borrow::Cow).
    ///
    /// Note that it is also possible to use `ZeroTrieSimpleAscii<ZeroVec<u8>>` for a similar result.
    ///
    /// ***Enable this impl with the `"alloc"` feature.***
    ///
    /// # Examples
    ///
    /// ```
    /// use std::borrow::Cow;
    /// use asciitrie::ZeroTrieSimpleAscii;
    ///
    /// let trie: Cow<ZeroTrieSimpleAscii<[u8]>> = Cow::Borrowed(ZeroTrieSimpleAscii::from_bytes(b"abc\x85"));
    /// assert_eq!(trie.get(b"abc"), Some(5));
    /// ```
    fn to_owned(&self) -> Self::Owned {
        let bytes: &[u8] = self.store.as_ref();
        ZeroTrieSimpleAscii {
            store: Vec::from(bytes).into_boxed_slice(),
        }
    }
}

impl<S> ZeroTrieSimpleAscii<S>
where
    S: AsRef<[u8]> + ?Sized,
{
    /// Converts a possibly-borrowed ZeroTrieSimpleAscii to an owned one.
    ///
    /// ***Enable this impl with the `"alloc"` feature.***
    ///
    /// # Examples
    ///
    /// ```
    /// use std::borrow::Cow;
    /// use asciitrie::ZeroTrieSimpleAscii;
    ///
    /// let trie: &ZeroTrieSimpleAscii<[u8]> = ZeroTrieSimpleAscii::from_bytes(b"abc\x85");
    /// let owned: ZeroTrieSimpleAscii<Vec<u8>> = trie.to_owned();
    ///
    /// assert_eq!(trie.get(b"abc"), Some(5));
    /// assert_eq!(owned.get(b"abc"), Some(5));
    /// ```
    pub fn to_owned(&self) -> ZeroTrieSimpleAscii<Vec<u8>> {
        ZeroTrieSimpleAscii {
            store: Vec::from(self.store.as_ref()),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Box<AsciiStr>, usize)> + '_ {
        AsciiTrieIterator::new(self.as_bytes())
    }
}

impl ZeroTrieSimpleAscii<Vec<u8>> {
    pub fn wrap_bytes_into_cow(self) -> ZeroTrieSimpleAscii<Cow<'static, [u8]>> {
        ZeroTrieSimpleAscii {
            store: Cow::Owned(self.store),
        }
    }
}

impl ZeroTrieSimpleAscii<[u8]> {
    pub fn wrap_bytes_into_cow<'a>(&'a self) -> ZeroTrieSimpleAscii<Cow<'a, [u8]>> {
        ZeroTrieSimpleAscii {
            store: Cow::Borrowed(self.as_bytes()),
        }
    }
}

impl<'a> ZeroTrieSimpleAscii<&'a [u8]> {
    pub fn wrap_bytes_into_cow(self) -> ZeroTrieSimpleAscii<Cow<'a, [u8]>> {
        ZeroTrieSimpleAscii {
            store: Cow::Borrowed(self.store),
        }
    }
}
