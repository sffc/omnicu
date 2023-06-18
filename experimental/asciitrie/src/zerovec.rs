// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ZeroTrieSimpleAscii;
use zerovec::ule::VarULE;
use zerovec::ZeroVecError;

// TODO(#2778): Auto-derive these impls based on the repr(transparent).

// Safety (based on the safety checklist on the VarULE trait):
//  1. ZeroTrieSimpleAscii<S> does not include any uninitialized or padding bytes (transparent over S, a VarULE)
//  2. ZeroTrieSimpleAscii<S> is aligned to 1 byte (transparent over S, a VarULE)
//  3. The impl of `validate_byte_slice()` returns an error if any byte is not valid (defers to S)
//  4. The impl of `validate_byte_slice()` returns an error if the slice cannot be used in its entirety (defers to S)
//  5. The impl of `from_byte_slice_unchecked()` returns a reference to the same data (transmutes the pointer)
//  6. All other methods are defaulted
//  7. `[T]` byte equality is semantic equality (transparent over S, a VarULE)
unsafe impl<S> VarULE for ZeroTrieSimpleAscii<S>
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
