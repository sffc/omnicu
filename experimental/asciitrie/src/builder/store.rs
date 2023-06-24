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
    pub fn atbs_swap_ranges(mut self, start: usize, mid: usize, limit: usize) -> Self {
        self.data = self.data.swap_ranges(start, mid, limit);
        self
    }
    pub const fn take_or_panic(self) -> [u8; N] {
        self.data.const_take_or_panic()
    }
}

pub(crate) struct ConstStackChildrenStore {
    // There are 128 ASCII bytes, so this should always be enough.
    // Note: This needs 1160 stack bytes on x64.
    ascii: [AsciiByte; 128],
    sizes: [usize; 128],
    len: usize,
}

// Note: This impl block is intended to be a trait, but we can't use traits in const.
impl ConstStackChildrenStore {
    pub const fn cs_new_empty() -> Self {
        Self {
            ascii: [AsciiByte::nul(); 128],
            sizes: [0; 128],
            len: 0,
        }
    }
    pub const fn cs_len(&self) -> usize {
        self.len
    }

    pub const fn cs_push(mut self, ascii: AsciiByte, size: usize) -> Self {
        self.ascii[self.len] = ascii;
        self.sizes[self.len] = size;
        self.len += 1;
        self
    }
    pub const fn cs_ascii_slice(&self) -> ConstSlice<AsciiByte> {
        ConstSlice::from_manual_slice(&self.ascii, 0, self.len)
    }
    pub const fn cs_sizes_slice(&self) -> ConstSlice<usize> {
        ConstSlice::from_manual_slice(&self.sizes, 0, self.len)
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

    #[must_use]
    pub const fn pop_or_panic(mut self) -> (Self, BranchMeta) {
        if self.idx == 0 {
            panic!("AsciiTrie Builder: Attempted to pop from an empty stack");
        }
        self.idx -= 1;
        let value = match self.data[self.idx] {
            Some(x) => x,
            None => unreachable!(),
        };
        (self, value)
    }

    pub const fn peek_or_panic(&self) -> BranchMeta {
        if self.idx == 0 {
            panic!("AsciiTrie Builder: Attempted to peek from an empty stack");
        }
        self.get_or_panic(0)
    }

    pub const fn get_or_panic(&self, index: usize) -> BranchMeta {
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
        let mut result = ConstArrayBuilder::new_empty([BranchMeta::const_default(); 256], 256);
        let mut ix = 0;
        loop {
            if ix == len {
                break;
            }
            let i = self.idx - ix - 1;
            result = result.push_front(match self.data[i] {
                Some(x) => x,
                None => panic!("Not enough items in the ConstLengthsStack")
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
            result = result.const_push_front(value.ascii);
        });
        result
    }
}
