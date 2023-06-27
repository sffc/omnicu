// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A data structure offering zero-copy storage and retrieval of byte strings, with a focus
//! on the efficient storage of ASCII strings. Strings are mapped to a `usize` values.
//!
//! [`ZeroTrie`] does not support mutation because doing so would require recomputing the entire
//! data structure. Instead, it supports conversion to and from [`LiteMap`] and [`BTreeMap`].
//!
//! There are multiple variants of [`ZeroTrie`] optimized for different use cases.
//!
//! # Examples
//!
//! ```
//! use asciitrie::ZeroTrie;
//!
//! let data: &[(&[u8], usize)] = &[
//!     (b"abc", 11),
//!     (b"xyz", 22),
//!     (b"axyb", 33),
//! ];
//!
//! let trie: ZeroTrie<Vec<u8>> = data.iter().copied().collect();
//!
//! assert_eq!(trie.get("axyb"), Some(33));
//! assert_eq!(trie.byte_len(), 18);
//! ```
//!
//! [`LiteMap`]: litemap::LiteMap
//! [`BTreeMap`]: alloc::collections::BTreeMap

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod builder;
#[doc(hidden)]
pub mod byte_phf;
mod error;
mod reader;
#[cfg(feature = "serde")]
mod serde;
mod varint;
mod zerotrie;

pub use error::Error as AsciiTrieError;
pub use zerotrie::ZeroTrie;
pub use zerotrie::ZeroTrieExtendedCapacity;
pub use zerotrie::ZeroTriePerfectHash;
pub use zerotrie::ZeroTrieSimpleAscii;
