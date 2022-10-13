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

    pub fn byte_len(&self) -> usize {
        self.data.atbs_len()
    }

    fn prepend_ascii(&mut self, ascii: AsciiByte) {
        self.data.atbs_push_front(ascii.get())
    }

    fn prepend_value(&mut self, value: usize) {
        if value > 0b00011111 {
            todo!()
        }
        self.data.atbs_push_front((value as u8) | 0b10000000);
    }

    fn make_branch(targets: &[(AsciiByte, Self)]) -> Self {
        let n = targets.len();
        if n > 0b00011111 {
            todo!()
        }
        let trie_lengths = targets
            .iter()
            .map(|(_, builder)| builder.byte_len())
            .sum::<usize>();
        if trie_lengths > 256 {
            todo!()
        }
        // 1 for header, N bytes, N jump indices, and all tries
        let capacity = 1 + n * 2 + trie_lengths;
        let mut data = B::atbs_with_capacity(capacity);
        data.atbs_push_back((n as u8) | 0b11000000);
        for (ascii, _) in targets.iter() {
            data.atbs_push_back(ascii.get());
        }
        let mut index = 0;
        for (_, trie) in targets.iter() {
            data.atbs_push_back(index.try_into().unwrap());
            index += trie.byte_len();
        }
        for (_, trie) in targets.iter() {
            data.atbs_extend(&trie.data);
        }
        debug_assert_eq!(capacity, data.atbs_len());
        Self { data }
    }

    pub fn from_litemap<'a, S>(items: LiteMap<&'a AsciiStr, usize, S>) -> Self
    where
        S: litemap::store::StoreSlice<&'a AsciiStr, usize>,
        for<'l> &'l S::Slice: litemap::store::StoreSlice<&'a AsciiStr, usize, Slice = S::Slice>,
    {
        Self::create_recursive(items.as_sliced(), 0)
    }

    fn create_recursive<'a, 'b, S: ?Sized>(
        items: LiteMap<&'a AsciiStr, usize, &'b S>,
        prefix_len: usize,
    ) -> Self
    where
        for<'l> &'l S: litemap::store::StoreSlice<&'a AsciiStr, usize, Slice = S>,
        'a: 'b,
    {
        let first: (&'a AsciiStr, usize) = match items.first() {
            Some((k, v)) => (*k, *v),
            None => return Self::new(),
        };
        let mut initial_value = None;
        let items = if first.0.len() == prefix_len {
            initial_value = Some(first.1);
            items.get_indexed_range(1..items.len()).unwrap()
        } else {
            items.as_sliced()
        };
        let mut result;
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
                    let inner = Self::create_recursive(
                        items.get_indexed_range(i..j).unwrap(),
                        prefix_len + 1,
                    );
                    children.insert(0, (current_ascii, inner));
                    current_ascii = c;
                    j = i;
                }
                i -= 1;
            }
            let last_child =
                Self::create_recursive(items.get_indexed_range(i..j).unwrap(), prefix_len + 1);
            if children.is_empty() {
                // All strings start with same byte
                result = last_child;
                result.prepend_ascii(current_ascii);
            } else {
                // Need to make a branch node
                children.insert(0, (current_ascii, last_child));
                result = Self::make_branch(&children);
            }
        } else {
            result = Self::new();
        }
        if let Some(value) = initial_value {
            result.prepend_value(value);
        }
        result
    }
}
