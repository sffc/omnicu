// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use zerovec::ZeroVec;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LineBreakPropertyTable<'data>(
    #[serde(borrow)]
    #[serde(deserialize_with = "deserialize_line_break_property_table")]
    ZeroVec<'data, u8>
);

fn deserialize_line_break_property_table<'de, 'a, D>(deserializer: D) -> Result<ZeroVec<'a, u8>, D::Error>
where
    D: serde::Deserializer<'de>,
    'de: 'a
{
    let zv: ZeroVec<u8> = serde::Deserialize::deserialize(deserializer)?;
    if zv.len() != 1024 * 128 {
        todo!()
    }
    Ok(zv)
}

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
