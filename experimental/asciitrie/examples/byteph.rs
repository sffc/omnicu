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

/// To speed up the search algorithm, we limit the number of times the level-2 parameter (q)
/// can hit its max value of 255 before we try the next level-1 parameter (p). In practice,
/// this has a small impact on the resulting perfect hash, resulting in about 1 in 10000
/// hash maps that fall back to the slow path.
const MAX_L2_SEARCH_MISSES: usize = 24;

fn f1(byte: u8, p: u8, n: usize) -> usize {
    if p == 0 {
        byte as usize % n
    } else {
        use core::hash::Hasher;
        // let mut hasher = t1ha::T1haHasher::with_seed(p as u64);
        // let mut hasher = t1ha::T1haHasher::default();
        let mut hasher = wyhash::WyHash::with_seed(p as u64);
        // let mut hasher = wyhash::WyHash::default();
        core::hash::Hash::hash(&[byte], &mut hasher);
        // hasher.write_u8(p);
        // hasher.write_u8(byte);
        hasher.finish() as usize % n
    }
}

fn f2(byte: u8, q: u8, n: usize) -> usize {
    ((byte ^ q) as usize) % n
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

    let mut bqs = vec![0u8; N];
    let mut seen = vec![false; N];

    'p_loop: loop {
        let mut buckets: Vec<(usize, Vec<u8>)> = (0..N).map(|i| (i, vec![])).collect();
        for byte in bytes {
            buckets[f1(*byte, p, N)].1.push(*byte);
        }
        buckets.sort_by_key(|(_, v)| -(v.len() as isize));
        // println!("New P: p={p:?}, buckets={buckets:?}");
        let mut i = 0;
        let mut num_max_q = 0;
        bqs.fill(0);
        seen.fill(false);
        'q_loop: loop {
            if i == buckets.len() {
                for (local_j, real_j) in buckets.iter().map(|(j, _)| *j).enumerate() {
                    qq[real_j] = bqs[local_j];
                }
                // println!("Success: p={p:?}, num_max_q={num_max_q:?}, bqs={bqs:?}, qq={qq:?}");
                if num_max_q > 0 {
                    println!("num_max_q={num_max_q:?}");
                }
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
                        num_max_q += 1;
                        bqs[i] = 0;
                        if i == 0 || num_max_q > MAX_L2_SEARCH_MISSES {
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
    use rand::seq::SliceRandom;
    use rand::SeedableRng;
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
    for len in 0..256 {
        for seed in 0..100 {
            let bytes = random_alphanums(seed, len);
            // println!("{len} {seed}");
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
