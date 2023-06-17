// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use asciitrie::AsciiTrie;
use postcard::ser_flavors::{AllocVec, Flavor};
use serde::Serialize;
use zerovec::ZeroMap;

mod testdata {
    include!("data.rs");
}

#[test]
fn test_basic() {
    let trie = testdata::basic::TRIE;
    let trie4 = testdata::basic::TRIE4;
    let trie5 = testdata::basic::TRIE5;
    let trie6 = testdata::basic::TRIE6;
    let data = testdata::basic::DATA;

    // Check that the getter works
    for (key, expected) in data {
        let actual = match AsciiTrie::from_bytes(trie).get(key.as_bytes()) {
            Some(v) => v,
            None => panic!("value should be in trie: {:?} => {}", key, expected),
        };
        assert_eq!(*expected, actual);
        let actual4 = match asciitrie::reader4::get(trie4, key.as_bytes()) {
            Some(v) => v,
            None => panic!("value should be in trie4: {:?} => {}", key, expected),
        };
        assert_eq!(*expected, actual4);
        let actual5 = match asciitrie::reader5::get(trie5, key.as_bytes()) {
            Some(v) => v,
            None => panic!("value should be in trie5: {:?} => {}", key, expected),
        };
        assert_eq!(*expected, actual5);
        let actual6 = match asciitrie::reader6::get(trie6, key.as_bytes()) {
            Some(v) => v,
            None => panic!("value should be in trie6: {:?} => {}", key, expected),
        };
        assert_eq!(*expected, actual6);
    }

    // Compare the size to a postcard ZeroMap
    let zm: ZeroMap<[u8], usize> = data.iter().copied().collect();
    let mut serializer = postcard::Serializer {
        output: AllocVec::new(),
    };
    Serialize::serialize(&zm, &mut serializer).unwrap();
    let zeromap_bytes = serializer
        .output
        .finalize()
        .expect("Failed to finalize serializer output");

    assert_eq!(28, trie.len());
    assert_eq!(61, zeromap_bytes.len());
}
