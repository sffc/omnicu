// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::reader6::get as get6;
use crate::reader6::get_iter as get_iter6;
use crate::reader7::get as get1;
use crate::reader7::get_iter as get_iter1;
use crate::AsciiStr;

use core::borrow::Borrow;
use ref_cast::RefCast;

#[cfg(feature = "alloc")]
use alloc::{boxed::Box, collections::VecDeque, vec::Vec};

/// A data structure that compactly maps from byte sequences to integers.
///
/// There are several variants of `ZeroTrie` which are very similar but are optimized
/// for different use cases:
///
/// - [`ZeroTrieSimpleAscii`] is the most compact structure. Very fast for small data.
///   Only stores ASCII-encoded strings. Can be const-constructed!
/// - [`ZeroTriePerfectHash`] is also compact, but it also supports arbitrary binary
///   strings. It also scales better to large data. Cannot be const-constructed.
/// - [`ZeroTrieExtendedCapacity`] can be used if more than 2^32 bytes are required.
pub struct ZeroTrie<S>(ZeroTrieInner<S>);

enum ZeroTrieInner<S> {
    SimpleAscii(ZeroTrieSimpleAscii<S>),
    PerfectHash(ZeroTriePerfectHash<S>),
    ExtendedCapacity(ZeroTrieExtendedCapacity<S>),
}

/// A data structure that compactly maps from ASCII strings to integers.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, ref_cast::RefCast)]
pub struct ZeroTrieSimpleAscii<S: ?Sized> {
    pub(crate) store: S,
}

/// A data structure that compactly maps from byte strings to integers.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, ref_cast::RefCast)]
pub struct ZeroTriePerfectHash<S: ?Sized> {
    pub(crate) store: S,
}

/// A data structure that maps from a large number of byte strings to integers.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, ref_cast::RefCast)]
pub struct ZeroTrieExtendedCapacity<S: ?Sized> {
    pub(crate) store: S,
}

macro_rules! impl_zerotrie_subtype {
    ($name:ident, $variant:ident, $getter_fn:path, $iter_ty:ty, $iter_fn:path) => {
        impl<S> $name<S> {
            /// Wrap this specific ZeroTrie variant into a ZeroTrie.
            pub const fn into_zerotrie(self) -> ZeroTrie<S> {
                ZeroTrie(ZeroTrieInner::$variant(self))
            }
            /// Create a trie directly from a store.
            ///
            /// If the store does not contain valid bytes, unexpected behavior may occur.
            pub const fn from_store(store: S) -> Self {
                Self { store }
            }
            /// Takes the byte store from this trie.
            pub fn take_store(self) -> S {
                self.store
            }
        }
        impl<S> $name<S>
        where
            S: AsRef<[u8]> + ?Sized,
        {
            /// Queries the trie for a byte string.
            pub fn get(&self, key: &[u8]) -> Option<usize> {
                $getter_fn(self.store.as_ref(), key)
            }
            /// Queries the trie for a UTF-8 string.
            pub fn get_str(&self, key: &str) -> Option<usize> {
                self.get(key.as_bytes())
            }
            /// Returns `true` if the trie is empty.
            pub fn is_empty(&self) -> bool {
                self.store.as_ref().is_empty()
            }
            /// Returns the size of the trie in number of bytes.
            ///
            /// To get the number of keys in the trie, use `.iter().count()`:
            ///
            /// ```
            #[doc = concat!("use asciitrie::", stringify!($name), ";")]
            ///
            /// // A trie with two values: "abc" and "abcdef"
            #[doc = concat!("let trie: &", stringify!($name), "<[u8]> = ", stringify!($name), "::from_bytes(b\"abc\\x80def\\x81\");")]
            ///
            /// assert_eq!(8, trie.byte_len());
            /// assert_eq!(2, trie.iter().count());
            /// ```
            pub fn byte_len(&self) -> usize {
                self.store.as_ref().len()
            }
            /// Returns the bytes contained in the underlying store.
            pub fn as_bytes(&self) -> &[u8] {
                self.store.as_ref()
            }
            /// Returns this trie as a reference transparent over a byte slice.
            pub fn as_borrowed(&self) -> &$name<[u8]> {
                $name::from_bytes(self.store.as_ref())
            }
        }
        #[cfg(feature = "alloc")]
        impl<S> $name<S>
        where
            S: AsRef<[u8]> + ?Sized,
        {
            /// Converts a possibly-borrowed $name to an owned one.
            ///
            /// ***Enable this impl with the `"alloc"` feature.***
            ///
            /// # Examples
            ///
            /// ```
            /// use std::borrow::Cow;
            #[doc = concat!("use asciitrie::", stringify!($name), ";")]
            ///
            #[doc = concat!("let trie: &", stringify!($name), "<[u8]> = ", stringify!($name), "::from_bytes(b\"abc\\x85\");")]
            #[doc = concat!("let owned: ", stringify!($name), "<Vec<u8>> = trie.to_owned();")]
            ///
            /// assert_eq!(trie.get(b"abc"), Some(5));
            /// assert_eq!(owned.get(b"abc"), Some(5));
            /// ```
            pub fn to_owned(&self) -> $name<Vec<u8>> {
                $name::from_store(
                    Vec::from(self.store.as_ref()),
                )
            }
            pub fn iter(&self) -> impl Iterator<Item = (Box<$iter_ty>, usize)> + '_ {
                 $iter_fn(self.as_bytes())
            }
        }
        impl $name<[u8]> {
            /// Casts from a byte slice to a reference to a trie with the same lifetime.
            ///
            /// If the bytes are not a valid trie, unexpected behavior may occur.
            pub fn from_bytes(trie: &[u8]) -> &Self {
                Self::ref_cast(trie)
            }
        }
        // Note: Can't generalize this impl due to the `core::borrow::Borrow` blanket impl.
        impl Borrow<$name<[u8]>> for $name<&[u8]> {
            fn borrow(&self) -> &$name<[u8]> {
                self.as_borrowed()
            }
        }
        // Note: Can't generalize this impl due to the `core::borrow::Borrow` blanket impl.
        #[cfg(feature = "alloc")]
        impl Borrow<$name<[u8]>> for $name<Box<[u8]>> {
            fn borrow(&self) -> &$name<[u8]> {
                self.as_borrowed()
            }
        }
        // Note: Can't generalize this impl due to the `core::borrow::Borrow` blanket impl.
        #[cfg(feature = "alloc")]
        impl Borrow<$name<[u8]>> for $name<Vec<u8>> {
            fn borrow(&self) -> &$name<[u8]> {
                self.as_borrowed()
            }
        }
        #[cfg(feature = "alloc")]
        impl alloc::borrow::ToOwned for $name<[u8]> {
            type Owned = $name<Box<[u8]>>;
            /// This impl allows [`$name`] to be used inside of a [`Cow`](std::borrow::Cow).
            ///
            /// Note that it is also possible to use `$name<ZeroVec<u8>>` for a similar result.
            ///
            /// ***Enable this impl with the `"alloc"` feature.***
            ///
            /// # Examples
            ///
            /// ```
            /// use std::borrow::Cow;
            #[doc = concat!("use asciitrie::", stringify!($name), ";")]
            ///
            #[doc = concat!("let trie: Cow<", stringify!($name), "<[u8]>> = Cow::Borrowed(", stringify!($name), "::from_bytes(b\"abc\\x85\"));")]
            /// assert_eq!(trie.get(b"abc"), Some(5));
            /// ```
            fn to_owned(&self) -> Self::Owned {
                let bytes: &[u8] = self.store.as_ref();
                $name::from_store(
                    Vec::from(bytes).into_boxed_slice(),
                )
            }
        }
        #[cfg(feature = "litemap")]
        impl<S> $name<S>
        where
            S: AsRef<[u8]> + ?Sized,
        {
            /// ***Enable this function with the `"litemap"` feature.***
            ///
            /// # Examples
            ///
            /// ```
            /// use asciitrie::AsciiStr;
            #[doc = concat!("use asciitrie::", stringify!($name), ";")]
            /// use litemap::LiteMap;
            ///
            #[doc = concat!("let trie = ", stringify!($name), "::from_bytes(b\"abc\\x81def\\x82\");")]
            /// let items = trie.to_litemap();
            ///
            /// assert_eq!(items.len(), 2);
            /// assert_eq!(items.get("abc".as_bytes()), Some(&1));
            /// assert_eq!(items.get("abcdef".as_bytes()), Some(&2));
            ///
            #[doc = concat!("let recovered_trie = ", stringify!($name), "::try_from_litemap(")]
            ///     &items.to_borrowed_keys::<_, Vec<_>>()
            /// ).unwrap();
            /// assert_eq!(trie.as_bytes(), recovered_trie.as_bytes());
            /// ```
            pub fn to_litemap(&self) -> litemap::LiteMap<Box<$iter_ty>, usize> {
                self.iter().collect()
            }
        }
        #[cfg(feature = "alloc")]
        impl<'a> FromIterator<(&'a AsciiStr, usize)> for $name<Vec<u8>> {
            /// ***Enable this function with the `"alloc"` feature.***
            ///
            /// ```
            /// use asciitrie::AsciiStr;
            #[doc = concat!("use asciitrie::", stringify!($name), ";")]
            ///
            #[doc = concat!("let trie: ", stringify!($name), "<Vec<u8>> = [")]
            ///     ("foo", 1),
            ///     ("bar", 2),
            ///     ("bazzoo", 3),
            ///     ("internationalization", 18),
            /// ]
            /// .into_iter()
            /// .map(AsciiStr::try_from_str_with_value)
            /// .collect::<Result<_, _>>()
            /// .unwrap();
            ///
            /// assert_eq!(trie.get(b"foo"), Some(1));
            /// assert_eq!(trie.get(b"bar"), Some(2));
            /// assert_eq!(trie.get(b"bazzoo"), Some(3));
            /// assert_eq!(trie.get(b"internationalization"), Some(18));
            /// assert_eq!(trie.get(b"unknown"), None);
            /// ```
            fn from_iter<T: IntoIterator<Item = (&'a AsciiStr, usize)>>(iter: T) -> Self {
                use crate::builder::nonconst::ZeroTrieBuilder;
                ZeroTrieBuilder::<VecDeque<u8>>::from_asciistr_iter(
                    iter,
                    Self::BUILDER_OPTIONS
                )
                .map(|s| Self {
                    store: s.to_bytes(),
                })
                .unwrap()
            }
        }
        #[cfg(feature = "alloc")]
        impl<'a> FromIterator<(&'a [u8], usize)> for $name<Vec<u8>> {
            /// ***Enable this function with the `"alloc"` feature.***
            ///
            /// ```
            #[doc = concat!("use asciitrie::", stringify!($name), ";")]
            ///
            #[doc = concat!("let trie: ", stringify!($name), "<Vec<u8>> = [")]
            ///     ("foo", 1),
            ///     ("bar", 2),
            ///     ("bazzoo", 3),
            ///     ("internationalization", 18),
            /// ]
            /// .into_iter()
            /// .map(|(s, x)| (s.as_bytes(), x))
            /// .collect();
            ///
            /// assert_eq!(trie.get(b"foo"), Some(1));
            /// assert_eq!(trie.get(b"bar"), Some(2));
            /// assert_eq!(trie.get(b"bazzoo"), Some(3));
            /// assert_eq!(trie.get(b"internationalization"), Some(18));
            /// assert_eq!(trie.get(b"unknown"), None);
            /// ```
            fn from_iter<T: IntoIterator<Item = (&'a [u8], usize)>>(iter: T) -> Self {
                use crate::builder::nonconst::ZeroTrieBuilder;
                ZeroTrieBuilder::<VecDeque<u8>>::from_bytes_iter(
                    iter,
                    Self::BUILDER_OPTIONS
                )
                .map(|s| Self {
                    store: s.to_bytes(),
                })
                .unwrap()
            }
        }
        // TODO(#2778): Auto-derive these impls based on the repr(transparent).
        // Safety: $name is repr(transparent) over S, a VarULE
        #[cfg(feature = "zerovec")]
        unsafe impl<S> zerovec::ule::VarULE for $name<S>
        where
            S: zerovec::ule::VarULE,
        {
            #[inline]
            fn validate_byte_slice(bytes: &[u8]) -> Result<(), zerovec::ZeroVecError> {
                S::validate_byte_slice(bytes)
            }
            #[inline]
            unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
                core::mem::transmute(S::from_byte_slice_unchecked(bytes))
            }
        }

    };
}

impl_zerotrie_subtype!(ZeroTrieSimpleAscii, SimpleAscii, get1, AsciiStr, get_iter1);
impl_zerotrie_subtype!(ZeroTriePerfectHash, PerfectHash, get6, [u8], get_iter6);
impl_zerotrie_subtype!(
    ZeroTrieExtendedCapacity,
    ExtendedCapacity,
    get6,
    [u8],
    get_iter6
);

macro_rules! impl_dispatch {
    ($self:ident, $inner_fn:ident) => {
        match &$self.0 {
            ZeroTrieInner::SimpleAscii(subtype) => subtype.$inner_fn(),
            ZeroTrieInner::PerfectHash(subtype) => subtype.$inner_fn(),
            ZeroTrieInner::ExtendedCapacity(subtype) => subtype.$inner_fn(),
        }
    };
    ($self:ident, $inner_fn:ident($arg:ident)) => {
        match &$self.0 {
            ZeroTrieInner::SimpleAscii(subtype) => subtype.$inner_fn($arg),
            ZeroTrieInner::PerfectHash(subtype) => subtype.$inner_fn($arg),
            ZeroTrieInner::ExtendedCapacity(subtype) => subtype.$inner_fn($arg),
        }
    };
}

impl<S> ZeroTrie<S>
where
    S: AsRef<[u8]>,
{
    /// Queries the trie for a byte string.
    pub fn get(&self, key: &[u8]) -> Option<usize> {
        impl_dispatch!(self, get(key))
    }
    /// Queries the trie for a UTF-8 string.
    pub fn get_str(&self, key: &str) -> Option<usize> {
        impl_dispatch!(self, get_str(key))
    }
    /// Returns `true` if the trie is empty.
    pub fn is_empty(&self) -> bool {
        impl_dispatch!(self, is_empty)
    }
    /// Returns the size of the trie in number of bytes.
    ///
    /// To get the number of keys in the trie, use `.iter().count()`.
    pub fn byte_len(&self) -> usize {
        impl_dispatch!(self, byte_len)
    }
    /// Returns the bytes contained in the underlying store.
    pub fn as_bytes(&self) -> &[u8] {
        impl_dispatch!(self, as_bytes)
    }
}
