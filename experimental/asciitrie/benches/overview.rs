// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use asciitrie::AsciiTrie;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use zerovec::ZeroMap;

mod testdata {
    include!("../tests/data.rs");
}

fn get_basic_bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("get/basic");

    let trie = testdata::basic::TRIE;
    let trie2 = testdata::basic::TRIE2;
    let data = testdata::basic::DATA;

    g.bench_function("AsciiTrie", |b| {
        let trie = AsciiTrie::from_bytes(&trie);
        b.iter(|| {
            for (key, expected) in black_box(data) {
                let actual = black_box(&trie).get(key.as_bytes());
                assert_eq!(Some(*expected), actual);
            }
        });
    });

    g.bench_function("AsciiTrie2", |b| {
        b.iter(|| {
            for (key, expected) in black_box(data) {
                let actual = asciitrie::reader2::get(black_box(&trie2), key.as_bytes());
                assert_eq!(Some(*expected), actual);
            }
        });
    });

    g.bench_function("ZeroMap/usize", |b| {
        let zm: ZeroMap<[u8], usize> = data.iter().copied().collect();
        b.iter(|| {
            for (key, expected) in black_box(data) {
                let actual = black_box(&zm).get_copied(key.as_bytes());
                assert_eq!(Some(*expected), actual);
            }
        });
    });

    g.bench_function("ZeroMap/u8", |b| {
        let zm: ZeroMap<[u8], u8> = data.iter().map(|(k, v)| (*k, *v as u8)).collect();
        b.iter(|| {
            for (key, expected) in black_box(data) {
                let actual = black_box(&zm).get_copied(key.as_bytes());
                assert_eq!(Some(*expected as u8), actual);
            }
        });
    });

    g.bench_function("HashMap", |b| {
        let hm: HashMap<&[u8], usize> = data
            .iter()
            .copied()
            .map(|(a, b)| (a.as_bytes(), b))
            .collect();
        b.iter(|| {
            for (key, expected) in black_box(data) {
                let actual = black_box(&hm).get(key.as_bytes());
                assert_eq!(Some(expected), actual);
            }
        });
    });
}

fn get_subtags_bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("get/subtags");

    let strings = testdata::short_subtags_10pct::STRINGS;
    let litemap = testdata::strings_to_litemap(&strings).unwrap();

    g.bench_function("AsciiTrie", |b| {
        let trie = AsciiTrie::from_litemap(&litemap);
        b.iter(|| {
            for key in black_box(strings) {
                let actual = black_box(&trie).get(key.as_bytes());
                assert_eq!(Some(0), actual);
            }
        });
    });

    g.bench_function("AsciiTrie2", |b| {
        let trie2 = asciitrie::make2_litemap(&litemap);
        b.iter(|| {
            for key in black_box(strings) {
                let actual = asciitrie::reader2::get(black_box(&trie2), key.as_bytes());
                assert_eq!(Some(0), actual);
            }
        });
    });

    g.bench_function("ZeroMap/usize", |b| {
        let zm: ZeroMap<[u8], usize> = litemap.iter().map(|(a, b)| (a.as_bytes(), b)).collect();
        b.iter(|| {
            for key in black_box(strings) {
                let actual = black_box(&zm).get_copied(key.as_bytes());
                assert_eq!(Some(0), actual);
            }
        });
    });

    g.bench_function("ZeroMap/u8", |b| {
        let zm: ZeroMap<[u8], u8> = litemap.iter().map(|(k, v)| (*k, *v as u8)).collect();
        b.iter(|| {
            for key in black_box(strings) {
                let actual = black_box(&zm).get_copied(key.as_bytes());
                assert_eq!(Some(0), actual);
            }
        });
    });

    g.bench_function("HashMap", |b| {
        let hm: HashMap<&[u8], usize> = litemap
            .iter()
            .map(|(a, b)| (a.as_bytes(), *b))
            .collect();
        b.iter(|| {
            for key in black_box(strings) {
                let actual = black_box(&hm).get(key.as_bytes());
                assert_eq!(Some(&0), actual);
            }
        });
    });
}

criterion_group!(benches, get_basic_bench, get_subtags_bench);
criterion_main!(benches);
