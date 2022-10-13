// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::collections::VecDeque;
use super::AsciiByte;

/// # Panics
/// Panics if `start..limit` is not a valid range in `slice`
const fn const_subslice<T>(slice: &[T], limit: usize) -> &[T] {
    unsafe {
        let (ptr, len) = core::mem::transmute::<&[T], (*const T, usize)>(slice);
        assert!(limit <= len);
        core::mem::transmute((ptr, limit))
    }
}

pub(crate) trait AsciiTrieBuilderStore {
    fn atbs_new_empty() -> Self;
    fn atbs_with_capacity(capacity: usize) -> Self;
    fn atbs_len(&self) -> usize;

    fn atbs_make_contiguous(&mut self) -> &mut [u8];
    fn atbs_push_front(&mut self, byte: u8);
}

impl AsciiTrieBuilderStore for VecDeque<u8> {
    fn atbs_new_empty() -> Self {
        Self::new()
    }
    fn atbs_with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }
    fn atbs_len(&self) -> usize {
        self.len()
    }

    fn atbs_make_contiguous(&mut self) -> &mut [u8] {
        self.make_contiguous()
    }
    fn atbs_push_front(&mut self, byte: u8) {
        self.push_front(byte)
    }
}

pub(crate) struct ConstStackChildrenStore {
    slice: [(AsciiByte, usize); 128],
    len: usize
}

impl ConstStackChildrenStore {
    pub const fn cs_new_empty() -> Self {
        Self {
            slice: [(AsciiByte::nul(), 0); 128],
            len: 0
        }
    }
    pub const fn cs_len(&self) -> usize {
        self.len
    }

    pub const fn cs_push(mut self, ascii: AsciiByte, size: usize) -> Self {
        self.slice[self.len].0 = ascii;
        self.slice[self.len].1 = size;
        self.len += 1;
        self
    }
    pub const fn cs_as_slice(&self) -> &[(AsciiByte, usize)] {
        const_subslice(&self.slice, self.len)
    }
}
