// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Display)]
pub enum Error {
    /// Non-ASCII data was added to an ASCII-only collection.
    #[displaydoc("Non-ASCII cannot be added to an ASCII-only collection")]
    NonAsciiError,
    /// The collection reached its maximum supported capacity.
    #[displaydoc("Reached maximum capacity of collection")]
    CapacityExceeded,
    /// The const builder is unable to process the specified data.
    #[displaydoc("Const builder failed to run to completion: {0}")]
    ConstBuilder(&'static str),
    /// The keys provided to the builder were not in lexicographic order.
    #[displaydoc("The provided keys are not in order")]
    KeysOutOfOrder,
    #[displaydoc("Failed to solve the perfect hash function. This is rare! Please report your case to the ICU4X team.")]
    CouldNotSolvePerfectHash,
}
