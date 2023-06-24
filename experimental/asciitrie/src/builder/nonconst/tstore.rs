// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::super::branch_meta::BranchMeta;
use super::super::konst::ConstArrayBuilder;
use alloc::collections::VecDeque;
use alloc::vec::Vec;

pub trait TrieBuilderStore {
    fn atbs_new_empty() -> Self;
    fn atbs_len(&self) -> usize;
    fn atbs_push_front(&mut self, byte: u8);
    fn atbs_extend_front(&mut self, other: &[u8]);
    fn atbs_to_bytes(&self) -> Vec<u8>;
    fn atbs_bitor_assign(&mut self, index: usize, other: u8);
    fn atbs_swap_ranges(&mut self, start: usize, mid: usize, limit: usize);
    fn atbs_split_first(&mut self) -> Option<u8>;

    fn atbs_prepend_n_zeros(&mut self, n: usize) {
        let mut i = 0;
        while i < n {
            self.atbs_push_front(0);
            i += 1;
        }
    }
}

impl TrieBuilderStore for VecDeque<u8> {
    fn atbs_new_empty() -> Self {
        VecDeque::new()
    }
    fn atbs_len(&self) -> usize {
        self.len()
    }
    fn atbs_push_front(&mut self, byte: u8) {
        self.push_front(byte);
    }
    fn atbs_extend_front(&mut self, other: &[u8]) {
        // TODO: No extend_front on VecDeque?
        self.reserve(other.len());
        for b in other.iter().rev() {
            self.push_front(*b);
        }
    }
    fn atbs_to_bytes(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(self.len());
        let (a, b) = self.as_slices();
        v.extend(a);
        v.extend(b);
        v
    }
    fn atbs_bitor_assign(&mut self, index: usize, other: u8) {
        self[index] |= other;
    }
    fn atbs_swap_ranges(&mut self, mut start: usize, mut mid: usize, mut limit: usize) {
        if start > mid || mid > limit {
            panic!("Invalid args to atbs_swap_ranges(): start > mid || mid > limit");
        }
        if limit > self.len() {
            panic!(
                "Invalid args to atbs_swap_ranges(): limit out of range: {limit} > {}",
                self.len()
            );
        }
        loop {
            if start == mid || mid == limit {
                return;
            }
            let len0 = mid - start;
            let len1 = limit - mid;
            let mut i = start;
            let mut j = limit - core::cmp::min(len0, len1);
            while j < limit {
                self.swap(i, j);
                i += 1;
                j += 1;
            }
            if len0 < len1 {
                mid = start + len0;
                limit -= len0;
            } else {
                start += len1;
                mid = limit - len1;
            }
        }
    }
    fn atbs_split_first(&mut self) -> Option<u8> {
        self.pop_front()
    }
}

pub(crate) struct MutableLengthsStack1b {
    data: Vec<BranchMeta>,
}

impl core::fmt::Debug for MutableLengthsStack1b {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl MutableLengthsStack1b {
    pub const fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn push(&mut self, meta: BranchMeta) {
        self.data.push(meta);
    }

    pub fn peek_or_panic(&self) -> BranchMeta {
        *self.data.last().unwrap()
    }

    pub fn pop_many_or_panic(&mut self, len: usize) -> ConstArrayBuilder<256, BranchMeta> {
        debug_assert!(len <= 256);
        let mut result = ConstArrayBuilder::new_empty([BranchMeta::const_default(); 256], 256);
        let mut ix = 0;
        loop {
            if ix == len {
                break;
            }
            let i = self.data.len() - ix - 1;
            // Won't panic because len <= 256
            result = result.push_front_or_panic(match self.data.get(i) {
                Some(x) => *x,
                None => panic!("Not enough items in the ConstLengthsStack"),
            });
            ix += 1;
        }
        self.data.truncate(self.data.len() - len);
        result
    }

    fn as_slice(&self) -> &[BranchMeta] {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_ranges() {
        let s = b"..abcdefghijkl=";
        let mut s = s.iter().copied().collect::<VecDeque<u8>>();
        s.atbs_swap_ranges(2, 7, 14);
        assert_eq!(s.atbs_to_bytes(), b"..fghijklabcde=");
    }
}
