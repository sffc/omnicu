// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ZeroTrieSimpleAscii;
use core::borrow::Borrow;
use ref_cast::RefCast;

impl ZeroTrieSimpleAscii<[u8]> {
    pub fn from_bytes(trie: &[u8]) -> &Self {
        Self::ref_cast(trie)
    }
}

// Note: Can't generalize this impl due to the `core::borrow::Borrow` blanket impl.
impl Borrow<ZeroTrieSimpleAscii<[u8]>> for ZeroTrieSimpleAscii<&[u8]> {
    fn borrow(&self) -> &ZeroTrieSimpleAscii<[u8]> {
        self.as_borrowed()
    }
}

impl<S> ZeroTrieSimpleAscii<S>
where
    S: AsRef<[u8]> + ?Sized,
{
    pub fn as_borrowed(&self) -> &ZeroTrieSimpleAscii<[u8]> {
        ZeroTrieSimpleAscii::from_bytes(self.store.as_ref())
    }
}
