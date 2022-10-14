// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::vec::Vec;
use super::AsciiByte;

pub(crate) struct SafeConstSlice<'a, T> {
    full_slice: &'a [T],
    start: usize,
    limit: usize,
}

impl<'a, T> SafeConstSlice<'a, T> {
    pub const fn len(&self) -> usize {
        self.limit - self.start
    }

    pub const fn get_or_panic(&self, index: usize) -> &T {
        &self.full_slice[index + self.start]
    }
}

macro_rules! const_for_each {
    ($safe_const_slice:expr, $item:tt, $inner:expr) => {{
        let mut i = 0;
        while i < $safe_const_slice.len() {
            let $item = $safe_const_slice.get_or_panic(i);
            $inner;
            i += 1;
        }
    }};
}

pub(crate) use const_for_each;

pub(crate) trait AsciiTrieBuilderStore {
    fn atbs_new_empty() -> Self;
    fn atbs_with_capacity(capacity: usize) -> Self;
    fn atbs_len(&self) -> usize;

    fn atbs_as_bytes(&self) -> &[u8];
    fn atbs_push_front(self, byte: u8) -> Self;
}

impl AsciiTrieBuilderStore for Vec<u8> {
    fn atbs_new_empty() -> Self {
        Self::new()
    }
    fn atbs_with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }
    fn atbs_len(&self) -> usize {
        self.len()
    }

    fn atbs_as_bytes(&self) -> &[u8] {
        self.as_slice()
    }
    fn atbs_push_front(mut self, byte: u8) -> Self {
        self.insert(0, byte);
        self
    }
}

pub(crate) struct ConstAsciiTrieBuilderStore<const N: usize> {
    data: [u8; N],
    start: usize
}

impl<const N: usize> ConstAsciiTrieBuilderStore<N> {
    pub const fn push_front(mut self, byte: u8) -> Self {
        if self.start == 0 {
            panic!("AsciiTrieBuilder buffer out of capacity");
        }
        self.start -= 1;
        self.data[self.start] = byte;
        self
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
    pub const fn cs_as_slice(&self) -> SafeConstSlice<(AsciiByte, usize)> {
        SafeConstSlice {
            full_slice: &self.slice,
            start: 0,
            limit: self.len
        }
    }
}
