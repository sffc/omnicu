// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use zerovec::ZeroVec;
use icu_provider::yoke::{self, *};

/// UAX 14 line break rule table.
#[derive(Debug, PartialEq, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Yokeable, ZeroCopyFrom)]
pub struct LineBreakRuleTable<'data> {
    #[serde(borrow)]
    table: ZeroVec<'data, i8>,
    property_count: u32,
    keep_rule: i8,
}

impl LineBreakRuleTable<'_> {
    #[inline]
    pub fn get_break_state(&self, left: u8, right: u8) -> i8 {
        self.table.get(((left as usize) - 1) * (self.property_count as usize) + (right as usize) - 1).unwrap_or_default()
    }

    #[inline]
    pub fn is_break(&self, left: u8, right: u8) -> bool {
        let rule = self.get_break_state(left, right);
        if rule == self.keep_rule {
            return false;
        }
        if rule >= 0 {
            // need additional next characters to get break rule.
            return false;
        }
        true
    }
}
