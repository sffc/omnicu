// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A data structure that efficiently stores and retrieves ASCII strings.
//!
//! Strings are mapped to a `usize` value.

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod alloc_impls;
mod builder;
pub mod reader;
pub mod reader2;
mod ref_cast;
#[cfg(feature = "serde")]
mod serde;
mod trie;
mod varint;
#[cfg(feature = "zerovec")]
mod zerovec;

pub use builder::make2_litemap;
pub use builder::AsciiStr;
pub use builder::NonAsciiError;
pub use trie::AsciiTrie;
