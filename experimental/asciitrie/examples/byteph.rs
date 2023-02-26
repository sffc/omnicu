// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// This example demonstrates the use of AsciiTrie to look up data based on a region code.

#![no_main] // https://github.com/unicode-org/icu4x/issues/395

#![allow(unused_labels)]
#![allow(dead_code)]

icu_benchmark_macros::static_setup!();

fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}

fn f1(byte: u8, p: u8, n: usize) -> usize {
    // (byte as usize + p as usize) % n

    // ((byte ^ p) as usize) % n

    // (byte as usize * p as usize) % n

    // ((byte as usize) ^ (byte as usize >> (p as usize + 1))) % n

    // let mut x = byte as usize;
    // for _ in 0..p {
    //     x = x.wrapping_mul(257);
    // }
    // x % n

    if p == 0 { byte as usize % n } else {

    use core::hash::{Hasher, Hash};
    let mut hasher = t1ha::T1haHasher::with_seed(p as u64);
    [byte].hash(&mut hasher);
    hasher.finish() as usize % n


    // const K: usize = 0x517cc1b727220a95;
    // (((p as usize) << 5) ^ (byte as usize)).wrapping_mul(K) % n

    }
}

fn f2(byte: u8, q: u8, n: usize) -> usize {
    // (byte as usize + q as usize) % n

    ((byte ^ q) as usize) % n

    // (byte as usize * (q as usize + 1)) % n

    // ((byte as usize ^ (n - 1)) + q as usize) % n

    // let mut x = byte as usize;
    // x ^= x << 13;
    // x ^= x >> 17;
    // x ^= x << 5;
    // x ^= q as usize;
    // x % n

    // use core::hash::{Hasher, Hash};
    // let mut hasher = t1ha::T1haHasher::with_seed(q as u64);
    // [byte].hash(&mut hasher);
    // hasher.finish() as usize % n
}

fn print_byte_to_stdout(byte: u8) {
    if let Ok(c) = char::try_from(byte) {
        if c.is_ascii_alphanumeric() {
            print!("'{c}'");
            return;
        }
    }
    print!("0x{byte:X}");
}

fn find_ph(bytes: &[u8]) -> Result<(u8, Vec<u8>), &'static str> {
    #[allow(non_snake_case)]
    let N = bytes.len();

    let mut p = 0u8;
    let mut qq = vec![0u8; N];

    'p_loop: loop {
        let mut buckets: Vec<(usize, Vec<u8>)> = (0..N).map(|i| (i, vec![])).collect();
        for byte in bytes {
            buckets[f1(*byte, p, N)].1.push(*byte);
        }
        buckets.sort_by_key(|(_, v)| -(v.len() as isize));
        // println!("New P: p={p:?}, buckets={buckets:?}");
        let mut i = 0;
        let mut bqs = vec![0u8; N];
        let mut seen = vec![false; N];
        'q_loop: loop {
            if i == buckets.len() {
                for (local_j, real_j) in buckets.iter().map(|(j, _)| *j).enumerate() {
                    qq[real_j] = bqs[local_j];
                }
                println!("Success: p={p:?}, bqs={bqs:?}, qq={qq:?}");
                return Ok((p, qq));
            }
            let mut bucket = buckets[i].1.as_slice();
            'byte_loop: for (j, byte) in bucket.iter().enumerate() {
                if seen[f2(*byte, bqs[i], N)] {
                    // println!("Skipping Q: p={p:?}, i={i:?}, byte={byte:}, q={i:?}, l2={:?}", f2(*byte, bqs[i], N));
                    for k in 0..j {
                        let k_byte = bucket[k];
                        assert!(seen[f2(k_byte, bqs[i], N)]);
                        seen[f2(k_byte, bqs[i], N)] = false;
                    }
                    'reset_loop: loop {
                        if bqs[i] < u8::MAX {
                            bqs[i] += 1;
                            continue 'q_loop;
                        }
                        println!("!!! reached max Q!");
                        bqs[i] = 0;
                        i = 0; // !!!
                        if i == 0 {
                            if p == u8::MAX {
                                println!("Could not solve PHF function");
                                return Err("Could not solve PHF function");
                            }
                            p += 1;
                            continue 'p_loop;
                        }
                        i -= 1;
                        bucket = buckets[i].1.as_slice();
                        for byte in bucket {
                            assert!(seen[f2(*byte, bqs[i], N)]);
                            seen[f2(*byte, bqs[i], N)] = false;
                        }
                    }
                } else {
                    // println!("Marking as seen: i={i:?}, byte={byte:}, l2={:?}", f2(*byte, bqs[i], N));
                    seen[f2(*byte, bqs[i], N)] = true;
                }
            }
            // println!("Found Q: i={i:?}, q={:?}", bqs[i]);
            i += 1;
        }
    }
}

fn random_alphanums(seed: u64, len: usize) -> Vec<u8> {
    use rand::SeedableRng;
    use rand::seq::SliceRandom;
    const BYTES: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand_pcg::Lcg64Xsh32::seed_from_u64(seed);
    BYTES.choose_multiple(&mut rng, len).copied().collect()
}

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    icu_benchmark_macros::main_setup!();

    // let bytes = b"abdeghi";
    // let bytes = b"abdeghklmopuvxz";
    // let bytes = b"qwertyuiopasdfgh";
    // let bytes = b"qwrtuipadgklzxcbmQWRUOPADHKZVM";

    let mut p_distr = vec![0; 256];
    for len in 1..32 {
        for seed in 0..32 {
            let bytes = random_alphanums(seed, len);
            println!("{len} {seed}");
            let (p, _) = find_ph(bytes.as_slice()).unwrap();
            p_distr[p as usize] += 1;
        }
    }
    println!("p_distr: {p_distr:?}");

    let bytes = random_alphanums(0, 16);

    #[allow(non_snake_case)]
    let N = bytes.len();

    let (p, qq) = find_ph(bytes.as_slice()).unwrap();

    println!("Results:");
    for byte in bytes.iter() {
        print_byte_to_stdout(*byte);
        let l1 = f1(*byte, p, N);
        let q = qq[l1];
        let l2 = f2(*byte, q, N);
        println!(" => l1 {l1} => q {q} => l2 {l2}");
    }

    return 0;
}
