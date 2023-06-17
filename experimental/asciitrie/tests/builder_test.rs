// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use asciitrie::AsciiStr;
use asciitrie::AsciiTrie;
use litemap::LiteMap;

mod testdata {
    include!("data.rs");
}

use testdata::strings_to_litemap;

#[test]
fn test_basic() {
    let litemap: LiteMap<&AsciiStr, usize> = testdata::basic::DATA.iter().copied().collect();

    let expected_bytes = testdata::basic::TRIE;
    let trie: AsciiTrie<Vec<u8>> = litemap.iter().map(|(k, v)| (*k, *v)).collect();
    check_bytes_eq(28, trie.as_bytes(), expected_bytes);
    check_ascii_trie(&litemap, &trie);

    let expected_bytes4 = testdata::basic::TRIE4;
    let trie4 = asciitrie::make4_litemap(&litemap);
    check_bytes_eq(30, &trie4, expected_bytes4);
    check_ascii_trie4(&litemap, &trie4);

    let expected_bytes5 = testdata::basic::TRIE5;
    let trie5 = asciitrie::make5_litemap(&litemap);
    check_bytes_eq(26, &trie5, expected_bytes5);
    check_ascii_trie5(&litemap, &trie5);

    let expected_bytes6 = testdata::basic::TRIE6;
    let trie6 = asciitrie::make6_litemap(&litemap).unwrap();
    check_bytes_eq(26, &trie6, expected_bytes6);
    check_ascii_trie6(&litemap, &trie6);

    let trie1b = asciitrie::make1b_litemap(&litemap);
    check_bytes_eq(28, &trie1b, expected_bytes);
}

fn check_ascii_trie<S>(items: &LiteMap<&AsciiStr, usize>, trie: &AsciiTrie<S>)
where
    S: AsRef<[u8]> + ?Sized,
{
    for (k, v) in items.iter() {
        assert_eq!(trie.get(k.as_bytes()), Some(*v));
    }
    assert!(items
        .iter()
        .map(|(s, v)| (s.to_boxed(), *v))
        .eq(trie.iter()));
}

fn check_ascii_trie4(items: &LiteMap<&AsciiStr, usize>, trie: &[u8]) {
    for (k, v) in items.iter() {
        assert_eq!(asciitrie::reader4::get(trie, k.as_bytes()), Some(*v));
    }
    // assert!(items
    //     .iter()
    //     .map(|(s, v)| (s.to_boxed(), *v))
    //     .eq(trie.iter()));
}

fn check_ascii_trie5(items: &LiteMap<&AsciiStr, usize>, trie: &[u8]) {
    for (k, v) in items.iter() {
        assert_eq!(asciitrie::reader5::get(trie, k.as_bytes()), Some(*v));
    }
    // assert!(items
    //     .iter()
    //     .map(|(s, v)| (s.to_boxed(), *v))
    //     .eq(trie.iter()));
}

fn check_ascii_trie6(items: &LiteMap<&AsciiStr, usize>, trie: &[u8]) {
    for (k, v) in items.iter() {
        assert_eq!(asciitrie::reader6::get(trie, k.as_bytes()), Some(*v));
    }
    // assert!(items
    //     .iter()
    //     .map(|(s, v)| (s.to_boxed(), *v))
    //     .eq(trie.iter()));
}

fn check_ascii_trie6_bytes(items: &LiteMap<&[u8], usize>, trie: &[u8]) {
    for (k, v) in items.iter() {
        assert_eq!(asciitrie::reader6::get(trie, k), Some(*v));
    }
    // assert!(items
    //     .iter()
    //     .map(|(s, v)| (s.to_boxed(), *v))
    //     .eq(trie.iter()));
}

fn check_bytes_eq(len: usize, a: impl AsRef<[u8]>, b: &[u8]) {
    assert_eq!(len, a.as_ref().len());
    assert_eq!(a.as_ref(), b);
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
    assert_eq!(trie.get(b""), Some(10));
    assert_eq!(trie.get(b"x"), None);
    let expected_bytes = &[0b10001010];
    assert_eq!(trie.as_bytes(), expected_bytes);

    let expected_bytes4 = &[0b10001010];
    let trie4 = asciitrie::make4_litemap(&litemap);
    check_bytes_eq(1, &trie4, expected_bytes4);
    check_ascii_trie4(&litemap, &trie4);

    let expected_bytes6 = &[0b10001010];
    let trie6 = asciitrie::make6_litemap(&litemap).unwrap();
    check_bytes_eq(1, &trie6, expected_bytes6);
    check_ascii_trie6(&litemap, &trie6);

    let trie1b = asciitrie::make1b_litemap(&litemap);
    check_bytes_eq(1, &trie1b, expected_bytes);
}

#[test]
fn test_single_byte_string() {
    let litemap: LiteMap<&AsciiStr, usize> = [
        (AsciiStr::try_from_str("x").unwrap(), 10), //
    ]
    .into_iter()
    .collect();
    let trie = AsciiTrie::from_litemap(&litemap.as_sliced());
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"xy"), None);
    check_ascii_trie(&litemap, &trie);
    let expected_bytes = &[b'x', 0b10001010];
    check_bytes_eq(2, trie.as_bytes(), expected_bytes);

    let expected_bytes4 = &[b'x', 0b10001010];
    let trie4 = asciitrie::make4_litemap(&litemap);
    check_bytes_eq(2, &trie4, expected_bytes4);
    check_ascii_trie4(&litemap, &trie4);

    let expected_bytes6 = &[b'x', 0b10001010];
    let trie6 = asciitrie::make6_litemap(&litemap).unwrap();
    check_bytes_eq(2, &trie6, expected_bytes6);
    check_ascii_trie6(&litemap, &trie6);

    let trie1b = asciitrie::make1b_litemap(&litemap);
    check_bytes_eq(2, &trie1b, expected_bytes);
}

#[test]
fn test_single_string() {
    let litemap: LiteMap<&AsciiStr, usize> = [
        (AsciiStr::try_from_str("xyz").unwrap(), 10), //
    ]
    .into_iter()
    .collect();
    let trie = AsciiTrie::from_litemap(&litemap.as_sliced());
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"x"), None);
    assert_eq!(trie.get(b"xy"), None);
    assert_eq!(trie.get(b"xyzz"), None);
    check_ascii_trie(&litemap, &trie);
    let expected_bytes = &[b'x', b'y', b'z', 0b10001010];
    check_bytes_eq(4, trie.as_bytes(), expected_bytes);

    let expected_bytes4 = &[b'x', b'y', b'z', 0b10001010];
    let trie4 = asciitrie::make4_litemap(&litemap);
    check_bytes_eq(4, &trie4, expected_bytes4);
    check_ascii_trie4(&litemap, &trie4);

    let expected_bytes6 = &[b'x', b'y', b'z', 0b10001010];
    let trie6 = asciitrie::make6_litemap(&litemap).unwrap();
    check_bytes_eq(4, &trie6, expected_bytes6);
    check_ascii_trie6(&litemap, &trie6);

    let trie1b = asciitrie::make1b_litemap(&litemap);
    check_bytes_eq(4, &trie1b, expected_bytes);
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
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"xyz"), None);
    check_ascii_trie(&litemap, &trie);
    let expected_bytes = &[b'x', 0b10000000, b'y', 0b10000001];
    check_bytes_eq(4, trie.as_bytes(), expected_bytes);

    let expected_bytes4 = &[b'x', 0b10000000, b'y', 0b10000001];
    let trie4 = asciitrie::make4_litemap(&litemap);
    check_bytes_eq(4, &trie4, expected_bytes4);
    check_ascii_trie4(&litemap, &trie4);

    let expected_bytes6 = &[b'x', 0b10000000, b'y', 0b10000001];
    let trie6 = asciitrie::make6_litemap(&litemap).unwrap();
    check_bytes_eq(4, &trie6, expected_bytes6);
    check_ascii_trie6(&litemap, &trie6);

    let trie1b = asciitrie::make1b_litemap(&litemap);
    check_bytes_eq(4, &trie1b, expected_bytes);
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
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"xy"), None);
    check_ascii_trie(&litemap, &trie);
    let expected_bytes = &[0b11000010, b'x', b'y', 0, 1, 0b10000000, 0b10000001];
    check_bytes_eq(7, trie.as_bytes(), expected_bytes);

    let expected_bytes4 = &[0b11000010, 255, b'x', b'y', 0, 1, 0b10000000, 0b10000001];
    let trie4 = asciitrie::make4_litemap(&litemap);
    check_bytes_eq(8, &trie4, expected_bytes4);
    check_ascii_trie4(&litemap, &trie4);

    let expected_bytes5 = &[0b11001000, b'x', b'y', 1, 0b10000000, 0b10000001];
    let trie5 = asciitrie::make5_litemap(&litemap);
    check_bytes_eq(6, &trie5, expected_bytes5);
    check_ascii_trie5(&litemap, &trie5);

    let expected_bytes6 = &[0b11000010, b'x', b'y', 1, 0b10000000, 0b10000001];
    let trie6 = asciitrie::make6_litemap(&litemap).unwrap();
    check_bytes_eq(6, &trie6, expected_bytes6);
    check_ascii_trie6(&litemap, &trie6);

    let trie1b = asciitrie::make1b_litemap(&litemap);
    check_bytes_eq(7, &trie1b, expected_bytes);
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
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"a"), None);
    assert_eq!(trie.get(b"ax"), None);
    assert_eq!(trie.get(b"ay"), None);
    check_ascii_trie(&litemap, &trie);
    let expected_bytes = &[
        b'a', 0b11000010, b'x', b'y', 0, 2, b'b', 0b10000000, b'c', 0b10000001,
    ];
    check_bytes_eq(10, trie.as_bytes(), expected_bytes);

    let expected_bytes4 = &[
        b'a', 0b11000010, 255, b'x', b'y', 0, 2, b'b', 0b10000000, b'c', 0b10000001,
    ];
    let trie4 = asciitrie::make4_litemap(&litemap);
    check_bytes_eq(11, &trie4, expected_bytes4);
    check_ascii_trie4(&litemap, &trie4);

    let expected_bytes5 = &[
        b'a', 0b11001000, b'x', b'y', 2, b'b', 0b10000000, b'c', 0b10000001,
    ];
    let trie5 = asciitrie::make5_litemap(&litemap);
    check_bytes_eq(9, &trie5, expected_bytes5);
    check_ascii_trie5(&litemap, &trie5);

    let expected_bytes6 = &[
        b'a', 0b11000010, b'x', b'y', 2, b'b', 0b10000000, b'c', 0b10000001,
    ];
    let trie6 = asciitrie::make6_litemap(&litemap).unwrap();
    check_bytes_eq(9, &trie6, expected_bytes6);
    check_ascii_trie6(&litemap, &trie6);

    let trie1b = asciitrie::make1b_litemap(&litemap);
    check_bytes_eq(10, &trie1b, expected_bytes);
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
    assert_eq!(trie.get(b"xy"), None);
    assert_eq!(trie.get(b"xz"), None);
    assert_eq!(trie.get(b"xyzz"), None);
    check_ascii_trie(&litemap, &trie);
    let expected_bytes = &[0xA0, 0x44, b'x', 0xA3, 0x54, b'y', b'z', 0xA0, 0x86, 0x68];
    check_bytes_eq(10, trie.as_bytes(), expected_bytes);

    let expected_bytes4 = &[0xA0, 0x44, b'x', 0xA3, 0x54, b'y', b'z', 0xA0, 0x86, 0x68];
    let trie4 = asciitrie::make4_litemap(&litemap);
    check_bytes_eq(10, &trie4, expected_bytes4);
    check_ascii_trie4(&litemap, &trie4);

    let expected_bytes5 = &[0xA0, 0x44, b'x', 0xA3, 0x54, b'y', b'z', 0xA0, 0x86, 0x68];
    let trie5 = asciitrie::make5_litemap(&litemap);
    check_bytes_eq(10, &trie5, expected_bytes5);
    check_ascii_trie5(&litemap, &trie5);

    let expected_bytes6 = &[0x90, 0x54, b'x', 0x93, 0x64, b'y', b'z', 0x90, 0x96, 0x78];
    let trie6 = asciitrie::make6_litemap(&litemap).unwrap();
    check_bytes_eq(10, &trie6, expected_bytes6);
    check_ascii_trie6(&litemap, &trie6);

    let trie1b = asciitrie::make1b_litemap(&litemap);
    check_bytes_eq(10, &trie1b, expected_bytes);
}

#[test]
fn test_varint_branch() {
    let chars =
        AsciiStr::try_from_str("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz").unwrap();
    let litemap: LiteMap<&AsciiStr, usize> = (0..chars.len())
        .map(|i| (chars.substring(i..i + 1).unwrap(), i))
        .collect();
    let trie = AsciiTrie::from_litemap(&litemap.as_sliced());
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"ax"), None);
    assert_eq!(trie.get(b"ay"), None);
    check_ascii_trie(&litemap, &trie);
    #[rustfmt::skip]
    let expected_bytes = &[
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
    ];
    check_bytes_eq(178, trie, expected_bytes);

    #[rustfmt::skip]
    let expected_bytes4 = &[
        0b11100000, // branch varint lead
        0x14,       // branch varint trail
        // PHF metadata:
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 10, 12, 16, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 16, 16, 16, 16, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 7,
        // search array:
        b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o',
        b'p', b'u', b'v', b'w', b'D', b'E', b'F', b'q',
        b'r', b'A', b'B', b'C', b'x', b'y', b'z', b's',
        b'H', b'I', b'J', b'G', b'P', b'Q', b'R', b'S',
        b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'K',
        b'L', b'M', b'N', b'O', b'g', b'a', b'b', b'c',
        b't', b'd', b'f', b'e',
        // offset array:
        0, 2, 4, 6, 8, 10, 12, 14,
        16, 18, 20, 22, 24, 25, 26, 27,
        29, 31, 32, 33, 34, 36, 38, 40,
        42, 43, 44, 45, 46, 47, 48, 49,
        50, 51, 52, 53, 54, 55, 56, 57,
        58, 59, 60, 61, 62, 64, 65, 66,
        67, 69, 70, 71,
        // values:
        0xA0, 1, 0xA0, 2, 0xA0, 3, 0xA0, 4, 0xA0, 5, 0xA0, 6, 0xA0, 7, 0xA0, 8,
        0xA0, 9, 0xA0, 14, 0xA0, 15, 0xA0, 16, 0x80 | 3, 0x80 | 4, 0x80 | 5, 0xA0, 10,
        0xA0, 11, 0x80 | 0, 0x80 | 1, 0x80 | 2, 0xA0, 17, 0xA0, 18, 0xA0, 19, 0xA0, 12,
        0x80 | 7, 0x80 | 8, 0x80 | 9, 0x80 | 6, 0x80 | 15, 0x80 | 16, 0x80 | 17, 0x80 | 18,
        0x80 | 19, 0x80 | 20, 0x80 | 21, 0x80 | 22, 0x80 | 23, 0x80 | 24, 0x80 | 25, 0x80 | 10,
        0x80 | 11, 0x80 | 12, 0x80 | 13, 0x80 | 14, 0xA0, 0, 0x80 | 26, 0x80 | 27, 0x80 | 28,
        0xA0, 13, 0x80 | 29, 0x80 | 31, 0x80 | 30,
    ];
    let trie4 = asciitrie::make4_litemap(&litemap);
    check_bytes_eq(231, &trie4, expected_bytes4);
    check_ascii_trie4(&litemap, &trie4);

    #[rustfmt::skip]
    let expected_bytes5 = &[
        0b11100001, // branch varint lead
        0x30,       // branch varint trail
        // PHF metadata:
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 10, 12, 16, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 16, 16, 16, 16, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 7,
        // search array:
        b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o',
        b'p', b'u', b'v', b'w', b'D', b'E', b'F', b'q',
        b'r', b'A', b'B', b'C', b'x', b'y', b'z', b's',
        b'H', b'I', b'J', b'G', b'P', b'Q', b'R', b'S',
        b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'K',
        b'L', b'M', b'N', b'O', b'g', b'a', b'b', b'c',
        b't', b'd', b'f', b'e',
        // offset array:
        2, 4, 6, 8, 10, 12, 14,
        16, 18, 20, 22, 24, 25, 26, 27,
        29, 31, 32, 33, 34, 36, 38, 40,
        42, 43, 44, 45, 46, 47, 48, 49,
        50, 51, 52, 53, 54, 55, 56, 57,
        58, 59, 60, 61, 62, 64, 65, 66,
        67, 69, 70, 71,
        // values:
        0xA0, 1, 0xA0, 2, 0xA0, 3, 0xA0, 4, 0xA0, 5, 0xA0, 6, 0xA0, 7, 0xA0, 8,
        0xA0, 9, 0xA0, 14, 0xA0, 15, 0xA0, 16, 0x80 | 3, 0x80 | 4, 0x80 | 5, 0xA0, 10,
        0xA0, 11, 0x80 | 0, 0x80 | 1, 0x80 | 2, 0xA0, 17, 0xA0, 18, 0xA0, 19, 0xA0, 12,
        0x80 | 7, 0x80 | 8, 0x80 | 9, 0x80 | 6, 0x80 | 15, 0x80 | 16, 0x80 | 17, 0x80 | 18,
        0x80 | 19, 0x80 | 20, 0x80 | 21, 0x80 | 22, 0x80 | 23, 0x80 | 24, 0x80 | 25, 0x80 | 10,
        0x80 | 11, 0x80 | 12, 0x80 | 13, 0x80 | 14, 0xA0, 0, 0x80 | 26, 0x80 | 27, 0x80 | 28,
        0xA0, 13, 0x80 | 29, 0x80 | 31, 0x80 | 30,
    ];
    let trie5 = asciitrie::make5_litemap(&litemap);
    check_bytes_eq(230, &trie5, expected_bytes5);
    check_ascii_trie5(&litemap, &trie5);

    #[rustfmt::skip]
    let expected_bytes6 = &[
        0b11100000, // branch varint lead
        0x14,       // branch varint trail
        // PHF metadata:
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 10, 12, 16, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 16, 16, 16, 16, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 7,
        // search array:
        b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o',
        b'p', b'u', b'v', b'w', b'D', b'E', b'F', b'q',
        b'r', b'A', b'B', b'C', b'x', b'y', b'z', b's',
        b'H', b'I', b'J', b'G', b'P', b'Q', b'R', b'S',
        b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'K',
        b'L', b'M', b'N', b'O', b'g', b'a', b'b', b'c',
        b't', b'd', b'f', b'e',
        // offset array:
        2, 4, 6, 8, 10, 12, 14,
        16, 18, 20, 22, 24, 25, 26, 27,
        29, 31, 32, 33, 34, 36, 38, 40,
        42, 43, 44, 45, 46, 47, 49, 51,
        53, 55, 57, 59, 61, 63, 65, 67,
        68, 69, 70, 71, 72, 74, 76, 78,
        80, 82, 84, 86,
        // values:
        0x90, 16+1, 0x90, 16+2, 0x90, 16+3, 0x90, 16+4, 0x90, 16+5, 0x90, 16+6, 0x90, 16+7, 0x90, 16+8,
        0x90, 16+9, 0x90, 16+14, 0x90, 16+15, 0x90, 16+16, 0x80 | 3, 0x80 | 4, 0x80 | 5, 0x90, 16+10,
        0x90, 16+11, 0x80 | 0, 0x80 | 1, 0x80 | 2, 0x90, 16+17, 0x90, 16+18, 0x90, 16+19, 0x90, 16+12,
        0x80 | 7, 0x80 | 8, 0x80 | 9, 0x80 | 6, 0x80 | 15, 0x90, 16-16, 0x90, 17-16, 0x90, 18-16,
        0x90, 19-16, 0x90, 20-16, 0x90, 21-16, 0x90, 22-16, 0x90, 23-16, 0x90, 24-16, 0x90, 25-16, 0x80 | 10,
        0x80 | 11, 0x80 | 12, 0x80 | 13, 0x80 | 14, 0x90, 16+0, 0x90, 26-16, 0x90, 27-16, 0x90, 28-16,
        0x90, 16+13, 0x90, 29-16, 0x90, 31-16, 0x90, 30-16,
    ];
    let trie6 = asciitrie::make6_litemap(&litemap).unwrap();
    check_bytes_eq(246, &trie6, expected_bytes6);
    check_ascii_trie6(&litemap, &trie6);

    let trie1b = asciitrie::make1b_litemap(&litemap);
    check_bytes_eq(178, &trie1b, expected_bytes);
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
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"abc"), None);
    check_ascii_trie(&litemap, &trie);
    #[rustfmt::skip]
    let expected_bytes = &[
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
    ];
    check_bytes_eq(276, trie, expected_bytes);

    #[rustfmt::skip]
    let expected_bytes4 = &[
        0b11001010, // branch
        // PHF metadata:
        255,
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
    ];
    let trie4 = asciitrie::make4_litemap(&litemap);
    check_bytes_eq(277, &trie4, expected_bytes4);
    check_ascii_trie4(&litemap, &trie4);

    let trie1b = asciitrie::make1b_litemap(&litemap);
    check_bytes_eq(276, &trie1b, expected_bytes);
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
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"abc"), None);
    check_ascii_trie(&litemap, &trie);
    #[rustfmt::skip]
    let expected_bytes = &[
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
    ];
    check_bytes_eq(287, trie, expected_bytes);

    #[rustfmt::skip]
    let expected_bytes4 = &[
        0b11001010, // branch
        // PHF metadata:
        255,
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
    ];
    let trie4 = asciitrie::make4_litemap(&litemap);
    check_bytes_eq(288, &trie4, expected_bytes4);
    check_ascii_trie4(&litemap, &trie4);

    let trie1b = asciitrie::make1b_litemap(&litemap);
    check_bytes_eq(287, &trie1b, expected_bytes);
}

#[test]
fn test_at_wide_plus() {
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
            AsciiStr::try_from_str("jklmnopqrstuvwxyzabcdef").unwrap(),
            10,
        ),
    ]
    .into_iter()
    .collect();
    let trie = AsciiTrie::from_litemap(&litemap.as_sliced());
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"abc"), None);
    check_ascii_trie(&litemap, &trie);
    #[rustfmt::skip]
    let expected_bytes = &[
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
        b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e', b'f',
        0x8A,
    ];
    check_bytes_eq(288, trie, expected_bytes);

    #[rustfmt::skip]
    let expected_bytes4 = &[
        0b11001010, // branch
        // PHF metadata:
        255,
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
        b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e', b'f',
        0x8A,
    ];
    let trie4 = asciitrie::make4_litemap(&litemap);
    check_bytes_eq(289, &trie4, expected_bytes4);
    check_ascii_trie4(&litemap, &trie4);

    let trie1b = asciitrie::make1b_litemap(&litemap);
    check_bytes_eq(288, &trie1b, expected_bytes);
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
    assert_eq!(trie.get(b""), Some(0));
    assert_eq!(trie.get(b"a"), None);
    assert_eq!(trie.get(b"ax"), None);
    assert_eq!(trie.get(b"ay"), None);
    check_ascii_trie(&litemap, &trie);
    let expected_bytes = &[
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
    ];
    check_bytes_eq(40, trie, expected_bytes);

    #[rustfmt::skip]
    let expected_bytes4 = &[
        0b10000000, // value 0
        0b11000010, // branch of 2
        255,        // PHF metadata
        b'a',       //
        b'b',       //
        0,          //
        15,         //
        0b11000011, // start of 'a' subtree: branch of 3
        255,        // PHF metadata
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
        b'x',       // start of 'b' subtree
        b'e',       //
        0b10000100, // value 4
        0b11000010, // branch of 2
        255,        // PHF metadata
        b'f',       //
        b'i',       //
        0,          //
        9,          //
        0b11000010, // branch of 2
        255,        // PHF metadata
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
    ];
    let trie4 = asciitrie::make4_litemap(&litemap);
    check_bytes_eq(44, &trie4, expected_bytes4);
    check_ascii_trie4(&litemap, &trie4);

    #[rustfmt::skip]
    let expected_bytes5 = &[
        0b10000000, // value 0
        0b11001000, // branch of 2
        b'a',       //
        b'b',       //
        13,         //
        0b11001100, // start of 'a' subtree: branch of 3
        b'x',       //
        b'y',       //
        b'z',       //
        3,          //
        5,          //
        b'b',       //
        0b10100000, // value 100 (lead)
        0x44,       // value 100 (trail)
        b'c',       //
        0b10000010, // value 2
        b'd',       //
        0b10000011, // value 3
        b'x',       // start of 'b' subtree
        b'e',       //
        0b10000100, // value 4
        0b11001000, // branch of 2
        b'f',       //
        b'i',       //
        7,          //
        0b11001000, // branch of 2
        b'g',       //
        b'h',       //
        2,          //
        0b10100011, // value 500 (lead)
        0x54,       // value 500 (trail)
        0b10000110, // value 6
        0b10000111, // value 7
        b'k',       //
        b'l',       //
        0b10001000, // value 8
    ];
    let trie5 = asciitrie::make5_litemap(&litemap);
    check_bytes_eq(36, &trie5, expected_bytes5);
    check_ascii_trie5(&litemap, &trie5);

    #[rustfmt::skip]
    let expected_bytes6 = &[
        0b10000000, // value 0
        0b11000010, // branch of 2
        b'a',       //
        b'b',       //
        13,         //
        0b11000011, // start of 'a' subtree: branch of 3
        b'x',       //
        b'y',       //
        b'z',       //
        3,          //
        5,          //
        b'b',       //
        0b10010000, // value 100 (lead)
        0x54,       // value 100 (trail)
        b'c',       //
        0b10000010, // value 2
        b'd',       //
        0b10000011, // value 3
        b'x',       // start of 'b' subtree
        b'e',       //
        0b10000100, // value 4
        0b11000010, // branch of 2
        b'f',       //
        b'i',       //
        7,          //
        0b11000010, // branch of 2
        b'g',       //
        b'h',       //
        2,          //
        0b10010011, // value 500 (lead)
        0x64,       // value 500 (trail)
        0b10000110, // value 6
        0b10000111, // value 7
        b'k',       //
        b'l',       //
        0b10001000, // value 8
    ];
    let trie6 = asciitrie::make6_litemap(&litemap).unwrap();
    check_bytes_eq(36, &trie6, expected_bytes6);
    check_ascii_trie6(&litemap, &trie6);

    let trie1b = asciitrie::make1b_litemap(&litemap);
    check_bytes_eq(40, &trie1b, expected_bytes);

    let zhm: zerovec::ZeroMap<[u8], usize> =
        litemap.iter().map(|(a, b)| (a.as_bytes(), b)).collect();
    let zhm_buf = postcard::to_allocvec(&zhm).unwrap();
    assert_eq!(zhm_buf.len(), 75);

    let zhm: zerovec::ZeroMap<[u8], u8> = litemap
        .iter()
        .map(|(a, b)| (a.as_bytes(), *b as u8))
        .collect();
    let zhm_buf = postcard::to_allocvec(&zhm).unwrap();
    assert_eq!(zhm_buf.len(), 65);

    let zhm: zerovec::ZeroHashMap<[u8], usize> =
        litemap.iter().map(|(a, b)| (a.as_bytes(), b)).collect();
    let zhm_buf = postcard::to_allocvec(&zhm).unwrap();
    assert_eq!(zhm_buf.len(), 148);

    let zhm: zerovec::ZeroHashMap<[u8], u8> = litemap
        .iter()
        .map(|(a, b)| (a.as_bytes(), *b as u8))
        .collect();
    let zhm_buf = postcard::to_allocvec(&zhm).unwrap();
    assert_eq!(zhm_buf.len(), 138);
}

macro_rules! utf8_byte {
    ($ch:expr, $i:literal) => {{
        let mut utf8_encoder_buf = [0u8; 4];
        $ch.encode_utf8(&mut utf8_encoder_buf);
        utf8_encoder_buf[$i]
    }};
}

#[test]
fn test_non_ascii() {
    let litemap: LiteMap<&[u8], usize> = [
        ("".as_bytes(), 0),
        ("axb".as_bytes(), 100),
        ("ayc".as_bytes(), 2),
        ("azd".as_bytes(), 3),
        ("bxe".as_bytes(), 4),
        ("bxefg".as_bytes(), 500),
        ("bxefh".as_bytes(), 6),
        ("bxei".as_bytes(), 7),
        ("bxeikl".as_bytes(), 8),
        ("bxeiklmΚαλημέρααα".as_bytes(), 9),
        ("bxeiklmαnλo".as_bytes(), 10),
        ("bxeiklmη".as_bytes(), 11),
    ]
    .into_iter()
    .collect();

    #[rustfmt::skip]
    let expected_bytes6 = &[
        0b10000000, // value 0
        0b11000010, // branch of 2
        b'a',       //
        b'b',       //
        13,         //
        0b11000011, // start of 'a' subtree: branch of 3
        b'x',       //
        b'y',       //
        b'z',       //
        3,          //
        5,          //
        b'b',       //
        0b10010000, // value 100 (lead)
        0x54,       // value 100 (trail)
        b'c',       //
        0b10000010, // value 2
        b'd',       //
        0b10000011, // value 3
        b'x',       // start of 'b' subtree
        b'e',       //
        0b10000100, // value 4
        0b11000010, // branch of 2
        b'f',       //
        b'i',       //
        7,          //
        0b11000010, // branch of 2
        b'g',       //
        b'h',       //
        2,          //
        0b10010011, // value 500 (lead)
        0x64,       // value 500 (trail)
        0b10000110, // value 6
        0b10000111, // value 7
        b'k',       //
        b'l',       //
        0b10001000, // value 8
        b'm',       //
        0b10100001, // span of length 1
        utf8_byte!('Κ', 0), // NOTE: all three letters have the same lead byte
        0b11000011, // branch of 3
        utf8_byte!('Κ', 1),
        utf8_byte!('α', 1),
        utf8_byte!('η', 1),
        21,
        27,
        0b10110000, // span of length 18 (lead)
        0b00000010, // span of length 18 (trail)
        utf8_byte!('α', 0),
        utf8_byte!('α', 1),
        utf8_byte!('λ', 0),
        utf8_byte!('λ', 1),
        utf8_byte!('η', 0),
        utf8_byte!('η', 1),
        utf8_byte!('μ', 0),
        utf8_byte!('μ', 1),
        utf8_byte!('έ', 0),
        utf8_byte!('έ', 1),
        utf8_byte!('ρ', 0),
        utf8_byte!('ρ', 1),
        utf8_byte!('α', 0),
        utf8_byte!('α', 1),
        utf8_byte!('α', 0),
        utf8_byte!('α', 1),
        utf8_byte!('α', 0),
        utf8_byte!('α', 1),
        0b10001001, // value 9
        b'n',
        0b10100010, // span of length 2
        utf8_byte!('λ', 0),
        utf8_byte!('λ', 1),
        b'o',
        0b10001010, // value 10
        0b10001011, // value 11
    ];
    let trie6 = asciitrie::make6_byte_litemap(&litemap).unwrap();
    check_bytes_eq(73, &trie6, expected_bytes6);
    check_ascii_trie6_bytes(&litemap, &trie6);
}

#[test]
fn test_max_branch() {
    // Evaluate a branch with all 256 possible children
    let mut litemap: LiteMap<&[u8], usize> = LiteMap::new_vec();
    let all_bytes: Vec<u8> = (u8::MIN..=u8::MAX).collect();
    assert_eq!(all_bytes.len(), 256);
    let all_bytes_prefixed: Vec<[u8; 2]> = (u8::MIN..=u8::MAX).map(|x| [b'\0', x]).collect();
    for b in all_bytes.iter() {
        litemap.insert(core::slice::from_ref(b), *b as usize);
    }
    for s in all_bytes_prefixed.iter() {
        litemap.insert(s, s[1] as usize);
    }
    let trie6 = asciitrie::make6_byte_litemap(&litemap).unwrap();
    assert_eq!(trie6.len(), 3042);
    check_ascii_trie6_bytes(&litemap, &trie6);
}

#[test]
fn test_short_subtags_10pct() {
    let litemap = strings_to_litemap(&testdata::short_subtags_10pct::STRINGS).unwrap();

    let trie = AsciiTrie::from_litemap(&litemap);
    assert_eq!(trie.byte_len(), 1077);
    check_ascii_trie(&litemap, &trie);

    let trie4 = asciitrie::make4_litemap(&litemap);
    assert_eq!(trie4.len(), 1190);
    check_ascii_trie4(&litemap, &trie4);

    let trie5 = asciitrie::make5_litemap(&litemap);
    assert_eq!(trie5.len(), 1091);
    check_ascii_trie5(&litemap, &trie5);

    let trie6 = asciitrie::make6_litemap(&litemap).unwrap();
    assert_eq!(trie6.len(), 1100);
    check_ascii_trie6(&litemap, &trie6);

    let trie1b = asciitrie::make1b_litemap(&litemap);
    check_bytes_eq(1077, &trie1b, trie.as_bytes());

    let zhm: zerovec::ZeroMap<[u8], usize> =
        litemap.iter().map(|(a, b)| (a.as_bytes(), b)).collect();
    let zhm_buf = postcard::to_allocvec(&zhm).unwrap();
    assert_eq!(zhm_buf.len(), 1331);

    let zhm: zerovec::ZeroMap<[u8], u8> = litemap
        .iter()
        .map(|(a, b)| (a.as_bytes(), *b as u8))
        .collect();
    let zhm_buf = postcard::to_allocvec(&zhm).unwrap();
    assert_eq!(zhm_buf.len(), 1330);

    let zhm: zerovec::ZeroHashMap<[u8], usize> =
        litemap.iter().map(|(a, b)| (a.as_bytes(), b)).collect();
    let zhm_buf = postcard::to_allocvec(&zhm).unwrap();
    assert_eq!(zhm_buf.len(), 2837);

    let zhm: zerovec::ZeroHashMap<[u8], u8> = litemap
        .iter()
        .map(|(a, b)| (a.as_bytes(), *b as u8))
        .collect();
    let zhm_buf = postcard::to_allocvec(&zhm).unwrap();
    assert_eq!(zhm_buf.len(), 2836);
}

#[test]
fn test_short_subtags() {
    let litemap = strings_to_litemap(testdata::short_subtags::STRINGS).unwrap();

    let trie4 = asciitrie::make4_litemap(&litemap);
    assert_eq!(trie4.len(), 10211);
    check_ascii_trie4(&litemap, &trie4);

    let trie5 = asciitrie::make5_litemap(&litemap);
    assert_eq!(trie5.len(), 9434);
    check_ascii_trie5(&litemap, &trie5);

    let trie6 = asciitrie::make6_litemap(&litemap).unwrap();
    assert_eq!(trie6.len(), 9400);
    check_ascii_trie6(&litemap, &trie6);

    let trie1b = asciitrie::make1b_litemap(&litemap);
    assert_eq!(trie1b.len(), 9204);
    check_ascii_trie(&litemap, AsciiTrie::from_bytes(&trie1b));

    let zm: zerovec::ZeroMap<[u8], usize> =
        litemap.iter().map(|(a, b)| (a.as_bytes(), b)).collect();
    let zhm_buf = postcard::to_allocvec(&zm).unwrap();
    assert_eq!(zhm_buf.len(), 15182);

    let zm: zerovec::ZeroMap<[u8], u8> = litemap
        .iter()
        .map(|(a, b)| (a.as_bytes(), *b as u8))
        .collect();
    let zhm_buf = postcard::to_allocvec(&zm).unwrap();
    assert_eq!(zhm_buf.len(), 13304);

    let zhm: zerovec::ZeroHashMap<[u8], usize> =
        litemap.iter().map(|(a, b)| (a.as_bytes(), b)).collect();
    let zhm_buf = postcard::to_allocvec(&zhm).unwrap();
    assert_eq!(zhm_buf.len(), 30200);

    let zhm: zerovec::ZeroHashMap<[u8], u8> = litemap
        .iter()
        .map(|(a, b)| (a.as_bytes(), *b as u8))
        .collect();
    let zhm_buf = postcard::to_allocvec(&zhm).unwrap();
    assert_eq!(zhm_buf.len(), 28322);
}
