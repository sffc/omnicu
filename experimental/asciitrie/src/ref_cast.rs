// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::AsciiTrie;
use core::borrow::Borrow;
use ref_cast::RefCast;

impl AsciiTrie<[u8]> {
    pub fn from_bytes(trie: &[u8]) -> &Self {
        Self::ref_cast(trie)
    }
}

// Note: Can't generalize this impl due to the `core::borrow::Borrow` blanket impl.
impl Borrow<AsciiTrie<[u8]>> for AsciiTrie<&[u8]> {
    fn borrow(&self) -> &AsciiTrie<[u8]> {
        self.as_borrowed()
    }
}

impl<S> AsciiTrie<S>
where
    S: AsRef<[u8]> + ?Sized,
{
    pub fn as_borrowed(&self) -> &AsciiTrie<[u8]> {
        AsciiTrie::from_bytes(self.0.as_ref())
    }
}
