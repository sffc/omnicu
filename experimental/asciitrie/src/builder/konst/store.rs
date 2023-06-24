// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains trait-like storage abstractions for AsciiTrieBuilder.

use super::super::branch_meta::BranchMeta;
use super::const_util::const_for_each;
use super::const_util::ConstArrayBuilder;

pub(crate) struct ConstLengthsStack1b<const N: usize> {
    data: [Option<BranchMeta>; N],
    idx: usize,
}

impl<const N: usize> core::fmt::Debug for ConstLengthsStack1b<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl<const N: usize> ConstLengthsStack1b<N> {
    pub const fn new() -> Self {
        Self {
            data: [None; N],
            idx: 0,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.idx == 0
    }

    #[must_use]
    pub const fn push_or_panic(mut self, meta: BranchMeta) -> Self {
        if self.idx >= N {
            panic!(concat!(
                "AsciiTrie Builder: Need more stack (max ",
                stringify!(N),
                ")"
            ));
        }
        self.data[self.idx] = Some(meta);
        self.idx += 1;
        self
    }

    pub const fn peek_or_panic(&self) -> BranchMeta {
        if self.idx == 0 {
            panic!("AsciiTrie Builder: Attempted to peek from an empty stack");
        }
        self.get_or_panic(0)
    }

    const fn get_or_panic(&self, index: usize) -> BranchMeta {
        if self.idx <= index {
            panic!("AsciiTrie Builder: Attempted to get too deep in a stack");
        }
        match self.data[self.idx - index - 1] {
            Some(x) => x,
            None => unreachable!(),
        }
    }

    pub const fn pop_many_or_panic(
        mut self,
        len: usize,
    ) -> (Self, ConstArrayBuilder<256, BranchMeta>) {
        debug_assert!(len <= 256);
        let mut result = ConstArrayBuilder::new_empty([BranchMeta::const_default(); 256], 256);
        let mut ix = 0;
        loop {
            if ix == len {
                break;
            }
            let i = self.idx - ix - 1;
            result = result.push_front_or_panic(match self.data[i] {
                Some(x) => x,
                None => panic!("Not enough items in the ConstLengthsStack"),
            });
            ix += 1;
        }
        self.idx -= len;
        (self, result)
    }

    fn as_slice(&self) -> &[Option<BranchMeta>] {
        &self.data[0..self.idx]
    }
}

impl<const N: usize> ConstArrayBuilder<N, BranchMeta> {
    pub const fn map_to_ascii_bytes(&self) -> ConstArrayBuilder<N, u8> {
        let mut result = ConstArrayBuilder::new_empty([0; N], N);
        let self_as_slice = self.as_const_slice();
        const_for_each!(self_as_slice, value, {
            result = result.const_push_front_or_panic(value.ascii);
        });
        result
    }
}
