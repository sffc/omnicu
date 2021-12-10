// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider definitions for Segmenter.

use icu_provider::yoke::{self, *};

mod property_table;
mod rule_table;

pub use property_table::LineBreakPropertyTable;
pub use rule_table::LineBreakRuleTable;

pub mod key {
    //! Resource keys for [`icu_decimal`](crate).
    use icu_provider::{resource_key, ResourceKey};

    /// Resource key: symbols used for basic decimal formatting.
    pub const LINE_BREAK_V1: ResourceKey = resource_key!(Segmenter, "line_break", 1);
}

/// Data struct for UAX 14 line segmentation.
#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LineBreakDataV1<'data> {
    #[serde(borrow)]
    pub property_table: LineBreakPropertyTable<'data>,
    #[serde(borrow)]
    pub rule_table: LineBreakRuleTable<'data>,
}
