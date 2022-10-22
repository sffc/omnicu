// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::store::const_for_each;
use super::store::ConstAsciiTrieBuilderStore;
use super::store::ConstStackChildrenStore;
use super::store::SafeConstSlice;
use crate::varint;
use super::AsciiByte;
use super::AsciiStr;
use crate::AsciiTrie;
use litemap::LiteMap;

/// A low-level builder for AsciiTrie.
pub(crate) struct AsciiTrieBuilder<const N: usize> {
    data: ConstAsciiTrieBuilderStore<N>,
}

impl<const N: usize> AsciiTrieBuilder<N> {
    pub fn to_ascii_trie(&mut self) -> AsciiTrie<&[u8]> {
        let slice = self.data.atbs_as_bytes();
        AsciiTrie(slice.as_slice())
    }

    pub const fn into_ascii_trie_or_panic(self) -> AsciiTrie<[u8; N]> {
        AsciiTrie(self.data.take_or_panic())
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
        let varint_array_slice = varint::write_varint(value);
        let data = self.data.atbs_extend_front(varint_array_slice.as_const_slice());
        let data = data.atbs_bitor_assign(0, 0b10000000);
        (Self { data }, 1)
    }

    #[must_use]
    const fn prepend_branch(
        self,
        ascii_rev: SafeConstSlice<AsciiByte>,
        sizes_rev: SafeConstSlice<usize>,
    ) -> (Self, usize) {
        debug_assert!(ascii_rev.len() == sizes_rev.len());
        let n = ascii_rev.len();
        if n > 0b00011111 {
            todo!()
        }
        let mut total_size = 0usize;
        const_for_each!(sizes_rev, size, {
            total_size += *size;
        });
        if total_size > 256 {
            todo!()
        }
        let mut index = total_size;
        let mut data = self.data;
        const_for_each!(sizes_rev, size, {
            index -= *size;
            data = data.atbs_push_front(index as u8);
        });
        const_for_each!(ascii_rev, ascii, {
            data = data.atbs_push_front(ascii.get());
        });
        data = data.atbs_push_front((n as u8) | 0b11000000);
        (Self { data }, 1 + n * 2)
    }

    pub fn from_litemap<'a, S>(items: LiteMap<&'a AsciiStr, usize, S>) -> Self
    where
        S: litemap::store::StoreSlice<&'a AsciiStr, usize, Slice = [(&'a AsciiStr, usize)]>,
    {
        if items.is_empty() {
            return Self::new();
        }
        let mut result = Self::new();
        let total_size;
        let items: SafeConstSlice<(&AsciiStr, usize)> = items.as_slice().into();
        (result, total_size) = result.create_recursive(items, 0);
        debug_assert_eq!(total_size, result.data.atbs_len());
        result
    }

    pub const fn from_sorted_tuple_vec<'a>(items: &[(&'a AsciiStr, usize)]) -> Self {
        if items.is_empty() {
            return Self::new();
        }
        let items = SafeConstSlice::from_slice(items);
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
        let mut result = Self::new();
        let total_size;
        (result, total_size) = result.create_recursive(items, 0);
        debug_assert!(total_size == result.data.atbs_len());
        result
    }

    #[must_use]
    const fn create_recursive<'a>(
        mut self,
        items: SafeConstSlice<(&'a AsciiStr, usize)>,
        prefix_len: usize,
    ) -> (Self, usize) {
        let first: (&'a AsciiStr, usize) = match items.first() {
            Some((k, v)) => (*k, *v),
            None => unreachable!(),
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
            while i > 0 {
                let c = items.get_or_panic(i - 1).0.ascii_at_or_panic(prefix_len);
                if c.get() != current_ascii.get() {
                    let size;
                    (self, size) =
                        self.create_recursive(items.get_subslice_or_panic(i, j), prefix_len + 1);
                    total_size += size;
                    children = children.cs_push(current_ascii, size);
                    current_ascii = c;
                    j = i;
                }
                i -= 1;
            }
            let size;
            (self, size) = self.create_recursive(items.get_subslice_or_panic(i, j), prefix_len + 1);
            total_size += size;
            if children.cs_len() == 0 {
                // All strings start with same byte
                let size;
                (self, size) = self.prepend_ascii(current_ascii);
                total_size += size;
            } else {
                // Need to make a branch node
                children = children.cs_push(current_ascii, size);
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
