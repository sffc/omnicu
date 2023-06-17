// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use asciitrie::AsciiTrie;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use litemap::LiteMap;
use std::collections::HashMap;
use zerovec::ZeroHashMap;
use zerovec::ZeroMap;

mod testdata {
    include!("../tests/data.rs");
}

fn get_basic_bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("get/basic");

    let trie = testdata::basic::TRIE;
    let trie4 = testdata::basic::TRIE4;
    let trie5 = testdata::basic::TRIE5;
    let trie6 = testdata::basic::TRIE6;
    let data = testdata::basic::DATA;

    g.bench_function("AsciiTrie1", |b| {
        let trie = AsciiTrie::from_bytes(&trie);
        b.iter(|| {
            for (key, expected) in black_box(data) {
                let actual = black_box(&trie).get(key.as_bytes());
                assert_eq!(Some(*expected), actual);
            }
        });
    });

    g.bench_function("AsciiTrie4", |b| {
        b.iter(|| {
            for (key, expected) in black_box(data) {
                let actual = asciitrie::reader4::get(black_box(&trie4), key.as_bytes());
                assert_eq!(Some(*expected), actual);
            }
        });
    });

    g.bench_function("AsciiTrie5", |b| {
        b.iter(|| {
            for (key, expected) in black_box(data) {
                let actual = asciitrie::reader5::get(black_box(&trie5), key.as_bytes());
                assert_eq!(Some(*expected), actual);
            }
        });
    });

    g.bench_function("AsciiTrie6", |b| {
        b.iter(|| {
            for (key, expected) in black_box(data) {
                let actual = asciitrie::reader6::get(black_box(&trie6), key.as_bytes());
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

    g.bench_function("ZeroHashMap/usize", |b| {
        let zhm: ZeroHashMap<[u8], usize> = data
            .iter()
            .copied()
            .collect();
        b.iter(|| {
            for (key, expected) in black_box(data) {
                let actual = black_box(&zhm).get(key.as_bytes());
                // No get_copied on ZHM so we need to do it manually
                let actual = actual.map(|x| <zerovec::vecs::FlexZeroSlice as zerovec::maps::ZeroVecLike<usize>>::zvl_get_as_t(x, |y| *y));
                assert_eq!(Some(*expected as usize), actual);
            }
        });
    });

    g.bench_function("ZeroHashMap/u8", |b| {
        let zhm: ZeroHashMap<[u8], u8> = data.iter().map(|(k, v)| (*k, *v as u8)).collect();
        b.iter(|| {
            for (key, expected) in black_box(data) {
                let actual = black_box(&zhm).get(key.as_bytes()).copied();
                assert_eq!(Some(*expected as u8), actual);
            }
        });
    });
}

fn get_subtags_bench_medium(c: &mut Criterion) {
    let g = c.benchmark_group("get/subtags_10pct");

    let strings = testdata::short_subtags_10pct::STRINGS;
    let litemap = testdata::strings_to_litemap(&strings).unwrap();

    get_subtags_bench_helper(g, strings, litemap);
}

fn get_subtags_bench_large(c: &mut Criterion) {
    let g = c.benchmark_group("get/subtags_full");

    let strings = testdata::short_subtags::STRINGS;
    let litemap = testdata::strings_to_litemap(&strings).unwrap();

    get_subtags_bench_helper(g, strings, litemap);
}

fn get_subtags_bench_helper<M: criterion::measurement::Measurement>(
    mut g: criterion::BenchmarkGroup<M>,
    strings: &[&str],
    litemap: LiteMap<&asciitrie::AsciiStr, usize>,
) {
    g.bench_function("AsciiTrie1", |b| {
        let trie1b = asciitrie::make1b_litemap(&litemap);
        let trie = AsciiTrie::from_bytes(&trie1b);
        b.iter(|| {
            for (i, key) in black_box(strings).iter().enumerate() {
                let actual = black_box(&trie).get(key.as_bytes());
                assert_eq!(Some(i), actual);
            }
        });
    });

    g.bench_function("AsciiTrie4", |b| {
        let trie4 = asciitrie::make4_litemap(&litemap);
        b.iter(|| {
            for (i, key) in black_box(strings).iter().enumerate() {
                let actual = asciitrie::reader4::get(black_box(&trie4), key.as_bytes());
                assert_eq!(Some(i), actual);
            }
        });
    });

    g.bench_function("AsciiTrie5", |b| {
        let trie5 = asciitrie::make5_litemap(&litemap);
        b.iter(|| {
            for (i, key) in black_box(strings).iter().enumerate() {
                let actual = asciitrie::reader5::get(black_box(&trie5), key.as_bytes());
                assert_eq!(Some(i), actual);
            }
        });
    });

    g.bench_function("AsciiTrie6", |b| {
        let trie6 = asciitrie::make6_litemap(&litemap).unwrap();
        b.iter(|| {
            for (i, key) in black_box(strings).iter().enumerate() {
                let actual = asciitrie::reader6::get(black_box(&trie6), key.as_bytes());
                assert_eq!(Some(i), actual);
            }
        });
    });

    g.bench_function("ZeroMap/usize", |b| {
        let zm: ZeroMap<[u8], usize> = litemap.iter().map(|(a, b)| (a.as_bytes(), b)).collect();
        b.iter(|| {
            for (i, key) in black_box(strings).iter().enumerate() {
                let actual = black_box(&zm).get_copied(key.as_bytes());
                assert_eq!(Some(i), actual);
            }
        });
    });

    g.bench_function("ZeroMap/u8", |b| {
        let zm: ZeroMap<[u8], u8> = litemap.iter().map(|(k, v)| (*k, *v as u8)).collect();
        b.iter(|| {
            for (i, key) in black_box(strings).iter().enumerate() {
                let actual = black_box(&zm).get_copied(key.as_bytes());
                assert_eq!(Some(i as u8), actual);
            }
        });
    });

    g.bench_function("HashMap", |b| {
        let hm: HashMap<&[u8], usize> = litemap.iter().map(|(a, b)| (a.as_bytes(), *b)).collect();
        b.iter(|| {
            for (i, key) in black_box(strings).iter().enumerate() {
                let actual = black_box(&hm).get(key.as_bytes());
                assert_eq!(Some(&i), actual);
            }
        });
    });

    g.bench_function("ZeroHashMap/usize", |b| {
        let zhm: ZeroHashMap<[u8], usize> = litemap
            .iter()
            .map(|(a, b)| (a.as_bytes(), b))
            .collect();
        b.iter(|| {
            for (i, key) in black_box(strings).iter().enumerate() {
                let actual = black_box(&zhm).get(key.as_bytes());
                // No get_copied on ZHM so we need to do it manually
                let actual = actual.map(|x| <zerovec::vecs::FlexZeroSlice as zerovec::maps::ZeroVecLike<usize>>::zvl_get_as_t(x, |y| *y));
                assert_eq!(Some(i), actual);
            }
        });
    });

    g.bench_function("ZeroHashMap/u8", |b| {
        let zhm: ZeroHashMap<[u8], u8> = litemap.iter().map(|(k, v)| (*k, *v as u8)).collect();
        b.iter(|| {
            for (i, key) in black_box(strings).iter().enumerate() {
                let actual = black_box(&zhm).get(key.as_bytes()).copied();
                assert_eq!(Some(i as u8), actual);
            }
        });
    });

    g.finish();
}

criterion_group!(
    benches,
    get_basic_bench,
    get_subtags_bench_medium,
    get_subtags_bench_large
);
criterion_main!(benches);
