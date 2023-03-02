// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::varint::read_varint;
use core::ops::Range;
use crate::byte_phf;

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

#[inline]
fn debug_get(slice: &[u8], index: usize) -> Option<u8> {
    match slice.get(index) {
        Some(x) => Some(*x),
        None => {
            debug_assert!(false, "debug_get: index expected to be in range");
            None
        }
    }
}

#[inline]
fn debug_get_range(slice: &[u8], range: Range<usize>) -> Option<&[u8]> {
    match slice.get(range) {
        Some(x) => Some(x),
        None => {
            debug_assert!(false, "debug_get_range: indices expected to be in range");
            None
        }
    }
}

/// Given a slice starting with an offset table, returns the trie for the given index.
///
/// Arguments:
/// - `trie` = a trie pointing at an offset table (after the branch node and search table)
/// - `i` = the desired index within the offset table
/// - `x` = the number of items in the offset table
fn get_branch(mut trie: &[u8], i: usize, x: usize) -> Option<&[u8]> {
    let mut p = 0usize;
    let mut q = 0usize;
    let mut h = 0usize;
    loop {
        let indices;
        (indices, trie) = debug_split_at(trie, x)?;
        p = (p << 8) + debug_get(indices, i)? as usize;
        q = match indices.get(i + 1) {
            Some(x) => (q << 8) + *x as usize,
            None => trie.len(),
        };
        h = (h << 8) + 0xff;
        if trie.len() <= h {
            break;
        }
    }
    debug_get_range(trie, p..q)
}

enum ByteType {
    Ascii,
    Value,
    Match,
}

impl core::fmt::Debug for ByteType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use ByteType::*;
        f.write_str(match *self {
            Ascii => "a",
            Value => "v",
            Match => "m",
        })
    }
}

#[inline]
fn byte_type(b: u8) -> ByteType {
    match b & 0b11000000 {
        0b10000000 => ByteType::Value,
        0b11000000 => ByteType::Match,
        _ => ByteType::Ascii,
    }
}

pub fn get(mut trie: &[u8], mut ascii: &[u8]) -> Option<usize> {
    loop {
        let (b, x, i, search);
        (b, trie) = trie.split_first()?;
        let byte_type = byte_type(*b);
        (x, trie) = match byte_type {
            ByteType::Ascii => (0, trie),
            _ => read_varint(*b, trie)?,
        };
        if let Some((c, temp)) = ascii.split_first() {
            if b == c {
                // Matched a byte (note: high bit of ASCII is expected to be 0)
                ascii = temp;
                continue;
            }
            if matches!(byte_type, ByteType::Ascii) {
                // Byte that doesn't match
                return None;
            }
            if matches!(byte_type, ByteType::Value) {
                // Value node, but not at end of string
                continue;
            }
            // Branch node
            if x <= 1 {
                debug_assert!(false, "there should be 2 or more branches");
                return None;
            }
            (search, trie) = debug_split_at(trie, x*2+1)?;
            i = byte_phf::get_from_standard_layout(search, *c)?;
            trie = get_branch(trie, i, x)?;
            ascii = temp;
            continue;
        } else {
            if matches!(byte_type, ByteType::Value) {
                // Value node at end of string
                return Some(x);
            }
            return None;
        }
    }
}
