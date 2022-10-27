// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use asciitrie::AsciiStr;

const fn single_byte_value(x: u8) -> u8 {
    debug_assert!(x <= 0b00011111);
    x | 0b10000000
}

const fn single_byte_match(x: u8) -> u8 {
    debug_assert!(x <= 0b00011111);
    x | 0b11000000
}

pub mod basic {
    use super::*;
    pub const TRIE: &[u8] = &[
        b'a',
        b'b',
        single_byte_value(1),
        b'c',
        single_byte_value(2),
        // Begin Match Node
        single_byte_match(3),
        b'd',
        b'e',
        b'f',
        0,
        5,
        8,
        // End Match Node
        // subslice @ 0
        single_byte_value(3),
        b'g',
        b'h',
        b'i',
        single_byte_value(4),
        // subslice @ 5
        b'j',
        b'k',
        single_byte_value(5),
        // subslice @ 8
        // Begin Match Node
        single_byte_match(2),
        b'l',
        b'm',
        0,
        1,
        // End Match Node
        // subslice @ 0
        single_byte_value(6),
        // subslice @ 1
        b'n',
        single_byte_value(7),
    ];
    pub const DATA: &[(&AsciiStr, usize)] = &[
        (AsciiStr::from_str_or_panic("ab"), 1),
        (AsciiStr::from_str_or_panic("abc"), 2),
        (AsciiStr::from_str_or_panic("abcd"), 3),
        (AsciiStr::from_str_or_panic("abcdghi"), 4),
        (AsciiStr::from_str_or_panic("abcejk"), 5),
        (AsciiStr::from_str_or_panic("abcfl"), 6),
        (AsciiStr::from_str_or_panic("abcfmn"), 7),
    ];
}
