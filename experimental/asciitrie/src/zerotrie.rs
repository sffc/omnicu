// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::reader::get as get1;
use crate::reader::get_iter as get_iter1;
use crate::reader6::get as get6;
use crate::reader6::get_iter as get_iter6;
use crate::AsciiStr;

use core::borrow::Borrow;
use ref_cast::RefCast;

pub struct ZeroTrie<S>(ZeroTrieInner<S>);

enum ZeroTrieInner<S> {
    SimpleAscii(ZeroTrieSimpleAscii<S>),
    PerfectHash(ZeroTriePerfectHash<S>),
    ExtendedCapacity(ZeroTrieExtendedCapacity<S>),
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, ref_cast::RefCast)]
pub struct ZeroTrieSimpleAscii<S: ?Sized> {
    pub(crate) store: S,
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, ref_cast::RefCast)]
pub struct ZeroTriePerfectHash<S: ?Sized> {
    pub(crate) store: S,
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, ref_cast::RefCast)]
pub struct ZeroTrieExtendedCapacity<S: ?Sized> {
    pub(crate) store: S,
}

macro_rules! impl_zerotrie_subtype {
    ($name:ident, $variant:ident, $getter_fn:path, $iter_ty:ty, $iter_fn:path) => {
        impl<S> $name<S> {
            pub fn into_zerotrie(self) -> ZeroTrie<S> {
                ZeroTrie(ZeroTrieInner::$variant(self))
            }
            pub fn from_store(store: S) -> Self {
                Self { store }
            }
            pub fn take_store(self) -> S {
                self.store
            }
        }
        impl<S> $name<S>
        where
            S: AsRef<[u8]> + ?Sized,
        {
            pub fn get(&self, key: &[u8]) -> Option<usize> {
                $getter_fn(self.store.as_ref(), key)
            }
            pub fn get_str(&self, key: &str) -> Option<usize> {
                self.get(key.as_bytes())
            }
            pub fn is_empty(&self) -> bool {
                self.store.as_ref().is_empty()
            }
            pub fn byte_len(&self) -> usize {
                self.store.as_ref().len()
            }
            pub fn as_bytes(&self) -> &[u8] {
                self.store.as_ref()
            }
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
            pub fn to_owned(&self) -> $name<alloc::vec::Vec<u8>> {
                $name {
                    store: alloc::vec::Vec::from(self.store.as_ref()),
                }
            }
            pub fn iter(&self) -> impl Iterator<Item = (alloc::boxed::Box<$iter_ty>, usize)> + '_ {
                 $iter_fn(self.as_bytes())
            }
        }
        impl $name<[u8]> {
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
    pub fn get(&self, key: &[u8]) -> Option<usize> {
        impl_dispatch!(self, get(key))
    }
    pub fn get_str(&self, key: &str) -> Option<usize> {
        impl_dispatch!(self, get_str(key))
    }
    pub fn is_empty(&self) -> bool {
        impl_dispatch!(self, is_empty)
    }
    pub fn byte_len(&self) -> usize {
        impl_dispatch!(self, byte_len)
    }
    pub fn as_bytes(&self) -> &[u8] {
        impl_dispatch!(self, as_bytes)
    }
}
