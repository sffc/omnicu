// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A data structure that efficiently stores and retrieves ASCII strings.
//!
//! Strings are mapped to a `usize` value.

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod algorithms;
#[cfg(feature = "alloc")]
mod alloc_impls;
#[cfg(feature = "builder")]
mod builder;
mod trie;
mod varint;

#[cfg(feature = "builder")]
pub use builder::AsciiStr;
#[cfg(feature = "builder")]
pub use builder::NonAsciiError;
pub use algorithms::get;
pub use trie::AsciiTrie;
