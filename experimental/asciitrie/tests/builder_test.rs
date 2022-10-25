// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use asciitrie::AsciiStr;
use asciitrie::AsciiTrie;
use litemap::LiteMap;

mod testdata {
    include!("data.rs");
}

#[test]
fn test_basic() {
    let trie = testdata::basic::TRIE;
    let data = testdata::basic::DATA;

    // Check that the builder works
    let built_trie: AsciiTrie<Vec<u8>> = data
        .iter()
        .copied()
        .map(AsciiStr::try_from_bytes_with_value)
        .collect::<Result<_, _>>()
        .unwrap();
    assert_eq!(built_trie.as_bytes(), trie);
}

fn check_ascii_trie<S>(items: &LiteMap<&AsciiStr, usize>, trie: &AsciiTrie<S>)
where
    S: AsRef<[u8]>,
{
    for (k, v) in items.iter() {
        assert_eq!(trie.get(k.as_bytes()), Some(*v));
    }
}

#[test]
fn test_empty() {
    let trie = AsciiTrie::from_litemap(&LiteMap::new_vec());
    assert_eq!(trie.byte_len(), 0);
    assert!(trie.is_empty());
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.as_bytes(), &[]);
}

#[test]
fn test_single_empty_value() {
    let litemap: LiteMap<&AsciiStr, usize> = [
        (AsciiStr::try_from_str("").unwrap(), 10), //
    ]
    .into_iter()
    .collect();
    let trie = AsciiTrie::from_litemap(&litemap.as_sliced());
    assert_eq!(trie.byte_len(), 1);
    assert_eq!(trie.get(b""), Some(10));
    assert_eq!(trie.get(b"x"), None);
    assert_eq!(trie.as_bytes(), &[0b10001010]);
}

#[test]
fn test_single_byte_string() {
    let litemap: LiteMap<&AsciiStr, usize> = [
        (AsciiStr::try_from_str("x").unwrap(), 10), //
    ]
    .into_iter()
    .collect();
    let trie = AsciiTrie::from_litemap(&litemap.as_sliced());
    assert_eq!(trie.byte_len(), 2);
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"xy"), None);
    check_ascii_trie(&litemap, &trie);
    assert_eq!(trie.as_bytes(), &[b'x', 0b10001010]);
}

#[test]
fn test_single_string() {
    let litemap: LiteMap<&AsciiStr, usize> = [
        (AsciiStr::try_from_str("xyz").unwrap(), 10), //
    ]
    .into_iter()
    .collect();
    let trie = AsciiTrie::from_litemap(&litemap.as_sliced());
    assert_eq!(trie.byte_len(), 4);
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"x"), None);
    assert_eq!(trie.get(b"xy"), None);
    assert_eq!(trie.get(b"xyzz"), None);
    check_ascii_trie(&litemap, &trie);
    assert_eq!(trie.as_bytes(), &[b'x', b'y', b'z', 0b10001010]);
}

#[test]
fn test_prefix_strings() {
    let litemap: LiteMap<&AsciiStr, usize> = [
        (AsciiStr::try_from_str("x").unwrap(), 0),
        (AsciiStr::try_from_str("xy").unwrap(), 1),
    ]
    .into_iter()
    .collect();
    let trie = AsciiTrie::from_litemap(&litemap.as_sliced());
    assert_eq!(trie.byte_len(), 4);
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"xyz"), None);
    check_ascii_trie(&litemap, &trie);
    assert_eq!(trie.as_bytes(), &[b'x', 0b10000000, b'y', 0b10000001]);
}

#[test]
fn test_single_byte_branch() {
    let litemap: LiteMap<&AsciiStr, usize> = [
        (AsciiStr::try_from_str("x").unwrap(), 0),
        (AsciiStr::try_from_str("y").unwrap(), 1),
    ]
    .into_iter()
    .collect();
    let trie = AsciiTrie::from_litemap(&litemap.as_sliced());
    assert_eq!(trie.byte_len(), 7);
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"xy"), None);
    check_ascii_trie(&litemap, &trie);
    assert_eq!(
        trie.as_bytes(),
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
    let trie = AsciiTrie::from_litemap(&litemap.as_sliced());
    assert_eq!(trie.byte_len(), 10);
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"a"), None);
    assert_eq!(trie.get(b"ax"), None);
    assert_eq!(trie.get(b"ay"), None);
    check_ascii_trie(&litemap, &trie);
    assert_eq!(
        trie.as_bytes(),
        &[b'a', 0b11000010, b'x', b'y', 0, 2, b'b', 0b10000000, b'c', 0b10000001]
    );
}

#[test]
fn test_linear_varint_values() {
    let litemap: LiteMap<&AsciiStr, usize> = [
        (AsciiStr::try_from_str("").unwrap(), 100),
        (AsciiStr::try_from_str("x").unwrap(), 500),
        (AsciiStr::try_from_str("xyz").unwrap(), 5000),
    ]
    .into_iter()
    .collect();
    let trie = AsciiTrie::from_litemap(&litemap.as_sliced());
    assert_eq!(trie.byte_len(), 10);
    assert_eq!(trie.get(b"xy"), None);
    assert_eq!(trie.get(b"xz"), None);
    assert_eq!(trie.get(b"xyzz"), None);
    check_ascii_trie(&litemap, &trie);
    assert_eq!(
        trie.as_bytes(),
        &[0xA0, 0x44, b'x', 0xA3, 0x54, b'y', b'z', 0xA0, 0x86, 0x68]
    );
}

#[test]
fn test_varint_branch() {
    let chars =
        AsciiStr::try_from_str("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz").unwrap();
    let litemap: LiteMap<&AsciiStr, usize> = (0..chars.len())
        .map(|i| (chars.substring(i..i + 1).unwrap(), i))
        .collect();
    let trie = AsciiTrie::from_litemap(&litemap.as_sliced());
    assert_eq!(trie.byte_len(), 178);
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"ax"), None);
    assert_eq!(trie.get(b"ay"), None);
    check_ascii_trie(&litemap, &trie);
    #[rustfmt::skip]
    assert_eq!(
        trie.as_bytes(),
        &[
            0b11100000, // branch varint lead
            0x14,       // branch varint trail
            // search array:
            b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J',
            b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T',
            b'U', b'V', b'W', b'X', b'Y', b'Z',
            b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j',
            b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't',
            b'u', b'v', b'w', b'x', b'y', b'z',
            // offset array:
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
            10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
            20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
            30, 31, 32, 34, 36, 38, 40, 42, 44, 46,
            48, 50, 52, 54, 56, 58, 60, 62, 64, 66,
            68, 70,
            // single-byte values:
            (0x80 | 0), (0x80 | 1), (0x80 | 2), (0x80 | 3), (0x80 | 4),
            (0x80 | 5), (0x80 | 6), (0x80 | 7), (0x80 | 8), (0x80 | 9),
            (0x80 | 10), (0x80 | 11), (0x80 | 12), (0x80 | 13), (0x80 | 14),
            (0x80 | 15), (0x80 | 16), (0x80 | 17), (0x80 | 18), (0x80 | 19),
            (0x80 | 20), (0x80 | 21), (0x80 | 22), (0x80 | 23), (0x80 | 24),
            (0x80 | 25), (0x80 | 26), (0x80 | 27), (0x80 | 28), (0x80 | 29),
            (0x80 | 30), (0x80 | 31),
            // multi-byte values:
            0xA0, 0, 0xA0, 1, 0xA0, 2, 0xA0, 3, 0xA0, 4,
            0xA0, 5, 0xA0, 6, 0xA0, 7, 0xA0, 8, 0xA0, 9,
            0xA0, 10, 0xA0, 11, 0xA0, 12, 0xA0, 13, 0xA0, 14,
            0xA0, 15, 0xA0, 16, 0xA0, 17, 0xA0, 18, 0xA0, 19,
        ]
    );
}

#[test]
fn test_below_wide() {
    let litemap: LiteMap<&AsciiStr, usize> = [
        (
            AsciiStr::try_from_str("abcdefghijklmnopqrstuvwxyz").unwrap(),
            1,
        ),
        (
            AsciiStr::try_from_str("bcdefghijklmnopqrstuvwxyza").unwrap(),
            2,
        ),
        (
            AsciiStr::try_from_str("cdefghijklmnopqrstuvwxyzab").unwrap(),
            3,
        ),
        (
            AsciiStr::try_from_str("defghijklmnopqrstuvwxyzabc").unwrap(),
            4,
        ),
        (
            AsciiStr::try_from_str("efghijklmnopqrstuvwxyzabcd").unwrap(),
            5,
        ),
        (
            AsciiStr::try_from_str("fghijklmnopqrstuvwxyzabcde").unwrap(),
            6,
        ),
        (
            AsciiStr::try_from_str("ghijklmnopqrstuvwxyzabcdef").unwrap(),
            7,
        ),
        (
            AsciiStr::try_from_str("hijklmnopqrstuvwxyzabcdefg").unwrap(),
            8,
        ),
        (
            AsciiStr::try_from_str("ijklmnopqrstuvwxyzabcdefgh").unwrap(),
            9,
        ),
        (AsciiStr::try_from_str("jklmnopqrstuvwxyzabcd").unwrap(), 10),
    ]
    .into_iter()
    .collect();
    let trie = AsciiTrie::from_litemap(&litemap.as_sliced());
    assert_eq!(trie.byte_len(), 276);
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"abc"), None);
    check_ascii_trie(&litemap, &trie);
    #[rustfmt::skip]
    assert_eq!(
        trie.as_bytes(),
        &[
            0b11001010, // branch
            // search array:
            b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j',
            // offset array:
            0, 26, 52, 78, 104, 130, 156, 182, 208, 234,
            // offset data:
            b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
            b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z',
            0x81,
            b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o',
            b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a',
            0x82,
            b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p',
            b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b',
            0x83,
            b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q',
            b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c',
            0x84,
            b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r',
            b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd',
            0x85,
            b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's',
            b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e',
            0x86,
            b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't',
            b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e', b'f',
            0x87,
            b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u',
            b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e', b'f', b'g',
            0x88,
            b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
            b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h',
            0x89,
            b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w',
            b'x', b'y', b'z', b'a', b'b', b'c', b'd',
            0x8A,
        ]
    );
}

#[test]
fn test_at_wide() {
    let litemap: LiteMap<&AsciiStr, usize> = [
        (
            AsciiStr::try_from_str("abcdefghijklmnopqrstuvwxyz").unwrap(),
            1,
        ),
        (
            AsciiStr::try_from_str("bcdefghijklmnopqrstuvwxyza").unwrap(),
            2,
        ),
        (
            AsciiStr::try_from_str("cdefghijklmnopqrstuvwxyzab").unwrap(),
            3,
        ),
        (
            AsciiStr::try_from_str("defghijklmnopqrstuvwxyzabc").unwrap(),
            4,
        ),
        (
            AsciiStr::try_from_str("efghijklmnopqrstuvwxyzabcd").unwrap(),
            5,
        ),
        (
            AsciiStr::try_from_str("fghijklmnopqrstuvwxyzabcde").unwrap(),
            6,
        ),
        (
            AsciiStr::try_from_str("ghijklmnopqrstuvwxyzabcdef").unwrap(),
            7,
        ),
        (
            AsciiStr::try_from_str("hijklmnopqrstuvwxyzabcdefg").unwrap(),
            8,
        ),
        (
            AsciiStr::try_from_str("ijklmnopqrstuvwxyzabcdefgh").unwrap(),
            9,
        ),
        (
            AsciiStr::try_from_str("jklmnopqrstuvwxyzabcde").unwrap(),
            10,
        ),
    ]
    .into_iter()
    .collect();
    let trie = AsciiTrie::from_litemap(&litemap.as_sliced());
    assert_eq!(trie.byte_len(), 287);
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"abc"), None);
    check_ascii_trie(&litemap, &trie);
    #[rustfmt::skip]
    assert_eq!(
        trie.as_bytes(),
        &[
            0b11001010, // branch
            // search array:
            b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j',
            // offset array (wide):
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 26, 52, 78, 104, 130, 156, 182, 208, 234,
            // offset data:
            b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
            b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z',
            0x81,
            b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o',
            b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a',
            0x82,
            b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p',
            b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b',
            0x83,
            b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q',
            b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c',
            0x84,
            b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r',
            b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd',
            0x85,
            b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's',
            b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e',
            0x86,
            b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't',
            b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e', b'f',
            0x87,
            b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u',
            b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e', b'f', b'g',
            0x88,
            b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
            b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h',
            0x89,
            b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w',
            b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e',
            0x8A,
        ]
    );
}

#[test]
fn test_everything() {
    let litemap: LiteMap<&AsciiStr, usize> = [
        (AsciiStr::try_from_str("").unwrap(), 0),
        (AsciiStr::try_from_str("axb").unwrap(), 100),
        (AsciiStr::try_from_str("ayc").unwrap(), 2),
        (AsciiStr::try_from_str("azd").unwrap(), 3),
        (AsciiStr::try_from_str("bxe").unwrap(), 4),
        (AsciiStr::try_from_str("bxefg").unwrap(), 500),
        (AsciiStr::try_from_str("bxefh").unwrap(), 6),
        (AsciiStr::try_from_str("bxei").unwrap(), 7),
        (AsciiStr::try_from_str("bxeikl").unwrap(), 8),
    ]
    .into_iter()
    .collect();
    let trie = AsciiTrie::from_litemap(&litemap.as_sliced());
    assert_eq!(trie.byte_len(), 40);
    assert_eq!(trie.get(b""), Some(0));
    assert_eq!(trie.get(b"a"), None);
    assert_eq!(trie.get(b"ax"), None);
    assert_eq!(trie.get(b"ay"), None);
    check_ascii_trie(&litemap, &trie);
    assert_eq!(
        trie.as_bytes(),
        &[
            0b10000000, // value 0
            0b11000010, // branch of 2
            b'a',       //
            b'b',       //
            0,          //
            14,         //
            0b11000011, // branch of 3
            b'x',       //
            b'y',       //
            b'z',       //
            0,          //
            3,          //
            5,          //
            b'b',       //
            0b10100000, // value 100 (lead)
            0x44,       // value 100 (trail)
            b'c',       //
            0b10000010, // value 2
            b'd',       //
            0b10000011, // value 3
            b'x',       //
            b'e',       //
            0b10000100, // value 4
            0b11000010, // branch of 2
            b'f',       //
            b'i',       //
            0,          //
            8,          //
            0b11000010, // branch of 2
            b'g',       //
            b'h',       //
            0,          //
            2,          //
            0b10100011, // value 500 (lead)
            0x54,       // value 500 (trail)
            0b10000110, // value 6
            0b10000111, // value 7
            b'k',       //
            b'l',       //
            0b10001000, // value 8
        ]
    );
}
