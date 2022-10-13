// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::collections::VecDeque;

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
