// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(clippy::exhaustive_structs)] // marker type
pub struct NonAsciiError;

/// A byte that is always ASCII.
/// TODO: Consider making this the same as tinystr AsciiByte?
/// FIXME: Add ULE validation
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, zerovec::ule::ULE)]
pub(crate) struct AsciiByte(u8);

impl AsciiByte {
    pub const fn try_from_u8(byte: u8) -> Result<Self, NonAsciiError> {
        if byte >= 0b10000000 {
            return Err(NonAsciiError);
        }
        Ok(Self(byte))
    }

    pub const fn nul() -> Self {
        Self(0)
    }

    pub const fn get(self) -> u8 {
        self.0
    }
}

/// FIXME: Add ULE validation
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, zerovec::ule::VarULE)]
pub struct AsciiStr([AsciiByte]);

impl AsciiStr {
    pub const fn try_from_bytes(bytes: &[u8]) -> Result<&Self, NonAsciiError> {
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
        // - AsciiStr is transparent over [AsciiByte]
        // - Therefore, AsciiStr is transparent over [u8]
        unsafe { core::mem::transmute(bytes) }
    }

    pub const fn try_from_str(s: &str) -> Result<&Self, NonAsciiError> {
        Self::try_from_bytes(s.as_bytes())
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

    pub(crate) fn ascii_at(&self, index: usize) -> Option<AsciiByte> {
        self.0.get(index).copied()
    }

    pub fn as_bytes(&self) -> &[u8] {
        // Safety:
        // - AsciiByte is transparent over u8
        // - AsciiStr is transparent over [AsciiByte]
        // - Therefore, AsciiStr is transparent over [u8]
        unsafe { core::mem::transmute(self) }
    }
}
