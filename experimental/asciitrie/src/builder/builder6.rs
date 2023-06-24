// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::const_util::ConstSlice;
use super::tstore::MutableLengthsStack1b;
use super::tstore::TrieBuilderStore;
use super::BranchMeta;
use crate::builder::bytestr::ByteStr;
use crate::byte_phf::PerfectByteHashMapCacheOwned;
use crate::error::Error;
use crate::varint;
use alloc::vec::Vec;

extern crate std;

pub enum PhfMode {
    BinaryOnly,
    UsePhf,
}

pub enum AsciiMode {
    AsciiOnly,
    BinarySpans,
}

pub enum CapacityMode {
    Normal,
    Extended,
}

pub struct ZeroTrieBuilderOptions {
    pub phf_mode: PhfMode,
    pub ascii_mode: AsciiMode,
    pub capacity_mode: CapacityMode,
}

/// A low-level builder for AsciiTrie.
pub(crate) struct AsciiTrieBuilder6<S> {
    data: S,
    phf_cache: PerfectByteHashMapCacheOwned,
    options: ZeroTrieBuilderOptions,
}

impl<S: TrieBuilderStore> AsciiTrieBuilder6<S> {
    // #[cfg(feature = "alloc")]
    // pub fn to_ascii_trie(&mut self) -> AsciiTrie<&[u8]> {
    //     let slice = self.data.atbs_as_bytes();
    //     AsciiTrie(slice.as_slice())
    // }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.data.atbs_to_bytes()
    }

    // pub const fn into_ascii_trie_or_panic(self) -> AsciiTrie<[u8; N]> {
    //     AsciiTrie(self.data.take_or_panic())
    // }

    pub fn new(options: ZeroTrieBuilderOptions) -> Self {
        Self {
            data: S::atbs_new_empty(),
            phf_cache: PerfectByteHashMapCacheOwned::new_empty(),
            options,
        }
    }

    #[must_use]
    fn prepend_ascii(&mut self, ascii: u8) -> usize {
        if ascii <= 127 {
            self.data.atbs_push_front(ascii);
            1
        } else if matches!(self.options.ascii_mode, AsciiMode::BinarySpans) {
            let old_byte_len = self.data.atbs_len();
            if old_byte_len != 0 {
                let old_front = self.data.atbs_split_first_or_panic();
                if old_front & 0b11100000 == 0b10100000 {
                    // Extend an existing span
                    let old_span_size =
                        varint::read_varint2_from_tstore_or_panic(old_front, &mut self.data);
                    self.data.atbs_push_front(ascii);
                    let varint_array = varint::write_varint2(old_span_size + 1);
                    self.data.atbs_extend_front(varint_array.as_slice());
                    self.data.atbs_bitor_assign(0, 0b10100000);
                    let new_byte_len = self.data.atbs_len();
                    return new_byte_len - old_byte_len;
                } else {
                    self.data.atbs_push_front(old_front);
                }
            }
            // Create a new span
            self.data.atbs_push_front(ascii);
            self.data.atbs_push_front(0b10100001);
            2
        } else {
            panic!("Tried inserting non-ASCII into ASCII-only trie");
        }
    }

    #[must_use]
    fn prepend_value(&mut self, value: usize) -> usize {
        let varint_array = varint::write_varint2(value);
        self.data.atbs_extend_front(varint_array.as_slice());
        self.data.atbs_bitor_assign(0, 0b10000000);
        varint_array.len()
    }

    #[must_use]
    fn prepend_branch(&mut self, value: usize) -> usize {
        let varint_array = varint::write_varint(value);
        self.data.atbs_extend_front(varint_array.as_slice());
        self.data.atbs_bitor_assign(0, 0b11000000);
        varint_array.len()
    }

    fn prepend_slice(&mut self, s: ConstSlice<u8>) {
        let mut i = s.len();
        while i > 0 {
            self.data.atbs_push_front(*s.get_or_panic(i - 1));
            i -= 1;
        }
    }

    /// Assumes that the items are sorted
    pub fn from_sorted_const_tuple_slice<'a>(
        items: ConstSlice<(&'a ByteStr, usize)>,
        options: ZeroTrieBuilderOptions,
    ) -> Result<Self, Error> {
        let mut result = Self::new(options);
        let total_size = result.create(items)?;
        debug_assert!(total_size == result.data.atbs_len());
        Ok(result)
    }

    #[must_use]
    fn create<'a>(&mut self, all_items: ConstSlice<(&'a ByteStr, usize)>) -> Result<usize, Error> {
        if all_items.is_empty() {
            return Ok(0);
        }
        let mut lengths_stack = MutableLengthsStack1b::new();
        let mut prefix_len = all_items.last().unwrap().0.len();
        let mut i = all_items.len() - 1;
        let mut j = all_items.len();
        let mut current_len = 0;
        loop {
            let item_i = all_items.get_or_panic(i);
            let item_j = all_items.get_or_panic(j - 1);
            assert!(item_i.0.prefix_eq(item_j.0, prefix_len));
            if item_i.0.len() == prefix_len {
                let len = self.prepend_value(item_i.1);
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
            let mut ascii_i = item_i.0.byte_at_or_panic(prefix_len);
            let mut ascii_j = item_j.0.byte_at_or_panic(prefix_len);
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
                let candidate = candidate.byte_at_or_panic(prefix_len);
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
                let candidate = candidate.byte_at_or_panic(prefix_len);
                if candidate != ascii_j {
                    diff_j += 1;
                    ascii_j = candidate;
                }
            }
            if diff_i == 0 && diff_j == 0 {
                let len = self.prepend_ascii(ascii_i);
                current_len += len;
                assert!(i == new_i || i == new_i + 1);
                i = new_i;
                assert_eq!(j, new_j);
                continue;
            }
            // Branch
            if diff_j == 0 {
                lengths_stack.push(BranchMeta {
                    ascii: key_ascii,
                    length: current_len,
                    local_length: current_len,
                    count: 1,
                });
            } else {
                let BranchMeta { length, count, .. } = lengths_stack.peek_or_panic();
                lengths_stack.push(BranchMeta {
                    ascii: key_ascii,
                    length: length + current_len,
                    local_length: current_len,
                    count: count + 1,
                });
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
            let mut branch_metas = lengths_stack.pop_many_or_panic(total_count);
            let original_keys = branch_metas.map_to_ascii_bytes();
            let use_phf = matches!(self.options.phf_mode, PhfMode::UsePhf);
            let opt_phf_vec = if total_count > 15 && use_phf {
                let phf_vec = self
                    .phf_cache
                    .try_get_or_insert(original_keys.as_const_slice().as_slice().to_vec())?;
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
                            self.data.atbs_swap_ranges(
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
            let w = (USIZE_BITS - (total_length.leading_zeros() as usize) - 1) / 8;
            if w > 3 && matches!(self.options.capacity_mode, CapacityMode::Normal) {
                return Err(Error::CapacityExceeded);
            }
            let mut k = 0;
            while k <= w {
                self.data.atbs_prepend_n_zeros(total_count - 1);
                current_len += total_count - 1;
                let mut l = 0;
                let mut length_to_write = 0;
                while l < total_count {
                    let BranchMeta { local_length, .. } = *branch_metas
                        .as_const_slice()
                        .get_or_panic(total_count - l - 1);
                    let mut adjusted_length = length_to_write;
                    let mut m = 0;
                    while m < k {
                        adjusted_length >>= 8;
                        m += 1;
                    }
                    if l > 0 {
                        self.data.atbs_bitor_assign(l - 1, adjusted_length as u8);
                    }
                    l += 1;
                    length_to_write += local_length;
                }
                k += 1;
            }
            // Write out the lookup table
            assert!(0 < total_count && total_count <= 256);
            let branch_value = (w << 8) + (total_count & 0xff);
            if let Some(phf_vec) = opt_phf_vec {
                self.data.atbs_extend_front(phf_vec.as_bytes());
                let phf_len = phf_vec.as_bytes().len();
                let branch_len = self.prepend_branch(branch_value);
                current_len += phf_len + branch_len;
            } else {
                self.prepend_slice(original_keys.as_const_slice());
                let branch_len = self.prepend_branch(branch_value);
                current_len += total_count + branch_len;
            }
            i = new_i;
            j = new_j;
        }
        assert!(lengths_stack.is_empty());
        Ok(current_len)
    }
}
