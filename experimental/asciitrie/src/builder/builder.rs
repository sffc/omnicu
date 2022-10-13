// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::store::AsciiTrieBuilderStore;
use super::AsciiByte;
use super::AsciiStr;
use crate::AsciiTrie;
use alloc::vec::Vec;
use litemap::LiteMap;

/// A low-level builder for AsciiTrie.
pub(crate) struct AsciiTrieBuilder<B> {
    data: B,
}

impl<B: AsciiTrieBuilderStore> AsciiTrieBuilder<B> {
    pub fn to_ascii_trie(&mut self) -> AsciiTrie<&[u8]> {
        let slice = self.data.atbs_make_contiguous();
        AsciiTrie(slice)
    }

    pub fn new() -> Self {
        Self {
            data: B::atbs_new_empty(),
        }
    }

    #[must_use]
    fn prepend_ascii(&mut self, ascii: AsciiByte) -> usize {
        self.data.atbs_push_front(ascii.get());
        1
    }

    #[must_use]
    fn prepend_value(&mut self, value: usize) -> usize {
        if value > 0b00011111 {
            todo!()
        }
        self.data.atbs_push_front((value as u8) | 0b10000000);
        1
    }

    #[must_use]
    fn prepend_branch(&mut self, targets_rev: &[(AsciiByte, usize)]) -> usize {
        let n = targets_rev.len();
        if n > 0b00011111 {
            todo!()
        }
        let trie_lengths = targets_rev.iter().map(|(_, size)| size).sum::<usize>();
        if trie_lengths > 256 {
            todo!()
        }
        let mut index = trie_lengths;
        for (_, size) in targets_rev.iter() {
            index -= size;
            self.data.atbs_push_front(index.try_into().unwrap());
        }
        for (ascii, _) in targets_rev.iter() {
            self.data.atbs_push_front(ascii.get());
        }
        self.data.atbs_push_front((n as u8) | 0b11000000);
        1 + n * 2
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
        let total_size = result.create_recursive(items.as_sliced(), 0);
        debug_assert_eq!(total_size, result.data.atbs_len());
        result
    }

    #[must_use]
    fn create_recursive<'a, 'b, S: ?Sized>(
        &mut self,
        items: LiteMap<&'a AsciiStr, usize, &'b S>,
        prefix_len: usize,
    ) -> usize
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
            let mut children = Vec::new();
            while i > 0 {
                let c = items
                    .get_indexed(i - 1)
                    .unwrap()
                    .0
                    .ascii_at(prefix_len)
                    .unwrap();
                if c != current_ascii {
                    let inner = self
                        .create_recursive(items.get_indexed_range(i..j).unwrap(), prefix_len + 1);
                    total_size += inner;
                    children.push((current_ascii, inner));
                    current_ascii = c;
                    j = i;
                }
                i -= 1;
            }
            let last_child =
                self.create_recursive(items.get_indexed_range(i..j).unwrap(), prefix_len + 1);
            total_size += last_child;
            if children.is_empty() {
                // All strings start with same byte
                total_size += self.prepend_ascii(current_ascii);
            } else {
                // Need to make a branch node
                children.push((current_ascii, last_child));
                total_size += self.prepend_branch(&children);
            }
        }
        if let Some(value) = initial_value {
            total_size += self.prepend_value(value);
        }
        total_size
    }
}
