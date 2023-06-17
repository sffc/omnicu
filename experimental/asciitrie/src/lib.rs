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
pub mod byte_phf;
mod error;
pub mod reader;
pub mod reader4;
pub mod reader5;
pub mod reader6;
mod ref_cast;
#[cfg(feature = "serde")]
mod serde;
mod trie;
mod varinta;
mod varintx;
mod zerotrie;
#[cfg(feature = "zerovec")]
mod zerovec;

pub(crate) use varinta as varint;

pub use builder::AsciiStr;
pub use builder::NonAsciiError;
#[cfg(feature = "litemap")]
pub use builder::{
    make1b_litemap, make1b_slice,
    make4_litemap, make4_slice, make5_litemap, make5_slice, make6_byte_litemap, make6_byte_slice,
    make6_litemap, make6_slice,
};
pub use error::Error as AsciiTrieError;
pub use trie::AsciiTrie;

pub use zerotrie::ZeroTrie;
pub use zerotrie::ZeroTrieSimpleAscii;
pub use zerotrie::ZeroTriePerfectHash;
pub use zerotrie::ZeroTrieExtendedCapacity;
