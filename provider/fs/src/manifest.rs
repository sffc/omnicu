// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::buf::BufferFormat;
use icu_provider::prelude::*;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, PartialEq)]
pub struct Manifest {
    /// Which data serialization file format is used.
    pub buffer_format: BufferFormat,
    /// The file extension associated with the given buffer format in the manifest.
    pub file_extension: &'static str,
    /// The locales that are supported as fully-resolved in this provider.
    pub supported_locales: Vec<Locale>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct JsonManifest {
    #[serde(rename = "syntax")]
    pub buffer_format: BufferFormat,
    pub supported_locales: Vec<Locale>,
}

impl Manifest {
    const NAME: &'static str = "manifest.json";

    pub(crate) fn try_new(buffer_format: BufferFormat, supported_locales: Vec<Locale>) -> Result<Self, DataError> {
        buffer_format.check_available()?;
        Ok(Self {
            buffer_format,
            file_extension: match buffer_format {
                BufferFormat::Json => "json",
                BufferFormat::Bincode1 => "bincode",
                BufferFormat::Postcard1 => "postcard",
                // BufferFormat is non_exhaustive, so we need a catchall case.
                // This case could be triggered if a new buffer format is added to the core library
                // before it gets added to FsDataProvider.
                bf => {
                    return Err(DataErrorKind::UnavailableBufferFormat(bf)
                        .with_str_context("Format not supported by FsDataProvider"))
                }
            },
            supported_locales,
        })
    }

    pub fn parse(root: &Path) -> Result<Self, DataError> {
        let path = root.join(Self::NAME);
        let result: JsonManifest = serde_json_core::from_str(
            &fs::read_to_string(&path).map_err(|e| DataError::from(e).with_path_context(&path))?,
        )
        .map_err(|e| {
            DataError::custom("FsDataProvider manifest deserialization")
                .with_path_context(&path)
                .with_display_context(&e)
        })?
        .0;
        Self::try_new(result.buffer_format, result.supported_locales)
    }

    #[cfg(feature = "export")]
    pub fn write(&self, root: &Path) -> Result<(), DataError> {
        let path = root.join(Self::NAME);
        let mut file = crlify::BufWriterWithLineEndingFix::new(
            fs::File::create(&path).map_err(|e| DataError::from(e).with_path_context(&path))?,
        );
        serde::Serialize::serialize(
            &JsonManifest {
                buffer_format: self.buffer_format,
                supported_locales: self.supported_locales.clone(),
            },
            &mut serde_json::Serializer::pretty(&mut file),
        )
        .map_err(|e| {
            DataError::custom("FsDataProvider manifest serialization")
                .with_path_context(&path)
                .with_display_context(&e)
        })?;
        use std::io::Write;
        writeln!(&mut file).map_err(|e| DataError::from(e).with_path_context(&path))
    }
}
