// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use asciitrie::AsciiStr;
use asciitrie::AsciiTrie;

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
