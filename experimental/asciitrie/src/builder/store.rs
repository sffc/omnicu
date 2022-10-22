// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::varint::ConstArraySlice;

use super::AsciiByte;

pub(crate) struct SafeConstSlice<'a, T> {
    full_slice: &'a [T],
    start: usize,
    limit: usize,
}

impl<'a, T> SafeConstSlice<'a, T> {
    pub const fn from_slice(other: &'a [T]) -> Self {
        SafeConstSlice {
            full_slice: other,
            start: 0,
            limit: other.len(),
        }
    }

    pub const fn from_manual_slice(full_slice: &'a [T], start: usize, limit: usize) -> Self {
        SafeConstSlice { full_slice, start, limit }
    }

    pub const fn len(&self) -> usize {
        self.limit - self.start
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub const fn get_or_panic(&self, index: usize) -> &T {
        &self.full_slice[index + self.start]
    }

    pub const fn first(&self) -> Option<&T> {
        if self.len() == 0 {
            None
        } else {
            Some(self.get_or_panic(0))
        }
    }

    pub const fn get_subslice_or_panic(
        &self,
        new_start: usize,
        new_limit: usize,
    ) -> SafeConstSlice<'a, T> {
        assert!(new_start <= new_limit);
        assert!(new_limit <= self.len());
        SafeConstSlice {
            full_slice: self.full_slice,
            start: self.start + new_start,
            limit: self.start + new_limit,
        }
    }

    pub fn as_slice(&self) -> &'a [T] {
        &self.full_slice[self.start..self.limit]
    }
}

impl<'a> SafeConstSlice<'a, u8> {
    pub const fn const_bitor_assign(mut self, index: usize, other: u8) -> Self {
        self.full_slice[self.start] |= other;
        self
    }
}

impl<'a, T> From<&'a [T]> for SafeConstSlice<'a, T> {
    fn from(other: &'a [T]) -> Self {
        Self::from_slice(other)
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

pub(crate) struct ConstAsciiTrieBuilderStore<const N: usize> {
    data: [u8; N],
    start: usize,
}

impl<const N: usize> ConstAsciiTrieBuilderStore<N> {
    pub const fn atbs_new_empty() -> Self {
        Self {
            data: [0; N],
            start: N,
        }
    }
    pub const fn atbs_len(&self) -> usize {
        N - self.start
    }
    pub const fn atbs_push_front(mut self, byte: u8) -> Self {
        if self.start == 0 {
            panic!("AsciiTrieBuilder buffer out of capacity");
        }
        self.start -= 1;
        self.data[self.start] = byte;
        self
    }
    pub const fn atbs_extend_front(mut self, other: SafeConstSlice<u8>) {
        todo!()
    }
    pub const fn atbs_as_bytes(&self) -> SafeConstSlice<u8> {
        SafeConstSlice {
            full_slice: &self.data,
            start: self.start,
            limit: N,
        }
    }
    pub const fn atbs_bitor_assign(mut self, index: usize, other: u8) -> Self {
        self.data = self.data.const_bitor_assign(index, other);
        self
    }
    pub const fn take_or_panic(self) -> [u8; N] {
        if self.start != 0 {
            panic!("AsciiTrieBuilder buffer is too large");
        }
        self.data
    }
}

pub(crate) struct ConstStackChildrenStore {
    // There are 128 ASCII bytes, so this should always be enough.
    // Note: This needs 1160 stack bytes on x64.
    ascii: [AsciiByte; 128],
    sizes: [usize; 128],
    len: usize,
}

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
    pub const fn cs_ascii_slice(&self) -> SafeConstSlice<AsciiByte> {
        SafeConstSlice {
            full_slice: &self.ascii,
            start: 0,
            limit: self.len,
        }
    }
    pub const fn cs_sizes_slice(&self) -> SafeConstSlice<usize> {
        SafeConstSlice {
            full_slice: &self.sizes,
            start: 0,
            limit: self.len,
        }
    }
}
