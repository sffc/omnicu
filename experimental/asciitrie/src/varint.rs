// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Varint spec for AsciiTrie:
//!
//! - Lead byte: top 2 bits are trie metadata; third is varint extender; rest is value
//! - Trail bytes: top bit is varint extender; add rest to current value * 2^7
//! - Add the "latent value" to the final result: (1<<5) + (1<<7) + (1<<14) + ...

pub fn read_varint(start: u8, remainder: &[u8]) -> Option<(usize, &[u8])> {
    let mut value = (start & 0b00011111) as usize;
    let mut remainder = remainder;
    if (start & 0b00100000) != 0 {
        loop {
            let next;
            (next, remainder) = remainder.split_first()?;
            // Note: value << 7 could drop high bits. The first addition can't overflow.
            // The second addition could overflow; in such a case we just inform the
            // developer via the debug assertion.
            value = (value << 7) + ((next & 0b01111111) as usize) + 32;
            if (next & 0b10000000) == 0 {
                break;
            }
        }
    }
    Some((value, remainder))
}

// *Upper Bound:* Each trail byte stores 7 bits of data, plus the latent value.
// Add an extra 1 since the lead byte holds only 5 bits of data.
const MAX_VARINT_LENGTH: usize = 1 + core::mem::size_of::<usize>() * 8 / 7;

pub const fn write_varint(value: usize) -> (usize, [u8; MAX_VARINT_LENGTH]) {
    let mut result = [0; MAX_VARINT_LENGTH];
    if value < 32 {
        result[0] = value as u8;
        return (1, result);
    }
    result[0] = 32;
    let mut latent = 32;
    let mut steps = 2;
    loop {
        let next_latent = (latent << 7) + 32;
        if value < next_latent || next_latent == latent {
            break;
        }
        latent = next_latent;
        steps += 1;
    }
    let mut value = value - latent;
    let mut i = steps;
    while i > 0 {
        i -= 1;
        result[i] |= (value as u8) & 0b01111111;
        value >>= 7;
        if i > 0 && i < steps - 1 {
            result[i] |= 0b10000000;
        }
    }
    (steps, result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write() {
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
            TestCase {
                bytes: &[0b00100000, 0b00000000],
                remainder: &[],
                value: 32,
            },
            TestCase {
                bytes: &[0b00100000, 0b00000001],
                remainder: &[],
                value: 33,
            },
            TestCase {
                bytes: &[0b00100000, 0b00100000],
                remainder: &[],
                value: 64,
            },
            TestCase {
                bytes: &[0b00100000, 0b01111111],
                remainder: &[],
                value: 159,
            },
            TestCase {
                bytes: &[0b00100001, 0b00000000],
                remainder: &[],
                value: 160,
            },
            TestCase {
                bytes: &[0b00100001, 0b00000001],
                remainder: &[],
                value: 161,
            },
            TestCase {
                bytes: &[0b00111111, 0b01111111],
                remainder: &[],
                value: 4127, // 32 + (1 << 12) - 1
            },
            TestCase {
                bytes: &[0b00100000, 0b10000000, 0b00000000],
                remainder: &[],
                value: 4128, // 32 + (1 << 12)
            },
            TestCase {
                bytes: &[0b00100000, 0b10000000, 0b00000001],
                remainder: &[],
                value: 4129, // 32 + (1 << 12) + 1
            },
            TestCase {
                bytes: &[0b00100000, 0b10000000, 0b01111111],
                remainder: &[],
                value: 4255, // 32 + (1 << 12) + 127
            },
            TestCase {
                bytes: &[0b00100000, 0b10000001, 0b00000000],
                remainder: &[],
                value: 4256, // 32 + (1 << 12) + 128
            },
            TestCase {
                bytes: &[0b00100000, 0b10000001, 0b00000001],
                remainder: &[],
                value: 4257, // 32 + (1 << 12) + 129
            },
            TestCase {
                bytes: &[0b00100000, 0b11111111, 0b01111111],
                remainder: &[],
                value: 20511, // 32 + (1 << 12) + (1 << 14) - 1
            },
            TestCase {
                bytes: &[0b00100001, 0b10000000, 0b00000000],
                remainder: &[],
                value: 20512, // 32 + (1 << 12) + (1 << 14)
            },
            TestCase {
                bytes: &[0b00111111, 0b11111111, 0b01111111],
                remainder: &[],
                value: 528415, // 32 + (1 << 12) + (1 << 19) - 1
            },
            TestCase {
                bytes: &[0b00100000, 0b10000000, 0b10000000, 0b00000000],
                remainder: &[],
                value: 528416, // 32 + (1 << 12) + (1 << 19)
            },
            TestCase {
                bytes: &[0b00100000, 0b10000000, 0b10000000, 0b00000001],
                remainder: &[],
                value: 528417, // 32 + (1 << 12) + (1 << 19) + 1
            },
            TestCase {
                bytes: &[0b00111111, 0b11111111, 0b11111111, 0b01111111],
                remainder: &[],
                value: 67637279, // 32 + (1 << 12) + (1 << 19) + (1 << 26) - 1
            },
            TestCase {
                bytes: &[0b00100000, 0b10000000, 0b10000000, 0b10000000, 0b00000000],
                remainder: &[],
                value: 67637280, // 32 + (1 << 12) + (1 << 19) + (1 << 26)
            },
        ];
        for cas in cases {
            let actual = read_varint(cas.bytes[0], &cas.bytes[1..]).unwrap();
            assert_eq!(actual, (cas.value, cas.remainder), "{:?}", cas);
            let (written_len, written_bytes) = write_varint(cas.value);
            assert_eq!(
                written_len,
                cas.bytes.len() - cas.remainder.len(),
                "{:?}",
                cas
            );
            assert_eq!(
                &written_bytes[0..written_len],
                &cas.bytes[0..written_len],
                "{:?}",
                cas
            );
        }
    }

    #[test]
    fn test_max() {
        let (written_len, written_bytes) = write_varint(usize::MAX);
        let (recovered_value, remainder) =
            read_varint(written_bytes[0], &written_bytes[1..written_len]).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(recovered_value, usize::MAX);
        assert_eq!(
            written_bytes,
            [
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
