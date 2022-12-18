// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::reader::get;

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, ref_cast::RefCast)]
pub struct AsciiTrie<S: ?Sized>(pub(crate) S);

impl<S> AsciiTrie<S> {
    pub fn from_store(store: S) -> Self {
        Self(store)
    }

    pub fn take_store(self) -> S {
        self.0
    }
}

impl<S> AsciiTrie<S>
where
    S: AsRef<[u8]> + ?Sized,
{
    pub fn get(&self, ascii: &[u8]) -> Option<usize> {
        get(self.0.as_ref(), ascii)
    }

    pub fn get_str(&self, ascii: &str) -> Option<usize> {
        self.get(ascii.as_bytes())
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
}

impl<S> AsRef<[u8]> for AsciiTrie<S>
where
    S: AsRef<[u8]> + ?Sized,
{
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
