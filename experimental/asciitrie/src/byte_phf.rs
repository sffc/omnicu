// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::vec;
use alloc::vec::Vec;

/// To speed up the search algorithm, we limit the number of times the level-2 parameter (q)
/// can hit its max value of 255 before we try the next level-1 parameter (p). In practice,
/// this has a small impact on the resulting perfect hash, resulting in about 1 in 10000
/// hash maps that fall back to the slow path.
const MAX_L2_SEARCH_MISSES: usize = 24;

#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    CouldNotSolve,
}

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
pub fn find(bytes: &[u8]) -> Result<(u8, Vec<u8>), Error> {
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
                                return Err(Error::CouldNotSolve);
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

// Standard layout: P, N bytes of Q, N bytes of expected keys
#[derive(Debug)]
pub struct PerfectByteHashMap<S: ?Sized>(S);

impl<S> PerfectByteHashMap<S> {
    pub fn from_store(store: S) -> Self {
        Self(store)
    }

    pub fn take_store(self) -> S {
        self.0
    }
}

impl<S> PerfectByteHashMap<S>
where
    S: AsRef<[u8]> + ?Sized,
{
    pub fn get(&self, key: u8) -> Option<usize> {
        let (p, buffer) = self.0.as_ref().split_first()?;
        let n = buffer.len() / 2;
        if n == 0 {
            return None;
        }
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
    pub fn len(&self) -> usize {
        self.0.as_ref().len() / 2
    }
    pub fn keys(&self) -> &[u8] {
        let n = self.len();
        debug_split_at(self.0.as_ref(), 1 + n)
            .map(|s| s.1)
            .unwrap_or(&[])
    }
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }
    #[cfg(test)]
    pub fn check(&self) -> Result<(), (&'static str, u8)> {
        let len = self.len();
        let mut seen = vec![false; len];
        for b in 0..=255u8 {
            let get_result = self.get(b);
            if self.keys().contains(&b) {
                let i = get_result.ok_or(("expected to find", b))?;
                if seen[i] {
                    return Err(("seen", b));
                }
                seen[i] = true;
            } else {
                if get_result.is_some() {
                    return Err(("did not expect to find", b));
                }
            }
        }
        Ok(())
    }
}

impl PerfectByteHashMap<Vec<u8>> {
    pub fn try_new(keys: &[u8]) -> Result<Self, Error> {
        let n = keys.len();
        let (p, mut qq) = find(keys)?;
        let mut keys_permuted = vec![0; n];
        for key in keys {
            let l1 = f1(*key, p, n);
            let q = qq[l1];
            let l2 = f2(*key, q, n);
            keys_permuted[l2] = *key;
        }
        let mut result = Vec::with_capacity(n * 2 + 1);
        result.push(p);
        result.append(&mut qq);
        result.append(&mut keys_permuted);
        Ok(Self(result))
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
                let keys = random_alphanums(seed, len);
                let computed = PerfectByteHashMap::try_new(&keys).unwrap();
                computed.check().expect(std::str::from_utf8(&keys).unwrap());
            }
        }
    }

    #[test]
    fn test_larger() {
        for len in 32..256 {
            for seed in 0..2 {
                let keys = random_alphanums(seed, len);
                let computed = PerfectByteHashMap::try_new(&keys).unwrap();
                computed.check().expect(std::str::from_utf8(&keys).unwrap());
            }
        }
    }

    #[test]
    fn test_build_read_small() {
        struct TestCase<'a> {
            keys: &'a [u8],
            expected: &'a [u8],
            reordered_keys: &'a [u8],
        }
        let cases = [
            TestCase {
                keys: b"ab",
                expected: &[0, 0, 0, b'b', b'a'],
                reordered_keys: b"ba",
            },
            TestCase {
                keys: b"abc",
                expected: &[0, 0, 0, 0, b'c', b'a', b'b'],
                reordered_keys: b"cab",
            },
            TestCase {
                // Note: splitting "a" and "c" into different buckets requires the heavier hash
                // function because the difference between "a" and "c" is the period (2).
                keys: b"ac",
                expected: &[1, 0, 1, b'a', b'c'],
                reordered_keys: b"ac",
            },
            TestCase {
                keys: b"abd",
                expected: &[0, 0, 1, 3, b'a', b'b', b'd'],
                reordered_keys: b"abd",
            },
            TestCase {
                keys: b"def",
                expected: &[0, 0, 0, 0, b'f', b'd', b'e'],
                reordered_keys: b"fde",
            },
            TestCase {
                keys: b"fi",
                expected: &[0, 0, 0, b'f', b'i'],
                reordered_keys: b"fi",
            },
            TestCase {
                keys: b"gh",
                expected: &[0, 0, 0, b'h', b'g'],
                reordered_keys: b"hg",
            },
            TestCase {
                keys: b"lm",
                expected: &[0, 0, 0, b'l', b'm'],
                reordered_keys: b"lm",
            },
            TestCase {
                // Note: "a" and "q" (0x61 and 0x71) are very hard to split; only a handful of
                // hash function crates can get them into separate buckets.
                keys: b"aq",
                expected: &[2, 0, 1, b'a', b'q'],
                reordered_keys: b"aq",
            },
            TestCase {
                keys: b"xy",
                expected: &[0, 0, 0, b'x', b'y'],
                reordered_keys: b"xy",
            },
            TestCase {
                keys: b"xyz",
                expected: &[0, 0, 0, 0, b'x', b'y', b'z'],
                reordered_keys: b"xyz",
            },
            TestCase {
                keys: b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
                expected: &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 10, 12, 16, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 16, 16, 16, 16, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 7, 104, 105, 106, 107, 108, 109, 110, 111, 112, 117, 118, 119, 68, 69, 70, 113, 114, 65, 66, 67, 120, 121, 122, 115, 72, 73, 74, 71, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 75, 76, 77, 78, 79, 103, 97, 98, 99, 116, 100, 102, 101],
                reordered_keys: b"hijklmnopuvwDEFqrABCxyzsHIJGPQRSTUVWXYZKLMNOgabctdfe",
            },
            TestCase {
                keys: b"abcdefghij",
                expected: &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100, 101, 102, 103, 104, 105, 106, 97, 98, 99],
                reordered_keys: b"defghijabc",
            }
        ];
        for cas in cases {
            let computed = PerfectByteHashMap::try_new(cas.keys).unwrap();
            assert_eq!(computed.as_bytes(), cas.expected);
            assert_eq!(computed.keys(), cas.reordered_keys);
            computed
                .check()
                .expect(std::str::from_utf8(cas.keys).unwrap());
        }
    }
}
