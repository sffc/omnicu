// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::collections::VecDeque;
use alloc::vec::Vec;
use super::AsciiByte;

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

pub(crate) trait ChildrenStore {
    fn cs_new_empty() -> Self;
    fn cs_len(&self) -> usize;

    fn cs_push(&mut self, ascii: AsciiByte, size: usize);
    fn cs_as_slice(&self) -> &[(AsciiByte, usize)];
}

impl ChildrenStore for Vec<(AsciiByte, usize)> {
    fn cs_new_empty() -> Self {
        Self::new()
    }
    fn cs_len(&self) -> usize {
        self.len()
    }

    fn cs_push(&mut self, ascii: AsciiByte, size: usize) {
        self.push((ascii, size))
    }
    fn cs_as_slice(&self) -> &[(AsciiByte, usize)] {
        self.as_slice()
    }
}

pub(crate) struct ConstStackChildrenStore {
    slice: [(AsciiByte, usize); 128],
    len: usize
}

impl ChildrenStore for ConstStackChildrenStore {
    fn cs_new_empty() -> Self {
        Self {
            slice: [(AsciiByte::nul(), 0); 128],
            len: 0
        }
    }
    fn cs_len(&self) -> usize {
        self.len
    }

    fn cs_push(&mut self, ascii: AsciiByte, size: usize) {
        self.slice[self.len].0 = ascii;
        self.slice[self.len].1 = size;
        self.len += 1;
    }
    fn cs_as_slice(&self) -> &[(AsciiByte, usize)] {
        &self.slice
    }
}
