// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::AsciiTrie;
use alloc::collections::BTreeMap;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use litemap::LiteMap;

#[derive(Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub(crate) enum AsciiTrieBuilderError {
    NonAscii,
}

/// A byte that is always ASCII.
/// TODO: Consider making this the same as tinystr AsciiByte?
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct AsciiByte(u8);

impl AsciiByte {
    pub const fn try_from_u8(byte: u8) -> Result<Self, AsciiTrieBuilderError> {
        if byte >= 0b10000000 {
            return Err(AsciiTrieBuilderError::NonAscii);
        }
        Ok(Self(byte))
    }

    pub fn get(self) -> u8 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct AsciiStr([AsciiByte]);

impl AsciiStr {
    pub const fn try_from_bytes(bytes: &[u8]) -> Result<&Self, AsciiTrieBuilderError> {
        let mut i = 0;
        while i < bytes.len() {
            match AsciiByte::try_from_u8(bytes[i]) {
                Ok(b) => (),
                Err(e) => return Err(e),
            };
            i += 1;
        }
        // Safety:
        // - AsciiByte is transparent over u8
        // - AsciiStr is transparent over [AsciiByte]
        // - Therefore, AsciiStr is transparent over [u8]
        unsafe { core::mem::transmute(bytes) }
    }

    pub const fn try_from_str(s: &str) -> Result<&Self, AsciiTrieBuilderError> {
        Self::try_from_bytes(s.as_bytes())
    }

    pub const fn from_slice(slice: &[AsciiByte]) -> &Self {
        // Safety: AsciiStr is transparent over [AsciiByte]
        unsafe { core::mem::transmute(slice) }
    }

    pub const fn len(&self) -> usize {
        self.0.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn ascii_at(&self, index: usize) -> Option<AsciiByte> {
        self.0.get(index).copied()
    }

    pub fn as_bytes(&self) -> &[u8] {
        // Safety:
        // - AsciiByte is transparent over u8
        // - AsciiStr is transparent over [AsciiByte]
        // - Therefore, AsciiStr is transparent over [u8]
        unsafe { core::mem::transmute(self) }
    }

    pub(crate) const fn split_first(&self) -> Option<(AsciiByte, &AsciiStr)> {
        match self.0.split_first() {
            Some((b, remainder)) => Some((*b, Self::from_slice(remainder))),
            None => None,
        }
    }
}

/// A low-level builder for AsciiTrie.
pub(crate) struct AsciiTrieBuilder {
    data: VecDeque<u8>,
}

impl AsciiTrieBuilder {
    pub fn to_ascii_trie(&mut self) -> AsciiTrie {
        let slice = self.data.make_contiguous();
        AsciiTrie::from_bytes(slice)
    }

    pub fn to_bytes(&mut self) -> &[u8] {
        let slice = self.data.make_contiguous();
        slice
    }

    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    pub fn byte_len(&self) -> usize {
        self.data.len()
    }

    pub fn new_with_value(value: usize) -> Self {
        let mut result = Self::new();
        result.prepend_value(value);
        result
    }

    pub fn prepend_ascii(&mut self, ascii: AsciiByte) {
        self.data.push_front(ascii.get())
    }

    pub fn prepend_value(&mut self, value: usize) {
        if value > 0b00011111 {
            todo!()
        }
        self.data.push_front((value as u8) | 0b10000000);
    }

    pub fn make_branch(targets: &[(AsciiByte, Self)]) -> Self {
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
        let mut data = VecDeque::with_capacity(capacity);
        data.push_back((n as u8) | 0b11000000);
        for (ascii, _) in targets.iter() {
            data.push_back(ascii.get());
        }
        let mut index = 0;
        for (_, trie) in targets.iter() {
            data.push_back(index.try_into().unwrap());
            index += trie.byte_len();
        }
        for (_, trie) in targets.iter() {
            data.extend(&trie.data);
        }
        debug_assert_eq!(capacity, data.len());
        Self { data }
    }

    pub fn create_from_sorted(mut values: &[(&AsciiStr, usize)]) -> Self {
        let mut initial_value = None;
        let mut current;
        (current, values) = match values.split_first() {
            Some((current, values)) => match current.0.len() {
                0 => {
                    let value = current.1;
                    initial_value = Some(current.1);
                    match values.split_first() {
                        Some(t) => t,
                        // Single value, empty string:
                        None => return Self::new_with_value(value),
                    }
                }
                _ => (current, values),
            },
            // No values:
            None => return Self::new(),
        };
        // let mut next = BTreeMap::new();
        // let mut current_ascii = first_pair.0.split_first().expect("non-empty").0;
        todo!()
    }

    pub fn try_from_litemap<'a, S>(mut values: LiteMap<&'a AsciiStr, usize, S>) -> Self
    where
        S: litemap::store::StoreSlice<&'a AsciiStr, usize>,
        for<'l> &'l S::Slice: litemap::store::StoreSlice<&'a AsciiStr, usize, Slice = S::Slice>,
    {
        Self::create_recursive(values.as_sliced(), 0)
    }

    fn create_recursive<'a, 'b, S: ?Sized>(
        values: LiteMap<&'a AsciiStr, usize, &'b S>,
        prefix_len: usize,
    ) -> Self
    where
        for<'l> &'l S: litemap::store::StoreSlice<&'a AsciiStr, usize, Slice = S>,
        'a: 'b,
    {
        let first: (&'a AsciiStr, usize) = match values.first() {
            Some((k, v)) => (*k, *v),
            None => return Self::new(),
        };
        let mut initial_value = None;
        let values = if first.0.len() == prefix_len {
            initial_value = Some(first.1);
            values.get_indexed_range(1..values.len()).unwrap()
        } else {
            values.as_sliced()
        };
        let mut result;
        if !values.is_empty() {
            let mut i = 0;
            let mut j = 1;
            let mut current_ascii = values.first().unwrap().0.ascii_at(prefix_len).unwrap();
            let mut children = Vec::new();
            while j < values.len() {
                let c = values
                    .get_indexed(j)
                    .unwrap()
                    .0
                    .ascii_at(prefix_len)
                    .unwrap();
                if c != current_ascii {
                    let inner = Self::create_recursive(
                        values.get_indexed_range(i..j).unwrap(),
                        prefix_len + 1,
                    );
                    children.push((current_ascii, inner));
                    current_ascii = c;
                    i = j;
                }
                j += 1;
            }
            let last_child =
                Self::create_recursive(values.get_indexed_range(i..j).unwrap(), prefix_len + 1);
            if children.is_empty() {
                // All strings start with same byte
                result = last_child;
                result.prepend_ascii(current_ascii);
            } else {
                // Need to make a branch node
                children.push((current_ascii, last_child));
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

impl<'a> FromIterator<(&'a [u8], usize)> for AsciiTrie<'static> {
    fn from_iter<T: IntoIterator<Item = (&'a [u8], usize)>>(iter: T) -> Self {
        todo!()
    }
}

mod tests {
    use super::*;

    fn check_ascii_trie(values: &LiteMap<&AsciiStr, usize>, trie: &AsciiTrie) {
        for (k, v) in values.iter() {
            assert_eq!(trie.get(k.as_bytes()), Some(*v));
        }
    }

    #[test]
    fn test_empty() {
        let mut builder = AsciiTrieBuilder::try_from_litemap(LiteMap::new_vec());
        assert_eq!(builder.byte_len(), 0);
        assert!(builder.to_ascii_trie().is_empty());
        assert_eq!(builder.to_ascii_trie().get(b""), None);
        assert_eq!(builder.to_bytes(), &[]);
    }

    #[test]
    fn test_single_empty_value() {
        let litemap: LiteMap<&AsciiStr, usize> = [
            (AsciiStr::try_from_str("").unwrap(), 10), //
        ]
        .into_iter()
        .collect();
        let mut builder = AsciiTrieBuilder::try_from_litemap(litemap.as_sliced());
        assert_eq!(builder.byte_len(), 1);
        assert_eq!(builder.to_ascii_trie().get(b""), Some(10));
        assert_eq!(builder.to_ascii_trie().get(b"x"), None);
        assert_eq!(builder.to_bytes(), &[0b10001010]);
    }

    #[test]
    fn test_single_byte_string() {
        let litemap: LiteMap<&AsciiStr, usize> = [
            (AsciiStr::try_from_str("x").unwrap(), 10), //
        ]
        .into_iter()
        .collect();
        let mut builder = AsciiTrieBuilder::try_from_litemap(litemap.as_sliced());
        assert_eq!(builder.byte_len(), 2);
        assert_eq!(builder.to_ascii_trie().get(b""), None);
        assert_eq!(builder.to_ascii_trie().get(b"xy"), None);
        check_ascii_trie(&litemap, &builder.to_ascii_trie());
        assert_eq!(builder.to_bytes(), &[b'x', 0b10001010]);
    }

    #[test]
    fn test_single_string() {
        let litemap: LiteMap<&AsciiStr, usize> = [
            (AsciiStr::try_from_str("xyz").unwrap(), 10), //
        ]
        .into_iter()
        .collect();
        let mut builder = AsciiTrieBuilder::try_from_litemap(litemap.as_sliced());
        assert_eq!(builder.byte_len(), 4);
        assert_eq!(builder.to_ascii_trie().get(b""), None);
        assert_eq!(builder.to_ascii_trie().get(b"x"), None);
        assert_eq!(builder.to_ascii_trie().get(b"xy"), None);
        assert_eq!(builder.to_ascii_trie().get(b"xyzz"), None);
        check_ascii_trie(&litemap, &builder.to_ascii_trie());
        assert_eq!(builder.to_bytes(), &[b'x', b'y', b'z', 0b10001010]);
    }

    #[test]
    fn test_prefix_strings() {
        let litemap: LiteMap<&AsciiStr, usize> = [
            (AsciiStr::try_from_str("x").unwrap(), 0),
            (AsciiStr::try_from_str("xy").unwrap(), 1),
        ]
        .into_iter()
        .collect();
        let mut builder = AsciiTrieBuilder::try_from_litemap(litemap.as_sliced());
        assert_eq!(builder.byte_len(), 4);
        assert_eq!(builder.to_ascii_trie().get(b""), None);
        assert_eq!(builder.to_ascii_trie().get(b"xyz"), None);
        check_ascii_trie(&litemap, &builder.to_ascii_trie());
        assert_eq!(
            builder.to_bytes(),
            &[b'x', 0b10000000, b'y', 0b10000001]
        );
    }

    #[test]
    fn test_single_byte_branch() {
        let litemap: LiteMap<&AsciiStr, usize> = [
            (AsciiStr::try_from_str("x").unwrap(), 0),
            (AsciiStr::try_from_str("y").unwrap(), 1),
        ]
        .into_iter()
        .collect();
        let mut builder = AsciiTrieBuilder::try_from_litemap(litemap.as_sliced());
        assert_eq!(builder.byte_len(), 7);
        assert_eq!(builder.to_ascii_trie().get(b""), None);
        assert_eq!(builder.to_ascii_trie().get(b"xy"), None);
        check_ascii_trie(&litemap, &builder.to_ascii_trie());
        assert_eq!(
            builder.to_bytes(),
            &[0b11000010, b'x', b'y', 0, 1, 0b10000000, 0b10000001]
        );
    }

    #[test]
    fn test_multi_byte_branch() {
        let litemap: LiteMap<&AsciiStr, usize> = [
            (AsciiStr::try_from_str("axb").unwrap(), 0),
            (AsciiStr::try_from_str("ayc").unwrap(), 1),
        ]
        .into_iter()
        .collect();
        let mut builder = AsciiTrieBuilder::try_from_litemap(litemap.as_sliced());
        assert_eq!(builder.byte_len(), 10);
        assert_eq!(builder.to_ascii_trie().get(b""), None);
        assert_eq!(builder.to_ascii_trie().get(b"a"), None);
        assert_eq!(builder.to_ascii_trie().get(b"ax"), None);
        assert_eq!(builder.to_ascii_trie().get(b"ay"), None);
        check_ascii_trie(&litemap, &builder.to_ascii_trie());
        assert_eq!(
            builder.to_bytes(),
            &[b'a', 0b11000010, b'x', b'y', 0, 2, b'b', 0b10000000, b'c', 0b10000001]
        );
    }

    #[test]
    fn test_everything() {
        let litemap: LiteMap<&AsciiStr, usize> = [
            (AsciiStr::try_from_str("").unwrap(), 0),
            (AsciiStr::try_from_str("axb").unwrap(), 1),
            (AsciiStr::try_from_str("ayc").unwrap(), 2),
            (AsciiStr::try_from_str("azd").unwrap(), 3),
            (AsciiStr::try_from_str("bxe").unwrap(), 4),
            (AsciiStr::try_from_str("bxefg").unwrap(), 5),
            (AsciiStr::try_from_str("bxefh").unwrap(), 6),
            (AsciiStr::try_from_str("bxei").unwrap(), 7),
            (AsciiStr::try_from_str("bxeikl").unwrap(), 8),
        ]
        .into_iter()
        .collect();
        let mut builder = AsciiTrieBuilder::try_from_litemap(litemap.as_sliced());
        assert_eq!(builder.byte_len(), 38);
        assert_eq!(builder.to_ascii_trie().get(b""), Some(0));
        assert_eq!(builder.to_ascii_trie().get(b"a"), None);
        assert_eq!(builder.to_ascii_trie().get(b"ax"), None);
        assert_eq!(builder.to_ascii_trie().get(b"ay"), None);
        check_ascii_trie(&litemap, &builder.to_ascii_trie());
        assert_eq!(
            builder.to_bytes(),
            &[
                0b10000000, // value 0
                0b11000010, // branch of 2
                b'a',
                b'b',
                0,
                13,
                0b11000011, // branch of 3
                b'x',
                b'y',
                b'z',
                0,
                2,
                4,
                b'b',
                0b10000001, // value 1
                b'c',
                0b10000010, // value 2
                b'd',
                0b10000011, // value 3
                b'x',
                b'e',
                0b10000100, // value 4
                0b11000010, // branch of 2
                b'f',
                b'i',
                0,
                7,
                0b11000010, // branch of 2
                b'g',
                b'h',
                0,
                1,
                0b10000101, // value 5
                0b10000110, // value 6
                0b10000111, // value 7
                b'k',
                b'l',
                0b10001000, // value 8
            ]
        );
    }
}
