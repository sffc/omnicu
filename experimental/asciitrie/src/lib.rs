// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A data structure that efficiently stores and retrieves ASCII strings.
//!
//! Strings are mapped to a `usize` value.

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod builder;
#[doc(hidden)]
pub mod byte_phf;
mod error;
mod reader6;
mod reader7;
#[cfg(feature = "serde")]
mod serde;
mod varinta;
mod zerotrie;

pub(crate) use varinta as varint;

pub use builder::AsciiStr;
pub use builder::NonAsciiError;
pub use error::Error as AsciiTrieError;
pub use zerotrie::ZeroTrie;
pub use zerotrie::ZeroTrieExtendedCapacity;
pub use zerotrie::ZeroTriePerfectHash;
pub use zerotrie::ZeroTrieSimpleAscii;
