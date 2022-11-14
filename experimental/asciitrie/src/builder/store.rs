// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains trait-like storage abstractions for AsciiTrieBuilder.

use super::const_util::ConstArrayBuilder;
use super::const_util::ConstSlice;
use super::AsciiByte;

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
pub(crate) enum BranchType {
    Equal2(AsciiByte),
    Equal3(AsciiByte),
    Greater,
}

pub(crate) struct ConstLengthsStack<const N: usize> {
    data: [(Option<BranchType>, usize); N],
    idx: usize,
}

impl<const N: usize> ConstLengthsStack<N> {
    pub const fn new() -> Self {
        Self {
            data: [(None, 0); N],
            idx: 0,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.idx == 0
    }

    #[must_use]
    pub const fn push(mut self, branch_type: BranchType, length: usize) -> Self {
        if self.idx >= N {
            panic!(concat!("AsciiTrie Builder: Need more stack (max ", stringify!(N), ")"));
        }
        self.data[self.idx] = (Some(branch_type), length);
        self.idx += 1;
        self
    }

    #[must_use]
    pub fn pop_or_panic(mut self) -> (Self, (BranchType, usize)) {
        if self.idx == 0 {
            panic!("AsciiTrie Builder: Attempted to pop from an empty stack");
        }
        self.idx -= 1;
        let (option, len) = self.data[self.idx];
        (self, (option.unwrap(), len))
    }
}
