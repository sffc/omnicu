// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use litemap::LiteMap;

pub struct PerfectByteHashMapCacheBorrowed<'a> {
    data: LiteMap<
        &'a [u8],
        &'a PerfectByteHashMap<[u8]>,
        &'a [(&'a [u8], &'a PerfectByteHashMap<[u8]>)],
    >,
}

impl<'a> PerfectByteHashMapCacheBorrowed<'a> {}
