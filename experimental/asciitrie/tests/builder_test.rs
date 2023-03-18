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
    let trie = testdata::basic::TRIE;
    let data = testdata::basic::DATA;

    // Check that the builder works
    let built_trie: AsciiTrie<Vec<u8>> = data.iter().copied().collect();
    assert_eq!(built_trie.as_bytes(), trie);

    assert!(data
        .iter()
        .copied()
        .map(|(s, v)| (s.to_boxed(), v))
        .eq(built_trie.iter()));
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

fn check_ascii_trie2(items: &LiteMap<&AsciiStr, usize>, trie: &[u8]) {
    for (k, v) in items.iter() {
        assert_eq!(asciitrie::reader2::get(trie, k.as_bytes()), Some(*v));
    }
    // assert!(items
    //     .iter()
    //     .map(|(s, v)| (s.to_boxed(), *v))
    //     .eq(trie.iter()));
}

fn check_ascii_trie3(items: &LiteMap<&AsciiStr, usize>, trie: &[u8]) {
    for (k, v) in items.iter() {
        assert_eq!(asciitrie::reader3::get(trie, k.as_bytes()), Some(*v));
    }
    // assert!(items
    //     .iter()
    //     .map(|(s, v)| (s.to_boxed(), *v))
    //     .eq(trie.iter()));
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

    let expected_bytes2 = &[0b10101010];
    let trie2 = asciitrie::make2_litemap(&litemap);
    check_bytes_eq(1, &trie2, expected_bytes2);
    check_ascii_trie2(&litemap, &trie2);

    let expected_bytes3 = &[0b10101010];
    let trie3 = asciitrie::make3_litemap(&litemap);
    check_bytes_eq(1, &trie3, expected_bytes3);
    check_ascii_trie3(&litemap, &trie3);

    let expected_bytes4 = &[0b10001010];
    let trie4 = asciitrie::make4_litemap(&litemap);
    check_bytes_eq(1, &trie4, expected_bytes4);
    check_ascii_trie4(&litemap, &trie4);

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

    let expected_bytes2 = &[b'x', 0b10101010];
    let trie2 = asciitrie::make2_litemap(&litemap);
    check_bytes_eq(2, &trie2, expected_bytes2);
    check_ascii_trie2(&litemap, &trie2);

    let expected_bytes3 = &[b'x', 0b10101010];
    let trie3 = asciitrie::make3_litemap(&litemap);
    check_bytes_eq(2, &trie3, expected_bytes3);
    check_ascii_trie3(&litemap, &trie3);

    let expected_bytes4 = &[b'x', 0b10001010];
    let trie4 = asciitrie::make4_litemap(&litemap);
    check_bytes_eq(2, &trie4, expected_bytes4);
    check_ascii_trie4(&litemap, &trie4);

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

    let expected_bytes2 = &[b'x', b'y', b'z', 0b10101010];
    let trie2 = asciitrie::make2_litemap(&litemap);
    check_bytes_eq(4, &trie2, expected_bytes2);
    check_ascii_trie2(&litemap, &trie2);

    let expected_bytes3 = &[b'x', b'y', b'z', 0b10101010];
    let trie3 = asciitrie::make3_litemap(&litemap);
    check_bytes_eq(4, &trie3, expected_bytes3);
    check_ascii_trie3(&litemap, &trie3);

    let expected_bytes4 = &[b'x', b'y', b'z', 0b10001010];
    let trie4 = asciitrie::make4_litemap(&litemap);
    check_bytes_eq(4, &trie4, expected_bytes4);
    check_ascii_trie4(&litemap, &trie4);

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

    let expected_bytes2 = &[b'x', 0b10000000, b'y', 0b10100001];
    let trie2 = asciitrie::make2_litemap(&litemap);
    check_bytes_eq(4, &trie2, expected_bytes2);
    check_ascii_trie2(&litemap, &trie2);

    let expected_bytes3 = &[b'x', 0b10000000, b'y', 0b10100001];
    let trie3 = asciitrie::make3_litemap(&litemap);
    check_bytes_eq(4, &trie3, expected_bytes3);
    check_ascii_trie3(&litemap, &trie3);

    let expected_bytes4 = &[b'x', 0b10000000, b'y', 0b10000001];
    let trie4 = asciitrie::make4_litemap(&litemap);
    check_bytes_eq(4, &trie4, expected_bytes4);
    check_ascii_trie4(&litemap, &trie4);

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

    let expected_bytes2 = &[0b11000010, b'y', b'x', 0b10100000, 0b10100001];
    let trie2 = asciitrie::make2_litemap(&litemap);
    check_bytes_eq(5, &trie2, expected_bytes2);
    check_ascii_trie2(&litemap, &trie2);

    let expected_bytes3 = &[0b11100010, b'y', b'x', 0b10100000, 0b10100001];
    let trie3 = asciitrie::make3_litemap(&litemap);
    check_bytes_eq(5, &trie3, expected_bytes3);
    check_ascii_trie3(&litemap, &trie3);

    let expected_bytes4 = &[0b11000010, 0, 0, 0, b'x', b'y', 0, 1, 0b10000000, 0b10000001];
    let trie4 = asciitrie::make4_litemap(&litemap);
    check_bytes_eq(10, &trie4, expected_bytes4);
    check_ascii_trie4(&litemap, &trie4);

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

    let expected_bytes2 = &[
        b'a', 0b11000011, b'y', b'x', b'b', 0b10100000, b'c', 0b10100001,
    ];
    let trie2 = asciitrie::make2_litemap(&litemap);
    check_bytes_eq(8, &trie2, expected_bytes2);
    check_ascii_trie2(&litemap, &trie2);

    let expected_bytes3 = &[
        b'a', 0b11100011, b'y', b'x', b'b', 0b10100000, b'c', 0b10100001,
    ];
    let trie3 = asciitrie::make3_litemap(&litemap);
    check_bytes_eq(8, &trie3, expected_bytes3);
    check_ascii_trie3(&litemap, &trie3);

    let expected_bytes4 = &[b'a', 0b11000010, 0, 0, 0, b'x', b'y', 0, 2, b'b', 0b10000000, b'c', 0b10000001,];
    let trie4 = asciitrie::make4_litemap(&litemap);
    check_bytes_eq(13, &trie4, expected_bytes4);
    check_ascii_trie4(&litemap, &trie4);

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

    let expected_bytes2 = &[0x90, 0x54, b'x', 0x93, 0x64, b'y', b'z', 0xB0, 0x96, 0x78];
    let trie2 = asciitrie::make2_litemap(&litemap);
    check_bytes_eq(10, &trie2, expected_bytes2);
    check_ascii_trie2(&litemap, &trie2);

    let expected_bytes3 = &[0x90, 0x54, b'x', 0x93, 0x64, b'y', b'z', 0xB0, 0x96, 0x78];
    let trie3 = asciitrie::make3_litemap(&litemap);
    check_bytes_eq(10, &trie3, expected_bytes3);
    check_ascii_trie3(&litemap, &trie3);

    let expected_bytes4 = &[0xA0, 0x44, b'x', 0xA3, 0x54, b'y', b'z', 0xA0, 0x86, 0x68];
    let trie4 = asciitrie::make4_litemap(&litemap);
    check_bytes_eq(10, &trie4, expected_bytes4);
    check_ascii_trie4(&litemap, &trie4);

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

    let expected_bytes2 = &[
        0xd0, // branch equal varint
        0x4a, // branch equal 90
        0xe2, // branch greater 2
        b'a', // ascii
        0xd0, // branch equal varint
        0x17, // branch equal 39
        0xe1, // branch greater 1
        b'N', // ascii
        0xd0, // branch equal varint
        0x01, // branch equal 17
        0xe1, // branch greater 1
        b'G', // ascii
        0xc8, // branch equal 8
        0xe1, // branch greater 1
        b'D', // ascii
        0xc2, // branch equal 2
        0xe1, // branch greater 1
        b'B', // ascii
        b'A', // ascii
        0xa0, // final value 0
        0xa1, // final value 1
        b'C', // ascii
        0xa2, // final value 2
        0xa3, // final value 3
        0xc2, // branch equal 2
        b'F', // ascii
        b'E', // ascii
        0xa4, // final value 4
        0xa5, // final value 5
        0xa6, // final value 6
        0xc8, // branch equal 8
        0xe1, // branch greater 1
        b'K', // ascii
        0xc2, // branch equal 2
        0xe1, // branch greater 1
        b'I', // ascii
        b'H', // ascii
        0xa7, // final value 7
        0xa8, // final value 8
        b'J', // ascii
        0xa9, // final value 9
        0xaa, // final value 10
        0xc2, // branch equal 2
        b'M', // ascii
        b'L', // ascii
        0xab, // final value 11
        0xac, // final value 12
        0xad, // final value 13
        0xd0, // branch equal varint
        0x05, // branch equal 21
        0xe2, // branch greater 2
        b'U', // ascii
        0xc9, // branch equal 9
        0xe2, // branch greater 2
        b'R', // ascii
        0xc2, // branch equal 2
        0xe1, // branch greater 1
        b'P', // ascii
        b'O', // ascii
        0xae, // final value 14
        0xaf, // final value 15
        b'Q', // ascii
        0xb0, // final value varint
        0x00, // final value 16
        0xb0, // final value varint
        0x01, // final value 17
        0xc3, // branch equal 3
        b'T', // ascii
        b'S', // ascii
        0xb0, // final value varint
        0x02, // final value 18
        0xb0, // final value varint
        0x03, // final value 19
        0xb0, // final value varint
        0x04, // final value 20
        0xc7, // branch equal 7
        0xe2, // branch greater 2
        b'X', // ascii
        0xc3, // branch equal 3
        b'W', // ascii
        b'V', // ascii
        0xb0, // final value varint
        0x05, // final value 21
        0xb0, // final value varint
        0x06, // final value 22
        0xb0, // final value varint
        0x07, // final value 23
        0xc3, // branch equal 3
        b'Z', // ascii
        b'Y', // ascii
        0xb0, // final value varint
        0x08, // final value 24
        0xb0, // final value varint
        0x09, // final value 25
        0xb0, // final value varint
        0x0a, // final value 26
        0xd0, // branch equal varint
        0x20, // branch equal 48
        0xe2, // branch greater 2
        b'n', // ascii
        0xd0, // branch equal varint
        0x07, // branch equal 23
        0xe2, // branch greater 2
        b'h', // ascii
        0xcb, // branch equal 12
        0xe2, // branch greater 2
        b'e', // ascii
        0xc3, // branch equal 3
        0xe2, // branch greater 2
        b'c', // ascii
        b'b', // ascii
        0xb0, // final value varint
        0x0b, // final value 27
        0xb0, // final value varint
        0x0c, // final value 28
        b'd', // ascii
        0xb0, // final value varint
        0x0d, // final value 29
        0xb0, // final value varint
        0x0e, // final value 30
        0xc3, // branch equal 3
        b'g', // ascii
        b'f', // ascii
        0xb0, // final value varint
        0x0f, // final value 31
        0xb0, // final value varint
        0x10, // final value 32
        0xb0, // final value varint
        0x11, // final value 33
        0xc7, // branch equal 7
        0xe2, // branch greater 2
        b'k', // ascii
        0xc3, // branch equal 3
        b'j', // ascii
        b'i', // ascii
        0xb0, // final value varint
        0x12, // final value 34
        0xb0, // final value varint
        0x13, // final value 35
        0xb0, // final value varint
        0x14, // final value 36
        0xc3, // branch equal 3
        b'm', // ascii
        b'l', // ascii
        0xb0, // final value varint
        0x15, // final value 37
        0xb0, // final value varint
        0x16, // final value 38
        0xb0, // final value varint
        0x17, // final value 39
        0xd0, // branch equal varint
        0x07, // branch equal 23
        0xe2, // branch greater 2
        b'u', // ascii
        0xcb, // branch equal b
        0xe2, // branch greater 2
        b'r', // ascii
        0xc3, // branch equal 3
        0xe2, // branch greater 2
        b'p', // ascii
        b'o', // ascii
        0xb0, // final value varint
        0x18, // final value 40
        0xb0, // final value varint
        0x19, // final value 41
        b'q', // ascii
        0xb0, // final value varint
        0x1a, // final value 42
        0xb0, // final value varint
        0x1b, // final value 43
        0xc3, // branch equal 3
        b't', // ascii
        b's', // ascii
        0xb0, // final value varint
        0x1c, // final value 44
        0xb0, // final value varint
        0x1d, // final value 45
        0xb0, // final value varint
        0x1e, // final value 46
        0xc7, // branch equal 7
        0xe2, // branch greater 2
        b'x', // ascii
        0xc3, // branch equal 3
        b'w', // ascii
        b'v', // ascii
        0xb0, // final value varint
        0x1f, // final value 47
        0xb0, // final value varint
        0x20, // final value 48
        0xb0, // final value varint
        0x21, // final value 49
        0xc3, // branch equal 3
        b'z', // ascii
        b'y', // ascii
        0xb0, // final value varint
        0x22, // final value 50
        0xb0, // final value varint
        0x23, // final value 51
    ];
    let trie2 = asciitrie::make2_litemap(&litemap);
    check_bytes_eq(198, &trie2, expected_bytes2);
    check_ascii_trie2(&litemap, &trie2);

    let expected_bytes3 = &[
        0xd0, // branch varint
        0x59, // branch 105
        b'a', // ascii
        0xd0, // branch varint
        0x1e, // branch 46
        b'N', // ascii
        0xd0, // branch varint
        0x04, // branch 20
        b'G', // ascii
        0xc9, // branch 9
        b'D', // ascii
        0xc2, // branch 2
        b'B', // ascii
        b'A', // ascii
        0xa0, // final value 0
        0xe2, // branch final 2
        b'C', // ascii
        b'B', // ascii
        0xa1, // final value 1
        0xa2, // final value 2
        0xc2, // branch 2
        b'E', // ascii
        b'D', // ascii
        0xa3, // final value 3
        0xe2, // branch final 2
        b'F', // ascii
        b'E', // ascii
        0xa4, // final value 4
        0xa5, // final value 5
        0xc9, // branch 9
        b'J', // ascii
        0xc2, // branch 2
        b'H', // ascii
        b'G', // ascii
        0xa6, // final value 6
        0xe2, // branch final 2
        b'I', // ascii
        b'H', // ascii
        0xa7, // final value 7
        0xa8, // final value 8
        0xc5, // branch 5
        b'L', // ascii
        0xe2, // branch final 2
        b'K', // ascii
        b'J', // ascii
        0xa9, // final value 9
        0xaa, // final value 10
        0xe2, // branch final 2
        b'M', // ascii
        b'L', // ascii
        0xab, // final value 11
        0xac, // final value 12
        0xd0, // branch varint
        0x07, // branch 23
        b'T', // ...
        0xc9, b'Q', 0xc2, b'O', b'N', 0xad, 0xe2, b'P', b'O', 0xae, 0xaf, 0xc3, b'R', b'Q', 0xb0,
        0x00, 0xe3, b'S', b'R', 0xb0, 0x01, 0xb0, 0x02, 0xcc, b'W', 0xc3, b'U', b'T', 0xb0, 0x03,
        0xe3, b'V', b'U', 0xb0, 0x04, 0xb0, 0x05, 0xc7, b'Y', 0xe3, b'X', b'W', 0xb0, 0x06, 0xb0,
        0x07, 0xe3, b'Z', b'Y', 0xb0, 0x08, 0xb0, 0x09, 0xd0, 0x2b, b'n', 0xd0, 0x0a, b'g', 0xcc,
        b'd', 0xc3, b'b', b'a', 0xb0, 0x0a, 0xe3, b'c', b'b', 0xb0, 0x0b, 0xb0, 0x0c, 0xc3, b'e',
        b'd', 0xb0, 0x0d, 0xe3, b'f', b'e', 0xb0, 0x0e, 0xb0, 0x0f, 0xcc, b'j', 0xc3, b'h', b'g',
        0xb0, 0x10, 0xe3, b'i', b'h', 0xb0, 0x11, 0xb0, 0x12, 0xc7, b'l', 0xe3, b'k', b'j', 0xb0,
        0x13, 0xb0, 0x14, 0xe3, b'm', b'l', 0xb0, 0x15, 0xb0, 0x16, 0xd0, 0x0a, b't', 0xcc, b'q',
        0xc3, b'o', b'n', 0xb0, 0x17, 0xe3, b'p', b'o', 0xb0, 0x18, 0xb0, 0x19, 0xc3, b'r', b'q',
        0xb0, 0x1a, 0xe3, b's', b'r', 0xb0, 0x1b, 0xb0, 0x1c, 0xcc, b'w', 0xc3, b'u', b't', 0xb0,
        0x1d, 0xe3, b'v', b'u', 0xb0, 0x1e, 0xb0, 0x1f, 0xc7, b'y', 0xe3, b'x', b'w', 0xb0, 0x20,
        0xb0, 0x21, 0xe3, b'z', b'y', 0xb0, 0x22, 0xb0, 0x23,
    ];
    let trie3 = asciitrie::make3_litemap(&litemap);
    check_bytes_eq(229, &trie3, expected_bytes3);
    check_ascii_trie3(&litemap, &trie3);

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
    let expected_bytes2 = &[
        0b11010000, // 'f' branch equal varint
        0b01111111, // 'f' branch equal 143 (to linear sequence starting with 'g')
        0b11110000, // 'f' branch greater varint
        0b00001010, // 'f' branch greater 26 (to 'i' branch)
        b'f',
        0b11010000, // 'c' branch equal varint
        0b00101000, // 'c' branch equal 56 (to 1st linear sequence starting with 'd')
        0b11110000, // 'c' branch greater varint
        0b00001010, // 'c' branch greater 26 (to 'e' branch)
        b'c',
        0b11010000, // 'b' branch equal varint
        0b00001011, // 'b' branch equal 27 (to linear sequence starting with 'c')
        b'b',
        b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm',
        b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z',
        0xA1, // final value 1
        b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o',
        b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a',
        0xA2, // final value 2
        b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p',
        b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b',
        0xA3, // final value 3
        0b11010000, // 'e' branch equal varint
        0b00001011, // 'e' branch equal 27 (to linear sequence starting with 'f')
        b'e',
        b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p',
        b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c',
        0xA4, // final value 4
        b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r',
        b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd',
        0xA5, // final value 5
        b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's',
        b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e',
        0xA6, // final value 6
        0b11010000, // 'i' branch equal varint
        0b00101000, // 'i' branch equal 56 (to 1st linear sequence starting with 'j')
        0b11110000, // 'i' branch greater varint
        0b00001010, // 'i' branch greater 26 (to 2nd linear sequence starting with 'j')
        b'i',
        0b11010000, // 'h' branch equal varint
        0b00001011, // 'h' branch equal 27 (to linear sequence starting with 'i')
        b'h',
        b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's',
        b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e', b'f',
        0xA7, // final value 7
        b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u',
        b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e', b'f', b'g',
        0xA8, // final value 8
        b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
        b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h',
        0xA9, // final value 9
        b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
        b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd',
        0xAA, // final value 10
    ];
    let trie2 = asciitrie::make2_litemap(&litemap);
    check_bytes_eq(283, &trie2, expected_bytes2);
    check_ascii_trie2(&litemap, &trie2);

    #[rustfmt::skip]
    let expected_bytes3 = &[
        0xd1, // branch varint
        0x01, // branch 145
        b'f', // ascii
        0xd0, // branch varint
        0x28, // branch 56
        b'c', // ascii
        0xf0, // branch final varint
        0x0b, // branch final 27
        b'b',
        b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm',
        b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z',
        0xa1, // final value 1
        /***/ b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
        b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a',
        0xa2, // final value 2
        0xd0, // branch varint
        0x0b, // branch 27
        b'd',
        b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o',
        b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b',
        0xa3, // final value 3
        0xf0, // branch final varint
        0x0b, // branch final 27
        b'e',
        b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p',
        b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c',
        0xa4, // final value 4
        /***/ b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q',
        b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd',
        0xa5, // final value 5
        0xd0, // branch varint
        0x28, // branch 56
        b'h',
        0xf0, // branch final varint
        0x0b, // branch final 27
        b'g',
        b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r',
        b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e',
        0xa6, // final value 6
        /***/ b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's',
        b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e', b'f',
        0xa7, // final value 7
        0xd0, // branch varint
        0x0b, // branch 27
        b'i',
        b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't',
        b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e', b'f', b'g',
        0xa8, // final value 8
        0xf0, // branch final varint
        0x0b, // branch final 27
        b'j',
        b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u',
        b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h',
        0xa9, // final value 9
        /***/ b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
        b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd',
        0xaa, // final value 10
    ];
    let trie3 = asciitrie::make3_litemap(&litemap);
    check_bytes_eq(288, &trie3, expected_bytes3);
    check_ascii_trie3(&litemap, &trie3);

    #[rustfmt::skip]
    let expected_bytes4 = &[
        0b11001010, // branch
        // PHF metadata:
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // search array:
        b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'a', b'b', b'c',
        // offset array:
        0, 26, 52, 78, 104, 130, 156, 177, 203, 229,
        // offset data:
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
        b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
        b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z',
        0x81,
        b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o',
        b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a',
        0x82,
        b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p',
        b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b',
        0x83,
    ];
    let trie4 = asciitrie::make4_litemap(&litemap);
    check_bytes_eq(287, &trie4, expected_bytes4);
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
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // search array:
        b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'a', b'b', b'c',
        // offset array (wide):
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 26, 52, 78, 104, 130, 156, 178, 204, 230,
        // offset data:
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
        b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
        b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z',
        0x81,
        b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o',
        b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a',
        0x82,
        b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p',
        b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b',
        0x83,
    ];
    let trie4 = asciitrie::make4_litemap(&litemap);
    check_bytes_eq(298, &trie4, expected_bytes4);
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
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // search array:
        b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'a', b'b', b'c',
        // offset array (wide):
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 26, 52, 78, 104, 130, 156, 179, 205, 231,
        // offset data:
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
        b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
        b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z',
        0x81,
        b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o',
        b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a',
        0x82,
        b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p',
        b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b',
        0x83,
    ];
    let trie4 = asciitrie::make4_litemap(&litemap);
    check_bytes_eq(299, &trie4, expected_bytes4);
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

    let expected_bytes2 = &[
        0x80, // intermediate value 0
        0xCD, // branch equal 13 (to the 2nd 'x')
        b'b', //
        b'a', //
        0xC4, // branch equal 4 (to the 'c')
        0xE2, // branch greater 2 (to the 'z')
        b'y', //
        b'x', //
        b'b', //
        0xB0, // final value varint
        0x54, // final value 100
        b'c', //
        0xA2, // final value 2
        b'z', //
        b'd', //
        0xA3, // final value 3
        b'x', //
        b'e', //
        0x84, // intermediate value 4
        0xC7, // branch equal 7 (to intermediate value 7)
        b'i', //
        b'f', //
        0xC3, // branch equal 3 (to final value 6)
        b'h', //
        b'g', //
        0xB3, // final value varint
        0x64, // final value 500
        0xA6, // final value 6
        0x87, // intermediate value 7
        b'k', //
        b'l', //
        0xA8, // final value 8
    ];
    let trie2 = asciitrie::make2_litemap(&litemap);
    check_bytes_eq(32, &trie2, expected_bytes2);
    check_ascii_trie2(&litemap, &trie2);

    let expected_bytes3 = &[
        0x80, // intermediate value 0
        0xee, // final branch 14 (to the 'x')
        b'b', //
        b'a', //
        0xc4, // branch 4 (to the 0xe3)
        b'y', //
        b'x', //
        b'b', //
        0xb0, // final value varint
        0x54, // final value 100
        0xe3, // final branch 3 (to the 'd')
        b'z', //
        b'y', //
        b'c', //
        0xa2, // final value 2
        b'd', //
        0xa3, // final value 3
        b'x', //
        b'e', //
        0x84, // intermediate value 4
        0xe7, // final branch 7 (to the 0x87)
        b'i', //
        b'f', //
        0xe3, // final branch 3 (to the 0xa6)
        b'h', //
        b'g', //
        0xb3, // final value varint
        0x64, // final value 500
        0xa6, // final value 6
        0x87, // intermediate value 7
        b'k', //
        b'l', //
        0xa8, // final value 8
    ];
    let trie3 = asciitrie::make3_litemap(&litemap);
    check_bytes_eq(33, &trie3, expected_bytes3);
    check_ascii_trie3(&litemap, &trie3);

    #[rustfmt::skip]
    let expected_bytes4 = &[
        0b10000000, // value 0
        0b11000010, // branch of 2
        0,          // PHF metadata
        0,          //
        0,          //
        b'b',       // 
        b'a',       //
        0,          //
        26,         //
        b'x',       // start of 'b' subtree
        b'e',       //
        0b10000100, // value 4
        0b11000010, // branch of 2
        0,          // PHF metadata
        0,          //
        0,          //
        b'f',       //
        b'i',       //
        0,          //
        11,          //
        0b11000010, // branch of 2
        0,          // PHF metadata
        0,          //
        0,          //
        b'h',       //
        b'g',       //
        0,          //
        1,          //
        0b10000110, // value 6
        0b10100011, // value 500 (lead)
        0x54,       // value 500 (trail)
        0b10000111, // value 7
        b'k',       //
        b'l',       //
        0b10001000, // value 8
        0b11000011, // start of 'a' subtree: branch of 3
        0,          // PHF metadata
        0,          //
        0,          //
        0,          //
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
    ];
    let trie4 = asciitrie::make4_litemap(&litemap);
    check_bytes_eq(53, &trie4, expected_bytes4);
    check_ascii_trie4(&litemap, &trie4);

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

#[test]
fn test_short_subtags_10pct() {
    let litemap = strings_to_litemap(&testdata::short_subtags_10pct::STRINGS).unwrap();

    let trie = AsciiTrie::from_litemap(&litemap);
    assert_eq!(trie.byte_len(), 1077);
    check_ascii_trie(&litemap, &trie);

    let trie2 = asciitrie::make2_litemap(&litemap);
    assert_eq!(trie2.len(), 1017);
    check_ascii_trie2(&litemap, &trie2);

    let trie3 = asciitrie::make3_litemap(&litemap);
    assert_eq!(trie3.len(), 1110);
    check_ascii_trie3(&litemap, &trie3);

    let trie4 = asciitrie::make4_litemap(&litemap);
    assert_eq!(trie4.len(), 1266);
    check_ascii_trie4(&litemap, &trie4);

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

    let trie2 = asciitrie::make2_litemap(&litemap);
    assert_eq!(trie2.len(), 8424);
    check_ascii_trie2(&litemap, &trie2);

    let litemap = strings_to_litemap(testdata::short_subtags::STRINGS).unwrap();
    let trie3 = asciitrie::make3_litemap(&litemap);
    assert_eq!(trie3.len(), 9289);
    check_ascii_trie3(&litemap, &trie3);

    let trie4 = asciitrie::make4_litemap(&litemap);
    assert_eq!(trie4.len(), 11782);
    check_ascii_trie4(&litemap, &trie4);

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
