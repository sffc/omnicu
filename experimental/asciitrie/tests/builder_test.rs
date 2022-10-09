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
    let trie = AsciiTrie::from_litemap(&litemap.as_sliced());
    assert_eq!(trie.byte_len(), 38);
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
            13,         //
            0b11000011, // branch of 3
            b'x',       //
            b'y',       //
            b'z',       //
            0,          //
            2,          //
            4,          //
            b'b',       //
            0b10000001, // value 1
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
            7,          //
            0b11000010, // branch of 2
            b'g',       //
            b'h',       //
            0,          //
            1,          //
            0b10000101, // value 5
            0b10000110, // value 6
            0b10000111, // value 7
            b'k',       //
            b'l',       //
            0b10001000, // value 8
        ]
    );
}
