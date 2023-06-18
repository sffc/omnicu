// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// This example demonstrates the use of ZeroTrieSimpleAscii to look up data based on a region code.

#![no_main] // https://github.com/unicode-org/icu4x/issues/395
#![allow(dead_code)]

use asciitrie::AsciiStr;
use asciitrie::ZeroTrieSimpleAscii;

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
static TRIE: ZeroTrieSimpleAscii<[u8; 561]> = ZeroTrieSimpleAscii::from_asciistr_value_slice(DATA);

static TRIE4: [u8; 610] = [
    219, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 2, 0, 15, 0, 81,
    82, 83, 84, 85, 86, 87, 88, 89, 90, 79, 65, 66, 67, 68, 69, 70, 71, 72, 73, 75, 74, 48, 76, 78,
    77, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 2,
    16, 45, 65, 82, 96, 98, 100, 102, 110, 112, 156, 197, 226, 246, 7, 21, 57, 71, 97, 117, 128,
    131, 157, 174, 218, 65, 134, 196, 255, 69, 79, 83, 85, 0, 1, 2, 3, 129, 129, 129, 129, 201,
    255, 65, 68, 69, 71, 73, 75, 77, 86, 89, 0, 1, 2, 3, 4, 5, 6, 7, 8, 135, 134, 129, 135, 129,
    129, 129, 135, 134, 198, 255, 72, 74, 77, 82, 84, 87, 0, 1, 2, 3, 4, 5, 135, 129, 129, 129,
    135, 135, 197, 255, 65, 77, 83, 89, 90, 0, 1, 2, 3, 4, 129, 135, 135, 129, 129, 196, 255, 65,
    69, 73, 78, 0, 1, 2, 3, 129, 135, 135, 129, 83, 135, 75, 129, 69, 135, 194, 255, 65, 87, 0, 1,
    135, 135, 77, 134, 206, 255, 68, 69, 70, 71, 73, 76, 77, 78, 82, 83, 84, 85, 88, 90, 0, 1, 2,
    3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 129, 134, 134, 135, 129, 129, 129, 129, 129, 135, 129,
    129, 129, 129, 205, 255, 65, 68, 69, 71, 72, 77, 78, 82, 83, 84, 87, 89, 90, 0, 1, 2, 3, 4, 5,
    6, 7, 8, 9, 10, 11, 12, 129, 135, 129, 129, 134, 129, 129, 135, 135, 135, 135, 129, 135, 201,
    255, 65, 72, 76, 77, 78, 79, 82, 89, 90, 0, 1, 2, 3, 4, 5, 6, 7, 8, 135, 129, 129, 129, 135,
    135, 129, 129, 129, 198, 255, 69, 74, 75, 77, 79, 90, 0, 1, 2, 3, 4, 5, 129, 134, 129, 135,
    135, 134, 197, 255, 67, 69, 71, 83, 84, 0, 1, 2, 3, 4, 129, 129, 134, 129, 135, 196, 255, 73,
    74, 79, 82, 0, 1, 2, 3, 129, 129, 129, 129, 199, 255, 66, 69, 70, 80, 82, 84, 85, 0, 14, 15,
    16, 17, 18, 19, 129, 45, 97, 108, 116, 45, 118, 97, 114, 105, 97, 110, 116, 135, 129, 129, 129,
    129, 135, 135, 196, 255, 75, 78, 82, 85, 0, 1, 2, 3, 135, 135, 129, 129, 200, 255, 68, 69, 76,
    78, 81, 82, 83, 84, 0, 1, 2, 3, 4, 5, 6, 7, 135, 129, 135, 135, 134, 134, 129, 129, 198, 255,
    69, 71, 72, 82, 87, 90, 0, 1, 2, 3, 4, 5, 135, 129, 135, 135, 134, 129, 195, 255, 77, 79, 80,
    0, 1, 2, 135, 134, 135, 48, 49, 129, 200, 255, 65, 66, 73, 75, 84, 85, 86, 89, 0, 1, 2, 3, 4,
    5, 6, 7, 135, 129, 129, 129, 129, 129, 129, 134, 197, 255, 73, 76, 79, 80, 90, 0, 1, 2, 3, 4,
    135, 129, 129, 135, 129, 206, 255, 67, 68, 69, 72, 75, 77, 78, 79, 81, 84, 86, 88, 89, 90, 0,
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 129, 129, 129, 135, 129, 135, 129, 135, 129, 135,
    133, 135, 129, 135, 200, 255, 65, 69, 72, 75, 76, 82, 84, 89, 0, 1, 2, 3, 4, 5, 6, 7, 135, 135,
    135, 135, 129, 135, 135, 135,
];

static TRIE6: [u8; 567] = [
    225, 123, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 2, 0, 15, 0,
    81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 79, 65, 66, 67, 68, 69, 70, 71, 72, 73, 75, 74, 48, 76,
    78, 77, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2,
    14, 41, 59, 74, 86, 88, 90, 92, 98, 100, 142, 181, 208, 226, 241, 253, 31, 43, 67, 85, 94, 97,
    121, 136, 178, 65, 134, 196, 69, 79, 83, 85, 1, 2, 3, 129, 129, 129, 129, 201, 65, 68, 69, 71,
    73, 75, 77, 86, 89, 1, 2, 3, 4, 5, 6, 7, 8, 135, 134, 129, 135, 129, 129, 129, 135, 134, 198,
    72, 74, 77, 82, 84, 87, 1, 2, 3, 4, 5, 135, 129, 129, 129, 135, 135, 197, 65, 77, 83, 89, 90,
    1, 2, 3, 4, 129, 135, 135, 129, 129, 196, 65, 69, 73, 78, 1, 2, 3, 129, 135, 135, 129, 83, 135,
    75, 129, 69, 135, 194, 65, 87, 1, 135, 135, 77, 134, 206, 68, 69, 70, 71, 73, 76, 77, 78, 82,
    83, 84, 85, 88, 90, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 129, 134, 134, 135, 129, 129,
    129, 129, 129, 135, 129, 129, 129, 129, 205, 65, 68, 69, 71, 72, 77, 78, 82, 83, 84, 87, 89,
    90, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 129, 135, 129, 129, 134, 129, 129, 135, 135, 135,
    135, 129, 135, 201, 65, 72, 76, 77, 78, 79, 82, 89, 90, 1, 2, 3, 4, 5, 6, 7, 8, 135, 129, 129,
    129, 135, 135, 129, 129, 129, 198, 69, 74, 75, 77, 79, 90, 1, 2, 3, 4, 5, 129, 134, 129, 135,
    135, 134, 197, 67, 69, 71, 83, 84, 1, 2, 3, 4, 129, 129, 134, 129, 135, 196, 73, 74, 79, 82, 1,
    2, 3, 129, 129, 129, 129, 199, 66, 69, 70, 80, 82, 84, 85, 14, 15, 16, 17, 18, 19, 129, 45, 97,
    108, 116, 45, 118, 97, 114, 105, 97, 110, 116, 135, 129, 129, 129, 129, 135, 135, 196, 75, 78,
    82, 85, 1, 2, 3, 135, 135, 129, 129, 200, 68, 69, 76, 78, 81, 82, 83, 84, 1, 2, 3, 4, 5, 6, 7,
    135, 129, 135, 135, 134, 134, 129, 129, 198, 69, 71, 72, 82, 87, 90, 1, 2, 3, 4, 5, 135, 129,
    135, 135, 134, 129, 195, 77, 79, 80, 1, 2, 135, 134, 135, 48, 49, 129, 200, 65, 66, 73, 75, 84,
    85, 86, 89, 1, 2, 3, 4, 5, 6, 7, 135, 129, 129, 129, 129, 129, 129, 134, 197, 73, 76, 79, 80,
    90, 1, 2, 3, 4, 135, 129, 129, 135, 129, 206, 67, 68, 69, 72, 75, 77, 78, 79, 81, 84, 86, 88,
    89, 90, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 129, 129, 129, 135, 129, 135, 129, 135, 129,
    135, 133, 135, 129, 135, 200, 65, 69, 72, 75, 76, 82, 84, 89, 1, 2, 3, 4, 5, 6, 7, 135, 135,
    135, 135, 129, 135, 135, 135,
];

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

    // {
    //     if ZeroTrieSimpleAscii::get(black_box(&TRIE), b"MV") == Some(weekday::FRI) {
    //         return 0;
    //     } else {
    //         return 1;
    //     }
    // }

    // {
    //     let trie2 = asciitrie::make2_slice(DATA);
    //     assert_eq!(trie2, TRIE2);
    //     if asciitrie::reader2::get(black_box(TRIE2), b"MV") == Some(weekday::FRI) {
    //         return 0;
    //     } else {
    //         return 1;
    //     }
    // }

    // {
    //     let trie3 = asciitrie::make3_slice(DATA);
    //     assert_eq!(trie3, &TRIE3);
    //     if asciitrie::reader3::get(black_box(&TRIE3), b"MV") == Some(weekday::FRI) {
    //         return 0;
    //     } else {
    //         return 1;
    //     }
    // }

    // {
    //     // let trie4 = asciitrie::make4_slice(DATA);
    //     // assert_eq!(trie4, &TRIE4);
    //     if asciitrie::reader4::get(black_box(&TRIE4), b"MV") == Some(weekday::FRI) {
    //         return 0;
    //     } else {
    //         return 1;
    //     }
    // }

    {
        // let trie6 = asciitrie::make6_slice(DATA);
        // assert_eq!(trie6, &TRIE6);
        if asciitrie::reader6::get(black_box(&TRIE6), b"MV") == Some(weekday::FRI) {
            return 0;
        } else {
            return 1;
        }
    }
}
