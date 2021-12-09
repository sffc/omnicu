// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider definitions for Segmenter.

mod property_table;
mod rule_table;

pub use property_table::LineBreakPropertyTable;
pub use rule_table::LineBreakRuleTable;

/// Data struct for UAX 14 line segmentation.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LineBreakDataV1<'data> {
    #[serde(borrow)]
    pub property_table: LineBreakPropertyTable<'data>,
    #[serde(borrow)]
    pub rule_table: LineBreakRuleTable<'data>,
}
