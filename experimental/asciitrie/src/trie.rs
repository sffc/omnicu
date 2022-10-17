// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::get;
use core::borrow::Borrow;
use zerovec::ule::VarULE;
use zerovec::ule::ULE;
use zerovec::ZeroVecError;

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy)]
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

// Safety (based on the safety checklist on the ULE trait):
//  1. AsciiTrie<S> does not include any uninitialized or padding bytes (transparent over S, a ULE)
//  2. AsciiTrie<S> is aligned to 1 byte (transparent over S, a ULE)
//  3. The impl of validate_byte_slice() returns an error if any byte is not valid (defers to S)
//  4. The impl of validate_byte_slice() returns an error if there are extra bytes (defers to S)
//  5. The other ULE methods use the default impl.
//  6. CharULE byte equality is semantic equality
unsafe impl<S> ULE for AsciiTrie<S>
where
    S: ULE,
{
    #[inline]
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        S::validate_byte_slice(bytes)
    }
}

// Safety (based on the safety checklist on the VarULE trait):
//  1. AsciiTrie<S> does not include any uninitialized or padding bytes (transparent over S, a VarULE)
//  2. AsciiTrie<S> is aligned to 1 byte (transparent over S, a VarULE)
//  3. The impl of `validate_byte_slice()` returns an error if any byte is not valid (defers to S)
//  4. The impl of `validate_byte_slice()` returns an error if the slice cannot be used in its entirety (defers to S)
//  5. The impl of `from_byte_slice_unchecked()` returns a reference to the same data (transmutes the pointer)
//  6. All other methods are defaulted
//  7. `[T]` byte equality is semantic equality (transparent over S, a VarULE)
unsafe impl<S> VarULE for AsciiTrie<S>
where
    S: VarULE,
{
    #[inline]
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        S::validate_byte_slice(bytes)
    }
    #[inline]
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        core::mem::transmute(S::from_byte_slice_unchecked(bytes))
    }
}

// Note: Can't generalize this impl due to the `core::borrow::Borrow` blanket impl.
impl Borrow<AsciiTrie<[u8]>> for AsciiTrie<&[u8]> {
    fn borrow(&self) -> &AsciiTrie<[u8]> {
        self.as_borrowed()
    }
}
