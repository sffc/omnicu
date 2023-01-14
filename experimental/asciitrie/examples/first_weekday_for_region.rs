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
static DATA: &[(&AsciiStr, usize)] = &[
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
];
static TRIE: AsciiTrie<[u8; 561]> = AsciiTrie::from_asciistr_value_slice(DATA);

static TRIE2: [u8; 508] = [
    210, 29, 240, 26, 77, 209, 12, 235, 70, 208, 74, 240, 10, 67, 196, 240, 26, 65, 48, 48, 49,
    161, 208, 4, 225, 78, 200, 225, 71, 194, 225, 69, 68, 161, 166, 70, 166, 167, 194, 225, 76, 73,
    161, 161, 77, 161, 161, 200, 225, 85, 194, 225, 83, 82, 161, 167, 84, 161, 161, 194, 90, 88,
    161, 161, 66, 208, 1, 225, 78, 200, 225, 71, 194, 225, 68, 65, 161, 167, 69, 161, 161, 194, 77,
    72, 166, 161, 161, 200, 225, 87, 194, 225, 83, 82, 167, 167, 84, 167, 167, 194, 90, 89, 161,
    167, 203, 225, 78, 197, 225, 76, 194, 72, 65, 167, 161, 161, 77, 161, 167, 197, 225, 89, 194,
    82, 79, 167, 161, 161, 90, 161, 208, 2, 69, 68, 200, 225, 77, 194, 225, 74, 69, 161, 166, 75,
    161, 167, 194, 90, 79, 167, 166, 197, 225, 71, 194, 69, 67, 161, 161, 166, 194, 84, 83, 161,
    167, 197, 225, 79, 194, 74, 73, 161, 161, 161, 82, 161, 208, 58, 232, 74, 208, 19, 235, 72, 71,
    208, 5, 225, 80, 207, 225, 69, 66, 129, 45, 97, 108, 116, 45, 118, 97, 114, 105, 97, 110, 116,
    167, 161, 70, 161, 161, 194, 225, 84, 82, 161, 167, 85, 167, 197, 225, 82, 194, 78, 75, 167,
    167, 161, 85, 161, 73, 203, 225, 81, 197, 225, 76, 194, 69, 68, 167, 161, 167, 78, 167, 166,
    194, 225, 83, 82, 166, 161, 84, 161, 194, 225, 79, 77, 167, 166, 80, 167, 208, 2, 76, 75, 200,
    225, 82, 194, 225, 71, 69, 167, 161, 72, 167, 167, 194, 90, 87, 166, 161, 203, 225, 84, 197,
    225, 73, 194, 66, 65, 167, 161, 161, 75, 161, 161, 194, 225, 86, 85, 161, 161, 89, 166, 208, 4,
    225, 79, 200, 225, 72, 194, 225, 68, 67, 161, 161, 69, 161, 167, 194, 225, 77, 75, 161, 167,
    78, 161, 167, 200, 225, 88, 194, 225, 84, 81, 161, 167, 86, 165, 167, 194, 90, 89, 161, 167,
    208, 74, 240, 1, 84, 208, 28, 226, 81, 207, 226, 79, 78, 197, 225, 79, 194, 76, 73, 167, 161,
    161, 194, 90, 80, 167, 161, 77, 166, 80, 203, 225, 76, 197, 225, 72, 194, 69, 65, 167, 167,
    167, 75, 167, 161, 194, 225, 84, 82, 167, 167, 89, 167, 65, 166, 204, 83, 82, 197, 225, 83,
    194, 79, 69, 161, 161, 161, 85, 161, 203, 225, 73, 197, 225, 69, 194, 68, 65, 167, 166, 161,
    71, 167, 161, 197, 225, 86, 194, 77, 75, 161, 161, 167, 89, 166, 200, 225, 82, 194, 225, 74,
    72, 167, 161, 77, 161, 161, 194, 87, 84, 167, 167, 208, 16, 226, 88, 207, 235, 86, 85, 197,
    225, 83, 194, 77, 65, 161, 167, 167, 194, 90, 89, 161, 161, 197, 225, 73, 194, 69, 65, 161,
    167, 167, 78, 161, 87, 83, 167, 75, 161, 195, 90, 89, 69, 167, 194, 87, 65, 167, 167,
];

static TRIE3: [u8; 590] = [210, 76, 77, 209, 38, 70, 208, 89, 67, 196, 65, 48, 48, 49, 161, 240, 34, 66, 65, 208, 7, 78, 201, 71, 194, 69, 68, 161, 226, 70, 69, 166, 166, 197, 76, 226, 73, 71, 167, 161, 226, 77, 76, 161, 161, 201, 84, 194, 82, 78, 161, 226, 83, 82, 161, 167, 197, 88, 226, 85, 84, 161, 161, 226, 90, 88, 161, 161, 208, 4, 78, 201, 71, 194, 68, 65, 161, 226, 69, 68, 167, 161, 194, 72, 71, 161, 226, 77, 72, 166, 161, 201, 84, 194, 82, 78, 161, 226, 83, 82, 167, 167, 197, 89, 226, 87, 84, 167, 167, 226, 90, 89, 161, 167, 208, 15, 68, 67, 204, 78, 197, 76, 226, 72, 65, 167, 161, 226, 77, 76, 161, 161, 197, 82, 226, 79, 78, 167, 167, 194, 89, 82, 161, 226, 90, 89, 161, 161, 240, 5, 69, 68, 201, 77, 194, 74, 69, 161, 226, 75, 74, 166, 161, 194, 79, 77, 167, 226, 90, 79, 167, 166, 197, 71, 226, 69, 67, 161, 161, 194, 83, 71, 166, 226, 84, 83, 161, 167, 208, 52, 73, 205, 71, 70, 197, 79, 226, 74, 73, 161, 161, 226, 82, 79, 161, 161, 240, 22, 72, 71, 208, 6, 80, 207, 69, 66, 129, 45, 97, 108, 116, 45, 118, 97, 114, 105, 97, 110, 116, 167, 226, 70, 69, 161, 161, 197, 84, 226, 82, 80, 161, 161, 226, 85, 84, 167, 167, 197, 82, 226, 78, 75, 167, 167, 226, 85, 82, 161, 161, 208, 23, 75, 240, 11, 74, 73, 204, 81, 197, 76, 226, 69, 68, 167, 161, 226, 78, 76, 167, 167, 197, 83, 226, 82, 81, 166, 166, 226, 84, 83, 161, 161, 194, 79, 77, 167, 226, 80, 79, 166, 167, 240, 5, 76, 75, 201, 82, 194, 71, 69, 167, 226, 72, 71, 161, 167, 194, 87, 82, 167, 226, 90, 87, 166, 161, 204, 84, 197, 73, 226, 66, 65, 167, 161, 226, 75, 73, 161, 161, 197, 86, 226, 85, 84, 161, 161, 226, 89, 86, 161, 166, 209, 14, 84, 208, 59, 80, 208, 34, 78, 77, 208, 7, 79, 201, 72, 194, 68, 67, 161, 226, 69, 68, 161, 161, 197, 77, 226, 75, 72, 167, 161, 226, 78, 77, 167, 161, 201, 86, 194, 81, 79, 167, 226, 84, 81, 161, 167, 197, 89, 226, 88, 86, 165, 167, 226, 90, 89, 161, 167, 240, 1, 79, 78, 197, 79, 226, 76, 73, 167, 161, 194, 80, 79, 161, 226, 90, 80, 167, 161, 77, 166, 208, 16, 82, 240, 11, 81, 80, 204, 76, 197, 72, 226, 69, 65, 167, 167, 226, 75, 72, 167, 167, 197, 84, 226, 82, 76, 161, 167, 226, 89, 84, 167, 167, 65, 166, 237, 83, 82, 197, 83, 226, 79, 69, 161, 161, 226, 85, 83, 161, 161, 204, 73, 197, 69, 226, 68, 65, 167, 166, 226, 71, 69, 161, 167, 197, 77, 226, 75, 73, 161, 161, 194, 86, 77, 161, 226, 89, 86, 167, 166, 208, 40, 87, 208, 5, 85, 84, 201, 82, 194, 74, 72, 167, 226, 77, 74, 161, 161, 194, 84, 82, 161, 226, 87, 84, 167, 167, 240, 1, 86, 85, 197, 83, 226, 77, 65, 161, 167, 194, 89, 83, 167, 226, 90, 89, 161, 161, 197, 73, 226, 69, 65, 161, 167, 226, 78, 73, 167, 161, 199, 89, 227, 88, 87, 83, 167, 75, 161, 227, 90, 89, 69, 167, 226, 87, 65, 167, 167];

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

    // #[cfg(any())]
    {
        if AsciiTrie::get(black_box(&TRIE), b"MV") == Some(weekday::FRI) {
            return 0;
        } else {
            return 1;
        }
    }

    #[cfg(any())]
    {
        let trie2 = asciitrie::make2_slice(DATA);
        assert_eq!(trie2, TRIE2);
        if asciitrie::reader2::get(black_box(TRIE2), b"MV") == Some(weekday::FRI) {
            return 0;
        } else {
            return 1;
        }
    }

    #[cfg(any())]
    {
        let trie3 = asciitrie::make3_slice(DATA);
        assert_eq!(trie3, &TRIE3);
        if asciitrie::reader3::get(black_box(&TRIE3), b"MV") == Some(weekday::FRI) {
            return 0;
        } else {
            return 1;
        }
    }
}
