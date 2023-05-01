// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
use core::borrow::Borrow;
use core::ops::Range;
use ref_cast::{ref_cast_custom, RefCastCustom};

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, RefCastCustom)]
pub(crate) struct ByteStr([u8]);

impl ByteStr {
    #[ref_cast_custom]
    pub(crate) const fn from_byte_slice(byte_slice: &[u8]) -> &Self;

    pub const fn from_str(s: &str) -> &Self {
        Self::from_byte_slice(s.as_bytes())
    }

    #[cfg(feature = "alloc")]
    pub(crate) fn from_boxed_byte_slice(byte_slice: Box<[u8]>) -> Box<Self> {
        // Safety: same reason ref-cast works on references
        unsafe { core::mem::transmute(byte_slice) }
    }

    #[cfg(feature = "alloc")]
    pub fn from_boxed_str(s: Box<str>) -> Box<ByteStr> {
        Self::from_boxed_bytes(s.into_boxed_bytes())
    }

    #[cfg(feature = "alloc")]
    pub fn from_boxed_bytes(bytes: Box<[u8]>) -> Box<ByteStr> {
        Self::from_boxed_byte_slice(bytes)
    }

    pub const fn from_ascii_str_slice_with_value<'a, 'l>(input: &'l [(&'a crate::AsciiStr, usize)]) -> &'l [(&'a ByteStr, usize)] {
        // Safety: AsciiStr and ByteStr have the same layout, and ByteStr is less restrictive
        unsafe { core::mem::transmute(input) }
    }

    pub const fn empty() -> &'static ByteStr {
        Self::from_str("")
    }

    pub const fn len(&self) -> usize {
        self.0.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn substring(&self, range: Range<usize>) -> Option<&ByteStr> {
        let slice = self.0.get(range)?;
        Some(Self::from_byte_slice(slice))
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

    #[allow(dead_code)] // may want this in the future
    pub(crate) fn byte_at(&self, index: usize) -> Option<u8> {
        self.0.get(index).copied()
    }

    pub(crate) const fn byte_at_or_panic(&self, index: usize) -> u8 {
        self.0[index]
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

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    #[cfg(feature = "alloc")]
    pub fn to_boxed(&self) -> Box<ByteStr> {
        Self::from_boxed_byte_slice(Box::from(&self.0))
    }
}
