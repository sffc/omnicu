// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use zerovec::ZeroVec;

/// UAX 14 line break rule table.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LineBreakRuleTable<'data>(
    #[serde(borrow)]
    ZeroVec<'data, i8>
);

impl LineBreakRuleTable<'_> {
    #[inline]
    fn get_break_state_from_table(&self, property_count: usize, left: u8, right: u8) -> i8 {
        self.0.get(((left as usize) - 1) * property_count + (right as usize) - 1).unwrap_or_default()
    }
}
