// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rand::SeedableRng;
use rand_distr::{Distribution, LogNormal};
use rand_pcg::Lcg64Xsh32;
use std::fmt;

use serde_unaligned::ule::*;
use serde_unaligned::uvec::*;

#[repr(align(8))]
#[derive(Default)]
struct AlignedBuffer(Vec<u8>);

fn vec_to_unaligned_uvec<'a, T>(vec: &Vec<T>, buffer: &'a mut AlignedBuffer) -> UVec<'a, T>
where
    T: AsULE + Copy + PartialEq + fmt::Debug,
    <<T as AsULE>::ULE as ULE>::Error: fmt::Debug,
{
    // Pad with zero to ensure it is not aligned
    buffer.0.push(0);
    UVec::from(black_box(vec.as_slice()))
        .write_to_stream_le(&mut buffer.0)
        .unwrap();
    let uvec = UVec::<T>::from_unaligned_le_bytes(&buffer.0[1..]).unwrap();
    uvec
}

fn overview_bench(c: &mut Criterion) {
    c.bench_function("uvec/overview", |b| {
        b.iter(|| {
            UVec::<u32>::from_unaligned_le_bytes(black_box(&TEST_BUFFER_LE))
                .unwrap()
                .sum()
        });
    });

    #[cfg(feature = "bench")]
    {
        sum_benches(c);
        binary_search_benches(c);
    }
}

#[cfg(feature = "bench")]
fn sum_benches(c: &mut Criterion) {
    c.bench_function("sum/uvec/test_slice", |b| {
        b.iter(|| UVec::from(black_box(TEST_SLICE)).sum());
    });

    c.bench_function("sum/uvec/test_buffer_le", |b| {
        b.iter(|| {
            UVec::<u32>::from_unaligned_le_bytes(black_box(&TEST_BUFFER_LE))
                .unwrap()
                .sum()
        });
    });

    c.bench_function("sum_u32/uvec/test_slice", |b| {
        b.iter(|| UVec::from(black_box(TEST_SLICE)).sum_u32());
    });

    c.bench_function("sum_u32/uvec/test_buffer_le", |b| {
        b.iter(|| {
            UVec::<u32>::from_unaligned_le_bytes(black_box(&TEST_BUFFER_LE))
                .unwrap()
                .sum_u32()
        });
    });
}

#[cfg(feature = "bench")]
fn binary_search_benches(c: &mut Criterion) {
    // Generate a large list of u32s for stress testing.
    // Lcg64Xsh32 is a PRNG with a fixed seed for reproducible benchmarks.
    // LogNormal(10, 1) generates numbers with mean 36315 and mode 8103, a distribution that, in
    // spirit, correlates with Unicode properties (many low values and a long tail of high values)
    let mut rng = Lcg64Xsh32::seed_from_u64(2021);
    let dist = LogNormal::new(10.0, 1.0).unwrap();
    let haystack = {
        let mut unsorted: Vec<u32> = (&dist)
            .sample_iter(&mut rng)
            .take(1000)
            .map(|f| f as u32)
            .collect();
        unsorted.sort();
        unsorted
    };
    let needles: Vec<u32> = (&dist)
        .sample_iter(&mut rng)
        .take(100)
        .map(|f| f as u32)
        .collect();

    let uvec = haystack.clone();
    c.bench_with_input(BenchmarkId::new("binary_search/uvec/log_normal/vec_of_aligned", haystack.len()), &(&uvec, &needles), |b, &(uvec, needles)| {
        b.iter(|| {
            for needle in needles.iter() {
                black_box(uvec.binary_search(&needle));
            }
        });
    });

    let uvec: Vec<_> = haystack.iter().map(|u| u.as_unaligned()).collect();
    c.bench_with_input(BenchmarkId::new("binary_search/uvec/log_normal/aligned_vec_of_unaligned", haystack.len()), &(&uvec, &needles), |b, &(uvec, needles)| {
        b.iter(|| {
            for needle in needles.iter() {
                black_box(uvec.binary_search_by(|probe| u32::from_unaligned(probe).cmp(&needle)));
            }
        });
    });
    let mut uvec = vec![1u8];
    for hay in &haystack {
        uvec.extend(&hay.to_le_bytes());
    }
    let uvec: &[<u32 as AsULE>::ULE] = unsafe { ::std::mem::transmute(&uvec[1..]) };
    c.bench_with_input(BenchmarkId::new("binary_search/uvec/log_normal/unaligned_vec_of_unaligned", haystack.len()), &(&uvec, &needles), |b, &(uvec, needles)| {
        b.iter(|| {
            for needle in needles.iter() {
                black_box(uvec.binary_search_by(|probe| u32::from_unaligned(probe).cmp(&needle)));
            }
        });
    });

    let uvec = UVec::from(black_box(haystack.as_slice()));
    assert_eq!(uvec, haystack.as_slice());


    println!("{:p} {:p}", &needles[0], uvec.get_ptr());


    c.bench_with_input(BenchmarkId::new("binary_search/uvec/log_normal/aligned", haystack.len()), &(&uvec, &needles), |b, &(uvec, needles)| {
        b.iter(|| {
            for needle in needles.iter() {
                black_box(uvec.binary_search(&needle));
            }
        });
    });

    let mut buffer = AlignedBuffer::default();
    let uvec = vec_to_unaligned_uvec(&haystack, &mut buffer);
    assert_eq!(uvec, haystack.as_slice());

    println!("{:p} {:p}", &needles[0], uvec.get_ptr());


    c.bench_with_input(BenchmarkId::new("binary_search/uvec/log_normal/unaligned", haystack.len()), &(&uvec, &needles), |b, &(uvec, needles)| {
        b.iter(|| {
            for needle in needles.iter() {
                black_box(uvec.binary_search(&needle));
            }
        });
    });

}

criterion_group!(benches, overview_bench,);
criterion_main!(benches);
