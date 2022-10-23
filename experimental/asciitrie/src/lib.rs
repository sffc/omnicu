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
#[cfg(feature = "builder")]
mod builder;
mod reader;
#[cfg(feature = "ref-cast")]
mod ref_cast;
mod trie;
mod varint;
#[cfg(feature = "zerovec")]
mod zerovec;

#[cfg(feature = "builder")]
pub use builder::AsciiStr;
#[cfg(feature = "builder")]
pub use builder::NonAsciiError;
pub use reader::get;
pub use trie::AsciiTrie;
