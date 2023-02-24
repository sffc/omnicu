// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::varint::read_varint2;
use core::cmp::Ordering;

extern crate std;

/// Like slice::split_at but returns an Option instead of panicking
#[inline]
fn debug_split_at(slice: &[u8], mid: usize) -> Option<(&[u8], &[u8])> {
    if mid > slice.len() {
        debug_assert!(false, "debug_split_at: index expected to be in range");
        None
    } else {
        // Note: We're trusting the compiler to inline this and remove the assertion
        // hiding on the top of slice::split_at: `assert(mid <= self.len())`
        Some(slice.split_at(mid))
    }
}

enum ByteType {
    Ascii,
    IntermediateValue,
    FinalValue,
    Branch,
    BranchFinal,
}

impl core::fmt::Debug for ByteType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use ByteType::*;
        f.write_str(match *self {
            Ascii => "a",
            IntermediateValue => "i",
            FinalValue => "f\n",
            Branch => "b",
            BranchFinal => "e",
        })
    }
}

#[inline]
fn byte_type(b: u8) -> ByteType {
    match b & 0b11100000 {
        0b10000000 => ByteType::IntermediateValue,
        0b10100000 => ByteType::FinalValue,
        0b11000000 => ByteType::Branch,
        0b11100000 => ByteType::BranchFinal,
        _ => ByteType::Ascii,
    }
}

pub fn get(mut trie: &[u8], mut ascii: &[u8]) -> Option<usize> {
    let mut branch_final = false;
    let mut branch_jump = 0;
    loop {
        let (b, x);
        (b, trie) = trie.split_first()?;
        let byte_type = byte_type(*b);
        (x, trie) = match byte_type {
            ByteType::Ascii => (0, trie),
            _ => read_varint2(*b, trie)?,
        };
        if let Some((c, temp)) = ascii.split_first() {
            match byte_type {
                ByteType::Ascii => {
                    match branch_jump {
                        0 => {
                            if c == b {
                                // Matched a byte (note: high bit of ASCII is expected to be 0)
                                ascii = temp;
                                continue;
                            } else {
                                // Byte that doesn't match
                                return None;
                            }
                        }
                        branch_jump_x => {
                            match (c.cmp(b), branch_final) {
                                (Ordering::Less, _) => {
                                    // Continue forward
                                }
                                (_, false) => {
                                    // Jump forward
                                    (_, trie) = debug_split_at(trie, branch_jump_x)?;
                                }
                                (_, true) => {
                                    // Jump forward and consume the ascii
                                    (_, trie) = debug_split_at(trie, branch_jump_x)?;
                                    ascii = temp;
                                }
                            }
                            branch_jump = 0;
                            continue;
                        }
                    }
                }
                ByteType::IntermediateValue => {
                    // Value node, but not at end of string
                    continue;
                }
                ByteType::FinalValue => {
                    // Final value, but there is still content left in the string
                    return None;
                }
                ByteType::Branch => {
                    // Branch node metadata
                    branch_jump = x;
                    branch_final = false;
                    continue;
                }
                ByteType::BranchFinal => {
                    // Branch node metadata
                    branch_jump = x;
                    branch_final = true;
                    continue;
                }
            }
        } else {
            if matches!(byte_type, ByteType::IntermediateValue)
                || matches!(byte_type, ByteType::FinalValue)
            {
                // Value node at end of string
                return Some(x);
            }
            return None;
        }
    }
}
