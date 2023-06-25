// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use asciitrie::ZeroTrieSimpleAscii;
use postcard::ser_flavors::{AllocVec, Flavor};
use serde::Serialize;
use zerovec::ZeroMap;

mod testdata {
    include!("data.rs");
}

#[test]
fn test_basic() {
    let trie = testdata::basic::TRIE_ASCII;
    let data = testdata::basic::DATA_ASCII;
    let trie_unicode = testdata::basic::TRIE_UNICODE;
    let data_unicode = testdata::basic::DATA_UNICODE;
    let trie_binary = testdata::basic::TRIE_BINARY;
    let data_binary = testdata::basic::DATA_BINARY;

    // Check that the getter works
    for (key, expected) in data {
        let actual = match ZeroTrieSimpleAscii::from_bytes(trie).get(key.as_bytes()) {
            Some(v) => v,
            None => panic!("value should be in trie: {:?} => {}", key, expected),
        };
        assert_eq!(*expected, actual);
        let actual6 = match asciitrie::reader6::get(trie, key.as_bytes()) {
            Some(v) => v,
            None => panic!("value should be in trie6: {:?} => {}", key, expected),
        };
        assert_eq!(*expected, actual6);
    }

    for (key, expected) in data_unicode {
        let actual_u6 = match asciitrie::reader6::get(trie_unicode, key) {
            Some(v) => v,
            None => panic!("value should be in trie6: {:?} => {}", key, expected),
        };
        assert_eq!(*expected, actual_u6);
    }

    for (key, expected) in data_binary {
        let actual_bin6 = match asciitrie::reader6::get(trie_binary, key) {
            Some(v) => v,
            None => panic!("value should be in trie6: {:?} => {}", key, expected),
        };
        assert_eq!(*expected, actual_bin6);
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

    assert_eq!(26, trie.len());
    assert_eq!(61, zeromap_bytes.len());
}
