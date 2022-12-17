// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Varint spec for AsciiTrie:
//!
//! - Lead byte: top L bits for trie metadata; 2 for length; rest is value
//! - Trail bytes: low bits of value

use crate::builder::const_util::ConstArrayBuilder;

extern crate std;

pub const fn read_varint(start: u8, remainder: &[u8]) -> Option<(usize, &[u8])> {
    let mut value = (start & 0b00001111) as usize;
    let mut count = 0b11 & (start >> 4);
    let mut remainder = remainder;
    while count > 0 {
        let next;
        (next, remainder) = match remainder.split_first() {
            Some(t) => t,
            None => return None,
        };
        value = (value << 8) + (*next as usize);
        count -= 1;
    }
    Some((value, remainder))
}

pub const fn read_varint2(start: u8, remainder: &[u8]) -> Option<(usize, &[u8])> {
    let mut value = (start & 0b00000111) as usize;
    let mut count = 0b11 & (start >> 3);
    let mut remainder = remainder;
    while count > 0 {
        let next;
        (next, remainder) = match remainder.split_first() {
            Some(t) => t,
            None => return None,
        };
        value = (value << 8) + (*next as usize);
        count -= 1;
    }
    Some((value, remainder))
}

// *Upper Bound:* Each trail byte stores 7 bits of data, plus the latent value.
// Add an extra 1 since the lead byte holds only 5 bits of data.
const MAX_VARINT_LENGTH: usize = 1 + core::mem::size_of::<usize>() * 8 / 7;

pub(crate) const fn write_varint(value: usize) -> ConstArrayBuilder<MAX_VARINT_LENGTH, u8> {
    let mut result = [0; MAX_VARINT_LENGTH];
    let mut i = MAX_VARINT_LENGTH - 1;
    let mut value = value;
    loop {
        result[i] = value as u8;
        if value < 16 {
            result[i] |= ((MAX_VARINT_LENGTH - i - 1) << 4) as u8;
            break;
        }
        value >>= 8;
        i -= 1;
    }
    // The bytes are from i to the end.
    ConstArrayBuilder::from_manual_slice(result, i, MAX_VARINT_LENGTH)
}

pub(crate) const fn write_varint2(value: usize) -> ConstArrayBuilder<MAX_VARINT_LENGTH, u8> {
    let mut result = [0; MAX_VARINT_LENGTH];
    let mut i = MAX_VARINT_LENGTH - 1;
    let mut value = value;
    loop {
        result[i] = value as u8;
        if value < 8 {
            result[i] |= ((MAX_VARINT_LENGTH - i - 1) << 3) as u8;
            break;
        }
        value >>= 8;
        i -= 1;
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
            bytes: &[0b00001111],
            remainder: &[],
            value: 15,
        },
        TestCase {
            bytes: &[0b00010000, 0b00010000, 0b10101010],
            remainder: &[0b10101010],
            value: 16,
        },
        TestCase {
            bytes: &[0b00010000, 0b00100000],
            remainder: &[],
            value: 32,
        },
        TestCase {
            bytes: &[0b00010000, 0b00100001],
            remainder: &[],
            value: 33,
        },
        // TestCase {
        //     bytes: &[0b00100000, 0b00100000],
        //     remainder: &[],
        //     value: 64,
        // },
        // TestCase {
        //     bytes: &[0x20, 0x44],
        //     remainder: &[],
        //     value: 100,
        // },
        // TestCase {
        //     bytes: &[0b00100000, 0b01111111],
        //     remainder: &[],
        //     value: 159,
        // },
        // TestCase {
        //     bytes: &[0b00100001, 0b00000000],
        //     remainder: &[],
        //     value: 160,
        // },
        // TestCase {
        //     bytes: &[0b00100001, 0b00000001],
        //     remainder: &[],
        //     value: 161,
        // },
        // TestCase {
        //     bytes: &[0x23, 0x54],
        //     remainder: &[],
        //     value: 500,
        // },
        // TestCase {
        //     bytes: &[0b00111111, 0b01111111],
        //     remainder: &[],
        //     value: 4127, // 32 + (1 << 12) - 1
        // },
        // TestCase {
        //     bytes: &[0b00100000, 0b10000000, 0b00000000],
        //     remainder: &[],
        //     value: 4128, // 32 + (1 << 12)
        // },
        // TestCase {
        //     bytes: &[0b00100000, 0b10000000, 0b00000001],
        //     remainder: &[],
        //     value: 4129, // 32 + (1 << 12) + 1
        // },
        // TestCase {
        //     bytes: &[0b00100000, 0b10000000, 0b01111111],
        //     remainder: &[],
        //     value: 4255, // 32 + (1 << 12) + 127
        // },
        // TestCase {
        //     bytes: &[0b00100000, 0b10000001, 0b00000000],
        //     remainder: &[],
        //     value: 4256, // 32 + (1 << 12) + 128
        // },
        // TestCase {
        //     bytes: &[0b00100000, 0b10000001, 0b00000001],
        //     remainder: &[],
        //     value: 4257, // 32 + (1 << 12) + 129
        // },
        // TestCase {
        //     bytes: &[0x20, 0x86, 0x68],
        //     remainder: &[],
        //     value: 5000,
        // },
        // TestCase {
        //     bytes: &[0b00100000, 0b11111111, 0b01111111],
        //     remainder: &[],
        //     value: 20511, // 32 + (1 << 12) + (1 << 14) - 1
        // },
        // TestCase {
        //     bytes: &[0b00100001, 0b10000000, 0b00000000],
        //     remainder: &[],
        //     value: 20512, // 32 + (1 << 12) + (1 << 14)
        // },
        // TestCase {
        //     bytes: &[0b00111111, 0b11111111, 0b01111111],
        //     remainder: &[],
        //     value: 528415, // 32 + (1 << 12) + (1 << 19) - 1
        // },
        // TestCase {
        //     bytes: &[0b00100000, 0b10000000, 0b10000000, 0b00000000],
        //     remainder: &[],
        //     value: 528416, // 32 + (1 << 12) + (1 << 19)
        // },
        // TestCase {
        //     bytes: &[0b00100000, 0b10000000, 0b10000000, 0b00000001],
        //     remainder: &[],
        //     value: 528417, // 32 + (1 << 12) + (1 << 19) + 1
        // },
        // TestCase {
        //     bytes: &[0b00111111, 0b11111111, 0b11111111, 0b01111111],
        //     remainder: &[],
        //     value: 67637279, // 32 + (1 << 12) + (1 << 19) + (1 << 26) - 1
        // },
        // TestCase {
        //     bytes: &[0b00100000, 0b10000000, 0b10000000, 0b10000000, 0b00000000],
        //     remainder: &[],
        //     value: 67637280, // 32 + (1 << 12) + (1 << 19) + (1 << 26)
        // },
    ];

    #[test]
    fn test_read() {
        for cas in CASES {
            let recovered = read_varint(cas.bytes[0], &cas.bytes[1..]).expect(&std::format!("{:?}", cas));
            assert_eq!(recovered, (cas.value, cas.remainder), "{:?}", cas);
        }
    }

    #[test]
    fn test_read_write() {
        for cas in CASES {
            let recovered = read_varint(cas.bytes[0], &cas.bytes[1..]).expect(&std::format!("{:?}", cas));
            assert_eq!(recovered, (cas.value, cas.remainder), "{:?}", cas);
            let write_bytes = write_varint(cas.value);
            assert_eq!(
                &cas.bytes[0..(cas.bytes.len()-cas.remainder.len())],
                write_bytes.as_slice(),
                "{:?}",
                cas
            );
        }
    }

    #[test]
    fn test_roundtrip() {
        let mut i = 0usize;
        while i < u32::MAX as usize {
            let bytes = write_varint(i);
            let recovered = read_varint(bytes.as_slice()[0], &bytes.as_slice()[1..]).expect(&std::format!("{:?}", i));
            assert_eq!(i, recovered.0, "{:?}", bytes.as_slice());
            i <<= 1;
            i += 1;
        }
    }

    #[test]
    fn test_max() {
        let write_bytes = write_varint(usize::MAX);
        assert_eq!(write_bytes.len(), MAX_VARINT_LENGTH);
        let subarray = write_bytes
            .as_const_slice()
            .get_subslice_or_panic(1, write_bytes.len());
        let (recovered_value, remainder) = read_varint(
            *write_bytes.as_const_slice().first().unwrap(),
            subarray.as_slice(),
        )
        .unwrap();
        assert!(remainder.is_empty());
        assert_eq!(recovered_value, usize::MAX);
        assert_eq!(
            write_bytes.as_slice(),
            &[
                0b00100001, //
                0b11011111, //
                0b11011111, //
                0b11011111, //
                0b11011111, //
                0b11011111, //
                0b11011111, //
                0b11011111, //
                0b11011111, //
                0b01011111, //
            ]
        );
    }
}
