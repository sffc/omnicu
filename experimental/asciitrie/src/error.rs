// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    /// Non-ASCII data was added to an ASCII-only collection.
    NonAsciiError,
    /// The collection reached its maximum supported capacity.
    CapacityExceeded,
    /// The const builder is unable to process the specified data.
    ConstBuilder(&'static str),
    /// The keys provided to the builder were not in lexicographic order.
    KeysOutOfOrder,
    CouldNotSolvePerfectHash,
}
