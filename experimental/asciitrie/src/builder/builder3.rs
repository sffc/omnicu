// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::const_util::const_for_each;
use super::const_util::ConstSlice;
use super::store::BranchType3;
use super::store::ConstAsciiTrieBuilderStore;
use super::store::ConstLengthsStack3;
use super::AsciiByte;
use super::AsciiStr;
use crate::varint;

extern crate std;

#[derive(Debug)]
enum BranchPosition {
    Lesser(usize),
    EqualGreater,
}

fn get_branch_position(diff_i: usize, diff_j: usize) -> BranchPosition {
    assert!(diff_i + diff_j > 0);
    let n = diff_i + diff_j + 1;
    let mut i = 0;
    let mut j = n;
    let mut levels = 0;
    loop {
        let width = j - i;
        if width == 1 {
            return BranchPosition::Lesser(levels);
        }
        let mid = i + width / 2;
        if width == 2 {
            if mid == diff_i {
                return BranchPosition::EqualGreater;
            } else {
                return BranchPosition::Lesser(levels + 1);
            }
        }
        if mid <= diff_i {
            levels = 0;
            i = mid;
        } else {
            levels += 1;
            j = mid;
        }
    }
}

/// A low-level builder for AsciiTrie.
pub(crate) struct AsciiTrieBuilder3<const N: usize> {
    data: ConstAsciiTrieBuilderStore<N>,
}

impl<const N: usize> AsciiTrieBuilder3<N> {
    // #[cfg(feature = "alloc")]
    // pub fn to_ascii_trie(&mut self) -> AsciiTrie<&[u8]> {
    //     let slice = self.data.atbs_as_bytes();
    //     AsciiTrie(slice.as_slice())
    // }

    #[cfg(feature = "alloc")]
    pub fn as_bytes(&self) -> &[u8] {
        self.data.atbs_as_bytes().as_slice()
    }

    // pub const fn into_ascii_trie_or_panic(self) -> AsciiTrie<[u8; N]> {
    //     AsciiTrie(self.data.take_or_panic())
    // }

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
    const fn prepend_value(self, value: usize, is_final: bool) -> (Self, usize) {
        let mut data = self.data;
        let varint_array = varint::write_varint2(value);
        data = data.atbs_extend_front(varint_array.as_const_slice());
        if is_final {
            data = data.atbs_bitor_assign(0, 0b10100000);
        } else {
            data = data.atbs_bitor_assign(0, 0b10000000);
        }
        (Self { data }, varint_array.len())
    }

    #[must_use]
    fn prepend_branch(self, value: usize, is_final: bool) -> (Self, usize) {
        let mut data = self.data;
        let varint_array = varint::write_varint2(value);
        data = data.atbs_extend_front(varint_array.as_const_slice());
        if is_final {
            data = data.atbs_bitor_assign(0, 0b11100000);
        } else {
            data = data.atbs_bitor_assign(0, 0b11000000);
        }
        (Self { data }, varint_array.len())
    }

    /// Panics if the items are not sorted
    pub fn from_tuple_slice<'a>(items: &[(&'a AsciiStr, usize)]) -> Self {
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
    pub fn from_sorted_const_tuple_slice<'a>(items: ConstSlice<(&'a AsciiStr, usize)>) -> Self {
        let mut result = Self::new();
        let total_size;
        (result, total_size) = result.create(items);
        debug_assert!(total_size == result.data.atbs_len());
        result
    }

    #[must_use]
    fn create<'a>(mut self, all_items: ConstSlice<(&'a AsciiStr, usize)>) -> (Self, usize) {
        if all_items.is_empty() {
            return (Self::new(), 0);
        }
        let mut lengths_stack = ConstLengthsStack3::<100>::new();
        let mut prefix_len = all_items.last().unwrap().0.len();
        let mut i = all_items.len() - 1;
        let mut j = all_items.len();
        let mut current_len = 0;
        loop {
            let item_i = all_items.get_or_panic(i);
            let item_j = all_items.get_or_panic(j - 1);
            assert!(item_i.0.prefix_eq(item_j.0, prefix_len));
            if item_i.0.len() == prefix_len {
                let len;
                (self, len) = self.prepend_value(item_i.1, j - i == 1);
                current_len += len;
            }
            if prefix_len == 0 {
                break;
            }
            prefix_len -= 1;
            let mut new_i = i;
            let mut new_j = j;
            let mut diff_i = 0;
            let mut diff_j = 0;
            let mut ascii_i = item_i.0.ascii_at_or_panic(prefix_len);
            let mut ascii_j = item_j.0.ascii_at_or_panic(prefix_len);
            assert_eq!(ascii_i, ascii_j);
            let key_ascii = ascii_i;
            loop {
                if new_i == 0 {
                    break;
                }
                let candidate = all_items.get_or_panic(new_i - 1).0;
                if candidate.len() < prefix_len {
                    // Too short
                    break;
                }
                if item_i.0.prefix_eq(candidate, prefix_len) {
                    new_i -= 1;
                } else {
                    break;
                }
                if candidate.len() == prefix_len {
                    // A string of length prefix_len can't be preceded by another with that prefix
                    break;
                }
                let candidate = candidate.ascii_at_or_panic(prefix_len);
                if candidate != ascii_i {
                    diff_i += 1;
                    ascii_i = candidate;
                }
            }
            loop {
                if new_j == all_items.len() {
                    break;
                }
                let candidate = all_items.get_or_panic(new_j).0;
                if candidate.len() < prefix_len {
                    // Too short
                    break;
                }
                if item_j.0.prefix_eq(candidate, prefix_len) {
                    new_j += 1;
                } else {
                    break;
                }
                if candidate.len() == prefix_len {
                    panic!("A shorter string should be earlier in the sequence");
                }
                let candidate = candidate.ascii_at_or_panic(prefix_len);
                if candidate != ascii_j {
                    diff_j += 1;
                    ascii_j = candidate;
                }
            }
            if diff_i == 0 && diff_j == 0 {
                let len;
                (self, len) = self.prepend_ascii(ascii_i);
                current_len += len;
                assert!(i == new_i || i == new_i + 1);
                i = new_i;
                assert_eq!(j, new_j);
                continue;
            }
            // Branch
            let branch_position = get_branch_position(diff_i, diff_j);
            match branch_position {
                BranchPosition::Lesser(count) => {
                    let len;
                    (self, len) = self.prepend_ascii(key_ascii);
                    current_len += len;
                    let mut k = 0;
                    while k < count {
                        let (branch_type, size);
                        (lengths_stack, (branch_type, size)) = lengths_stack.pop_or_panic();
                        match branch_type {
                            BranchType3::EqualGreater(ascii) => {
                                let len;
                                (self, len) = self.prepend_ascii(ascii);
                                let len2;
                                (self, len2) = self.prepend_branch(current_len, false);
                                current_len += len + len2 + size;
                            }
                            BranchType3::EqualGreaterFinal(ascii) => {
                                let len;
                                (self, len) = self.prepend_ascii(ascii);
                                let len2;
                                (self, len2) = self.prepend_branch(current_len, true);
                                current_len += len + len2 + size;
                            }
                        }
                        k += 1;
                    }
                    lengths_stack =
                        lengths_stack.push(BranchType3::EqualGreater(key_ascii), current_len);
                    current_len = 0;
                }
                BranchPosition::EqualGreater => {
                    lengths_stack =
                        lengths_stack.push(BranchType3::EqualGreaterFinal(key_ascii), current_len);
                    current_len = 0;
                }
            }
            if diff_i == 0 {
                i = new_i;
                j = new_j;
                let (branch_type, len);
                (lengths_stack, (branch_type, len)) = lengths_stack.pop_or_panic();
                assert!(matches!(branch_type, BranchType3::EqualGreater(_)));
                current_len += len;
            } else {
                j = i;
                i -= 1;
                prefix_len = all_items.get_or_panic(i).0.len();
            }
        }
        assert!(lengths_stack.is_empty());
        (self, current_len)
    }
}
