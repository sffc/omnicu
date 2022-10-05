// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub fn read_varint(start: u8, remainder: &[u8]) -> Option<(usize, &[u8])> {
    if (start & 0b00100000) != 0 {
        todo!()
    }
    Some(((start & 0b00011111) as usize, remainder))
}
