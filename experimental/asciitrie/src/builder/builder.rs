// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::store::AsciiTrieBuilderStore;
use super::store::ConstStackChildrenStore;
use super::store::SafeConstSlice;
use super::store::const_for_each;
use super::AsciiByte;
use super::AsciiStr;
use crate::AsciiTrie;
use litemap::LiteMap;

/// A low-level builder for AsciiTrie.
pub(crate) struct AsciiTrieBuilder<B> {
    data: B,
}

impl<B: AsciiTrieBuilderStore> AsciiTrieBuilder<B> {
    pub fn to_ascii_trie(&mut self) -> AsciiTrie<&[u8]> {
        let slice = self.data.atbs_as_bytes();
        AsciiTrie(slice.as_slice())
    }

    pub fn new() -> Self {
        Self {
            data: B::atbs_new_empty(),
        }
    }

    #[must_use]
    fn prepend_ascii(self, ascii: AsciiByte) -> (Self, usize) {
        let data = self.data.atbs_push_front(ascii.get());
        (Self { data }, 1)
    }

    #[must_use]
    fn prepend_value(self, value: usize) -> (Self, usize) {
        if value > 0b00011111 {
            todo!()
        }
        let data = self.data.atbs_push_front((value as u8) | 0b10000000);
        (Self { data }, 1)
    }

    #[must_use]
    fn prepend_branch(self, targets_rev: SafeConstSlice<(AsciiByte, usize)>) -> (Self, usize) {
        let n = targets_rev.len();
        if n > 0b00011111 {
            todo!()
        }
        let mut total_size = 0;
        const_for_each!(targets_rev, (_, size), {
            total_size += size;
        });
        if total_size > 256 {
            todo!()
        }
        let mut index = total_size;
        let mut data = self.data;
        const_for_each!(targets_rev, (_, size), {
            index -= size;
            data = data.atbs_push_front(index.try_into().unwrap());
        });
        const_for_each!(targets_rev, (ascii, _), {
            data = data.atbs_push_front(ascii.get());
        });
        data = data.atbs_push_front((n as u8) | 0b11000000);
        (Self { data }, 1 + n * 2)
    }

    pub fn from_litemap<'a, S>(items: LiteMap<&'a AsciiStr, usize, S>) -> Self
    where
        S: litemap::store::StoreSlice<&'a AsciiStr, usize>,
        for<'l> &'l S::Slice: litemap::store::StoreSlice<&'a AsciiStr, usize, Slice = S::Slice>,
    {
        if items.is_empty() {
            return Self::new();
        }
        let mut result = Self::new();
        let total_size;
        (result, total_size) = result.create_recursive(items.as_sliced(), 0);
        debug_assert_eq!(total_size, result.data.atbs_len());
        result
    }

    #[must_use]
    fn create_recursive<'a, 'b, S: ?Sized>(
        mut self,
        items: LiteMap<&'a AsciiStr, usize, &'b S>,
        prefix_len: usize,
    ) -> (Self, usize)
    where
        for<'l> &'l S: litemap::store::StoreSlice<&'a AsciiStr, usize, Slice = S>,
        'a: 'b,
    {
        let first: (&'a AsciiStr, usize) = match items.first() {
            Some((k, v)) => (*k, *v),
            None => unreachable!(),
        };
        let mut initial_value = None;
        let mut total_size = 0;
        let items = if first.0.len() == prefix_len {
            initial_value = Some(first.1);
            items.get_indexed_range(1..items.len()).unwrap()
        } else {
            items.as_sliced()
        };
        if !items.is_empty() {
            let mut i = items.len() - 1;
            let mut j = items.len();
            let mut current_ascii = items.last().unwrap().0.ascii_at(prefix_len).unwrap();
            let mut children = ConstStackChildrenStore::cs_new_empty();
            while i > 0 {
                let c = items
                    .get_indexed(i - 1)
                    .unwrap()
                    .0
                    .ascii_at(prefix_len)
                    .unwrap();
                if c != current_ascii {
                    let size;
                    (self, size) = self.create_recursive(items.get_indexed_range(i..j).unwrap(), prefix_len + 1);
                    total_size += size;
                    children = children.cs_push(current_ascii, size);
                    current_ascii = c;
                    j = i;
                }
                i -= 1;
            }
            let size;
            (self, size) = self.create_recursive(items.get_indexed_range(i..j).unwrap(), prefix_len + 1);
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
                (self, size) = self.prepend_branch(children.cs_as_slice());
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
