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

// /// Modified example from https://serde.rs/deserialize-map.html
// struct AsciiTrieVisitor {
// }

// impl AsciiTrieVisitor {
//     fn new() -> Self {
//         Self {}
//     }
// }

// impl<'de> Visitor<'de> for AsciiTrieVisitor
// where
//     K: Deserialize<'de> + Ord,
//     V: Deserialize<'de>,
//     R: StoreMut<K, V>,
// {
//     type Value = AsciiTrie<Cow<'de, [u8]>>;

//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         formatter.write_str("a map produced by LiteMap")
//     }

//     fn visit_seq<S>(self, mut access: S) -> Result<Self::Value, S::Error>
//     where
//         S: SeqAccess<'de>,
//     {
//         let mut map = LiteMap::with_capacity(access.size_hint().unwrap_or(0));

//         // While there are entries remaining in the input, add them
//         // into our map.
//         while let Some((key, value)) = access.next_element()? {
//             // Try to append it at the end, hoping for a sorted map.
//             // If not sorted, insert as usual.
//             // This allows for arbitrary maps (e.g. from user JSON)
//             // to be deserialized into LiteMap
//             // without impacting performance in the case of deserializing
//             // a serialized map that came from another LiteMap
//             if let Some((key, value)) = map.try_append(key, value) {
//                 // Note: this effectively selection sorts the map,
//                 // which isn't efficient for large maps
//                 map.insert(key, value);
//             }
//         }

//         Ok(map)
//     }

//     fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
//     where
//         M: MapAccess<'de>,
//     {
//         let mut map = LiteMap::with_capacity(access.size_hint().unwrap_or(0));

//         // While there are entries remaining in the input, add them
//         // into our map.
//         while let Some((key, value)) = access.next_entry()? {
//             // Try to append it at the end, hoping for a sorted map.
//             // If not sorted, insert as usual.
//             // This allows for arbitrary maps (e.g. from user JSON)
//             // to be deserialized into LiteMap
//             // without impacting performance in the case of deserializing
//             // a serialized map that came from another LiteMap
//             if let Some((key, value)) = map.try_append(key, value) {
//                 // Note: this effectively selection sorts the map,
//                 // which isn't efficient for large maps
//                 map.insert(key, value);
//             }
//         }

//         Ok(map)
//     }
// }

impl<'de> Deserialize<'de> for &'de AsciiStr {
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

impl<'de> Deserialize<'de> for AsciiTrie<Cow<'de, [u8]>> {
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
