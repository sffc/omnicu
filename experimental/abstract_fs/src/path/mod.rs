// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod impl_path;

pub trait PathIterator {
    fn reset(&mut self);
    fn next(&mut self) -> Option<&str>;
    fn is_absolute(&self) -> bool;
}
