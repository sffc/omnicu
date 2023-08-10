// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::calendar::*;
use icu_calendar::any_calendar::AnyCalendarKind;
use icu_calendar::chinese::Chinese;
use icu_calendar::roc::Roc;
use icu_calendar::{
    buddhist::Buddhist, coptic::Coptic, ethiopian::Ethiopian, hebrew::Hebrew, indian::Indian,
    islamic::IslamicCivil, islamic::IslamicObservational, islamic::IslamicTabular,
    islamic::UmmAlQura, japanese::Japanese, japanese::JapaneseExtended, persian::Persian,
    Gregorian,
};
use icu_locid::extensions::unicode::{value, Value};
use icu_provider::prelude::*;

/// A calendar that can be found in CLDR
///
/// New implementors of this trait will likely also wish to modify `get_era_code_map()`
/// in the CLDR transformer to support any new era maps.
pub trait CldrCalendar {
    /// The Unicode BCP 47 identifier for the calendar
    /// If multiple BCP 47 identifiers work, this should be
    /// the default one when no others are provided
    const DEFAULT_BCP_47_IDENTIFIER: Value;

    /// The data marker for loading symbols for this calendar.
    type DateSymbolsV1Marker: KeyedDataMarker<Yokeable = DateSymbolsV1<'static>>;

    /// The data marker for loading length-patterns for this calendar.
    type DateLengthsV1Marker: KeyedDataMarker<Yokeable = DateLengthsV1<'static>>;

    /// Checks if a given BCP 47 identifier is allowed to be used with this calendar
    ///
    /// By default, just checks against DEFAULT_BCP_47_IDENTIFIER
    fn is_identifier_allowed_for_calendar(value: &Value) -> bool {
        *value == Self::DEFAULT_BCP_47_IDENTIFIER
    }
}

impl CldrCalendar for Gregorian {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("gregory");
    type DateSymbolsV1Marker = GregorianDateSymbolsV1Marker;
    type DateLengthsV1Marker = GregorianDateLengthsV1Marker;
}

impl CldrCalendar for Buddhist {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("buddhist");
    type DateSymbolsV1Marker = BuddhistDateSymbolsV1Marker;
    type DateLengthsV1Marker = BuddhistDateLengthsV1Marker;
}

impl CldrCalendar for Chinese {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("chinese");
    type DateSymbolsV1Marker = ChineseDateSymbolsV1Marker;
    type DateLengthsV1Marker = ChineseDateLengthsV1Marker;
}

impl CldrCalendar for Japanese {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("japanese");
    type DateSymbolsV1Marker = JapaneseDateSymbolsV1Marker;
    type DateLengthsV1Marker = JapaneseDateLengthsV1Marker;
}

impl CldrCalendar for JapaneseExtended {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("japanext");
    type DateSymbolsV1Marker = JapaneseExtendedDateSymbolsV1Marker;
    type DateLengthsV1Marker = JapaneseExtendedDateLengthsV1Marker;
}

impl CldrCalendar for Coptic {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("coptic");
    type DateSymbolsV1Marker = CopticDateSymbolsV1Marker;
    type DateLengthsV1Marker = CopticDateLengthsV1Marker;
}

impl CldrCalendar for Indian {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("indian");
    type DateSymbolsV1Marker = IndianDateSymbolsV1Marker;
    type DateLengthsV1Marker = IndianDateLengthsV1Marker;
}

impl CldrCalendar for Persian {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("persian");
    type DateSymbolsV1Marker = PersianDateSymbolsV1Marker;
    type DateLengthsV1Marker = PersianDateLengthsV1Marker;
}

impl CldrCalendar for Hebrew {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("hebrew");
    type DateSymbolsV1Marker = HebrewDateSymbolsV1Marker;
    type DateLengthsV1Marker = HebrewDateLengthsV1Marker;
}

impl CldrCalendar for IslamicObservational {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("islamic");
    type DateSymbolsV1Marker = IslamicObservationalDateSymbolsV1Marker;
    type DateLengthsV1Marker = IslamicObservationalDateLengthsV1Marker;
}

impl CldrCalendar for IslamicCivil {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("islamicc");
    type DateSymbolsV1Marker = IslamicCivilDateSymbolsV1Marker;
    type DateLengthsV1Marker = IslamicCivilDateLengthsV1Marker;
}

impl CldrCalendar for UmmAlQura {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("umalqura");
    type DateSymbolsV1Marker = UmmAlQuraDateSymbolsV1Marker;
    type DateLengthsV1Marker = UmmAlQuraDateLengthsV1Marker;
}

impl CldrCalendar for IslamicTabular {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("tbla");
    type DateSymbolsV1Marker = IslamicTabularDateSymbolsV1Marker;
    type DateLengthsV1Marker = IslamicTabularDateLengthsV1Marker;
}

impl CldrCalendar for Ethiopian {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("ethiopic");
    type DateSymbolsV1Marker = EthiopianDateSymbolsV1Marker;
    type DateLengthsV1Marker = EthiopianDateLengthsV1Marker;
    fn is_identifier_allowed_for_calendar(value: &Value) -> bool {
        *value == value!("ethiopic") || *value == value!("ethioaa")
    }
}

impl CldrCalendar for Roc {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("roc");
    type DateSymbolsV1Marker = RocDateSymbolsV1Marker;
    type DateLengthsV1Marker = RocDateLengthsV1Marker;
}

pub(crate) fn load_lengths_for_cldr_calendar<C, P>(
    provider: &P,
    locale: &DataLocale,
) -> Result<DataPayload<ErasedDateLengthsV1Marker>, DataError>
where
    C: CldrCalendar,
    P: DataProvider<<C as CldrCalendar>::DateLengthsV1Marker> + ?Sized,
{
    let payload = provider
        .load(DataRequest {
            locale,
            metadata: Default::default(),
        })?
        .take_payload()?;
    Ok(payload.cast())
}

pub(crate) fn load_symbols_for_cldr_calendar<C, P>(
    provider: &P,
    locale: &DataLocale,
) -> Result<DataPayload<ErasedDateSymbolsV1Marker>, DataError>
where
    C: CldrCalendar,
    P: DataProvider<<C as CldrCalendar>::DateSymbolsV1Marker> + ?Sized,
{
    let payload = provider
        .load(DataRequest {
            locale,
            metadata: Default::default(),
        })?
        .take_payload()?;
    Ok(payload.cast())
}

pub(crate) fn load_lengths_for_any_calendar_kind<P>(
    provider: &P,
    locale: &DataLocale,
    kind: AnyCalendarKind,
) -> Result<DataPayload<ErasedDateLengthsV1Marker>, DataError>
where
    P: DataProvider<GregorianDateLengthsV1Marker>
        + DataProvider<BuddhistDateLengthsV1Marker>
        + DataProvider<ChineseDateLengthsV1Marker>
        + DataProvider<JapaneseDateLengthsV1Marker>
        + DataProvider<JapaneseExtendedDateLengthsV1Marker>
        + DataProvider<CopticDateLengthsV1Marker>
        + DataProvider<IndianDateLengthsV1Marker>
        + DataProvider<IslamicObservationalDateLengthsV1Marker>
        + DataProvider<IslamicCivilDateLengthsV1Marker>
        + DataProvider<UmmAlQuraDateLengthsV1Marker>
        + DataProvider<IslamicTabularDateLengthsV1Marker>
        + DataProvider<PersianDateLengthsV1Marker>
        + DataProvider<HebrewDateLengthsV1Marker>
        + DataProvider<EthiopianDateLengthsV1Marker>
        + DataProvider<RocDateLengthsV1Marker>
        + ?Sized,
{
    let req = DataRequest {
        locale,
        metadata: Default::default(),
    };
    let payload = match kind {
        AnyCalendarKind::Gregorian => {
            DataProvider::<<Gregorian as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Buddhist => {
            DataProvider::<<Buddhist as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Chinese => {
            DataProvider::<<Chinese as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Japanese => {
            DataProvider::<<Japanese as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::JapaneseExtended => DataProvider::<
            <JapaneseExtended as CldrCalendar>::DateLengthsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::Indian => {
            DataProvider::<<Indian as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::IslamicObservational => DataProvider::<
            <IslamicObservational as CldrCalendar>::DateLengthsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::IslamicCivil => DataProvider::<
            <IslamicCivil as CldrCalendar>::DateLengthsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::UmmAlQura => {
            DataProvider::<<UmmAlQura as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::IslamicTabular => DataProvider::<
            <IslamicTabular as CldrCalendar>::DateLengthsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::Persian => {
            DataProvider::<<Persian as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Hebrew => {
            DataProvider::<<Hebrew as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Coptic => {
            DataProvider::<<Coptic as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Ethiopian => {
            DataProvider::<<Ethiopian as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::EthiopianAmeteAlem => {
            DataProvider::<<Ethiopian as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Roc => {
            DataProvider::<<Roc as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        _ => {
            return Err(
                DataError::custom("Don't know how to load data for specified calendar")
                    .with_debug_context(&kind),
            )
        }
    };
    Ok(payload)
}

pub(crate) fn load_symbols_for_any_calendar_kind<P>(
    provider: &P,
    locale: &DataLocale,
    kind: AnyCalendarKind,
) -> Result<DataPayload<ErasedDateSymbolsV1Marker>, DataError>
where
    P: DataProvider<GregorianDateSymbolsV1Marker>
        + DataProvider<BuddhistDateSymbolsV1Marker>
        + DataProvider<ChineseDateSymbolsV1Marker>
        + DataProvider<JapaneseDateSymbolsV1Marker>
        + DataProvider<JapaneseExtendedDateSymbolsV1Marker>
        + DataProvider<CopticDateSymbolsV1Marker>
        + DataProvider<IndianDateSymbolsV1Marker>
        + DataProvider<IslamicObservationalDateSymbolsV1Marker>
        + DataProvider<IslamicCivilDateSymbolsV1Marker>
        + DataProvider<UmmAlQuraDateSymbolsV1Marker>
        + DataProvider<IslamicTabularDateSymbolsV1Marker>
        + DataProvider<PersianDateSymbolsV1Marker>
        + DataProvider<HebrewDateSymbolsV1Marker>
        + DataProvider<EthiopianDateSymbolsV1Marker>
        + DataProvider<RocDateSymbolsV1Marker>
        + ?Sized,
{
    let req = DataRequest {
        locale,
        metadata: Default::default(),
    };
    let payload = match kind {
        AnyCalendarKind::Gregorian => {
            DataProvider::<<Gregorian as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Buddhist => {
            DataProvider::<<Buddhist as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Chinese => {
            DataProvider::<<Chinese as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Japanese => {
            DataProvider::<<Japanese as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::JapaneseExtended => DataProvider::<
            <JapaneseExtended as CldrCalendar>::DateSymbolsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::Indian => {
            DataProvider::<<Indian as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::IslamicObservational => DataProvider::<
            <IslamicObservational as CldrCalendar>::DateSymbolsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::IslamicCivil => DataProvider::<
            <IslamicCivil as CldrCalendar>::DateSymbolsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::UmmAlQura => {
            DataProvider::<<UmmAlQura as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::IslamicTabular => DataProvider::<
            <IslamicTabular as CldrCalendar>::DateSymbolsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::Persian => {
            DataProvider::<<Persian as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Hebrew => {
            DataProvider::<<Hebrew as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Coptic => {
            DataProvider::<<Coptic as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Ethiopian => {
            DataProvider::<<Ethiopian as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::EthiopianAmeteAlem => {
            DataProvider::<<Ethiopian as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Roc => {
            DataProvider::<<Roc as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        _ => {
            return Err(
                DataError::custom("Don't know how to load data for specified calendar")
                    .with_debug_context(&kind),
            )
        }
    };
    Ok(payload)
}
