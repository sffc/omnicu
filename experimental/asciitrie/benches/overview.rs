// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use zerovec::ZeroMap;

mod testdata {
    include!("../tests/data.rs");
}

fn get_bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("get/basic");

    let trie = testdata::basic::TRIE;
    let data = testdata::basic::DATA;

    g.bench_function("AsciiTrie", |b| {
        b.iter(|| {
            for (key, expected) in black_box(data) {
                let actual = asciitrie::get(black_box(trie), key);
                assert_eq!(Some(*expected), actual);
            }
        });
    });

    g.bench_function("ZeroMap/usize", |b| {
        let zm: ZeroMap<[u8], usize> = data.iter().copied().collect();
        b.iter(|| {
            for (key, expected) in black_box(data) {
                let actual = black_box(&zm).get_copied(key);
                assert_eq!(Some(*expected), actual);
            }
        });
    });

    g.bench_function("ZeroMap/u8", |b| {
        let zm: ZeroMap<[u8], u8> = data.iter().map(|(k, v)| (*k, *v as u8)).collect();
        b.iter(|| {
            for (key, expected) in black_box(data) {
                let actual = black_box(&zm).get_copied(key);
                assert_eq!(Some(*expected as u8), actual);
            }
        });
    });

    g.bench_function("HashMap", |b| {
        let hm: HashMap<&[u8], usize> = data.iter().copied().collect();
        b.iter(|| {
            for (key, expected) in black_box(data) {
                let actual = black_box(&hm).get(key);
                assert_eq!(Some(expected), actual);
            }
        });
    });
}

criterion_group!(benches, get_bench,);
criterion_main!(benches);
