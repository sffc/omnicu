// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Varint spec for AsciiTrie:
//!
//! - Lead byte: top 2 bits are trie metadata; third is varint extender; rest is value
//! - Trail bytes: top bit is varint extender; add rest to current value * 2^7
//! - Add the "latent value" to the final result: (1<<5) + (1<<7) + (1<<14) + ...

use crate::builder::const_util::ConstArrayBuilder;

const X_5BIT: u8 = 0b00011111;

#[cfg(test)]
const MAX_VARINT: usize = usize::MAX;

// *Upper Bound:* Each trail byte stores 7 bits of data, plus the latent value.
// The +6 gives us a ceiling division.
// Add an extra 1 since the lead byte holde very little data.
const MAX_VARINT_LENGTH: usize = 1 + (core::mem::size_of::<usize>() * 8 + 6) / 7;

pub const fn read_varintb_5bits(start: u8, remainder: &[u8]) -> Option<(usize, &[u8])> {
    const X: u8 = X_5BIT;
    let initial = start & X;
    let mut remainder = remainder;
    let mut value = 0usize;
    if initial == X {
        loop {
            let next;
            (next, remainder) = match remainder.split_first() {
                Some(t) => t,
                None => return None,
            };
            // Note: value << 7 could drop high bits. The first addition can't overflow.
            // The second addition could overflow; in such a case we just inform the
            // developer via the debug assertion.
            value = (value << 7) + ((*next & 0b01111111) as usize);
            if (*next & 0b10000000) == 0 {
                break;
            }
        }
    }
    value += initial as usize;
    Some((value, remainder))
}


pub(crate) const fn read_varintb_5bits_from_store_or_panic<const N: usize>(start: u8, remainder: ConstArrayBuilder<N, u8>) -> (usize, ConstArrayBuilder<N, u8>) {
    const X: u8 = X_5BIT;
    let initial = start & X;
    let mut remainder = remainder;
    let mut value = 0usize;
    if initial == X {
        loop {
            let next;
            (next, remainder) = remainder.split_first_or_panic();
            // Note: value << 7 could drop high bits. The first addition can't overflow.
            // The second addition could overflow; in such a case we just inform the
            // developer via the debug assertion.
            value = (value << 7) + ((next & 0b01111111) as usize);
            if (next & 0b10000000) == 0 {
                break;
            }
        }
    }
    value += initial as usize;
    (value, remainder)
}

pub(crate) const fn write_varintb_5bits(value: usize) -> ConstArrayBuilder<MAX_VARINT_LENGTH, u8> {
    const X: u8 = X_5BIT;
    let mut result = [0; MAX_VARINT_LENGTH];
    let mut i = MAX_VARINT_LENGTH - 1;
    if value < X as usize {
        result[i] = value as u8;
    } else {
        let mut value = value - X as usize;
        let mut last = true;
        loop {
            result[i] = (value as u8) & 0b01111111;
            if !last {
                result[i] |= 0b10000000;
            } else {
                last = false;
            }
            value >>= 7;
            i -= 1;
            if value == 0 {
                break;
            }
        }
        result[i] = X;
    }
    // The bytes are from i to the end.
    ConstArrayBuilder::from_manual_slice(result, i, MAX_VARINT_LENGTH)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestCase<'a> {
        bytes: &'a [u8],
        remainder: &'a [u8],
        value: usize,
    }
    static CASES: &[TestCase] = &[
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
            bytes: &[0b00011110],
            remainder: &[],
            value: 30,
        },
        TestCase {
            bytes: &[0b00011111, 0b00000000],
            remainder: &[],
            value: 31,
        },
        TestCase {
            bytes: &[0b00011111, 0b00000000, 0b10101010],
            remainder: &[0b10101010],
            value: 31,
        },
        TestCase {
            bytes: &[0b00011111, 0b00000001],
            remainder: &[],
            value: 32,
        },
        TestCase {
            bytes: &[0b00011111, 0b00000010],
            remainder: &[],
            value: 33,
        },
        TestCase {
            bytes: &[0b00011111, 0b00100001],
            remainder: &[],
            value: 64,
        },
        TestCase {
            bytes: &[0b00011111, 0b01000101],
            remainder: &[],
            value: 100,
        },
        TestCase {
            bytes: &[0b00011111, 0b01111111],
            remainder: &[],
            value: 158,
        },
        TestCase {
            bytes: &[0b00011111, 0b10000001, 0b00000000],
            remainder: &[],
            value: 159,
        },
        TestCase {
            bytes: &[0b00011111, 0b10000001, 0b00000001],
            remainder: &[],
            value: 160,
        },
        TestCase {
            bytes: &[0b00011111, 0b10000011, 0b01010101],
            remainder: &[],
            value: 500,
        },
        TestCase {
            bytes: &[0b00011111, 0b11111111, 0b01111111],
            remainder: &[],
            value: 16414,
        },
        TestCase {
            bytes: &[0b00011111, 0b10000001, 0b10000000, 0b00000000],
            remainder: &[],
            value: 16415,
        },
        TestCase {
            bytes: &[0b00011111, 0b10000001, 0b10000000, 0b00000001],
            remainder: &[],
            value: 16416,
        },
    ];

    #[test]
    fn test_read() {
        for cas in CASES {
            let recovered = read_varintb_5bits(cas.bytes[0], &cas.bytes[1..]);
            assert_eq!(recovered, Some((cas.value, cas.remainder)), "{:?}", cas);
        }
    }

    #[test]
    fn test_read_write() {
        for cas in CASES {
            let recovered = read_varintb_5bits(cas.bytes[0], &cas.bytes[1..]);
            assert_eq!(recovered, Some((cas.value, cas.remainder)), "{:?}", cas);
            let write_bytes = write_varintb_5bits(cas.value);
            assert_eq!(
                &cas.bytes[0..cas.bytes.len()-cas.remainder.len()],
                write_bytes.as_slice(),
                "{:?}",
                cas
            );
        }
    }

    #[test]
    fn test_roundtrip() {
        let mut i = 0usize;
        while i < MAX_VARINT as usize {
            let bytes = write_varintb_5bits(i);
            let recovered = read_varintb_5bits(bytes.as_slice()[0], &bytes.as_slice()[1..]);
            assert!(recovered.is_some(), "{:?}", i);
            assert_eq!(i, recovered.unwrap().0, "{:?}", bytes.as_slice());
            i <<= 1;
            i += 1;
        }
    }

    #[test]
    fn test_max() {
        let write_bytes = write_varintb_5bits(MAX_VARINT);
        assert_eq!(write_bytes.len(), MAX_VARINT_LENGTH);
        let subarray = write_bytes
            .as_const_slice()
            .get_subslice_or_panic(1, write_bytes.len());
        let (recovered_value, remainder) = read_varintb_5bits(
            *write_bytes.as_const_slice().first().unwrap(),
            subarray.as_slice(),
        )
        .unwrap();
        assert!(remainder.is_empty());
        assert_eq!(recovered_value, MAX_VARINT);
        assert_eq!(
            write_bytes.as_slice(),
            &[
                0b00011111, //
                0b10000001, //
                0b11111111, //
                0b11111111, //
                0b11111111, //
                0b11111111, //
                0b11111111, //
                0b11111111, //
                0b11111111, //
                0b11111111, //
                0b01100000, //
            ]
        );
    }
}
