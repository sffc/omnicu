// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
use core::ops::Range;
use ref_cast::{ref_cast_custom, RefCastCustom};

#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(clippy::exhaustive_structs)] // marker type
pub struct NonAsciiError;

/// A byte that is always ASCII.
/// TODO: Consider making this the same as tinystr AsciiByte?
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct AsciiByte(u8);

impl AsciiByte {
    pub const fn try_from_u8(byte: u8) -> Result<Self, NonAsciiError> {
        if byte >= 0b10000000 {
            return Err(NonAsciiError);
        }
        Ok(Self(byte))
    }

    #[cfg(feature = "alloc")]
    pub(crate) fn debug_from_u8(byte: u8) -> Self {
        match Self::try_from_u8(byte) {
            Ok(x) => x,
            Err(_) => {
                debug_assert!(false, "debug_from_u8: non-ascii byte: {:?}", byte);
                Self(0)
            }
        }
    }

    pub const fn nul() -> Self {
        Self(0)
    }

    pub const fn get(self) -> u8 {
        self.0
    }
}

const fn try_ascii_slice_from_bytes(bytes: &[u8]) -> Result<&[AsciiByte], NonAsciiError> {
    let mut i = 0;
    while i < bytes.len() {
        match AsciiByte::try_from_u8(bytes[i]) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        i += 1;
    }
    // Safety:
    // - AsciiByte is transparent over u8
    // - Therefore, [AsciiByte] is transparent over [u8]
    let ascii_slice = unsafe { core::mem::transmute(bytes) };
    Ok(ascii_slice)
}

const fn ascii_slice_to_bytes(ascii_slice: &[AsciiByte]) -> &[u8] {
    // Safety:
    // - AsciiByte is transparent over u8
    // - Therefore, [AsciiByte] is transparent over [u8]
    unsafe { core::mem::transmute(ascii_slice) }
}

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, RefCastCustom)]
pub struct AsciiStr([AsciiByte]);

impl core::fmt::Debug for AsciiStr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        f.write_str(self.as_str())
    }
}

impl AsciiStr {
    #[ref_cast_custom]
    pub(crate) const fn from_ascii_slice(ascii_slice: &[AsciiByte]) -> &Self;

    #[cfg(feature = "alloc")]
    pub(crate) fn from_boxed_ascii_slice(ascii_slice: Box<[AsciiByte]>) -> Box<Self> {
        // Safety: same reason ref-cast works on references
        unsafe { core::mem::transmute(ascii_slice) }
    }

    pub const fn try_from_bytes(bytes: &[u8]) -> Result<&Self, NonAsciiError> {
        match try_ascii_slice_from_bytes(bytes) {
            Ok(ascii_slice) => Ok(Self::from_ascii_slice(ascii_slice)),
            Err(e) => Err(e),
        }
    }

    pub const fn try_from_str(s: &str) -> Result<&Self, NonAsciiError> {
        Self::try_from_bytes(s.as_bytes())
    }

    pub const fn from_bytes_or_panic(s: &[u8]) -> &Self {
        match Self::try_from_bytes(s) {
            Ok(s) => s,
            Err(_) => panic!("Non-ASCII string passed to AsciiStr"),
        }
    }

    pub const fn from_str_or_panic(s: &str) -> &Self {
        Self::from_bytes_or_panic(s.as_bytes())
    }

    pub const fn empty() -> &'static AsciiStr {
        Self::from_str_or_panic("")
    }

    pub fn try_from_bytes_with_value<T>(tuple: (&[u8], T)) -> Result<(&Self, T), NonAsciiError> {
        let s = AsciiStr::try_from_bytes(tuple.0)?;
        Ok((s, tuple.1))
    }

    pub fn try_from_str_with_value<T>(tuple: (&str, T)) -> Result<(&Self, T), NonAsciiError> {
        let s = AsciiStr::try_from_str(tuple.0)?;
        Ok((s, tuple.1))
    }

    pub const fn len(&self) -> usize {
        self.0.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn substring(&self, range: Range<usize>) -> Option<&AsciiStr> {
        let slice = self.0.get(range)?;
        Some(Self::from_ascii_slice(slice))
    }

    pub(crate) const fn is_less_then(&self, other: &Self) -> bool {
        let mut i = 0;
        while i < self.len() && i < other.len() {
            if self.0[i].get() < other.0[i].get() {
                return true;
            }
            if self.0[i].get() > other.0[i].get() {
                return false;
            }
            i += 1;
        }
        self.len() < other.len()
    }

    #[allow(dead_code)] // may want this in the future
    pub(crate) fn ascii_at(&self, index: usize) -> Option<AsciiByte> {
        self.0.get(index).copied()
    }

    pub(crate) const fn ascii_at_or_panic(&self, index: usize) -> AsciiByte {
        self.0[index]
    }

    pub fn as_bytes(&self) -> &[u8] {
        ascii_slice_to_bytes(&self.0)
    }

    pub fn as_str(&self) -> &str {
        // Safety: all ASCII bytes are valid UTF-8 bytes
        unsafe { core::str::from_utf8_unchecked(self.as_bytes()) }
    }

    #[cfg(feature = "alloc")]
    pub fn to_boxed(&self) -> Box<AsciiStr> {
        Self::from_boxed_ascii_slice(Box::from(&self.0))
    }
}

impl core::borrow::Borrow<[u8]> for AsciiStr {
    fn borrow(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl core::borrow::Borrow<[u8]> for &AsciiStr {
    fn borrow(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl core::borrow::Borrow<str> for AsciiStr {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl core::borrow::Borrow<str> for &AsciiStr {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}
