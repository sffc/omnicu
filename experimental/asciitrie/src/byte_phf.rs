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

const P_FAST_MAX: u8 = 11;
const P_REAL_MAX: u8 = 15;
const Q_FAST_MAX: u8 = 95;
const Q_REAL_MAX: u8 = 127;

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

#[inline]
fn debug_get(slice: &[u8], index: usize) -> Option<u8> {
    match slice.get(index) {
        Some(x) => Some(*x),
        None => {
            debug_assert!(false, "debug_get: index expected to be in range");
            None
        }
    }
}

/// Invariant: n > 0
#[inline]
pub fn f1(byte: u8, p: u8, n: usize) -> usize {
    let n = if n > 0 {
        n
    } else {
        debug_assert!(false, "unreachable by invariant");
        1
    };
    if p == 0 {
        byte as usize % n
    } else {
        let mut result = byte ^ p ^ byte.wrapping_shr(p as u32);
        for _ in P_FAST_MAX..p {
            result = result ^ (result << 1) ^ (result >> 1);
        }
        result as usize % n
    }
}

/// Invariant: n > 0
#[inline]
pub fn f2(byte: u8, q: u8, n: usize) -> usize {
    let n = if n > 0 {
        n
    } else {
        debug_assert!(false, "unreachable by invariant");
        1
    };
    // ((byte ^ q) as usize) % n
    let mut result = byte ^ q;
    // if q >= Q_FAST_MAX {
    //     result = result ^ byte.wrapping_shr(q as u32);
    // }
    for _ in Q_FAST_MAX..q {
        result = result ^ (result << 1) ^ (result >> 1);
    }
    result as usize % n
}

#[allow(unused_labels)] // for readability
pub fn find(bytes: &[u8]) -> Result<(u8, Vec<u8>), Error> {
    #[allow(non_snake_case)]
    let N = bytes.len();

    let mut p = 0u8;
    let mut qq = vec![0u8; N];

    let mut bqs = vec![0u8; N];
    let mut seen = vec![false; N];
    let mut max_allowable_p = P_FAST_MAX;
    let mut max_allowable_q = Q_FAST_MAX;

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
                        if bqs[i] < max_allowable_q {
                            bqs[i] += 1;
                            continue 'q_loop;
                        }
                        num_max_q += 1;
                        bqs[i] = 0;
                        if i == 0 || num_max_q > MAX_L2_SEARCH_MISSES {
                            if p == max_allowable_p && max_allowable_p != P_REAL_MAX {
                                max_allowable_p = P_REAL_MAX;
                                max_allowable_q = Q_REAL_MAX;
                                p = 0;
                                continue 'p_loop;
                            } else if p == P_REAL_MAX {
                                // println!("Could not solve PHF function");
                                return Err(Error::CouldNotSolve);
                            } else {
                                p += 1;
                                continue 'p_loop;
                            }
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
        let q = debug_get(qq, f1(key, *p, n))?;
        let l2 = f2(key, q, n);
        let ek = debug_get(eks, l2)?;
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
    pub fn p_qmax(&self) -> Option<(u8, u8)> {
        let (p, buffer) = self.0.as_ref().split_first()?;
        let n = buffer.len() / 2;
        if n == 0 {
            return None;
        }
        let (qq, _) = debug_split_at(buffer, n)?;
        Some((*p, *qq.iter().max().unwrap()))
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
        let mut count_by_p = [0; 256];
        let mut count_by_qmax = [0; 256];
        for len in 1..16 {
            for seed in 0..150 {
                let keys = random_alphanums(seed, len);
                let keys_str = core::str::from_utf8(&keys).unwrap();
                let computed = PerfectByteHashMap::try_new(&keys).expect(keys_str);
                computed
                    .check()
                    .expect(std::str::from_utf8(&keys).expect(keys_str));
                let (p, qmax) = computed.p_qmax().unwrap();
                count_by_p[p as usize] += 1;
                count_by_qmax[qmax as usize] += 1;
            }
        }
        std::println!("count_by_p (smaller): {count_by_p:?}");
        std::println!("count_by_qmax (smaller): {count_by_qmax:?}");
        let count_fastq = count_by_qmax[0..=Q_FAST_MAX as usize].iter().sum::<usize>();
        let count_slowq = count_by_qmax[Q_FAST_MAX as usize + 1..]
            .iter()
            .sum::<usize>();
        std::println!("fastq/slowq: {count_fastq}/{count_slowq}");
        // Assert that 99% of cases resolve to the fast hash
        assert!(count_fastq >= count_slowq * 100);
    }

    #[test]
    fn test_larger() {
        let mut count_by_p = [0; 256];
        let mut count_by_qmax = [0; 256];
        for len in 16..60 {
            for seed in 0..75 {
                let keys = random_alphanums(seed, len);
                let keys_str = core::str::from_utf8(&keys).unwrap();
                let computed = PerfectByteHashMap::try_new(&keys).expect(keys_str);
                computed
                    .check()
                    .expect(std::str::from_utf8(&keys).expect(keys_str));
                let (p, qmax) = computed.p_qmax().unwrap();
                count_by_p[p as usize] += 1;
                count_by_qmax[qmax as usize] += 1;
            }
        }
        std::println!("count_by_p (larger): {count_by_p:?}");
        std::println!("count_by_qmax (larger): {count_by_qmax:?}");
        let count_fastq = count_by_qmax[0..=Q_FAST_MAX as usize].iter().sum::<usize>();
        let count_slowq = count_by_qmax[Q_FAST_MAX as usize + 1..]
            .iter()
            .sum::<usize>();
        std::println!("fastq/slowq: {count_fastq}/{count_slowq}");
        // Assert that 99% of cases resolve to the fast hash
        assert!(count_fastq >= count_slowq * 100);
    }

    #[test]
    fn test_build_read_small() {
        #[derive(Debug)]
        struct TestCase<'a> {
            keys: &'a str,
            expected: &'a [u8],
            reordered_keys: &'a str,
        }
        let cases = [
            TestCase {
                keys: "ab",
                expected: &[0, 0, 0, b'b', b'a'],
                reordered_keys: "ba",
            },
            TestCase {
                keys: "abc",
                expected: &[0, 0, 0, 0, b'c', b'a', b'b'],
                reordered_keys: "cab",
            },
            TestCase {
                // Note: splitting "a" and "c" into different buckets requires the heavier hash
                // function because the difference between "a" and "c" is the period (2).
                keys: "ac",
                expected: &[1, 0, 1, b'c', b'a'],
                reordered_keys: "ca",
            },
            TestCase {
                keys: "abd",
                expected: &[0, 0, 1, 3, b'a', b'b', b'd'],
                reordered_keys: "abd",
            },
            TestCase {
                keys: "def",
                expected: &[0, 0, 0, 0, b'f', b'd', b'e'],
                reordered_keys: "fde",
            },
            TestCase {
                keys: "fi",
                expected: &[0, 0, 0, b'f', b'i'],
                reordered_keys: "fi",
            },
            TestCase {
                keys: "gh",
                expected: &[0, 0, 0, b'h', b'g'],
                reordered_keys: "hg",
            },
            TestCase {
                keys: "lm",
                expected: &[0, 0, 0, b'l', b'm'],
                reordered_keys: "lm",
            },
            TestCase {
                // Note: "a" and "q" (0x61 and 0x71) are very hard to split; only a handful of
                // hash function crates can get them into separate buckets.
                keys: "aq",
                expected: &[4, 0, 1, b'a', b'q'],
                reordered_keys: "aq",
            },
            TestCase {
                keys: "xy",
                expected: &[0, 0, 0, b'x', b'y'],
                reordered_keys: "xy",
            },
            TestCase {
                keys: "xyz",
                expected: &[0, 0, 0, 0, b'x', b'y', b'z'],
                reordered_keys: "xyz",
            },
            TestCase {
                keys: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
                expected: &[
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 10, 12, 16, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 16,
                    16, 16, 16, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                    2, 0, 7, 104, 105, 106, 107, 108, 109, 110, 111, 112, 117, 118, 119, 68, 69,
                    70, 113, 114, 65, 66, 67, 120, 121, 122, 115, 72, 73, 74, 71, 80, 81, 82, 83,
                    84, 85, 86, 87, 88, 89, 90, 75, 76, 77, 78, 79, 103, 97, 98, 99, 116, 100, 102,
                    101,
                ],
                reordered_keys: "hijklmnopuvwDEFqrABCxyzsHIJGPQRSTUVWXYZKLMNOgabctdfe",
            },
            TestCase {
                keys: "abcdefghij",
                expected: &[
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100, 101, 102, 103, 104, 105, 106, 97, 98, 99,
                ],
                reordered_keys: "defghijabc",
            },
            TestCase {
                // This is a small case that resolves to the slow hasher
                keys: "Jbej",
                expected: &[2, 0, 0, 102, 0, b'j', b'e', b'b', b'J'],
                reordered_keys: "jebJ",
            },
            TestCase {
                // This is another small case that resolves to the slow hasher
                keys: "JFNv",
                expected: &[1, 98, 0, 2, 0, b'J', b'F', b'N', b'v'],
                reordered_keys: "JFNv",
            },
        ];
        for cas in cases {
            let computed = PerfectByteHashMap::try_new(cas.keys.as_bytes()).expect(cas.keys);
            assert_eq!(computed.as_bytes(), cas.expected, "{:?}", cas);
            assert_eq!(computed.keys(), cas.reordered_keys.as_bytes(), "{:?}", cas);
            computed.check().expect(cas.keys);
        }
    }
}
