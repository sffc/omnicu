// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Traits for inspecting the supported languages/locales of a provider

use crate::{ResourceOptions, ResourceKey};

// pub enum SupportedLocalesMode {
//     Resolved,
//     Exhaustive,
// }

pub enum SupportedLocalesJoiner {
    Union,
    Intersection,
}

impl Default for SupportedLocalesMode {
    fn default() -> Self {
        Self::Resolved
    }
}

pub trait SupportedLocales {
    pub fn supported_locales(&self, joiner: SupportedLocalesJoiner) -> Vec<Locale>;
    pub fn supports_locale(&self, locale: &Locale) -> bool;
    pub fn supported_locales_for_key(&self, key: ResourceKey) -> Result<Vec<Locale>, DataError>;
    // pub fn supports_locale_for_key(&self, key: ResourceKey, locale: &Locale) -> bool;
    pub fn supported_locales_for_key_exhaustive(&self, key: ResourceKey) -> Result<Vec<Locale>, DataError>;
}
