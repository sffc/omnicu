// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::get;
use core::borrow::Borrow;

#[repr(transparent)]
#[derive(Debug, Default)]
pub struct AsciiTrie<S: ?Sized>(pub S);

impl<S> AsciiTrie<S>
where
    S: AsRef<[u8]> + ?Sized,
{
    pub fn get(&self, ascii: &[u8]) -> Option<usize> {
        get(self.0.as_ref(), ascii)
    }

    pub fn is_empty(&self) -> bool {
        self.0.as_ref().is_empty()
    }

    pub fn byte_len(&self) -> usize {
        self.0.as_ref().len()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }

    pub fn as_borrowed(&self) -> &AsciiTrie<[u8]> {
        AsciiTrie::from_bytes(self.0.as_ref())
    }
}

impl AsciiTrie<[u8]> {
    pub fn from_bytes(trie: &[u8]) -> &Self {
        // Safety: repr(transparent)
        unsafe { core::mem::transmute(trie) }
    }
}

// Note: Can't generalize this impl due to the `core::borrow::Borrow` blanket impl.
impl Borrow<AsciiTrie<[u8]>> for AsciiTrie<&[u8]> {
    fn borrow(&self) -> &AsciiTrie<[u8]> {
        self.as_borrowed()
    }
}
