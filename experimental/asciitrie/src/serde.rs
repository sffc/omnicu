// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::AsciiStr;
use crate::ZeroTrieSimpleAscii;
use crate::ZeroTriePerfectHash;
use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use litemap::LiteMap;
use serde::de::Error;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;

#[cfg(feature = "zerovec")]
use zerovec::ZeroVec;

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

/// To ensure that we use `deserialize_bytes` instead of `deserialize_seq`,
/// use a helper struct for deserializing the bytes.
/// <https://github.com/serde-rs/serde/issues/309>
struct BytesVisitor;
impl<'de> Visitor<'de> for BytesVisitor {
    type Value = &'de [u8];
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a slice of borrowed bytes")
    }
    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E> {
        Ok(v)
    }
}

impl<'de, 'data> Deserialize<'de> for ZeroTrieSimpleAscii<Cow<'data, [u8]>>
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
            let trie_vec = crate::builder::make1b_litemap(&lm);
            let cow = Cow::Owned(trie_vec);
            Ok(ZeroTrieSimpleAscii::from_store(cow))
        } else {
            let bytes = deserializer.deserialize_bytes(BytesVisitor)?;
            let cow = Cow::Borrowed(bytes);
            Ok(ZeroTrieSimpleAscii::from_store(cow))
        }
    }
}

impl<'data> Serialize for ZeroTrieSimpleAscii<Cow<'data, [u8]>> {
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
impl<'de, 'data> Deserialize<'de> for ZeroTrieSimpleAscii<ZeroVec<'data, u8>>
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
            let trie_vec = ZeroTrieSimpleAscii::from_litemap(&lm);
            let zv = ZeroVec::new_owned(trie_vec.store);
            Ok(ZeroTrieSimpleAscii::from_store(zv))
        } else {
            let bytes = deserializer.deserialize_bytes(BytesVisitor)?;
            let zv = ZeroVec::new_borrowed(bytes);
            Ok(ZeroTrieSimpleAscii::from_store(zv))
        }
    }
}

#[cfg(feature = "zerovec")]
impl<'data> Serialize for ZeroTrieSimpleAscii<ZeroVec<'data, u8>> {
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

impl<'de, 'data> Deserialize<'de> for ZeroTriePerfectHash<Cow<'data, [u8]>>
where
    'de: 'data,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let lm = LiteMap::<Box<[u8]>, usize>::deserialize(deserializer)?;
            let lm = lm.to_borrowed_keys::<_, Vec<_>>();
            let trie_vec = crate::builder::make6_byte_litemap(&lm).map_err(|e| D::Error::custom(e))?;
            let cow = Cow::Owned(trie_vec);
            Ok(ZeroTriePerfectHash::from_store(cow))
        } else {
            let bytes = deserializer.deserialize_bytes(BytesVisitor)?;
            let cow = Cow::Borrowed(bytes);
            Ok(ZeroTriePerfectHash::from_store(cow))
        }
    }
}

impl<'data> Serialize for ZeroTriePerfectHash<Cow<'data, [u8]>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            let lm = self.to_litemap();
            if let Ok(lm2) = lm.iter().map(|(k, v)| {
                match AsciiStr::try_from_bytes(k) {
                    Ok(k2) => Ok((k2, v)),
                    Err(e) => Err(e)
                }
            }).collect::<Result<LiteMap<_, _>, _>>() {
                lm2.serialize(serializer)
            } else {
                lm.serialize(serializer)
            }
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
        // FIXME: Test doesn't pass yet
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
}

#[cfg(test)]
#[cfg(feature = "zerovec")]
mod tests_zerovec {
    use super::*;

    #[derive(Serialize, Deserialize)]
    pub struct ZeroTrieSimpleAsciiZeroVec<'a> {
        #[serde(borrow)]
        trie: ZeroTrieSimpleAscii<ZeroVec<'a, u8>>,
    }

    #[test]
    pub fn test_serde_zerovec() {
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
}
