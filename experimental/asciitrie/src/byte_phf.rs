// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::vec::Vec;
use alloc::vec;

/// To speed up the search algorithm, we limit the number of times the level-2 parameter (q)
/// can hit its max value of 255 before we try the next level-1 parameter (p). In practice,
/// this has a small impact on the resulting perfect hash, resulting in about 1 in 10000
/// hash maps that fall back to the slow path.
const MAX_L2_SEARCH_MISSES: usize = 24;

/// Like slice::split_at but returns an Option instead of panicking
#[inline]
fn debug_split_at(slice: &[u8], mid: usize) -> Option<(&[u8], &[u8])> {
    if mid > slice.len() {
        debug_assert!(false, "debug_split_at: index expected to be in range");
        None
    } else {
        // Note: We're trusting the compiler to inline this and remove the assertion
        // hiding on the top of slice::split_at: `assert(mid <= self.len())`
        Some(slice.split_at(mid))
    }
}

pub fn f1(byte: u8, p: u8, n: usize) -> usize {
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

pub fn f2(byte: u8, q: u8, n: usize) -> usize {
    ((byte ^ q) as usize) % n
}

#[allow(unused_labels)] // for readability
pub fn find(bytes: &[u8]) -> Result<(u8, Vec<u8>), &'static str> {
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
                // if num_max_q > 0 {
                //     println!("num_max_q={num_max_q:?}");
                // }
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
                                // println!("Could not solve PHF function");
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

pub fn create_standard_layout(keys: &[u8]) -> Result<Vec<u8>, &'static str> {
    let n = keys.len();
    let (p, mut qq) = find(keys)?;
    let mut keys_permuted = vec![0; n];
    for key in keys {
        let l1 = f1(*key, p, n);
        let q = qq[l1];
        let l2 = f2(*key, q, n);
        keys_permuted[l2] = *key;
    }
    let mut result = Vec::with_capacity(n*2+1);
    result.push(p);
    result.append(&mut qq);
    result.append(&mut keys_permuted);
    Ok(result)
}

/// Standard layout: P, N bytes of Q, N bytes of expected keys
pub fn get_from_standard_layout(buffer: &[u8], key: u8) -> Option<usize> {
    let (p, buffer) = buffer.split_first()?;
    let n = buffer.len() / 2;
    let (qq, eks) = debug_split_at(buffer, n)?;
    debug_assert_eq!(qq.len(), eks.len());
    let q = qq[f1(key, *p, n)];
    let l2 = f2(key, q, n);
    let ek = eks[l2];
    if ek == key {
        Some(l2)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
	use super::*;
    extern crate std;

	fn random_alphanums(seed: u64, len: usize) -> Vec<u8> {
	    use rand::seq::SliceRandom;
	    use rand::SeedableRng;
	    const BYTES: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
	    let mut rng = rand_pcg::Lcg64Xsh32::seed_from_u64(seed);
	    BYTES.choose_multiple(&mut rng, len).copied().collect()
	}

	#[test]
	fn test_smaller() {
	    for len in 0..32 {
	        for seed in 0..50 {
	            let bytes = random_alphanums(seed, len);
	            find(bytes.as_slice()).unwrap();
	        }
	    }
	}

	#[test]
	fn test_larger() {
	    for len in 32..256 {
	        for seed in 0..2 {
	            let bytes = random_alphanums(seed, len);
	            find(bytes.as_slice()).unwrap();
	        }
	    }
	}

    #[test]
    fn test_build_read_small() {
        struct TestCase<'a> {
            bytes: &'a [u8],
            expected: &'a [u8],
        }
        let cases = [
            TestCase {
                bytes: b"abc",
                expected: &[0, 0, 0, 0, b'c', b'a', b'b']
            },
            TestCase {
                bytes: b"abd",
                expected: &[0, 0, 1, 3, b'a', b'b', b'd']
            },
            TestCase {
                bytes: b"def",
                expected: &[0, 0, 0, 0, b'f', b'd', b'e']
            },
            TestCase {
                bytes: b"lm",
                expected: &[0, 0, 0, b'l', b'm']
            },
        ];
        for cas in cases {
            let n = cas.bytes.len();
            let computed = create_standard_layout(cas.bytes).unwrap();
            assert_eq!(computed, cas.expected);
            let mut seen = vec![false; n];
            for key in cas.bytes {
                let i = get_from_standard_layout(&computed, *key).expect(&std::format!("{key}"));
                assert!(!seen[i], "{key}");
                seen[i] = true;
            }
            assert!(get_from_standard_layout(&computed, b'_').is_none());
        }
    }
}
