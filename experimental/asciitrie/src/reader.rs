// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::varint::read_varint;

/// Like slice::split_at but returns an Option instead of panicking
#[inline]
fn maybe_split_at(slice: &[u8], mid: usize) -> Option<(&[u8], &[u8])> {
    if mid > slice.len() {
        None
    } else {
        // Note: We're trusting the compiler to inline this and remove the assertion
        // hiding on the top of slice::split_at: `assert(mid <= self.len())`
        Some(slice.split_at(mid))
    }
}

#[inline]
fn get_usize(slice: &[u8]) -> usize {
    let mut result = 0;
    let mut i = 0;
    while i < slice.len() {
        result <<= 8;
        result += slice[i] as usize;
        i += 1;
    }
    result
}

pub fn get(mut trie: &[u8], mut ascii: &[u8]) -> Option<usize> {
    loop {
        let (b, x, i, mut w, p, q, search, indices);
        (b, trie) = trie.split_first()?;
        if let Some((c, temp)) = ascii.split_first() {
            if b == c {
                // Matched a byte
                ascii = temp;
                continue;
            }
            if (0b10000000 & b) == 0 {
                // Byte that doesn't match
                return None;
            }
            (x, trie) = read_varint(*b, trie)?;
            if (0b01000000 & b) == 0 {
                // Value node, but not at end of string
                continue;
            }
            // Branch node
            (search, trie) = maybe_split_at(trie, x)?;
            i = search.binary_search(c).ok()?;
            w = 1;
            while trie.len() - w * x > 1 << (w * 8) {
                w += 1;
            }
            (indices, trie) = maybe_split_at(trie, x * w)?;
            let p_range = i * w..(i + 1) * w;
            let q_range = (i + 1) * w..(i + 2) * w;
            p = match indices.get(p_range).map(get_usize) {
                Some(x) => x,
                None => {
                    debug_assert!(false, "p_range should be in range due to binary search");
                    return None;
                }
            };
            q = indices.get(q_range).map(get_usize).unwrap_or(trie.len());
            trie = trie.get(p..q)?;
            ascii = temp;
            continue;
        } else {
            if (0b11000000 & b) == 0b10000000 {
                // Value node at end of string
                let (x, _trie) = read_varint(*b, trie)?;
                return Some(x);
            }
            return None;
        }
    }
}
