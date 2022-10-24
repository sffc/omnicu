// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::reader::get;

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, ref_cast::RefCast)]
pub struct AsciiTrie<S: ?Sized>(pub S);

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
