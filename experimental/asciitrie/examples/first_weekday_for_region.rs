// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// This example demonstrates the use of AsciiTrie to look up data based on a region code.

#![no_main] // https://github.com/unicode-org/icu4x/issues/395

use asciitrie::AsciiStr;
use asciitrie::AsciiTrie;

icu_benchmark_macros::static_setup!();

mod weekday {
    pub const MON: usize = 1;
    pub const FRI: usize = 5;
    pub const SAT: usize = 6;
    pub const SUN: usize = 7;
}

// This data originates from CLDR 41.
static TRIE: AsciiTrie<[u8; 561]> = AsciiTrie::from_sorted_asciistr_value_slice(&[
    (AsciiStr::from_str_or_panic("001"), weekday::MON),
    (AsciiStr::from_str_or_panic("AD"), weekday::MON),
    (AsciiStr::from_str_or_panic("AE"), weekday::SAT),
    (AsciiStr::from_str_or_panic("AF"), weekday::SAT),
    (AsciiStr::from_str_or_panic("AG"), weekday::SUN),
    (AsciiStr::from_str_or_panic("AI"), weekday::MON),
    (AsciiStr::from_str_or_panic("AL"), weekday::MON),
    (AsciiStr::from_str_or_panic("AM"), weekday::MON),
    (AsciiStr::from_str_or_panic("AN"), weekday::MON),
    (AsciiStr::from_str_or_panic("AR"), weekday::MON),
    (AsciiStr::from_str_or_panic("AS"), weekday::SUN),
    (AsciiStr::from_str_or_panic("AT"), weekday::MON),
    (AsciiStr::from_str_or_panic("AU"), weekday::MON),
    (AsciiStr::from_str_or_panic("AX"), weekday::MON),
    (AsciiStr::from_str_or_panic("AZ"), weekday::MON),
    (AsciiStr::from_str_or_panic("BA"), weekday::MON),
    (AsciiStr::from_str_or_panic("BD"), weekday::SUN),
    (AsciiStr::from_str_or_panic("BE"), weekday::MON),
    (AsciiStr::from_str_or_panic("BG"), weekday::MON),
    (AsciiStr::from_str_or_panic("BH"), weekday::SAT),
    (AsciiStr::from_str_or_panic("BM"), weekday::MON),
    (AsciiStr::from_str_or_panic("BN"), weekday::MON),
    (AsciiStr::from_str_or_panic("BR"), weekday::SUN),
    (AsciiStr::from_str_or_panic("BS"), weekday::SUN),
    (AsciiStr::from_str_or_panic("BT"), weekday::SUN),
    (AsciiStr::from_str_or_panic("BW"), weekday::SUN),
    (AsciiStr::from_str_or_panic("BY"), weekday::MON),
    (AsciiStr::from_str_or_panic("BZ"), weekday::SUN),
    (AsciiStr::from_str_or_panic("CA"), weekday::SUN),
    (AsciiStr::from_str_or_panic("CH"), weekday::MON),
    (AsciiStr::from_str_or_panic("CL"), weekday::MON),
    (AsciiStr::from_str_or_panic("CM"), weekday::MON),
    (AsciiStr::from_str_or_panic("CN"), weekday::SUN),
    (AsciiStr::from_str_or_panic("CO"), weekday::SUN),
    (AsciiStr::from_str_or_panic("CR"), weekday::MON),
    (AsciiStr::from_str_or_panic("CY"), weekday::MON),
    (AsciiStr::from_str_or_panic("CZ"), weekday::MON),
    (AsciiStr::from_str_or_panic("DE"), weekday::MON),
    (AsciiStr::from_str_or_panic("DJ"), weekday::SAT),
    (AsciiStr::from_str_or_panic("DK"), weekday::MON),
    (AsciiStr::from_str_or_panic("DM"), weekday::SUN),
    (AsciiStr::from_str_or_panic("DO"), weekday::SUN),
    (AsciiStr::from_str_or_panic("DZ"), weekday::SAT),
    (AsciiStr::from_str_or_panic("EC"), weekday::MON),
    (AsciiStr::from_str_or_panic("EE"), weekday::MON),
    (AsciiStr::from_str_or_panic("EG"), weekday::SAT),
    (AsciiStr::from_str_or_panic("ES"), weekday::MON),
    (AsciiStr::from_str_or_panic("ET"), weekday::SUN),
    (AsciiStr::from_str_or_panic("FI"), weekday::MON),
    (AsciiStr::from_str_or_panic("FJ"), weekday::MON),
    (AsciiStr::from_str_or_panic("FO"), weekday::MON),
    (AsciiStr::from_str_or_panic("FR"), weekday::MON),
    (AsciiStr::from_str_or_panic("GB"), weekday::MON),
    (AsciiStr::from_str_or_panic("GB-alt-variant"), weekday::SUN),
    (AsciiStr::from_str_or_panic("GE"), weekday::MON),
    (AsciiStr::from_str_or_panic("GF"), weekday::MON),
    (AsciiStr::from_str_or_panic("GP"), weekday::MON),
    (AsciiStr::from_str_or_panic("GR"), weekday::MON),
    (AsciiStr::from_str_or_panic("GT"), weekday::SUN),
    (AsciiStr::from_str_or_panic("GU"), weekday::SUN),
    (AsciiStr::from_str_or_panic("HK"), weekday::SUN),
    (AsciiStr::from_str_or_panic("HN"), weekday::SUN),
    (AsciiStr::from_str_or_panic("HR"), weekday::MON),
    (AsciiStr::from_str_or_panic("HU"), weekday::MON),
    (AsciiStr::from_str_or_panic("ID"), weekday::SUN),
    (AsciiStr::from_str_or_panic("IE"), weekday::MON),
    (AsciiStr::from_str_or_panic("IL"), weekday::SUN),
    (AsciiStr::from_str_or_panic("IN"), weekday::SUN),
    (AsciiStr::from_str_or_panic("IQ"), weekday::SAT),
    (AsciiStr::from_str_or_panic("IR"), weekday::SAT),
    (AsciiStr::from_str_or_panic("IS"), weekday::MON),
    (AsciiStr::from_str_or_panic("IT"), weekday::MON),
    (AsciiStr::from_str_or_panic("JM"), weekday::SUN),
    (AsciiStr::from_str_or_panic("JO"), weekday::SAT),
    (AsciiStr::from_str_or_panic("JP"), weekday::SUN),
    (AsciiStr::from_str_or_panic("KE"), weekday::SUN),
    (AsciiStr::from_str_or_panic("KG"), weekday::MON),
    (AsciiStr::from_str_or_panic("KH"), weekday::SUN),
    (AsciiStr::from_str_or_panic("KR"), weekday::SUN),
    (AsciiStr::from_str_or_panic("KW"), weekday::SAT),
    (AsciiStr::from_str_or_panic("KZ"), weekday::MON),
    (AsciiStr::from_str_or_panic("LA"), weekday::SUN),
    (AsciiStr::from_str_or_panic("LB"), weekday::MON),
    (AsciiStr::from_str_or_panic("LI"), weekday::MON),
    (AsciiStr::from_str_or_panic("LK"), weekday::MON),
    (AsciiStr::from_str_or_panic("LT"), weekday::MON),
    (AsciiStr::from_str_or_panic("LU"), weekday::MON),
    (AsciiStr::from_str_or_panic("LV"), weekday::MON),
    (AsciiStr::from_str_or_panic("LY"), weekday::SAT),
    (AsciiStr::from_str_or_panic("MC"), weekday::MON),
    (AsciiStr::from_str_or_panic("MD"), weekday::MON),
    (AsciiStr::from_str_or_panic("ME"), weekday::MON),
    (AsciiStr::from_str_or_panic("MH"), weekday::SUN),
    (AsciiStr::from_str_or_panic("MK"), weekday::MON),
    (AsciiStr::from_str_or_panic("MM"), weekday::SUN),
    (AsciiStr::from_str_or_panic("MN"), weekday::MON),
    (AsciiStr::from_str_or_panic("MO"), weekday::SUN),
    (AsciiStr::from_str_or_panic("MQ"), weekday::MON),
    (AsciiStr::from_str_or_panic("MT"), weekday::SUN),
    (AsciiStr::from_str_or_panic("MV"), weekday::FRI),
    (AsciiStr::from_str_or_panic("MX"), weekday::SUN),
    (AsciiStr::from_str_or_panic("MY"), weekday::MON),
    (AsciiStr::from_str_or_panic("MZ"), weekday::SUN),
    (AsciiStr::from_str_or_panic("NI"), weekday::SUN),
    (AsciiStr::from_str_or_panic("NL"), weekday::MON),
    (AsciiStr::from_str_or_panic("NO"), weekday::MON),
    (AsciiStr::from_str_or_panic("NP"), weekday::SUN),
    (AsciiStr::from_str_or_panic("NZ"), weekday::MON),
    (AsciiStr::from_str_or_panic("OM"), weekday::SAT),
    (AsciiStr::from_str_or_panic("PA"), weekday::SUN),
    (AsciiStr::from_str_or_panic("PE"), weekday::SUN),
    (AsciiStr::from_str_or_panic("PH"), weekday::SUN),
    (AsciiStr::from_str_or_panic("PK"), weekday::SUN),
    (AsciiStr::from_str_or_panic("PL"), weekday::MON),
    (AsciiStr::from_str_or_panic("PR"), weekday::SUN),
    (AsciiStr::from_str_or_panic("PT"), weekday::SUN),
    (AsciiStr::from_str_or_panic("PY"), weekday::SUN),
    (AsciiStr::from_str_or_panic("QA"), weekday::SAT),
    (AsciiStr::from_str_or_panic("RE"), weekday::MON),
    (AsciiStr::from_str_or_panic("RO"), weekday::MON),
    (AsciiStr::from_str_or_panic("RS"), weekday::MON),
    (AsciiStr::from_str_or_panic("RU"), weekday::MON),
    (AsciiStr::from_str_or_panic("SA"), weekday::SUN),
    (AsciiStr::from_str_or_panic("SD"), weekday::SAT),
    (AsciiStr::from_str_or_panic("SE"), weekday::MON),
    (AsciiStr::from_str_or_panic("SG"), weekday::SUN),
    (AsciiStr::from_str_or_panic("SI"), weekday::MON),
    (AsciiStr::from_str_or_panic("SK"), weekday::MON),
    (AsciiStr::from_str_or_panic("SM"), weekday::MON),
    (AsciiStr::from_str_or_panic("SV"), weekday::SUN),
    (AsciiStr::from_str_or_panic("SY"), weekday::SAT),
    (AsciiStr::from_str_or_panic("TH"), weekday::SUN),
    (AsciiStr::from_str_or_panic("TJ"), weekday::MON),
    (AsciiStr::from_str_or_panic("TM"), weekday::MON),
    (AsciiStr::from_str_or_panic("TR"), weekday::MON),
    (AsciiStr::from_str_or_panic("TT"), weekday::SUN),
    (AsciiStr::from_str_or_panic("TW"), weekday::SUN),
    (AsciiStr::from_str_or_panic("UA"), weekday::MON),
    (AsciiStr::from_str_or_panic("UM"), weekday::SUN),
    (AsciiStr::from_str_or_panic("US"), weekday::SUN),
    (AsciiStr::from_str_or_panic("UY"), weekday::MON),
    (AsciiStr::from_str_or_panic("UZ"), weekday::MON),
    (AsciiStr::from_str_or_panic("VA"), weekday::MON),
    (AsciiStr::from_str_or_panic("VE"), weekday::SUN),
    (AsciiStr::from_str_or_panic("VI"), weekday::SUN),
    (AsciiStr::from_str_or_panic("VN"), weekday::MON),
    (AsciiStr::from_str_or_panic("WS"), weekday::SUN),
    (AsciiStr::from_str_or_panic("XK"), weekday::MON),
    (AsciiStr::from_str_or_panic("YE"), weekday::SUN),
    (AsciiStr::from_str_or_panic("ZA"), weekday::SUN),
    (AsciiStr::from_str_or_panic("ZW"), weekday::SUN),
]);

fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    icu_benchmark_macros::main_setup!();

    if black_box(&TRIE).get(b"MV") == Some(weekday::FRI) {
        0
    } else {
        1
    }
}
