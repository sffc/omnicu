// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::fallback::LocaleFallbackLikelySubtagsV1;
use icu_locid::subtags::{Language, Region, Script};
use icu_locid_transform::provider::LikelySubtagsV1;

use super::provider::DEFAULT_SCRIPT;

/// A type that performs likely subtags operations required for locale fallback.
pub trait ExpanderAdapter {
    fn l2sr(&self, language: Language) -> (Option<Script>, Option<Region>);
    fn lr2s(&self, language: Language, region: Region) -> Option<Script>;
    fn ls2r(&self, language: Language, script: Script) -> Option<Region>;
    fn default_script(&self) -> Script;
}

impl<'a, T> ExpanderAdapter for &'a T
where
    T: ExpanderAdapter,
{
    fn l2sr(&self, language: Language) -> (Option<Script>, Option<Region>) {
        (*self).l2sr(language)
    }
    fn lr2s(&self, language: Language, region: Region) -> Option<Script> {
        (*self).lr2s(language, region)
    }
    fn ls2r(&self, language: Language, script: Script) -> Option<Region> {
        (*self).ls2r(language, script)
    }
    fn default_script(&self) -> Script {
        (*self).default_script()
    }
}

impl<'a> ExpanderAdapter for LocaleFallbackLikelySubtagsV1<'a> {
    fn l2sr(&self, language: Language) -> (Option<Script>, Option<Region>) {
        (
            self.l2s.get(&language.into()).copied(),
            self.l2r.get(&language.into()).copied(),
        )
    }
    fn lr2s(&self, language: Language, region: Region) -> Option<Script> {
        self.lr2s.get_copied_2d(&language.into(), &region.into())
    }
    fn ls2r(&self, language: Language, script: Script) -> Option<Region> {
        self.ls2r.get_2d(&language.into(), &script.into()).copied()
    }
    fn default_script(&self) -> Script {
        DEFAULT_SCRIPT
    }
}

impl<'a> ExpanderAdapter for LikelySubtagsV1<'a> {
    fn l2sr(&self, language: Language) -> (Option<Script>, Option<Region>) {
        match self.language.get_copied(&language.into()) {
            Some((script, region)) => (Some(script), Some(region)),
            None => (None, None),
        }
    }
    fn lr2s(&self, language: Language, region: Region) -> Option<Script> {
        self.language_region
            .get_copied(&(language.into(), region.into()))
    }
    fn ls2r(&self, language: Language, script: Script) -> Option<Region> {
        self.language_script
            .get_copied(&(language.into(), script.into()))
    }
    fn default_script(&self) -> Script {
        self.und.1
    }
}
