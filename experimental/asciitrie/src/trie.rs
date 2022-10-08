// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::varint::read_varint; 

pub trait AsciiTrieStore {
}

#[repr(transparent)]
pub struct AsciiTrie<S: ?Sized> {
    #[doc(hidden)]
    pub data: S,
}

/// Like slice::split_at but returns an Option instead of panicking
#[inline]
fn maybe_split_at(slice: &[u8], mid: usize) -> Option<(&[u8], &[u8])> {
    if mid > slice.len() {
        None
    } else {
        // Safety: `mid` is in bounds
        // Note: we could use split_at_unchecked once stabilized
        unsafe {
            Some((
                slice.get_unchecked(0..mid),
                slice.get_unchecked(mid..slice.len()),
            ))
        }
    }
}

impl AsciiTrie<[u8]> {
    pub fn from_bytes_2(trie: &[u8]) -> &Self {
        // Safety: repr(transparent)
        unsafe { core::mem::transmute(trie) }
    }
}

impl<'a> AsciiTrie<&'a [u8]> {
    pub fn from_bytes(trie: &'a [u8]) -> Self {
        Self {
            data: trie,
        }
    }
}

impl<S> AsciiTrie<S> where S: AsRef<[u8]> + ?Sized {
    pub fn get(&self, ascii: &[u8]) -> Option<usize> {
        get(self.data.as_ref(), ascii)
    }

    pub fn is_empty(&self) -> bool {
        self.data.as_ref().is_empty()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.data.as_ref()
    }

    pub fn as_borrowed(&self) -> &AsciiTrie<[u8]> {
        AsciiTrie::from_bytes_2(self.data.as_ref())
    }
}

// impl<S> alloc::borrow::ToOwned for AsciiTrie<S> where S: alloc::borrow::ToOwned {
//     type Owned = 
// }

#[cfg(feature = "alloc")]
impl<S> AsciiTrie<S> where S: alloc::borrow::ToOwned + ?Sized {
    pub fn to_owned(&self) -> AsciiTrie<S::Owned> {
        AsciiTrie {
            data: self.data.to_owned()
        }
    }
}

pub fn get(mut trie: &[u8], mut ascii: &[u8]) -> Option<usize> {
    loop {
        let (b, x, i, w, p, q, search, indices);
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
            w = if trie.len() > 256 { 2 } else { 1 };
            (indices, trie) = maybe_split_at(trie, x * w)?;
            (p, q) = if w == 1 {
                (
                    indices.get(i).copied().map(usize::from).unwrap(),
                    indices
                        .get(i + 1)
                        .copied()
                        .map(usize::from)
                        .unwrap_or(trie.len()),
                )
            } else {
                todo!()
            };
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
