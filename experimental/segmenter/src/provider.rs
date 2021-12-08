// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use zerovec::ZeroVec;

// TODO: Custom deserialize impl that checks that the length equals 1024*128
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LineBreakPropertyTable<'data>(#[serde(borrow)] ZeroVec<'data, u8>);

impl LineBreakPropertyTable<'_> {
    #[inline]
    pub fn get_prop_for_code_point(&self, codepoint: u32) -> u8 {
        let index = (codepoint / 1024 * 128) + (codepoint & 0x3ff);
        self.0.get(index as usize).unwrap_or(0)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LineBreakDataV1<'data> {
    #[serde(borrow)]
    table: LineBreakPropertyTable<'data>,
}
