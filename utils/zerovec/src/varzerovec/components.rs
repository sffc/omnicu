// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::flexzerovec::FlexZeroSlice;
use crate::ule::*;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::convert::TryFrom;
use core::marker::PhantomData;
use core::ops::Range;

// Also used by owned.rs
pub(super) const LENGTH_WIDTH: usize = 4;
pub(super) const METADATA_WIDTH: usize = 1;
pub(super) const INDEX_WIDTH: usize = 4;
pub(super) const MAX_LENGTH: usize = u32::MAX as usize;
pub(super) const MAX_INDEX: usize = u32::MAX as usize;

/// A more parsed version of `VarZeroSlice`. This type is where most of the VarZeroVec
/// internal representation code lies.
///
/// This is *basically* an `&'a [u8]` to a zero copy buffer, but split out into
/// the buffer components. Logically this is capable of behaving as
/// a `&'a [T::VarULE]`, but since `T::VarULE` is unsized that type does not actually
/// exist.
///
/// See [`VarZeroVecComponents::parse_byte_slice()`] for information on the internal invariants involved
pub struct VarZeroVecComponents<'a, T: ?Sized> {
    /// The number of elements
    len: u32,
    /// The list of indices into the `things` slice, with the flexzerovec meta byte
    indices_with_meta: &'a [u8],
    /// The contiguous list of `T::VarULE`s
    things: &'a [u8],
    /// The original slice this was constructed from
    entire_slice: &'a [u8],
    marker: PhantomData<&'a T>,
}

// #[derive()] won't work here since we do not want it to be
// bound on T: Copy
impl<'a, T: ?Sized> Copy for VarZeroVecComponents<'a, T> {}
impl<'a, T: ?Sized> Clone for VarZeroVecComponents<'a, T> {
    fn clone(&self) -> Self {
        VarZeroVecComponents {
            len: self.len,
            indices_with_meta: self.indices_with_meta,
            things: self.things,
            entire_slice: self.entire_slice,
            marker: PhantomData,
        }
    }
}

impl<'a, T: VarULE + ?Sized> Default for VarZeroVecComponents<'a, T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T: VarULE + ?Sized> VarZeroVecComponents<'a, T> {
    #[inline]
    pub fn new() -> Self {
        Self {
            len: 0,
            indices_with_meta: &[1],
            things: &[],
            entire_slice: &[],
            marker: PhantomData,
        }
    }

    /// Construct a new VarZeroVecComponents, checking invariants about the overall buffer size:
    ///
    /// - There must be either zero or at least four bytes (if four, this is the "length" parsed as a usize)
    /// - There must be at least `4*length + 4` bytes total, to form the the array `indices` of indices
    /// - `indices[i]..indices[i+1]` must index into a valid section of
    ///   `things`, such that it parses to a `T::VarULE`
    /// - `indices[len - 1]..things.len()` must index into a valid section of
    ///   `things`, such that it parses to a `T::VarULE`
    #[inline]
    pub fn parse_byte_slice(slice: &'a [u8]) -> Result<Self, ZeroVecError> {
        if slice.is_empty() {
            return Ok(VarZeroVecComponents {
                len: 0,
                indices_with_meta: &[1],
                things: &[],
                entire_slice: slice,
                marker: PhantomData,
            });
        }
        let len_bytes = slice
            .get(0..LENGTH_WIDTH)
            .ok_or(ZeroVecError::VarZeroVecFormatError)?;
        let len_ule = RawBytesULE::<LENGTH_WIDTH>::parse_byte_slice(len_bytes)
            .map_err(|_| ZeroVecError::VarZeroVecFormatError)?;

        let len = len_ule
            .get(0)
            .ok_or(ZeroVecError::VarZeroVecFormatError)?
            .as_unsigned_int();
        assert_eq!(METADATA_WIDTH, 1);
        if slice.get(LENGTH_WIDTH) != Some(&(INDEX_WIDTH as u8)) {
            return Err(ZeroVecError::VarZeroVecFormatError);
        }
        let indices_bytes = slice
            .get(LENGTH_WIDTH..LENGTH_WIDTH + METADATA_WIDTH + INDEX_WIDTH * (len as usize))
            .ok_or(ZeroVecError::VarZeroVecFormatError)?;
        let things = slice
            .get(INDEX_WIDTH * (len as usize) + LENGTH_WIDTH + METADATA_WIDTH..)
            .ok_or(ZeroVecError::VarZeroVecFormatError)?;

        let borrowed = VarZeroVecComponents {
            len,
            indices_with_meta: indices_bytes,
            things,
            entire_slice: slice,
            marker: PhantomData,
        };

        for thing in borrowed.iter_checked() {
            let _ = thing?;
        }

        Ok(borrowed)
    }

    /// Construct a [`VarZeroVecComponents`] from a byte slice that has previously
    /// successfully returned a [`VarZeroVecComponents`] when passed to
    /// [`VarZeroVecComponents::parse_byte_slice()`]. Will return the same
    /// object as one would get from calling [`VarZeroVecComponents::parse_byte_slice()`].
    ///
    /// # Safety
    /// The bytes must have previously successfully run through
    /// [`VarZeroVecComponents::parse_byte_slice()`]
    pub unsafe fn from_bytes_unchecked(slice: &'a [u8]) -> Self {
        if slice.is_empty() {
            return VarZeroVecComponents {
                len: 0,
                indices_with_meta: &[1],
                things: &[],
                entire_slice: slice,
                marker: PhantomData,
            };
        }
        let len_bytes = slice.get_unchecked(0..LENGTH_WIDTH);
        let len_ule = RawBytesULE::<LENGTH_WIDTH>::from_byte_slice_unchecked(len_bytes);

        let len = len_ule.get_unchecked(0).as_unsigned_int();
        let indices_bytes = slice.get_unchecked(
            LENGTH_WIDTH..LENGTH_WIDTH + METADATA_WIDTH + INDEX_WIDTH * (len as usize),
        );
        let things =
            slice.get_unchecked(LENGTH_WIDTH + METADATA_WIDTH + INDEX_WIDTH * (len as usize)..);

        VarZeroVecComponents {
            len,
            indices_with_meta: indices_bytes,
            things,
            entire_slice: slice,
            marker: PhantomData,
        }
    }

    /// Get the number of elements in this vector
    #[inline]
    pub fn len(self) -> usize {
        self.len as usize
    }

    /// Returns `true` if the vector contains no elements.
    #[inline]
    pub fn is_empty(self) -> bool {
        self.len == 0
    }

    /// Get the idx'th element out of this slice. Returns `None` if out of bounds.
    #[inline]
    pub fn get(self, idx: usize) -> Option<&'a T> {
        if idx >= self.len() {
            return None;
        }
        Some(unsafe { self.get_unchecked(idx) })
    }

    /// Get the idx'th element out of this slice. Does not bounds check.
    ///
    /// Safety:
    /// - `idx` must be in bounds (`idx < self.len()`)
    #[inline]
    pub(crate) unsafe fn get_unchecked(self, idx: usize) -> &'a T {
        let range = self.get_things_range(idx);
        let things_slice = self.things.get_unchecked(range);
        T::from_byte_slice_unchecked(things_slice)
    }

    /// Get the range in `things` for the element at `idx`. Does not bounds check.
    ///
    /// Safety:
    /// - `idx` must be in bounds (`idx < self.len()`)
    #[inline]
    unsafe fn get_things_range(self, idx: usize) -> Range<usize> {
        let start = self.indices_slice().get_unchecked(idx);
        let end = if idx + 1 == self.len() {
            self.things.len()
        } else {
            self.indices_slice().get_unchecked(idx + 1)
        };
        debug_assert!(start <= end);
        start..end
    }

    /// Get the range in `entire_slice` for the element at `idx`. Does not bounds check.
    ///
    /// Safety:
    /// - `idx` must be in bounds (`idx < self.len()`)
    #[inline]
    pub(crate) unsafe fn get_range(self, idx: usize) -> Range<usize> {
        let range = self.get_things_range(idx);
        let offset = (self.things as *const [u8] as *const u8)
            .offset_from(self.entire_slice as *const [u8] as *const u8)
            as usize;
        range.start + offset..range.end + offset
    }

    /// Create an iterator over the Ts contained in VarZeroVecComponents, checking internal invariants:
    ///
    /// - `indices[i]..indices[i+1]` must index into a valid section of
    ///   `things`, such that it parses to a `T::VarULE`
    /// - `indices[len - 1]..things.len()` must index into a valid section of
    ///   `things`, such that it parses to a `T::VarULE`
    /// - `indices` is monotonically increasing
    ///
    /// This method is NOT allowed to call any other methods on VarZeroVecComponents since all other methods
    /// assume that the slice has been passed through iter_checked
    #[inline]
    fn iter_checked(self) -> impl Iterator<Item = Result<&'a T, ZeroVecError>> {
        self.indices_slice()
            .iter_pairs()
            .map(move |(start, maybe_end)| {
                let end = match maybe_end {
                    Some(x) => x,
                    None => self.things.len(),
                };
                // the .get() here automatically verifies that end>=start
                self.things
                    .get(start..end)
                    .ok_or(ZeroVecError::VarZeroVecFormatError)
            })
            .map(|s| s.and_then(|s| T::parse_byte_slice(s)))
    }

    /// Create an iterator over the Ts contained in VarZeroVecComponents
    #[inline]
    pub fn iter(self) -> impl Iterator<Item = &'a T> {
        self.indices_slice()
            .iter_pairs()
            .map(move |(start, maybe_end)| {
                let end = match maybe_end {
                    Some(x) => x,
                    None => self.things.len(),
                };
                // Safety: invariants on the indices array
                unsafe { self.things.get_unchecked(start..end) }
            })
            .map(|s| unsafe { T::from_byte_slice_unchecked(s) })
    }

    pub fn to_vec(self) -> Vec<Box<T>> {
        self.iter().map(T::to_boxed).collect()
    }

    #[inline]
    fn indices_slice(&self) -> &'a FlexZeroSlice {
        unsafe { FlexZeroSlice::from_byte_slice_unchecked(self.indices_with_meta) }
    }

    // Dump a debuggable representation of this type
    #[allow(unused)] // useful for debugging
    pub(crate) fn dump(&self) -> String {
        let indices = self.indices_slice().iter().collect::<Vec<_>>();
        format!("VarZeroVecComponents {{ indices: {:?} }}", indices)
    }
}

impl<'a, T> VarZeroVecComponents<'a, T>
where
    T: VarULE,
    T: ?Sized,
    T: Ord,
{
    /// Binary searches a sorted `VarZeroVecComponents<T>` for the given element.
    ///
    /// For more information, see the std function [`binary_search_by`](slice::binary_search_by).
    #[inline]
    pub fn binary_search(&self, needle: &T) -> Result<usize, usize> {
        self.binary_search_by(|probe| probe.cmp(needle))
    }

    /// Binary searches a sorted range of a `VarZeroVecComponents<T>` for the given element.
    ///
    /// For more information, see the std function [`binary_search_by`](slice::binary_search_by).
    #[inline]
    pub fn binary_search_in_range(
        &self,
        needle: &T,
        range: Range<usize>,
    ) -> Option<Result<usize, usize>> {
        self.binary_search_in_range_by(|probe| probe.cmp(needle), range)
    }
}

impl<'a, T> VarZeroVecComponents<'a, T>
where
    T: VarULE,
    T: ?Sized,
{
    /// Binary searches a sorted `VarZeroVecComponents<T>` for the given predicate.
    ///
    /// For more information, see the std function [`binary_search_by`](slice::binary_search_by).
    pub fn binary_search_by(
        &self,
        mut predicate: impl FnMut(&T) -> Ordering,
    ) -> Result<usize, usize> {
        self.indices_slice().binary_search_with_index(|index| {
            // Safety: `index` is in-range by contract
            let start = unsafe { self.indices_slice().get_unchecked(index) };
            let end = if index < self.indices_slice().len() {
                unsafe { self.indices_slice().get_unchecked(index + 1) }
            } else {
                self.things.len()
            };
            // Safety: invariants on the indices array
            predicate(unsafe {
                T::from_byte_slice_unchecked(self.things.get_unchecked(start..end))
            })
        })
    }

    /// Binary searches a sorted range of a `VarZeroVecComponents<T>` for the given predicate.
    ///
    /// For more information, see the std function [`binary_search_by`](slice::binary_search_by).
    pub fn binary_search_in_range_by(
        &self,
        mut predicate: impl FnMut(&T) -> Ordering,
        range: Range<usize>,
    ) -> Option<Result<usize, usize>> {
        self.indices_slice().binary_search_in_range_with_index(
            |index| {
                // Safety: `index` is in-range by contract
                let start = unsafe { self.indices_slice().get_unchecked(index) };
                let end = if index < self.indices_slice().len() {
                    unsafe { self.indices_slice().get_unchecked(index + 1) }
                } else {
                    self.things.len()
                };
                // Safety: invariants on the indices array
                predicate(unsafe {
                    T::from_byte_slice_unchecked(self.things.get_unchecked(start..end))
                })
            },
            range,
        )
    }
}

/// Collects the bytes for a VarZeroSlice into a Vec.
pub fn get_serializable_bytes<T, A>(elements: &[A]) -> Option<Vec<u8>>
where
    T: VarULE + ?Sized,
    A: EncodeAsVarULE<T>,
{
    let len = compute_serializable_len(elements)?;
    debug_assert!(len >= LENGTH_WIDTH as u32);
    let mut output: Vec<u8> = alloc::vec![0; len as usize];
    write_serializable_bytes(elements, &mut output);
    Some(output)
}

/// Writes the bytes for a VarZeroSlice into an output buffer.
///
/// Every byte in the buffer will be initialized after calling this function.
///
/// # Panics
///
/// Panics if the buffer is not exactly the correct length.
#[allow(clippy::indexing_slicing)] // Function contract allows panicky behavior
pub fn write_serializable_bytes<T, A>(elements: &[A], output: &mut [u8])
where
    T: VarULE + ?Sized,
    A: EncodeAsVarULE<T>,
{
    assert!(elements.len() <= MAX_LENGTH);
    let num_elements_bytes = elements.len().to_le_bytes();
    output[0..LENGTH_WIDTH].copy_from_slice(&num_elements_bytes[0..LENGTH_WIDTH]);

    assert_eq!(METADATA_WIDTH, 1);
    output[LENGTH_WIDTH] = INDEX_WIDTH as u8;

    // idx_offset = offset from the start of the buffer for the next index
    let mut idx_offset: usize = LENGTH_WIDTH + METADATA_WIDTH;
    // first_dat_offset = offset from the start of the buffer of the first data block
    let first_dat_offset: usize = idx_offset + elements.len() * INDEX_WIDTH;
    // dat_offset = offset from the start of the buffer of the next data block
    let mut dat_offset: usize = first_dat_offset;

    for element in elements.iter() {
        let element_len = element.encode_var_ule_len();

        let idx_limit = idx_offset + INDEX_WIDTH;
        let idx_slice = &mut output[idx_offset..idx_limit];
        // VZV expects data offsets to be stored relative to the first data block
        let idx = dat_offset - first_dat_offset;
        assert!(idx <= MAX_INDEX);
        idx_slice.copy_from_slice(&idx.to_le_bytes()[..INDEX_WIDTH]);

        let dat_limit = dat_offset + element_len;
        let dat_slice = &mut output[dat_offset..dat_limit];
        element.encode_var_ule_write(dat_slice);
        debug_assert_eq!(T::validate_byte_slice(dat_slice), Ok(()));

        idx_offset = idx_limit;
        dat_offset = dat_limit;
    }

    debug_assert_eq!(
        idx_offset,
        LENGTH_WIDTH + METADATA_WIDTH + INDEX_WIDTH * elements.len()
    );
    assert_eq!(dat_offset, output.len());
}

pub fn compute_serializable_len<T, A>(elements: &[A]) -> Option<u32>
where
    T: VarULE + ?Sized,
    A: EncodeAsVarULE<T>,
{
    let idx_len: u32 = u32::try_from(elements.len())
        .ok()?
        .checked_mul(INDEX_WIDTH as u32)?
        .checked_add(LENGTH_WIDTH as u32)?
        .checked_add(METADATA_WIDTH as u32)?;
    let data_len: u32 = elements
        .iter()
        .map(|v| u32::try_from(v.encode_var_ule_len()).ok())
        .fold(Some(0u32), |s, v| {
            s.and_then(|s| v.and_then(|v| s.checked_add(v)))
        })?;
    idx_len.checked_add(data_len)
}
