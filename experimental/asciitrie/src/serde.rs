// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::AsciiStr;
use crate::AsciiTrie;
use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use litemap::LiteMap;
use serde::de::Error;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;

#[cfg(feature = "zerovec")]
use zerovec::{ZeroSlice, ZeroVec};

impl<'de, 'data> Deserialize<'de> for &'data AsciiStr
where
    'de: 'data,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let s = <&str>::deserialize(deserializer)?;
            AsciiStr::try_from_str(s).map_err(|_| D::Error::custom("not an ASCII string"))
        } else {
            let s = <&[u8]>::deserialize(deserializer)?;
            AsciiStr::try_from_bytes(s).map_err(|_| D::Error::custom("not an ASCII string"))
        }
    }
}

impl<'data> Serialize for &'data AsciiStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            self.as_str().serialize(serializer)
        } else {
            self.as_bytes().serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for Box<AsciiStr> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let s = String::deserialize(deserializer)?;
            AsciiStr::try_from_boxed_str(s.into_boxed_str())
                .map_err(|_| D::Error::custom("not an ASCII string"))
        } else {
            let s = Vec::<u8>::deserialize(deserializer)?;
            AsciiStr::try_from_boxed_bytes(s.into_boxed_slice())
                .map_err(|_| D::Error::custom("not an ASCII string"))
        }
    }
}

impl Serialize for Box<AsciiStr> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            self.as_str().serialize(serializer)
        } else {
            self.as_bytes().serialize(serializer)
        }
    }
}

impl<'de, 'data> Deserialize<'de> for AsciiTrie<Cow<'data, [u8]>>
where
    'de: 'data,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let lm = LiteMap::<Box<AsciiStr>, usize>::deserialize(deserializer)?;
            let lm = lm.to_borrowed_keys::<_, Vec<_>>();
            let trie_vec = AsciiTrie::from_litemap(&lm);
            Ok(trie_vec.wrap_bytes_into_cow())
        } else {
            let bytes = <&[u8]>::deserialize(deserializer)?;
            let trie_slice = AsciiTrie::from_bytes(bytes);
            Ok(trie_slice.wrap_bytes_into_cow())
        }
    }
}

impl<'data> Serialize for AsciiTrie<Cow<'data, [u8]>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            let lm = self.to_litemap();
            lm.serialize(serializer)
        } else {
            let bytes = self.as_bytes();
            bytes.serialize(serializer)
        }
    }
}

#[cfg(feature = "zerovec")]
impl<'de, 'data> Deserialize<'de> for AsciiTrie<ZeroVec<'data, u8>>
where
    'de: 'data,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let lm = LiteMap::<Box<AsciiStr>, usize>::deserialize(deserializer)?;
            let lm = lm.to_borrowed_keys::<_, Vec<_>>();
            let trie_vec = AsciiTrie::from_litemap(&lm);
            let zv = ZeroVec::new_owned(trie_vec.0);
            Ok(AsciiTrie(zv))
        } else {
            let bytes = <&ZeroSlice<u8>>::deserialize(deserializer)?;
            let zv = bytes.as_zerovec();
            Ok(AsciiTrie(zv))
        }
    }
}

#[cfg(feature = "zerovec")]
impl<'data> Serialize for AsciiTrie<ZeroVec<'data, u8>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            let lm = self.to_litemap();
            lm.serialize(serializer)
        } else {
            let bytes = self.as_bytes();
            bytes.serialize(serializer)
        }
    }
}

#[cfg(test)]
mod testdata {
    use crate as asciitrie;
    include!("../tests/data.rs");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize)]
    pub struct AsciiTrieCow<'a> {
        #[serde(borrow)]
        trie: AsciiTrie<Cow<'a, [u8]>>,
    }

    #[test]
    pub fn test_serde_cow() {
        let trie = AsciiTrie::from_store(Cow::from(testdata::basic::TRIE));
        let original = AsciiTrieCow { trie };
        let json_str = serde_json::to_string(&original).unwrap();
        let bincode_bytes = bincode::serialize(&original).unwrap();

        assert_eq!(json_str, testdata::basic::JSON_STR);
        assert_eq!(bincode_bytes, testdata::basic::BINCODE_BYTES);

        let json_recovered: AsciiTrieCow = serde_json::from_str(&json_str).unwrap();
        let bincode_recovered: AsciiTrieCow = bincode::deserialize(&bincode_bytes).unwrap();

        assert_eq!(original.trie, json_recovered.trie);
        assert_eq!(original.trie, bincode_recovered.trie);

        assert!(matches!(json_recovered.trie.take_store(), Cow::Owned(_)));
        assert!(matches!(
            bincode_recovered.trie.take_store(),
            Cow::Borrowed(_)
        ));
    }
}

#[cfg(test)]
#[cfg(feature = "zerovec")]
mod tests_zerovec {
    use super::*;

    #[derive(Serialize, Deserialize)]
    pub struct AsciiTrieZeroVec<'a> {
        #[serde(borrow)]
        trie: AsciiTrie<ZeroVec<'a, u8>>,
    }

    #[test]
    pub fn test_serde_zerovec() {
        let trie = AsciiTrie::from_store(ZeroVec::new_borrowed(testdata::basic::TRIE));
        let original = AsciiTrieZeroVec { trie };
        let json_str = serde_json::to_string(&original).unwrap();
        let bincode_bytes = bincode::serialize(&original).unwrap();

        assert_eq!(json_str, testdata::basic::JSON_STR);
        assert_eq!(bincode_bytes, testdata::basic::BINCODE_BYTES);

        let json_recovered: AsciiTrieZeroVec = serde_json::from_str(&json_str).unwrap();
        let bincode_recovered: AsciiTrieZeroVec = bincode::deserialize(&bincode_bytes).unwrap();

        assert_eq!(original.trie, json_recovered.trie);
        assert_eq!(original.trie, bincode_recovered.trie);

        assert!(json_recovered.trie.take_store().is_owned());
        assert!(!bincode_recovered.trie.take_store().is_owned());
    }
}
