// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use asciitrie::ZeroTriePerfectHash;
use asciitrie::AsciiStr;
use asciitrie::ZeroTrieSimpleAscii;
use litemap::LiteMap;

mod testdata {
    include!("data.rs");
}

use testdata::strings_to_litemap;

#[test]
fn test_basic() {
    let litemap: LiteMap<&AsciiStr, usize> = testdata::basic::DATA.iter().copied().collect();
    let litemap_u: LiteMap<&[u8], usize> = testdata::basic::DATA_U.iter().copied().collect();
    let litemap_bin: LiteMap<&[u8], usize> = testdata::basic::DATA_BIN.iter().copied().collect();

    let expected_bytes = testdata::basic::TRIE;
    let trie: ZeroTrieSimpleAscii<Vec<u8>> = litemap.iter().map(|(k, v)| (*k, *v)).collect();
    check_bytes_eq(26, trie.as_bytes(), expected_bytes);
    check_ascii_trie(&litemap, &trie);

    let expected_bytes6 = testdata::basic::TRIE6;
    let trie6 = ZeroTriePerfectHash::try_from_litemap(&litemap.to_borrowed_keys::<[u8], Vec<_>>()).unwrap();
    check_bytes_eq(26, trie6.as_bytes(), expected_bytes6);
    check_ascii_trie6(&litemap, &trie6);

    let expected_bytes_u6 = testdata::basic::TRIE_U6;
    let trie_u6 = ZeroTriePerfectHash::try_from_litemap(&litemap_u).unwrap();
    check_bytes_eq(39, trie_u6.as_bytes(), expected_bytes_u6);
    check_ascii_trie6_bytes(&litemap_u, &trie_u6);

    let expected_bytes_bin6 = testdata::basic::TRIE_BIN6;
    let trie_bin6 = ZeroTriePerfectHash::try_from_litemap(&litemap_bin).unwrap();
    check_bytes_eq(26, trie_bin6.as_bytes(), expected_bytes_bin6);
    check_ascii_trie6_bytes(&litemap_bin, &trie_bin6);
}

fn check_ascii_trie<S>(items: &LiteMap<&AsciiStr, usize>, trie: &ZeroTrieSimpleAscii<S>)
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

fn check_ascii_trie6<S>(items: &LiteMap<&AsciiStr, usize>, trie: &ZeroTriePerfectHash<S>)
where S: AsRef<[u8]> + ?Sized
{
    for (k, v) in items.iter() {
        assert_eq!(trie.get(k.as_bytes()), Some(*v));
    }
    // Note: We can't compare the iterators because trie6 might not return items in order.
    let recovered_items: LiteMap<_, _> = trie.iter().collect();
    assert_eq!(
        items.to_borrowed_keys_values::<[u8], usize, Vec<_>>(),
        recovered_items.to_borrowed_keys_values()
    );
}

fn check_ascii_trie6_bytes<S>(items: &LiteMap<&[u8], usize>, trie: &ZeroTriePerfectHash<S>)
where S: AsRef<[u8]> + ?Sized
{
    for (k, v) in items.iter() {
        assert_eq!(trie.get(k), Some(*v));
    }
    // Note: We can't compare the iterators because trie6 might not return items in order.
    let recovered_items: LiteMap<_, _> = trie.iter().collect();
    assert_eq!(
        items.to_borrowed_keys_values::<[u8], usize, Vec<_>>(),
        recovered_items.to_borrowed_keys_values()
    );
}

fn check_ascii_trie7(items: &LiteMap<&AsciiStr, usize>, trie: &[u8]) {
    for (k, v) in items.iter() {
        assert_eq!(asciitrie::reader7::get(trie, k.as_bytes()), Some(*v));
    }
    // Note: We can't compare the iterators because trie7 might not return items in order.
    let recovered_items: LiteMap<_, _> = asciitrie::reader7::get_iter(trie).collect();
    assert_eq!(
        items.to_borrowed_keys_values::<[u8], usize, Vec<_>>(),
        recovered_items.to_borrowed_keys_values()
    );
}

fn check_bytes_eq(len: usize, a: impl AsRef<[u8]>, b: &[u8]) {
    assert_eq!(len, a.as_ref().len());
    assert_eq!(a.as_ref(), b);
}

#[test]
fn test_empty() {
    let trie = ZeroTrieSimpleAscii::try_from_litemap(&LiteMap::new_vec()).unwrap();
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
    let trie = ZeroTrieSimpleAscii::try_from_litemap(&litemap.as_sliced()).unwrap();
    assert_eq!(trie.get(b""), Some(10));
    assert_eq!(trie.get(b"x"), None);
    let expected_bytes = &[0b10001010];
    assert_eq!(trie.as_bytes(), expected_bytes);

    let expected_bytes6 = &[0b10001010];
    let trie6 = ZeroTriePerfectHash::try_from_litemap(&litemap.to_borrowed_keys::<[u8], Vec<_>>()).unwrap();
    check_bytes_eq(1, trie6.as_bytes(), expected_bytes6);
    check_ascii_trie6(&litemap, &trie6);
}

#[test]
fn test_single_byte_string() {
    let litemap: LiteMap<&AsciiStr, usize> = [
        (AsciiStr::try_from_str("x").unwrap(), 10), //
    ]
    .into_iter()
    .collect();
    let trie = ZeroTrieSimpleAscii::try_from_litemap(&litemap.as_sliced()).unwrap();
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"xy"), None);
    check_ascii_trie(&litemap, &trie);
    let expected_bytes = &[b'x', 0b10001010];
    check_bytes_eq(2, trie.as_bytes(), expected_bytes);

    let expected_bytes6 = &[b'x', 0b10001010];
    let trie6 = ZeroTriePerfectHash::try_from_litemap(&litemap.to_borrowed_keys::<[u8], Vec<_>>()).unwrap();
    check_bytes_eq(2, trie6.as_bytes(), expected_bytes6);
    check_ascii_trie6(&litemap, &trie6);
}

#[test]
fn test_single_string() {
    let litemap: LiteMap<&AsciiStr, usize> = [
        (AsciiStr::try_from_str("xyz").unwrap(), 10), //
    ]
    .into_iter()
    .collect();
    let trie = ZeroTrieSimpleAscii::try_from_litemap(&litemap.as_sliced()).unwrap();
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"x"), None);
    assert_eq!(trie.get(b"xy"), None);
    assert_eq!(trie.get(b"xyzz"), None);
    check_ascii_trie(&litemap, &trie);
    let expected_bytes = &[b'x', b'y', b'z', 0b10001010];
    check_bytes_eq(4, trie.as_bytes(), expected_bytes);

    let expected_bytes6 = &[b'x', b'y', b'z', 0b10001010];
    let trie6 = ZeroTriePerfectHash::try_from_litemap(&litemap.to_borrowed_keys::<[u8], Vec<_>>()).unwrap();
    check_bytes_eq(4, trie6.as_bytes(), expected_bytes6);
    check_ascii_trie6(&litemap, &trie6);
}

#[test]
fn test_prefix_strings() {
    let litemap: LiteMap<&AsciiStr, usize> = [
        (AsciiStr::try_from_str("x").unwrap(), 0),
        (AsciiStr::try_from_str("xy").unwrap(), 1),
    ]
    .into_iter()
    .collect();
    let trie = ZeroTrieSimpleAscii::try_from_litemap(&litemap.as_sliced()).unwrap();
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"xyz"), None);
    check_ascii_trie(&litemap, &trie);
    let expected_bytes = &[b'x', 0b10000000, b'y', 0b10000001];
    check_bytes_eq(4, trie.as_bytes(), expected_bytes);

    let expected_bytes6 = &[b'x', 0b10000000, b'y', 0b10000001];
    let trie6 = ZeroTriePerfectHash::try_from_litemap(&litemap.to_borrowed_keys::<[u8], Vec<_>>()).unwrap();
    check_bytes_eq(4, trie6.as_bytes(), expected_bytes6);
    check_ascii_trie6(&litemap, &trie6);
}

#[test]
fn test_single_byte_branch() {
    let litemap: LiteMap<&AsciiStr, usize> = [
        (AsciiStr::try_from_str("x").unwrap(), 0),
        (AsciiStr::try_from_str("y").unwrap(), 1),
    ]
    .into_iter()
    .collect();
    let trie = ZeroTrieSimpleAscii::try_from_litemap(&litemap.as_sliced()).unwrap();
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"xy"), None);
    check_ascii_trie(&litemap, &trie);
    let expected_bytes = &[0b11000010, b'x', b'y', 1, 0b10000000, 0b10000001];
    check_bytes_eq(6, trie.as_bytes(), expected_bytes);

    let expected_bytes6 = &[0b11000010, b'x', b'y', 1, 0b10000000, 0b10000001];
    let trie6 = ZeroTriePerfectHash::try_from_litemap(&litemap.to_borrowed_keys::<[u8], Vec<_>>()).unwrap();
    check_bytes_eq(6, trie6.as_bytes(), expected_bytes6);
    check_ascii_trie6(&litemap, &trie6);
}

#[test]
fn test_multi_byte_branch() {
    let litemap: LiteMap<&AsciiStr, usize> = [
        (AsciiStr::try_from_str("axb").unwrap(), 0),
        (AsciiStr::try_from_str("ayc").unwrap(), 1),
    ]
    .into_iter()
    .collect();
    let trie = ZeroTrieSimpleAscii::try_from_litemap(&litemap.as_sliced()).unwrap();
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"a"), None);
    assert_eq!(trie.get(b"ax"), None);
    assert_eq!(trie.get(b"ay"), None);
    check_ascii_trie(&litemap, &trie);
    let expected_bytes = &[
        b'a', 0b11000010, b'x', b'y', 2, b'b', 0b10000000, b'c', 0b10000001,
    ];
    check_bytes_eq(9, trie.as_bytes(), expected_bytes);

    let expected_bytes6 = &[
        b'a', 0b11000010, b'x', b'y', 2, b'b', 0b10000000, b'c', 0b10000001,
    ];
    let trie6 = ZeroTriePerfectHash::try_from_litemap(&litemap.to_borrowed_keys::<[u8], Vec<_>>()).unwrap();
    check_bytes_eq(9, trie6.as_bytes(), expected_bytes6);
    check_ascii_trie6(&litemap, &trie6);
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
    let trie = ZeroTrieSimpleAscii::try_from_litemap(&litemap.as_sliced()).unwrap();
    assert_eq!(trie.get(b"xy"), None);
    assert_eq!(trie.get(b"xz"), None);
    assert_eq!(trie.get(b"xyzz"), None);
    check_ascii_trie(&litemap, &trie);
    let expected_bytes = &[0x90, 0x54, b'x', 0x93, 0x64, b'y', b'z', 0x90, 0x96, 0x78];
    check_bytes_eq(10, trie.as_bytes(), expected_bytes);

    let expected_bytes6 = &[0x90, 0x54, b'x', 0x93, 0x64, b'y', b'z', 0x90, 0x96, 0x78];
    let trie6 = ZeroTriePerfectHash::try_from_litemap(&litemap.to_borrowed_keys::<[u8], Vec<_>>()).unwrap();
    check_bytes_eq(10, trie6.as_bytes(), expected_bytes6);
    check_ascii_trie6(&litemap, &trie6);
}

#[test]
fn test_varint_branch() {
    let chars =
        AsciiStr::try_from_str("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz").unwrap();
    let litemap: LiteMap<&AsciiStr, usize> = (0..chars.len())
        .map(|i| (chars.substring(i..i + 1).unwrap(), i))
        .collect();
    let trie = ZeroTrieSimpleAscii::try_from_litemap(&litemap.as_sliced()).unwrap();
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
        1, 2, 3, 4, 5, 6, 7, 8, 9,
        10, 11, 12, 13, 14, 15, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52, 54, 56, 58, 60, 62, 64, 66, 68, 70, 72, 74, 76, 78, 80, 82, 84, 86,
        // single-byte values:
        (0x80 | 0), (0x80 | 1), (0x80 | 2), (0x80 | 3), (0x80 | 4),
        (0x80 | 5), (0x80 | 6), (0x80 | 7), (0x80 | 8), (0x80 | 9),
        (0x80 | 10), (0x80 | 11), (0x80 | 12), (0x80 | 13), (0x80 | 14),
        (0x80 | 15),
        // multi-byte values:
        0x90, 0,
        0x90, 17-16, 0x90, 18-16, 0x90, 19-16,
        0x90, 20-16, 0x90, 21-16, 0x90, 22-16, 0x90, 23-16, 0x90, 24-16,
        0x90, 25-16, 0x90, 26-16, 0x90, 27-16, 0x90, 28-16, 0x90, 29-16,
        0x90, 30-16, 0x90, 31-16,
        0x90, 16+0, 0x90, 16+1, 0x90, 16+2, 0x90, 16+3, 0x90, 16+4,
        0x90, 16+5, 0x90, 16+6, 0x90, 16+7, 0x90, 16+8, 0x90, 16+9,
        0x90, 16+10, 0x90, 16+11, 0x90, 16+12, 0x90, 16+13, 0x90, 16+14,
        0x90, 16+15, 0x90, 16+16, 0x90, 16+17, 0x90, 16+18, 0x90, 16+19,
    ];
    check_bytes_eq(193, trie.as_bytes(), expected_bytes);

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
    let trie6 = ZeroTriePerfectHash::try_from_litemap(&litemap.to_borrowed_keys::<[u8], Vec<_>>()).unwrap();
    check_bytes_eq(246, trie6.as_bytes(), expected_bytes6);
    check_ascii_trie6(&litemap, &trie6);
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
    let trie = ZeroTrieSimpleAscii::try_from_litemap(&litemap.as_sliced()).unwrap();
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"abc"), None);
    check_ascii_trie(&litemap, &trie);
    #[rustfmt::skip]
    let expected_bytes = &[
        0b11001010, // branch
        // search array:
        b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j',
        // offset array:
        26, 52, 78, 104, 130, 156, 182, 208, 234,
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
    check_bytes_eq(275, trie.as_bytes(), expected_bytes);
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
    let trie = ZeroTrieSimpleAscii::try_from_litemap(&litemap.as_sliced()).unwrap();
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"abc"), None);
    check_ascii_trie(&litemap, &trie);
    #[rustfmt::skip]
    let expected_bytes = &[
        0b11100001, // branch lead
        0x6A, // branch trail
        // search array:
        b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j',
        // offset array (wide):
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        26, 52, 78, 104, 130, 156, 182, 208, 234,
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
    check_bytes_eq(286, trie.as_bytes(), expected_bytes);
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
    let trie = ZeroTrieSimpleAscii::try_from_litemap(&litemap.as_sliced()).unwrap();
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"abc"), None);
    check_ascii_trie(&litemap, &trie);
    #[rustfmt::skip]
    let expected_bytes = &[
        0b11100001, // branch lead
        0x6A, // branch trail
        // search array:
        b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j',
        // offset array (wide):
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        26, 52, 78, 104, 130, 156, 182, 208, 234,
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
    check_bytes_eq(287, trie.as_bytes(), expected_bytes);
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
    let trie = ZeroTrieSimpleAscii::try_from_litemap(&litemap.as_sliced()).unwrap();
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
        13,         //
        0b11000011, // branch of 3
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
        b'x',       //
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
    check_bytes_eq(36, trie.as_bytes(), expected_bytes);

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
    let trie6 = ZeroTriePerfectHash::try_from_litemap(&litemap.to_borrowed_keys::<[u8], Vec<_>>()).unwrap();
    check_bytes_eq(36, trie6.as_bytes(), expected_bytes6);
    check_ascii_trie6(&litemap, &trie6);

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
    let trie6 = ZeroTriePerfectHash::try_from_litemap(&litemap).unwrap();
    check_bytes_eq(73, trie6.as_bytes(), expected_bytes6);
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
    let trie6 = ZeroTriePerfectHash::try_from_litemap(&litemap).unwrap();
    assert_eq!(trie6.byte_len(), 3042);
    check_ascii_trie6_bytes(&litemap, &trie6);
}

#[test]
fn test_short_subtags_10pct() {
    let litemap = strings_to_litemap(&testdata::short_subtags_10pct::STRINGS).unwrap();

    let trie = ZeroTrieSimpleAscii::try_from_litemap(&litemap).unwrap();
    assert_eq!(trie.byte_len(), 1050);
    check_ascii_trie(&litemap, &trie);

    let trie6 = ZeroTriePerfectHash::try_from_litemap(&litemap.to_borrowed_keys::<[u8], Vec<_>>()).unwrap();
    assert_eq!(trie6.byte_len(), 1100);
    check_ascii_trie6(&litemap, &trie6);

    let trie7b = asciitrie::make7b_litemap(&litemap).unwrap();
    check_bytes_eq(1050, trie.as_bytes(), &trie7b);

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

    let trie = ZeroTrieSimpleAscii::try_from_litemap(&litemap).unwrap();
    assert_eq!(trie.byte_len(), 8793);
    check_ascii_trie(&litemap, &trie);

    let trie6 = ZeroTriePerfectHash::try_from_litemap(&litemap.to_borrowed_keys::<[u8], Vec<_>>()).unwrap();
    assert_eq!(trie6.byte_len(), 9400);
    check_ascii_trie6(&litemap, &trie6);

    let trie7b = asciitrie::make7b_litemap(&litemap).unwrap();
    check_bytes_eq(8793, trie.as_bytes(), &trie7b);

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
