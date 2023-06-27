// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "alloc")]
use crate::{AsciiStr, NonAsciiError};

#[cfg(feature = "alloc")]
use alloc::{borrow::Borrow, boxed::Box};

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ByteStr([u8]);

#[cfg(feature = "alloc")] // impls only needed with the nonconst builder
impl ByteStr {
    pub const fn from_ascii_str_slice_with_value<'a, 'l>(
        input: &'l [(&'a AsciiStr, usize)],
    ) -> &'l [(&'a ByteStr, usize)] {
        // Safety: AsciiStr and ByteStr have the same layout, and ByteStr is less restrictive
        unsafe { core::mem::transmute(input) }
    }

    pub const fn from_byte_slice_with_value<'a, 'l>(
        input: &'l [(&'a [u8], usize)],
    ) -> &'l [(&'a ByteStr, usize)] {
        // Safety: [u8] and ByteStr have the same layout and invariants
        unsafe { core::mem::transmute(input) }
    }

    pub const fn from_str_slice_with_value<'a, 'l>(
        input: &'l [(&'a str, usize)],
    ) -> &'l [(&'a ByteStr, usize)] {
        // Safety: str and ByteStr have the same layout, and ByteStr is less restrictive
        unsafe { core::mem::transmute(input) }
    }

    pub fn from_bytes(input: &[u8]) -> &Self {
        // Safety: [u8] and ByteStr have the same layout and invariants
        unsafe { core::mem::transmute(input) }
    }

    pub fn from_boxed_bytes(input: Box<[u8]>) -> Box<Self> {
        // Safety: [u8] and ByteStr have the same layout and invariants
        unsafe { core::mem::transmute(input) }
    }

    pub fn from_str(input: &str) -> &Self {
        Self::from_bytes(input.as_bytes())
    }

    pub fn empty() -> &'static Self {
        Self::from_bytes(&[])
    }

    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub const fn len(&self) -> usize {
        self.0.len()
    }

    pub fn try_as_ascii_str(&self) -> Result<&AsciiStr, NonAsciiError> {
        AsciiStr::try_from_bytes(&self.0)
    }

    #[allow(dead_code)] // may want this in the future
    pub(crate) fn byte_at(&self, index: usize) -> Option<u8> {
        self.0.get(index).copied()
    }

    pub(crate) const fn byte_at_or_panic(&self, index: usize) -> u8 {
        self.0[index]
    }

    pub(crate) const fn is_less_then(&self, other: &Self) -> bool {
        let mut i = 0;
        while i < self.len() && i < other.len() {
            if self.0[i] < other.0[i] {
                return true;
            }
            if self.0[i] > other.0[i] {
                return false;
            }
            i += 1;
        }
        self.len() < other.len()
    }

    pub(crate) const fn prefix_eq(&self, other: &ByteStr, prefix_len: usize) -> bool {
        assert!(prefix_len <= self.len());
        assert!(prefix_len <= other.len());
        let mut i = 0;
        while i < prefix_len {
            if self.0[i] != other.0[i] {
                return false;
            }
            i += 1;
        }
        return true;
    }
}

impl Borrow<[u8]> for ByteStr {
    fn borrow(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[cfg(feature = "alloc")]
impl Borrow<[u8]> for alloc::boxed::Box<ByteStr> {
    fn borrow(&self) -> &[u8] {
        self.as_bytes()
    }
}
