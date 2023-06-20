// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::AsciiStr;
use crate::ZeroTriePerfectHash;
use crate::ZeroTrieSimpleAscii;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt;
use litemap::LiteMap;
use serde::de::Error;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum BytesOrStr<'a> {
    Borrowed(&'a [u8]),
    Owned(Box<[u8]>),
}

impl BytesOrStr<'_> {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Borrowed(s) => &s,
            Self::Owned(s) => &s,
        }
    }
}

impl Borrow<[u8]> for BytesOrStr<'_> {
    fn borrow(&self) -> &[u8] {
        self.as_bytes()
    }
}

struct BytesOrStrVisitor;
impl<'de> Visitor<'de> for BytesOrStrVisitor {
    type Value = Box<[u8]>;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a slice of borrowed bytes or a string")
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> {
        Ok(Box::from(v))
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> {
        Ok(Box::from(v.as_bytes()))
    }
    fn visit_seq<A>(self, mut v: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut result = Vec::with_capacity(v.size_hint().unwrap_or(0));
        while let Some(x) = v.next_element::<u8>()? {
            result.push(x);
        }
        Ok(Box::from(result))
    }
}

impl<'de, 'data> Deserialize<'de> for BytesOrStr<'data>
where
    'de: 'data,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let s = deserializer.deserialize_any(BytesOrStrVisitor)?;
            Ok(BytesOrStr::Owned(s))
        } else {
            let s = <&'data [u8]>::deserialize(deserializer)?;
            Ok(BytesOrStr::Borrowed(s))
        }
    }
}

impl<'data> Serialize for BytesOrStr<'data> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self.as_bytes();
        if serializer.is_human_readable() {
            match core::str::from_utf8(bytes) {
                Ok(s) => serializer.serialize_str(s),
                Err(_) => serializer.serialize_bytes(bytes),
            }
        } else {
            serializer.serialize_bytes(bytes)
        }
    }
}

impl<'de, 'data, X> Deserialize<'de> for ZeroTrieSimpleAscii<X>
where
    'de: 'data,
    // DISCUSS: There are several possibilities for the bounds here that would
    // get the job done. I could look for Deserialize, but this would require
    // creating a custom Deserializer for the map case. I also considered
    // introducing a new trait instead of relying on From.
    X: From<&'data [u8]> + From<Vec<u8>> + 'data,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let store = if deserializer.is_human_readable() {
            let lm = LiteMap::<Box<AsciiStr>, usize>::deserialize(deserializer)?;
            let lm = lm.to_borrowed_keys::<_, Vec<_>>();
            crate::builder::make1b_litemap(&lm)
                .map_err(|e| D::Error::custom(e))?
                .into()
        } else {
            // Note: `impl Deserialize for &[u8]` uses visit_borrowed_bytes
            <&[u8]>::deserialize(deserializer)?.into()
        };
        Ok(ZeroTrieSimpleAscii::from_store(store))
    }
}

impl<'data, X> Serialize for ZeroTrieSimpleAscii<X>
where
    X: AsRef<[u8]>,
{
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

impl<'de, 'data, X> Deserialize<'de> for ZeroTriePerfectHash<X>
where
    'de: 'data,
    X: From<&'data [u8]> + From<Vec<u8>> + 'data,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let store = if deserializer.is_human_readable() {
            let lm = LiteMap::<BytesOrStr, usize>::deserialize(deserializer)?;
            let lm = lm.to_borrowed_keys::<_, Vec<_>>();
            crate::builder::make6_byte_litemap(&lm)
                .map_err(|e| D::Error::custom(e))?
                .into()
        } else {
            // Note: `impl Deserialize for &[u8]` uses visit_borrowed_bytes
            <&[u8]>::deserialize(deserializer)?.into()
        };
        Ok(ZeroTriePerfectHash::from_store(store))
    }
}

impl<'data, X> Serialize for ZeroTriePerfectHash<X>
where
    X: AsRef<[u8]>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            let lm = self.to_litemap();
            let lm = lm
                .iter()
                .map(|(k, v)| (BytesOrStr::Borrowed(k), v))
                .collect::<LiteMap<_, _>>();
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
    use alloc::borrow::Cow;

    #[derive(Serialize, Deserialize)]
    pub struct ZeroTrieSimpleAsciiCow<'a> {
        #[serde(borrow)]
        trie: ZeroTrieSimpleAscii<Cow<'a, [u8]>>,
    }

    #[test]
    pub fn test_serde_simpleascii_cow() {
        let trie = ZeroTrieSimpleAscii::from_store(Cow::from(testdata::basic::TRIE));
        let original = ZeroTrieSimpleAsciiCow { trie };
        let json_str = serde_json::to_string(&original).unwrap();
        let bincode_bytes = bincode::serialize(&original).unwrap();

        assert_eq!(json_str, testdata::basic::JSON_STR);
        assert_eq!(bincode_bytes, testdata::basic::BINCODE_BYTES);

        let json_recovered: ZeroTrieSimpleAsciiCow = serde_json::from_str(&json_str).unwrap();
        let bincode_recovered: ZeroTrieSimpleAsciiCow =
            bincode::deserialize(&bincode_bytes).unwrap();

        assert_eq!(original.trie, json_recovered.trie);
        assert_eq!(original.trie, bincode_recovered.trie);

        assert!(matches!(json_recovered.trie.take_store(), Cow::Owned(_)));
        assert!(matches!(
            bincode_recovered.trie.take_store(),
            Cow::Borrowed(_)
        ));
    }

    #[derive(Serialize, Deserialize)]
    pub struct ZeroTriePerfectHashCow<'a> {
        #[serde(borrow)]
        trie: ZeroTriePerfectHash<Cow<'a, [u8]>>,
    }

    #[test]
    pub fn test_serde_perfecthash_cow() {
        let trie = ZeroTriePerfectHash::from_store(Cow::from(testdata::basic::TRIE6));
        let original = ZeroTriePerfectHashCow { trie };
        let json_str = serde_json::to_string(&original).unwrap();
        let bincode_bytes = bincode::serialize(&original).unwrap();

        assert_eq!(json_str, testdata::basic::JSON_STR);
        assert_eq!(bincode_bytes, testdata::basic::BINCODE_BYTES6);

        let json_recovered: ZeroTriePerfectHashCow = serde_json::from_str(&json_str).unwrap();
        let bincode_recovered: ZeroTriePerfectHashCow =
            bincode::deserialize(&bincode_bytes).unwrap();

        assert_eq!(original.trie, json_recovered.trie);
        assert_eq!(original.trie, bincode_recovered.trie);

        assert!(matches!(json_recovered.trie.take_store(), Cow::Owned(_)));
        assert!(matches!(
            bincode_recovered.trie.take_store(),
            Cow::Borrowed(_)
        ));
    }

    #[test]
    pub fn test_serde_perfecthash_cow_u() {
        let trie = ZeroTriePerfectHash::from_store(Cow::from(testdata::basic::TRIE_U6));
        let original = ZeroTriePerfectHashCow { trie };
        let json_str = serde_json::to_string(&original).unwrap();
        let bincode_bytes = bincode::serialize(&original).unwrap();

        assert_eq!(json_str, testdata::basic::JSON_STR_U);
        assert_eq!(bincode_bytes, testdata::basic::BINCODE_BYTES_U6);

        let json_recovered: ZeroTriePerfectHashCow = serde_json::from_str(&json_str).unwrap();
        let bincode_recovered: ZeroTriePerfectHashCow =
            bincode::deserialize(&bincode_bytes).unwrap();

        assert_eq!(original.trie, json_recovered.trie);
        assert_eq!(original.trie, bincode_recovered.trie);

        assert!(matches!(json_recovered.trie.take_store(), Cow::Owned(_)));
        assert!(matches!(
            bincode_recovered.trie.take_store(),
            Cow::Borrowed(_)
        ));
    }

    #[test]
    pub fn test_serde_perfecthash_cow_bin() {
        let trie = ZeroTriePerfectHash::from_store(Cow::from(testdata::basic::TRIE_BIN6));
        let original = ZeroTriePerfectHashCow { trie };
        let json_str = serde_json::to_string(&original).unwrap();
        let bincode_bytes = bincode::serialize(&original).unwrap();

        assert_eq!(json_str, testdata::basic::JSON_STR_BIN);
        assert_eq!(bincode_bytes, testdata::basic::BINCODE_BYTES_BIN6);

        let json_recovered: ZeroTriePerfectHashCow = serde_json::from_str(&json_str).unwrap();
        let bincode_recovered: ZeroTriePerfectHashCow =
            bincode::deserialize(&bincode_bytes).unwrap();

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
    use zerovec::ZeroVec;

    #[derive(Serialize, Deserialize)]
    pub struct ZeroTrieSimpleAsciiZeroVec<'a> {
        #[serde(borrow)]
        trie: ZeroTrieSimpleAscii<ZeroVec<'a, u8>>,
    }

    #[test]
    pub fn test_serde_simpleascii_zerovec() {
        let trie = ZeroTrieSimpleAscii::from_store(ZeroVec::new_borrowed(testdata::basic::TRIE));
        let original = ZeroTrieSimpleAsciiZeroVec { trie };
        let json_str = serde_json::to_string(&original).unwrap();
        let bincode_bytes = bincode::serialize(&original).unwrap();

        assert_eq!(json_str, testdata::basic::JSON_STR);
        assert_eq!(bincode_bytes, testdata::basic::BINCODE_BYTES);

        let json_recovered: ZeroTrieSimpleAsciiZeroVec = serde_json::from_str(&json_str).unwrap();
        let bincode_recovered: ZeroTrieSimpleAsciiZeroVec =
            bincode::deserialize(&bincode_bytes).unwrap();

        assert_eq!(original.trie, json_recovered.trie);
        assert_eq!(original.trie, bincode_recovered.trie);

        assert!(json_recovered.trie.take_store().is_owned());
        assert!(!bincode_recovered.trie.take_store().is_owned());
    }

    #[derive(Serialize, Deserialize)]
    pub struct ZeroTriePerfectHashZeroVec<'a> {
        #[serde(borrow)]
        trie: ZeroTriePerfectHash<ZeroVec<'a, u8>>,
    }

    #[test]
    pub fn test_serde_perfecthash_zerovec() {
        let trie = ZeroTriePerfectHash::from_store(ZeroVec::new_borrowed(testdata::basic::TRIE6));
        let original = ZeroTriePerfectHashZeroVec { trie };
        let json_str = serde_json::to_string(&original).unwrap();
        let bincode_bytes = bincode::serialize(&original).unwrap();

        assert_eq!(json_str, testdata::basic::JSON_STR);
        assert_eq!(bincode_bytes, testdata::basic::BINCODE_BYTES6);

        let json_recovered: ZeroTriePerfectHashZeroVec = serde_json::from_str(&json_str).unwrap();
        let bincode_recovered: ZeroTriePerfectHashZeroVec =
            bincode::deserialize(&bincode_bytes).unwrap();

        assert_eq!(original.trie, json_recovered.trie);
        assert_eq!(original.trie, bincode_recovered.trie);

        assert!(json_recovered.trie.take_store().is_owned());
        assert!(!bincode_recovered.trie.take_store().is_owned());
    }
}
