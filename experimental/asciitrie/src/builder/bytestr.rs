// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
// #[derive(ref_cast::RefCastCustom)]
pub(crate) struct ByteStr([u8]);

impl ByteStr {
    pub const fn from_ascii_str_slice_with_value<'a, 'l>(
        input: &'l [(&'a crate::AsciiStr, usize)],
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

    pub const fn len(&self) -> usize {
        self.0.len()
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
}
