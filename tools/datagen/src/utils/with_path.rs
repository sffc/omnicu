// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::io;
use std::path::PathBuf;

trait WithPath {
    type Return;

    fn wrap_err_with_path(self, path: PathBuf) -> Self::Return;
}

impl WithPath for io::Error {
    type Return = eyre::Report;

    fn wrap_err_with_path(self, path: PathBuf) -> Self::Return {
        eyre::Report::new(self).wrap_err(format!("(path: {:?})", path))
    }
}

impl<T> WithPath for Result<T, io::Error> {
    type Return = Result<T, <io::Error as WithPath>::Return>;

    fn wrap_err_with_path(self, path: PathBuf) -> Self::Return {
        self.map_err(|err| err.wrap_err_with_path(path))
    }
}
