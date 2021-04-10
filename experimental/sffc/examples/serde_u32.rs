#![no_main]

use tinystr::*;
use std::borrow::Cow;

icu_benchmark_macros::static_setup!();

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, PartialEq)]
struct U32Array<'s> {
    #[serde(borrow)]
    arr: Cow<'s, [u32]>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, PartialEq)]
struct TinyStrArray<'s> {
    #[serde(borrow)]
    arr: Cow<'s, [TinyStr4]>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, PartialEq)]
struct FloatArray<'s> {
    #[serde(borrow, with = "floats")]
    arr: &'s [f64],
}

#[derive(rkyv::Archive)]
struct RkyvU32 {
    arr: Vec<u32>
}

mod floats {
    use serde::{de, Serializer, Deserialize, Deserializer};
    use pod::Pod;

    pub fn serialize<S>(data: &[f64], serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let bytes = Pod::map_slice(data).unwrap();
        serializer.serialize_bytes(bytes)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<&'de [f64], D::Error>
        where D: Deserializer<'de>
    {
        let bytes = <&[u8]>::deserialize(deserializer)?;
        Pod::map_slice(bytes).ok_or_else(|| de::Error::custom("not a multiple of 8 bytes"))
    }
}

#[repr(align(8))]
struct Aligned<T>(pub T);

const BINCODE_DATA: Aligned<[u8; 48]> = Aligned([40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 240, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 225, 123, 7, 65, 0, 0, 0, 0, 0, 0, 244, 191, 0, 0, 0, 232, 118, 72, 39, 66]);

fn generate() {
    let floats: [f64; 5] = [
        1.0,
        0.0,
        192380.125,
        -1.25,
        5e10,
    ];
    let floats_struct = FloatArray {
        arr: &floats,
    };
    let buf = bincode::serialize(&floats_struct).unwrap();
    println!("{:?}", buf);
}

fn parse() {
    let floats_struct: FloatArray = bincode::deserialize(&BINCODE_DATA.0).unwrap();
    println!("{:?}", floats_struct);
}

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    icu_benchmark_macros::main_setup!();

    // parse();

    0
}
