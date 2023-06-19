// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::const_util::const_for_each;
use super::const_util::ConstSlice;
use super::store::ConstAsciiTrieBuilderStore;
use super::store::ConstStackChildrenStore;
use super::AsciiByte;
use super::AsciiStr;
use crate::varint;
use crate::ZeroTrieSimpleAscii;

/// A low-level builder for ZeroTrieSimpleAscii.
pub(crate) struct AsciiTrieBuilder<const N: usize> {
    data: ConstAsciiTrieBuilderStore<N>,
}

impl<const N: usize> AsciiTrieBuilder<N> {
    #[cfg(feature = "alloc")]
    pub fn to_ascii_trie(&mut self) -> ZeroTrieSimpleAscii<&[u8]> {
        let slice = self.data.atbs_as_bytes();
        ZeroTrieSimpleAscii {
            store: slice.as_slice(),
        }
    }

    #[cfg(feature = "alloc")]
    pub fn as_bytes(&mut self) -> &[u8] {
        let slice = self.data.atbs_as_bytes();
        slice.as_slice()
    }

    pub const fn into_ascii_trie_or_panic(self) -> ZeroTrieSimpleAscii<[u8; N]> {
        ZeroTrieSimpleAscii {
            store: self.data.take_or_panic(),
        }
    }

    pub const fn new() -> Self {
        Self {
            data: ConstAsciiTrieBuilderStore::atbs_new_empty(),
        }
    }

    #[must_use]
    const fn prepend_ascii(self, ascii: AsciiByte) -> (Self, usize) {
        let data = self.data.atbs_push_front(ascii.get());
        (Self { data }, 1)
    }

    #[must_use]
    const fn prepend_value(self, value: usize) -> (Self, usize) {
        let mut data = self.data;
        let varint_array = varint::write_varint(value);
        data = data.atbs_extend_front(varint_array.as_const_slice());
        data = data.atbs_bitor_assign(0, 0b10000000);
        (Self { data }, varint_array.len())
    }

    #[must_use]
    const fn prepend_branch(
        self,
        ascii_rev: ConstSlice<AsciiByte>,
        sizes_rev: ConstSlice<usize>,
    ) -> (Self, usize) {
        debug_assert!(ascii_rev.len() == sizes_rev.len());
        let mut total_size = 0usize;
        const_for_each!(sizes_rev, size, {
            total_size += *size;
        });
        const USIZE_BITS: usize = core::mem::size_of::<usize>() * 8;
        let w = (USIZE_BITS - (total_size.leading_zeros() as usize) - 1) / 8 + 1;
        let mut data = self.data;
        let mut i = 0;
        while i < w {
            let mut index = total_size;
            const_for_each!(sizes_rev, size, {
                index -= *size;
                let mut x = index;
                let mut j = 0;
                while j < i {
                    x >>= 8;
                    j += 1;
                }
                data = data.atbs_push_front(x as u8);
            });
            i += 1;
        }
        const_for_each!(ascii_rev, ascii, {
            data = data.atbs_push_front(ascii.get());
        });
        let varint_array = varint::write_varint(ascii_rev.len());
        data = data.atbs_extend_front(varint_array.as_const_slice());
        data = data.atbs_bitor_assign(0, 0b11000000);
        (
            Self { data },
            varint_array.len() + ascii_rev.len() * (1 + w),
        )
    }

    /// Panics if the items are not sorted
    pub const fn from_tuple_slice<'a>(items: &[(&'a AsciiStr, usize)]) -> Self {
        let items = ConstSlice::from_slice(items);
        let mut prev: Option<&'a AsciiStr> = None;
        const_for_each!(items, (ascii_str, _), {
            match prev {
                None => (),
                Some(prev) => {
                    if !prev.is_less_then(ascii_str) {
                        panic!("Strings in AsciiStr constructor are not sorted");
                    }
                }
            };
            prev = Some(ascii_str)
        });
        Self::from_sorted_const_tuple_slice(items)
    }

    /// Assumes that the items are sorted
    pub const fn from_sorted_const_tuple_slice<'a>(
        items: ConstSlice<(&'a AsciiStr, usize)>,
    ) -> Self {
        let mut result = Self::new();
        let total_size;
        (result, total_size) = result.create_recursive(items, 0);
        debug_assert!(total_size == result.data.atbs_len());
        result
    }

    #[must_use]
    const fn create_recursive<'a>(
        mut self,
        items: ConstSlice<(&'a AsciiStr, usize)>,
        prefix_len: usize,
    ) -> (Self, usize) {
        let first: (&'a AsciiStr, usize) = match items.first() {
            Some((k, v)) => (*k, *v),
            None => return (Self::new(), 0), // empty slice
        };
        let mut initial_value = None;
        let mut total_size = 0;
        let items = if first.0.len() == prefix_len {
            initial_value = Some(first.1);
            items.get_subslice_or_panic(1, items.len())
        } else {
            items
        };
        if !items.is_empty() {
            let mut i = items.len() - 1;
            let mut j = items.len();
            let mut current_ascii = items
                .get_or_panic(items.len() - 1)
                .0
                .ascii_at_or_panic(prefix_len);
            let mut children = ConstStackChildrenStore::cs_new_empty();
            loop {
                let prev_ascii = current_ascii;
                let should_recurse = if i == 0 {
                    true
                } else {
                    current_ascii = items.get_or_panic(i - 1).0.ascii_at_or_panic(prefix_len);
                    current_ascii.get() != prev_ascii.get()
                };
                if should_recurse {
                    let size;
                    (self, size) =
                        self.create_recursive(items.get_subslice_or_panic(i, j), prefix_len + 1);
                    total_size += size;
                    children = children.cs_push(prev_ascii, size);
                    j = i;
                }
                if i == 0 {
                    break;
                } else {
                    i -= 1;
                }
            }
            if children.cs_len() == 1 {
                // All strings start with same byte
                let size;
                (self, size) = self.prepend_ascii(current_ascii);
                total_size += size;
            } else {
                // Need to make a branch node
                // children = children.cs_push(current_ascii, size);
                let size;
                (self, size) =
                    self.prepend_branch(children.cs_ascii_slice(), children.cs_sizes_slice());
                total_size += size;
            }
        }
        if let Some(value) = initial_value {
            let size;
            (self, size) = self.prepend_value(value);
            total_size += size;
        }
        (self, total_size)
    }
}
