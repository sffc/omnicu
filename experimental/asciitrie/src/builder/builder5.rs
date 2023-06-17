// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::const_util::const_for_each;
use super::const_util::ConstSlice;
use super::store::BranchMeta;
use super::store::ConstAsciiTrieBuilderStore;
use super::store::ConstLengthsStack1b;
use super::AsciiByte;
use super::AsciiStr;
use crate::byte_phf::PerfectByteHashMap;
use crate::varint;

extern crate std;

/// A low-level builder for AsciiTrie.
pub(crate) struct AsciiTrieBuilder5<const N: usize> {
    data: ConstAsciiTrieBuilderStore<N>,
}

impl<const N: usize> AsciiTrieBuilder5<N> {
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
    const fn prepend_value(self, value: usize) -> (Self, usize) {
        let mut data = self.data;
        let varint_array = varint::write_varint(value);
        data = data.atbs_extend_front(varint_array.as_const_slice());
        data = data.atbs_bitor_assign(0, 0b10000000);
        (Self { data }, varint_array.len())
    }

    #[must_use]
    fn prepend_branch(self, value: usize) -> (Self, usize) {
        let mut data = self.data;
        let varint_array = varint::write_varint(value);
        data = data.atbs_extend_front(varint_array.as_const_slice());
        data = data.atbs_bitor_assign(0, 0b11000000);
        (Self { data }, varint_array.len())
    }

    #[must_use]
    const fn prepend_n_zeros(self, n: usize) -> Self {
        let mut data = self.data;
        let mut i = 0;
        while i < n {
            data = data.atbs_push_front(0);
            i += 1;
        }
        Self { data }
    }

    #[must_use]
    const fn prepend_slice(self, s: ConstSlice<u8>) -> Self {
        let mut data = self.data;
        let mut i = s.len();
        while i > 0 {
            data = data.atbs_push_front(*s.get_or_panic(i - 1));
            i -= 1;
        }
        Self { data }
    }

    #[must_use]
    fn swap_ranges(self, start: usize, mid: usize, limit: usize) -> Self {
        let mut data = self.data;
        data = data.atbs_swap_ranges(start, mid, limit);
        Self { data }
    }

    const fn bitor_assign_at(self, index: usize, byte: u8) -> Self {
        let mut data = self.data;
        data = data.atbs_bitor_assign(index, byte);
        Self { data }
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
        let mut lengths_stack = ConstLengthsStack1b::<100>::new();
        let mut prefix_len = match all_items.last() {
            Some(item) => item.0.len(),
            None => unreachable!(),
        };
        let mut i = all_items.len() - 1;
        let mut j = all_items.len();
        let mut current_len = 0;
        loop {
            let item_i = all_items.get_or_panic(i);
            let item_j = all_items.get_or_panic(j - 1);
            assert!(item_i.0.prefix_eq(item_j.0, prefix_len));
            if item_i.0.len() == prefix_len {
                let len;
                (self, len) = self.prepend_value(item_i.1);
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
            assert!(ascii_i.get() == ascii_j.get());
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
                if candidate.get() != ascii_i.get() {
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
                if candidate.get() != ascii_j.get() {
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
                assert!(j == new_j);
                continue;
            }
            // Branch
            if diff_j == 0 {
                lengths_stack = lengths_stack
                    .push(BranchMeta {
                        ascii: key_ascii.get(),
                        length: current_len,
                        local_length: current_len,
                        count: 1,
                    })
                    .unwrap();
            } else {
                let BranchMeta { length, count, .. } = lengths_stack.peek_or_panic();
                lengths_stack = lengths_stack
                    .push(BranchMeta {
                        ascii: key_ascii.get(),
                        length: length + current_len,
                        local_length: current_len,
                        count: count + 1,
                    })
                    .unwrap();
            }
            if diff_i != 0 {
                j = i;
                i -= 1;
                prefix_len = all_items.get_or_panic(i).0.len();
                current_len = 0;
                continue;
            }
            // Branch (first)
            // std::println!("lengths_stack: {lengths_stack:?}");
            let (total_length, total_count) = {
                let BranchMeta { length, count, .. } = lengths_stack.peek_or_panic();
                (length, count)
            };
            let mut branch_metas;
            (lengths_stack, branch_metas) = lengths_stack.pop_many_or_panic(total_count);
            let original_keys = branch_metas.map_to_ascii_bytes();
            let phf_vec =
                PerfectByteHashMap::try_new(original_keys.as_const_slice().as_slice()).unwrap();
            let opt_phf_vec = if total_count > 15 {
                // std::println!("{phf_vec:?}");
                // Put everything in order via bubble sort
                // Note: branch_metas is stored in reverse order (0 = last element)
                loop {
                    let mut l = total_count - 1;
                    let mut changes = 0;
                    let mut start = 0;
                    while l > 0 {
                        let a = *branch_metas.as_const_slice().get_or_panic(l);
                        let b = *branch_metas.as_const_slice().get_or_panic(l - 1);
                        let a_idx = phf_vec.keys().iter().position(|x| x == &a.ascii).unwrap();
                        let b_idx = phf_vec.keys().iter().position(|x| x == &b.ascii).unwrap();
                        if a_idx > b_idx {
                            // std::println!("{a:?} <=> {b:?} ({phf_vec:?})");
                            self = self.swap_ranges(
                                start,
                                start + a.local_length,
                                start + a.local_length + b.local_length,
                            );
                            branch_metas = branch_metas.swap_or_panic(l - 1, l);
                            start += b.local_length;
                            changes += 1;
                            // FIXME: fix the `length` field
                        } else {
                            start += a.local_length;
                        }
                        l -= 1;
                    }
                    if changes == 0 {
                        break;
                    }
                }
                Some(phf_vec)
            } else {
                None
            };
            // Write out the offset table
            current_len = total_length;
            const USIZE_BITS: usize = core::mem::size_of::<usize>() * 8;
            // let max_len = total_length - branch_metas.as_const_slice().get_or_panic(0).local_length;
            let w = (USIZE_BITS - (total_length.leading_zeros() as usize) - 1) / 8;
            let mut k = 0;
            while k <= w {
                self = self.prepend_n_zeros(total_count - 1);
                current_len += total_count - 1;
                let mut l = 0;
                let mut length_to_write = 0;
                while l < total_count {
                    let BranchMeta { local_length, .. } = *branch_metas
                        .as_const_slice()
                        .get_or_panic(total_count - l - 1);
                    // std::println!("length_to_write = {length_to_write:?}");
                    let mut adjusted_length = length_to_write;
                    let mut m = 0;
                    while m < k {
                        adjusted_length >>= 8;
                        m += 1;
                    }
                    if l > 0 {
                        self = self.bitor_assign_at(l - 1, adjusted_length as u8);
                    }
                    l += 1;
                    length_to_write += local_length;
                }
                k += 1;
            }
            // Write out the lookup table
            let branch_len;
            if let Some(phf_vec) = opt_phf_vec {
                // TODO: Assert w <= 3
                // TODO: Assert p < 15
                self = self.prepend_slice(ConstSlice::from_slice(phf_vec.as_bytes()));
                (self, branch_len) = self.prepend_branch((total_count << 2) + w);
                current_len += phf_vec.as_bytes().len() + branch_len;
            } else {
                // TODO: Assert w <= 3
                self = self.prepend_slice(original_keys.as_const_slice());
                (self, branch_len) = self.prepend_branch((total_count << 2) + w);
                current_len += total_count + branch_len;
            }
            /*
            self = self.prepend_n_zeros(total_count);
            current_len += total_count;
            let mut l = 0;
            while l < total_count {
                let BranchMeta { ascii, .. } = *branch_metas.as_const_slice().get_or_panic(l);
                self = self.bitor_assign_at(l, ascii.get());
                l += 1;
            }
            */
            i = new_i;
            j = new_j;
        }
        assert!(lengths_stack.is_empty());
        (self, current_len)
    }
}
