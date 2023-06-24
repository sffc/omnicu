// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains trait-like storage abstractions for AsciiTrieBuilder.

use super::const_util::const_for_each;
use super::const_util::ConstArrayBuilder;
use super::const_util::ConstSlice;
use super::AsciiByte;
use crate::error::Error;

#[derive(Default)]
pub(crate) struct ConstAsciiTrieBuilderStore<const N: usize> {
    data: ConstArrayBuilder<N, u8>,
}

// Note: This impl block is intended to be a trait, but we can't use traits in const.
impl<const N: usize> ConstAsciiTrieBuilderStore<N> {
    pub const fn atbs_new_empty() -> Self {
        Self {
            data: ConstArrayBuilder::new_empty([0; N], N),
        }
    }
    pub const fn atbs_len(&self) -> usize {
        self.data.len()
    }
    pub const fn atbs_push_front(mut self, byte: u8) -> Self {
        self.data = self.data.const_push_front(byte);
        self
    }
    pub const fn atbs_extend_front(mut self, other: ConstSlice<u8>) -> Self {
        self.data = self.data.const_extend_front(other);
        self
    }
    #[cfg(feature = "alloc")]
    pub const fn atbs_as_bytes(&self) -> ConstSlice<u8> {
        self.data.as_const_slice()
    }
    pub const fn atbs_bitor_assign(mut self, index: usize, other: u8) -> Self {
        self.data = self.data.const_bitor_assign(index, other);
        self
    }
    pub const fn take_or_panic(self) -> [u8; N] {
        self.data.const_take_or_panic()
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct BranchMeta {
    pub ascii: u8,
    pub length: usize,
    pub local_length: usize,
    pub count: usize,
}

impl BranchMeta {
    pub const fn const_default() -> Self {
        BranchMeta {
            ascii: AsciiByte::nul().get(),
            length: 0,
            local_length: 0,
            count: 0,
        }
    }
}

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
    pub const fn push(mut self, meta: BranchMeta) -> Result<Self, Error> {
        if self.idx >= N {
            return Err(Error::ConstBuilder(concat!(
                "AsciiTrie Builder: Need more stack (max ",
                stringify!(N),
                ")"
            )));
        }
        self.data[self.idx] = Some(meta);
        self.idx += 1;
        Ok(self)
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
            result = match result.push_front(match self.data[i] {
                Some(x) => x,
                None => panic!("Not enough items in the ConstLengthsStack"),
            }) {
                Ok(x) => x,
                // Note: unreachable!("message") is not const
                Err(_) => panic!("unreachable: len <= 256")
            };
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
            result = result.const_push_front(value.ascii);
        });
        result
    }
}
