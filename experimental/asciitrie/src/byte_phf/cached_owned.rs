// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use crate::error::Error;
use alloc::vec::Vec;
use litemap::LiteMap;

pub struct PerfectByteHashMapCacheOwned {
    data: LiteMap<Vec<u8>, PerfectByteHashMap<Vec<u8>>>,
}

impl PerfectByteHashMapCacheOwned {
    pub const fn new_empty() -> Self {
        Self {
            data: LiteMap::new(),
        }
    }

    pub fn try_get_or_insert(&mut self, keys: Vec<u8>) -> Result<&PerfectByteHashMap<[u8]>, Error> {
        // TODO: Use the index returned by try_get_or_insert to speed up the second lookup
        self.data
            .try_get_or_insert(keys, |keys| PerfectByteHashMap::try_new(keys))
            .map(|p| p.1.as_borrowed())
    }

    pub fn get(&self, keys: &[u8]) -> Option<&PerfectByteHashMap<[u8]>> {
        self.data.get(keys).map(|p| p.as_borrowed())
    }
}
