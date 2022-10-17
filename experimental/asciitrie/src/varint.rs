// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Varint spec for AsciiTrie:
//!
//! - First byte: top 2 bits are trie metadata; third is varint extender; rest is value
//! - Remaining bytes: top bit is varint extender; add rest to current value * 2^7

pub fn read_varint(start: u8, remainder: &[u8]) -> Option<(usize, &[u8])> {
    let mut value = (start & 0b00011111) as usize;
    let mut remainder = remainder;
    if (start & 0b00100000) != 0 {
        loop {
            let next;
            (next, remainder) = remainder.split_first()?;
            // Note: value << 7 could drop high bits. The addition can't overflow.
            value = (value << 7) + ((next & 0b01111111) as usize);
            if (next & 0b10000000) == 0 {
                break;
            }
        }
    }
    Some((value, remainder))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        #[derive(Debug)]
        struct TestCase<'a> {
            bytes: &'a [u8],
            remainder: &'a [u8],
            value: usize,
        }
        let cases = [
            TestCase {
                bytes: &[0b00000000],
                remainder: &[],
                value: 0,
            },
            TestCase {
                bytes: &[0b00001010],
                remainder: &[],
                value: 10,
            },
            TestCase {
                bytes: &[0b00011111],
                remainder: &[],
                value: 31,
            },
            TestCase {
                bytes: &[0b00011111, 0b10101010],
                remainder: &[0b10101010],
                value: 31,
            },
            // NOTE: The bit patterns are not unique, as shown below.
            // This violates the weak ULE byte equality invariant.
            TestCase {
                bytes: &[0b00100000, 0b00000000],
                remainder: &[],
                value: 0,
            },
            TestCase {
                bytes: &[0b00100000, 0b00100000],
                remainder: &[],
                value: 32,
            },
            TestCase {
                bytes: &[0b00111111, 0b01111111],
                remainder: &[],
                value: (1 << 12) - 1,
            },
            TestCase {
                bytes: &[0b00100000, 0b10100000, 0b00000000],
                remainder: &[],
                value: (1 << 12),
            },
        ];
        for cas in cases {
            let actual = read_varint(cas.bytes[0], &cas.bytes[1..]).unwrap();
            assert_eq!(actual, (cas.value, cas.remainder), "{:?}", cas);
        }
    }
}
