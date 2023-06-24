// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::AsciiByte;

#[derive(Debug, Clone, Copy)]
pub(crate) struct BranchMeta {
    pub ascii: u8,
    pub length: usize,
    pub local_length: usize,
    pub count: usize,
}

impl BranchMeta {
    pub const fn const_default() -> Self {
        BranchMeta {
            ascii: AsciiByte::nul().get(),
            length: 0,
            local_length: 0,
            count: 0,
        }
    }
}
