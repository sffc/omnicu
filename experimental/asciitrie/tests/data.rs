// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[allow(dead_code)]
use asciitrie::{AsciiStr, NonAsciiError};
#[allow(dead_code)]
use litemap::LiteMap;

const fn single_byte_value(x: u8) -> u8 {
    debug_assert!(x <= 0b00011111);
    x | 0b10000000
}

const fn single_byte_match(x: u8) -> u8 {
    debug_assert!(x <= 0b00011111);
    x | 0b11000000
}

const fn single_byte_intermediate_value(x: u8) -> u8 {
    debug_assert!(x <= 0b00001111);
    x | 0b10000000
}

const fn single_byte_final_value(x: u8) -> u8 {
    debug_assert!(x <= 0b00001111);
    x | 0b10100000
}

const fn single_byte_branch_equal(x: u8) -> u8 {
    debug_assert!(x <= 0b00001111);
    x | 0b11000000
}

const fn single_byte_branch_greater(x: u8) -> u8 {
    debug_assert!(x <= 0b00001111);
    x | 0b11100000
}

const fn single_byte_intermediate_branch(x: u8) -> u8 {
    debug_assert!(x <= 0b00001111);
    x | 0b11000000
}

const fn single_byte_final_branch(x: u8) -> u8 {
    debug_assert!(x <= 0b00001111);
    x | 0b11100000
}

#[allow(dead_code)]
pub fn strings_to_litemap<'a>(
    strings: &[&'a str],
) -> Result<LiteMap<&'a AsciiStr, usize>, NonAsciiError> {
    strings
        .iter()
        .copied()
        .map(AsciiStr::try_from_str)
        .enumerate()
        .map(|(i, s)| s.map(|s| (s, i)))
        .collect()
}

#[allow(dead_code)]
pub mod basic {
    use super::*;
    pub static TRIE: &[u8] = &[
        b'a',
        b'b',
        single_byte_value(1),
        b'c',
        single_byte_value(2),
        // Begin Match Node
        single_byte_match(3),
        b'd',
        b'e',
        b'f',
        0,
        5,
        8,
        // End Match Node
        // subslice @ 0
        single_byte_value(3),
        b'g',
        b'h',
        b'i',
        single_byte_value(4),
        // subslice @ 5
        b'j',
        b'k',
        single_byte_value(5),
        // subslice @ 8
        // Begin Match Node
        single_byte_match(2),
        b'l',
        b'm',
        0,
        1,
        // End Match Node
        // subslice @ 0
        single_byte_value(6),
        // subslice @ 1
        b'n',
        single_byte_value(7),
    ];
    pub static TRIE2: &[u8] = &[
        b'a',
        b'b',
        single_byte_intermediate_value(1),
        b'c',
        single_byte_intermediate_value(2),
        single_byte_branch_equal(6),
        single_byte_branch_greater(3),
        b'e',
        b'd',
        single_byte_intermediate_value(3),
        b'g',
        b'h',
        b'i',
        single_byte_final_value(4),
        b'j',
        b'k',
        single_byte_final_value(5),
        b'f',
        single_byte_branch_equal(2),
        b'm',
        b'l',
        single_byte_final_value(6),
        b'n',
        single_byte_final_value(7),
    ];
    pub static TRIE3: &[u8] = &[
        b'a',
        b'b',
        single_byte_intermediate_value(1),
        b'c',
        single_byte_intermediate_value(2),
        single_byte_intermediate_branch(6),
        b'e',
        b'd',
        single_byte_intermediate_value(3),
        b'g',
        b'h',
        b'i',
        single_byte_final_value(4),
        single_byte_final_branch(4),
        b'f',
        b'e',
        b'j',
        b'k',
        single_byte_final_value(5),
        single_byte_final_branch(2),
        b'm',
        b'l',
        single_byte_final_value(6),
        b'n',
        single_byte_final_value(7),
    ];
    pub static TRIE4: &[u8] = &[
        b'a',
        b'b',
        single_byte_value(1),
        b'c',
        single_byte_value(2),
        // Begin Match Node
        single_byte_match(3),
        255,
        b'd',
        b'e',
        b'f',
        0,
        5,
        8,
        // End Match Node
        // subslice @ 0
        single_byte_value(3),
        b'g',
        b'h',
        b'i',
        single_byte_value(4),
        // subslice @ 5
        b'j',
        b'k',
        single_byte_value(5),
        // subslice @ 8
        // Begin Match Node
        single_byte_match(2),
        255,
        b'l',
        b'm',
        0,
        1,
        // End Match Node
        // subsubslice @ 0
        single_byte_value(6),
        // subsubslice @ 1
        b'n',
        single_byte_value(7),
    ];
    pub static TRIE5: &[u8] = &[
        b'a',
        b'b',
        single_byte_value(1),
        b'c',
        single_byte_value(2),
        // Begin Match Node
        single_byte_match(3 << 2),
        255,
        b'd',
        b'e',
        b'f',
        5,
        8,
        // End Match Node
        // subslice @ 0
        single_byte_value(3),
        b'g',
        b'h',
        b'i',
        single_byte_value(4),
        // subslice @ 5
        b'j',
        b'k',
        single_byte_value(5),
        // subslice @ 8
        // Begin Match Node
        single_byte_match(2 << 2),
        255,
        b'l',
        b'm',
        1,
        // End Match Node
        // subsubslice @ 0
        single_byte_value(6),
        // subsubslice @ 1
        b'n',
        single_byte_value(7),
    ];
    pub static DATA: &[(&AsciiStr, usize)] = &[
        (AsciiStr::from_str_or_panic("ab"), 1),
        (AsciiStr::from_str_or_panic("abc"), 2),
        (AsciiStr::from_str_or_panic("abcd"), 3),
        (AsciiStr::from_str_or_panic("abcdghi"), 4),
        (AsciiStr::from_str_or_panic("abcejk"), 5),
        (AsciiStr::from_str_or_panic("abcfl"), 6),
        (AsciiStr::from_str_or_panic("abcfmn"), 7),
    ];

    // Note: Cow and ZeroVec have the same serialized form
    pub static JSON_STR: &str = "{\"trie\":{\"ab\":1,\"abc\":2,\"abcd\":3,\"abcdghi\":4,\"abcejk\":5,\"abcfl\":6,\"abcfmn\":7}}";
    pub static BINCODE_BYTES: &[u8] = &[
        28, 0, 0, 0, 0, 0, 0, 0, 97, 98, 129, 99, 130, 195, 100, 101, 102, 0, 5, 8, 131, 103, 104,
        105, 132, 106, 107, 133, 194, 108, 109, 0, 1, 134, 110, 135,
    ];
}

#[allow(dead_code)]
pub mod short_subtags {
    pub static STRINGS: &[&str] = &[
        "aa",
        "aai",
        "aak",
        "aau",
        "ab",
        "abi",
        "abq",
        "abr",
        "abt",
        "aby",
        "acd",
        "ace",
        "ach",
        "ada",
        "ade",
        "adj",
        "adp",
        "ady",
        "adz",
        "ae",
        "aeb",
        "aey",
        "af",
        "agc",
        "agd",
        "agg",
        "agm",
        "ago",
        "agq",
        "aha",
        "ahl",
        "aho",
        "ajg",
        "ak",
        "akk",
        "ala",
        "ali",
        "aln",
        "alt",
        "am",
        "amm",
        "amn",
        "amo",
        "amp",
        "an",
        "anc",
        "ank",
        "ann",
        "any",
        "aoj",
        "aom",
        "aoz",
        "apc",
        "apd",
        "ape",
        "apr",
        "aps",
        "apz",
        "ar",
        "arc",
        "arc-Nbat",
        "arc-Palm",
        "arh",
        "arn",
        "aro",
        "arq",
        "ars",
        "ary",
        "arz",
        "as",
        "asa",
        "ase",
        "asg",
        "aso",
        "ast",
        "ata",
        "atg",
        "atj",
        "auy",
        "av",
        "avl",
        "avn",
        "avt",
        "avu",
        "awa",
        "awb",
        "awo",
        "awx",
        "ay",
        "ayb",
        "az",
        "az-Arab",
        "az-IQ",
        "az-IR",
        "az-RU",
        "ba",
        "bal",
        "ban",
        "bap",
        "bar",
        "bas",
        "bav",
        "bax",
        "bba",
        "bbb",
        "bbc",
        "bbd",
        "bbj",
        "bbp",
        "bbr",
        "bcf",
        "bch",
        "bci",
        "bcm",
        "bcn",
        "bco",
        "bcq",
        "bcu",
        "bdd",
        "be",
        "bef",
        "beh",
        "bej",
        "bem",
        "bet",
        "bew",
        "bex",
        "bez",
        "bfd",
        "bfq",
        "bft",
        "bfy",
        "bg",
        "bgc",
        "bgn",
        "bgx",
        "bhb",
        "bhg",
        "bhi",
        "bhl",
        "bho",
        "bhy",
        "bi",
        "bib",
        "big",
        "bik",
        "bim",
        "bin",
        "bio",
        "biq",
        "bjh",
        "bji",
        "bjj",
        "bjn",
        "bjo",
        "bjr",
        "bjt",
        "bjz",
        "bkc",
        "bkm",
        "bkq",
        "bku",
        "bkv",
        "bla",
        "blg",
        "blt",
        "bm",
        "bmh",
        "bmk",
        "bmq",
        "bmu",
        "bn",
        "bng",
        "bnm",
        "bnp",
        "bo",
        "boj",
        "bom",
        "bon",
        "bpy",
        "bqc",
        "bqi",
        "bqp",
        "bqv",
        "br",
        "bra",
        "brh",
        "brx",
        "brz",
        "bs",
        "bsj",
        "bsq",
        "bss",
        "bst",
        "bto",
        "btt",
        "btv",
        "bua",
        "buc",
        "bud",
        "bug",
        "buk",
        "bum",
        "buo",
        "bus",
        "buu",
        "bvb",
        "bwd",
        "bwr",
        "bxh",
        "bye",
        "byn",
        "byr",
        "bys",
        "byv",
        "byx",
        "bza",
        "bze",
        "bzf",
        "bzh",
        "bzw",
        "ca",
        "cad",
        "can",
        "cbj",
        "cch",
        "ccp",
        "ce",
        "ceb",
        "cfa",
        "cgg",
        "ch",
        "chk",
        "chm",
        "cho",
        "chp",
        "chr",
        "cic",
        "cja",
        "cjm",
        "cjv",
        "ckb",
        "ckl",
        "cko",
        "cky",
        "cla",
        "clc",
        "cme",
        "cmg",
        "co",
        "cop",
        "cps",
        "cr",
        "crg",
        "crh",
        "crk",
        "crl",
        "crs",
        "cs",
        "csb",
        "csw",
        "ctd",
        "cu",
        "cu-Glag",
        "cv",
        "cy",
        "da",
        "dad",
        "daf",
        "dag",
        "dah",
        "dak",
        "dar",
        "dav",
        "dbd",
        "dbq",
        "dcc",
        "ddn",
        "de",
        "ded",
        "den",
        "dga",
        "dgh",
        "dgi",
        "dgl",
        "dgr",
        "dgz",
        "dia",
        "dje",
        "dmf",
        "dnj",
        "dob",
        "doi",
        "dop",
        "dow",
        "drh",
        "dri",
        "drs",
        "dsb",
        "dtm",
        "dtp",
        "dts",
        "dty",
        "dua",
        "duc",
        "dud",
        "dug",
        "dv",
        "dva",
        "dww",
        "dyo",
        "dyu",
        "dz",
        "dzg",
        "ebu",
        "ee",
        "efi",
        "egl",
        "egy",
        "eka",
        "eky",
        "el",
        "ema",
        "emi",
        "en",
        "en-Shaw",
        "enn",
        "enq",
        "eo",
        "eri",
        "es",
        "esg",
        "esu",
        "et",
        "etr",
        "ett",
        "etu",
        "etx",
        "eu",
        "ewo",
        "ext",
        "eza",
        "fa",
        "faa",
        "fab",
        "fag",
        "fai",
        "fan",
        "ff",
        "ff-Adlm",
        "ffi",
        "ffm",
        "fi",
        "fia",
        "fil",
        "fit",
        "fj",
        "flr",
        "fmp",
        "fo",
        "fod",
        "fon",
        "for",
        "fpe",
        "fqs",
        "fr",
        "frc",
        "frp",
        "frr",
        "frs",
        "fub",
        "fud",
        "fue",
        "fuf",
        "fuh",
        "fuq",
        "fur",
        "fuv",
        "fuy",
        "fvr",
        "fy",
        "ga",
        "gaa",
        "gaf",
        "gag",
        "gah",
        "gaj",
        "gam",
        "gan",
        "gaw",
        "gay",
        "gba",
        "gbf",
        "gbm",
        "gby",
        "gbz",
        "gcr",
        "gd",
        "gde",
        "gdn",
        "gdr",
        "geb",
        "gej",
        "gel",
        "gez",
        "gfk",
        "ggn",
        "ghs",
        "gil",
        "gim",
        "gjk",
        "gjn",
        "gju",
        "gkn",
        "gkp",
        "gl",
        "glk",
        "gmm",
        "gmv",
        "gn",
        "gnd",
        "gng",
        "god",
        "gof",
        "goi",
        "gom",
        "gon",
        "gor",
        "gos",
        "got",
        "grb",
        "grc",
        "grc-Linb",
        "grt",
        "grw",
        "gsw",
        "gu",
        "gub",
        "guc",
        "gud",
        "gur",
        "guw",
        "gux",
        "guz",
        "gv",
        "gvf",
        "gvr",
        "gvs",
        "gwc",
        "gwi",
        "gwt",
        "gyi",
        "ha",
        "ha-CM",
        "ha-SD",
        "hag",
        "hak",
        "ham",
        "haw",
        "haz",
        "hbb",
        "hdy",
        "he",
        "hhy",
        "hi",
        "hi-Latn",
        "hia",
        "hif",
        "hig",
        "hih",
        "hil",
        "hla",
        "hlu",
        "hmd",
        "hmt",
        "hnd",
        "hne",
        "hnj",
        "hnn",
        "hno",
        "ho",
        "hoc",
        "hoj",
        "hot",
        "hr",
        "hsb",
        "hsn",
        "ht",
        "hu",
        "hui",
        "hur",
        "hy",
        "hz",
        "ia",
        "ian",
        "iar",
        "iba",
        "ibb",
        "iby",
        "ica",
        "ich",
        "id",
        "idd",
        "idi",
        "idu",
        "ife",
        "ig",
        "igb",
        "ige",
        "ii",
        "ijj",
        "ik",
        "ikk",
        "ikw",
        "ikx",
        "ilo",
        "imo",
        "in",
        "inh",
        "io",
        "iou",
        "iri",
        "is",
        "it",
        "iu",
        "iw",
        "iwm",
        "iws",
        "izh",
        "izi",
        "ja",
        "jab",
        "jam",
        "jar",
        "jbo",
        "jbu",
        "jen",
        "jgk",
        "jgo",
        "ji",
        "jib",
        "jmc",
        "jml",
        "jra",
        "jut",
        "jv",
        "jw",
        "ka",
        "kaa",
        "kab",
        "kac",
        "kad",
        "kai",
        "kaj",
        "kam",
        "kao",
        "kaw",
        "kbd",
        "kbm",
        "kbp",
        "kbq",
        "kbx",
        "kby",
        "kcg",
        "kck",
        "kcl",
        "kct",
        "kde",
        "kdh",
        "kdl",
        "kdt",
        "kea",
        "ken",
        "kez",
        "kfo",
        "kfr",
        "kfy",
        "kg",
        "kge",
        "kgf",
        "kgp",
        "kha",
        "khb",
        "khn",
        "khq",
        "khs",
        "kht",
        "khw",
        "khz",
        "ki",
        "kij",
        "kiu",
        "kiw",
        "kj",
        "kjd",
        "kjg",
        "kjs",
        "kjy",
        "kk",
        "kk-AF",
        "kk-Arab",
        "kk-CN",
        "kk-IR",
        "kk-MN",
        "kkc",
        "kkj",
        "kl",
        "kln",
        "klq",
        "klt",
        "klx",
        "km",
        "kmb",
        "kmh",
        "kmo",
        "kms",
        "kmu",
        "kmw",
        "kn",
        "knf",
        "knp",
        "ko",
        "koi",
        "kok",
        "kol",
        "kos",
        "koz",
        "kpe",
        "kpf",
        "kpo",
        "kpr",
        "kpx",
        "kqb",
        "kqf",
        "kqs",
        "kqy",
        "kr",
        "krc",
        "kri",
        "krj",
        "krl",
        "krs",
        "kru",
        "ks",
        "ksb",
        "ksd",
        "ksf",
        "ksh",
        "ksj",
        "ksr",
        "ktb",
        "ktm",
        "kto",
        "ktr",
        "ku",
        "ku-Arab",
        "ku-LB",
        "ku-Yezi",
        "kub",
        "kud",
        "kue",
        "kuj",
        "kum",
        "kun",
        "kup",
        "kus",
        "kv",
        "kvg",
        "kvr",
        "kvx",
        "kw",
        "kwj",
        "kwk",
        "kwo",
        "kwq",
        "kxa",
        "kxc",
        "kxe",
        "kxl",
        "kxm",
        "kxp",
        "kxw",
        "kxz",
        "ky",
        "ky-Arab",
        "ky-CN",
        "ky-Latn",
        "ky-TR",
        "kye",
        "kyx",
        "kzh",
        "kzj",
        "kzr",
        "kzt",
        "la",
        "lab",
        "lad",
        "lag",
        "lah",
        "laj",
        "las",
        "lb",
        "lbe",
        "lbu",
        "lbw",
        "lcm",
        "lcp",
        "ldb",
        "led",
        "lee",
        "lem",
        "lep",
        "leq",
        "leu",
        "lez",
        "lg",
        "lgg",
        "li",
        "lia",
        "lid",
        "lif",
        "lif-Limb",
        "lig",
        "lih",
        "lij",
        "lil",
        "lis",
        "ljp",
        "lki",
        "lkt",
        "lle",
        "lln",
        "lmn",
        "lmo",
        "lmp",
        "ln",
        "lns",
        "lnu",
        "lo",
        "loj",
        "lok",
        "lol",
        "lor",
        "los",
        "loz",
        "lrc",
        "lt",
        "ltg",
        "lu",
        "lua",
        "luo",
        "luy",
        "luz",
        "lv",
        "lwl",
        "lzh",
        "lzz",
        "mad",
        "maf",
        "mag",
        "mai",
        "mak",
        "man",
        "man-GN",
        "man-Nkoo",
        "mas",
        "maw",
        "maz",
        "mbh",
        "mbo",
        "mbq",
        "mbu",
        "mbw",
        "mci",
        "mcp",
        "mcq",
        "mcr",
        "mcu",
        "mda",
        "mde",
        "mdf",
        "mdh",
        "mdj",
        "mdr",
        "mdx",
        "med",
        "mee",
        "mek",
        "men",
        "mer",
        "met",
        "meu",
        "mfa",
        "mfe",
        "mfn",
        "mfo",
        "mfq",
        "mg",
        "mgh",
        "mgl",
        "mgo",
        "mgp",
        "mgy",
        "mh",
        "mhi",
        "mhl",
        "mi",
        "mic",
        "mif",
        "min",
        "miw",
        "mk",
        "mki",
        "mkl",
        "mkp",
        "mkw",
        "ml",
        "mle",
        "mlp",
        "mls",
        "mmo",
        "mmu",
        "mmx",
        "mn",
        "mn-CN",
        "mn-Mong",
        "mna",
        "mnf",
        "mni",
        "mnw",
        "mo",
        "moa",
        "moe",
        "moh",
        "mos",
        "mox",
        "mpp",
        "mps",
        "mpt",
        "mpx",
        "mql",
        "mr",
        "mrd",
        "mrj",
        "mro",
        "ms",
        "ms-CC",
        "mt",
        "mtc",
        "mtf",
        "mti",
        "mtr",
        "mua",
        "mur",
        "mus",
        "mva",
        "mvn",
        "mvy",
        "mwk",
        "mwr",
        "mwv",
        "mww",
        "mxc",
        "mxm",
        "my",
        "myk",
        "mym",
        "myv",
        "myw",
        "myx",
        "myz",
        "mzk",
        "mzm",
        "mzn",
        "mzp",
        "mzw",
        "mzz",
        "na",
        "nac",
        "naf",
        "nak",
        "nan",
        "nap",
        "naq",
        "nas",
        "nb",
        "nca",
        "nce",
        "ncf",
        "nch",
        "nco",
        "ncu",
        "nd",
        "ndc",
        "nds",
        "ne",
        "neb",
        "new",
        "nex",
        "nfr",
        "ng",
        "nga",
        "ngb",
        "ngl",
        "nhb",
        "nhe",
        "nhw",
        "nif",
        "nii",
        "nij",
        "nin",
        "niu",
        "niy",
        "niz",
        "njo",
        "nkg",
        "nko",
        "nl",
        "nmg",
        "nmz",
        "nn",
        "nnf",
        "nnh",
        "nnk",
        "nnm",
        "nnp",
        "no",
        "nod",
        "noe",
        "non",
        "nop",
        "nou",
        "nqo",
        "nr",
        "nrb",
        "nsk",
        "nsn",
        "nso",
        "nss",
        "nst",
        "ntm",
        "ntr",
        "nui",
        "nup",
        "nus",
        "nuv",
        "nux",
        "nv",
        "nwb",
        "nxq",
        "nxr",
        "ny",
        "nym",
        "nyn",
        "nzi",
        "oc",
        "ogc",
        "oj",
        "ojs",
        "oka",
        "okr",
        "okv",
        "om",
        "ong",
        "onn",
        "ons",
        "opm",
        "or",
        "oro",
        "oru",
        "os",
        "osa",
        "ota",
        "otk",
        "oui",
        "ozm",
        "pa",
        "pa-Arab",
        "pa-PK",
        "pag",
        "pal",
        "pal-Phlp",
        "pam",
        "pap",
        "pau",
        "pbi",
        "pcd",
        "pcm",
        "pdc",
        "pdt",
        "ped",
        "peo",
        "pex",
        "pfl",
        "phl",
        "phn",
        "pil",
        "pip",
        "pka",
        "pko",
        "pl",
        "pla",
        "pms",
        "png",
        "pnn",
        "pnt",
        "pon",
        "ppa",
        "ppo",
        "pqm",
        "pra",
        "prd",
        "prg",
        "ps",
        "pss",
        "pt",
        "ptp",
        "puu",
        "pwa",
        "qu",
        "quc",
        "qug",
        "rai",
        "raj",
        "rao",
        "rcf",
        "rej",
        "rel",
        "res",
        "rgn",
        "rhg",
        "ria",
        "rif",
        "rif-NL",
        "rjs",
        "rkt",
        "rm",
        "rmf",
        "rmo",
        "rmt",
        "rmu",
        "rn",
        "rna",
        "rng",
        "ro",
        "rob",
        "rof",
        "roo",
        "rro",
        "rtm",
        "ru",
        "rue",
        "rug",
        "rw",
        "rwk",
        "rwo",
        "ryu",
        "sa",
        "saf",
        "sah",
        "saq",
        "sas",
        "sat",
        "sav",
        "saz",
        "sba",
        "sbe",
        "sbp",
        "sc",
        "sck",
        "scl",
        "scn",
        "sco",
        "sd",
        "sd-Deva",
        "sd-IN",
        "sd-Khoj",
        "sd-Sind",
        "sdc",
        "sdh",
        "se",
        "sef",
        "seh",
        "sei",
        "ses",
        "sg",
        "sga",
        "sgs",
        "sgw",
        "sgz",
        "shi",
        "shk",
        "shn",
        "shu",
        "si",
        "sid",
        "sig",
        "sil",
        "sim",
        "sjr",
        "sk",
        "skc",
        "skr",
        "sks",
        "sl",
        "sld",
        "sli",
        "sll",
        "sly",
        "sm",
        "sma",
        "smj",
        "smn",
        "smp",
        "smq",
        "sms",
        "sn",
        "snc",
        "snk",
        "snp",
        "snx",
        "sny",
        "so",
        "sog",
        "sok",
        "soq",
        "sou",
        "soy",
        "spd",
        "spl",
        "sps",
        "sq",
        "sr",
        "sr-ME",
        "sr-RO",
        "sr-RU",
        "sr-TR",
        "srb",
        "srn",
        "srr",
        "srx",
        "ss",
        "ssd",
        "ssg",
        "ssy",
        "st",
        "stk",
        "stq",
        "su",
        "sua",
        "sue",
        "suk",
        "sur",
        "sus",
        "sv",
        "sw",
        "swb",
        "swc",
        "swg",
        "swp",
        "swv",
        "sxn",
        "sxw",
        "syl",
        "syr",
        "szl",
        "ta",
        "taj",
        "tal",
        "tan",
        "taq",
        "tbc",
        "tbd",
        "tbf",
        "tbg",
        "tbo",
        "tbw",
        "tbz",
        "tci",
        "tcy",
        "tdd",
        "tdg",
        "tdh",
        "tdu",
        "te",
        "ted",
        "tem",
        "teo",
        "tet",
        "tfi",
        "tg",
        "tg-Arab",
        "tg-PK",
        "tgc",
        "tgo",
        "tgu",
        "th",
        "thl",
        "thq",
        "thr",
        "ti",
        "tif",
        "tig",
        "tik",
        "tim",
        "tio",
        "tiv",
        "tk",
        "tkl",
        "tkr",
        "tkt",
        "tl",
        "tlf",
        "tlx",
        "tly",
        "tmh",
        "tmy",
        "tn",
        "tnh",
        "to",
        "tof",
        "tog",
        "toq",
        "tpi",
        "tpm",
        "tpz",
        "tqo",
        "tr",
        "tru",
        "trv",
        "trw",
        "ts",
        "tsd",
        "tsf",
        "tsg",
        "tsj",
        "tsw",
        "tt",
        "ttd",
        "tte",
        "ttj",
        "ttr",
        "tts",
        "ttt",
        "tuh",
        "tul",
        "tum",
        "tuq",
        "tvd",
        "tvl",
        "tvu",
        "twh",
        "twq",
        "txg",
        "txo",
        "ty",
        "tya",
        "tyv",
        "tzm",
        "ubu",
        "udi",
        "udm",
        "ug",
        "ug-Cyrl",
        "ug-KZ",
        "ug-MN",
        "uga",
        "uk",
        "uli",
        "umb",
        "und",
        "und-002",
        "und-003",
        "und-005",
        "und-009",
        "und-011",
        "und-013",
        "und-014",
        "und-015",
        "und-017",
        "und-018",
        "und-019",
        "und-021",
        "und-029",
        "und-030",
        "und-034",
        "und-035",
        "und-039",
        "und-053",
        "und-054",
        "und-057",
        "und-061",
        "und-142",
        "und-143",
        "und-145",
        "und-150",
        "und-151",
        "und-154",
        "und-155",
        "und-202",
        "und-419",
        "und-AD",
        "und-Adlm",
        "und-AE",
        "und-AF",
        "und-Aghb",
        "und-Ahom",
        "und-AL",
        "und-AM",
        "und-AO",
        "und-AQ",
        "und-AR",
        "und-Arab",
        "und-Arab-CC",
        "und-Arab-CN",
        "und-Arab-GB",
        "und-Arab-ID",
        "und-Arab-IN",
        "und-Arab-KH",
        "und-Arab-MM",
        "und-Arab-MN",
        "und-Arab-MU",
        "und-Arab-NG",
        "und-Arab-PK",
        "und-Arab-TG",
        "und-Arab-TH",
        "und-Arab-TJ",
        "und-Arab-TR",
        "und-Arab-YT",
        "und-Armi",
        "und-Armn",
        "und-AS",
        "und-AT",
        "und-Avst",
        "und-AW",
        "und-AX",
        "und-AZ",
        "und-BA",
        "und-Bali",
        "und-Bamu",
        "und-Bass",
        "und-Batk",
        "und-BD",
        "und-BE",
        "und-Beng",
        "und-BF",
        "und-BG",
        "und-BH",
        "und-Bhks",
        "und-BI",
        "und-BJ",
        "und-BL",
        "und-BN",
        "und-BO",
        "und-Bopo",
        "und-BQ",
        "und-BR",
        "und-Brah",
        "und-Brai",
        "und-BT",
        "und-Bugi",
        "und-Buhd",
        "und-BV",
        "und-BY",
        "und-Cakm",
        "und-Cans",
        "und-Cari",
        "und-CD",
        "und-CF",
        "und-CG",
        "und-CH",
        "und-Cham",
        "und-Cher",
        "und-Chrs",
        "und-CI",
        "und-CL",
        "und-CM",
        "und-CN",
        "und-CO",
        "und-Copt",
        "und-CP",
        "und-Cpmn",
        "und-Cpmn-CY",
        "und-Cprt",
        "und-CR",
        "und-CU",
        "und-CV",
        "und-CW",
        "und-CY",
        "und-Cyrl",
        "und-Cyrl-AL",
        "und-Cyrl-BA",
        "und-Cyrl-GE",
        "und-Cyrl-GR",
        "und-Cyrl-MD",
        "und-Cyrl-RO",
        "und-Cyrl-SK",
        "und-Cyrl-TR",
        "und-Cyrl-XK",
        "und-CZ",
        "und-DE",
        "und-Deva",
        "und-Deva-BT",
        "und-Deva-FJ",
        "und-Deva-MU",
        "und-Deva-PK",
        "und-Diak",
        "und-DJ",
        "und-DK",
        "und-DO",
        "und-Dogr",
        "und-Dupl",
        "und-DZ",
        "und-EA",
        "und-EC",
        "und-EE",
        "und-EG",
        "und-Egyp",
        "und-EH",
        "und-Elba",
        "und-Elym",
        "und-ER",
        "und-ES",
        "und-ET",
        "und-Ethi",
        "und-EU",
        "und-EZ",
        "und-FI",
        "und-FO",
        "und-FR",
        "und-GA",
        "und-GE",
        "und-Geor",
        "und-GF",
        "und-GH",
        "und-GL",
        "und-Glag",
        "und-GN",
        "und-Gong",
        "und-Gonm",
        "und-Goth",
        "und-GP",
        "und-GQ",
        "und-GR",
        "und-Gran",
        "und-Grek",
        "und-Grek-TR",
        "und-GS",
        "und-GT",
        "und-Gujr",
        "und-Guru",
        "und-GW",
        "und-Hanb",
        "und-Hang",
        "und-Hani",
        "und-Hano",
        "und-Hans",
        "und-Hant",
        "und-Hant-CA",
        "und-Hebr",
        "und-Hebr-SE",
        "und-Hebr-UA",
        "und-Hebr-US",
        "und-Hira",
        "und-HK",
        "und-Hluw",
        "und-HM",
        "und-Hmng",
        "und-Hmnp",
        "und-HN",
        "und-HR",
        "und-HT",
        "und-HU",
        "und-Hung",
        "und-IC",
        "und-ID",
        "und-IL",
        "und-IN",
        "und-IQ",
        "und-IR",
        "und-IS",
        "und-IT",
        "und-Ital",
        "und-Jamo",
        "und-Java",
        "und-JO",
        "und-JP",
        "und-Jpan",
        "und-Kali",
        "und-Kana",
        "und-Kawi",
        "und-KE",
        "und-KG",
        "und-KH",
        "und-Khar",
        "und-Khmr",
        "und-Khoj",
        "und-Kits",
        "und-KM",
        "und-Knda",
        "und-Kore",
        "und-KP",
        "und-KR",
        "und-Kthi",
        "und-KW",
        "und-KZ",
        "und-LA",
        "und-Lana",
        "und-Laoo",
        "und-Latn-AF",
        "und-Latn-AM",
        "und-Latn-CN",
        "und-Latn-CY",
        "und-Latn-DZ",
        "und-Latn-ET",
        "und-Latn-GE",
        "und-Latn-IR",
        "und-Latn-KM",
        "und-Latn-MA",
        "und-Latn-MK",
        "und-Latn-MM",
        "und-Latn-MO",
        "und-Latn-MR",
        "und-Latn-RU",
        "und-Latn-SY",
        "und-Latn-TN",
        "und-Latn-TW",
        "und-Latn-UA",
        "und-LB",
        "und-Lepc",
        "und-LI",
        "und-Limb",
        "und-Lina",
        "und-Linb",
        "und-Lisu",
        "und-LK",
        "und-LS",
        "und-LT",
        "und-LU",
        "und-LV",
        "und-LY",
        "und-Lyci",
        "und-Lydi",
        "und-MA",
        "und-Mahj",
        "und-Maka",
        "und-Mand",
        "und-Mani",
        "und-Marc",
        "und-MC",
        "und-MD",
        "und-ME",
        "und-Medf",
        "und-Mend",
        "und-Merc",
        "und-Mero",
        "und-MF",
        "und-MG",
        "und-MK",
        "und-ML",
        "und-Mlym",
        "und-MM",
        "und-MN",
        "und-MO",
        "und-Modi",
        "und-Mong",
        "und-MQ",
        "und-MR",
        "und-Mroo",
        "und-MT",
        "und-Mtei",
        "und-MU",
        "und-Mult",
        "und-MV",
        "und-MX",
        "und-MY",
        "und-Mymr",
        "und-Mymr-IN",
        "und-Mymr-TH",
        "und-MZ",
        "und-NA",
        "und-Nagm",
        "und-Nand",
        "und-Narb",
        "und-Nbat",
        "und-NC",
        "und-NE",
        "und-Newa",
        "und-NI",
        "und-Nkoo",
        "und-NL",
        "und-NO",
        "und-NP",
        "und-Nshu",
        "und-Ogam",
        "und-Olck",
        "und-OM",
        "und-Orkh",
        "und-Orya",
        "und-Osge",
        "und-Osma",
        "und-Ougr",
        "und-PA",
        "und-Palm",
        "und-Pauc",
        "und-PE",
        "und-Perm",
        "und-PF",
        "und-PG",
        "und-PH",
        "und-Phag",
        "und-Phli",
        "und-Phlp",
        "und-Phnx",
        "und-PK",
        "und-PL",
        "und-Plrd",
        "und-PM",
        "und-PR",
        "und-Prti",
        "und-PS",
        "und-PT",
        "und-PW",
        "und-PY",
        "und-QA",
        "und-QO",
        "und-RE",
        "und-Rjng",
        "und-RO",
        "und-Rohg",
        "und-RS",
        "und-RU",
        "und-Runr",
        "und-RW",
        "und-SA",
        "und-Samr",
        "und-Sarb",
        "und-Saur",
        "und-SC",
        "und-SD",
        "und-SE",
        "und-Sgnw",
        "und-Shaw",
        "und-Shrd",
        "und-SI",
        "und-Sidd",
        "und-Sind",
        "und-Sinh",
        "und-SJ",
        "und-SK",
        "und-SM",
        "und-SN",
        "und-SO",
        "und-Sogd",
        "und-Sogo",
        "und-Sora",
        "und-Soyo",
        "und-SR",
        "und-ST",
        "und-Sund",
        "und-SV",
        "und-SY",
        "und-Sylo",
        "und-Syrc",
        "und-Tagb",
        "und-Takr",
        "und-Tale",
        "und-Talu",
        "und-Taml",
        "und-Tang",
        "und-Tavt",
        "und-TD",
        "und-Telu",
        "und-TF",
        "und-Tfng",
        "und-TG",
        "und-Tglg",
        "und-TH",
        "und-Thaa",
        "und-Thai",
        "und-Thai-CN",
        "und-Thai-KH",
        "und-Thai-LA",
        "und-Tibt",
        "und-Tirh",
        "und-TJ",
        "und-TK",
        "und-TL",
        "und-TM",
        "und-TN",
        "und-Tnsa",
        "und-TO",
        "und-Toto",
        "und-TR",
        "und-TV",
        "und-TW",
        "und-TZ",
        "und-UA",
        "und-UG",
        "und-Ugar",
        "und-UY",
        "und-UZ",
        "und-VA",
        "und-Vaii",
        "und-VE",
        "und-Vith",
        "und-VN",
        "und-VU",
        "und-Wara",
        "und-Wcho",
        "und-WF",
        "und-WS",
        "und-XK",
        "und-Xpeo",
        "und-Xsux",
        "und-YE",
        "und-Yezi",
        "und-Yiii",
        "und-YT",
        "und-Zanb",
        "und-ZW",
        "unr",
        "unr-Deva",
        "unr-NP",
        "unx",
        "uok",
        "ur",
        "uri",
        "urt",
        "urw",
        "usa",
        "uth",
        "utr",
        "uvh",
        "uvl",
        "uz",
        "uz-AF",
        "uz-Arab",
        "uz-CN",
        "vag",
        "vai",
        "van",
        "ve",
        "vec",
        "vep",
        "vi",
        "vic",
        "viv",
        "vls",
        "vmf",
        "vmw",
        "vo",
        "vot",
        "vro",
        "vun",
        "vut",
        "wa",
        "wae",
        "waj",
        "wal",
        "wan",
        "war",
        "wbp",
        "wbq",
        "wbr",
        "wci",
        "wer",
        "wgi",
        "whg",
        "wib",
        "wiu",
        "wiv",
        "wja",
        "wji",
        "wls",
        "wmo",
        "wnc",
        "wni",
        "wnu",
        "wo",
        "wob",
        "wos",
        "wrs",
        "wsg",
        "wsk",
        "wtm",
        "wuu",
        "wuv",
        "wwa",
        "xav",
        "xbi",
        "xco",
        "xcr",
        "xes",
        "xh",
        "xla",
        "xlc",
        "xld",
        "xmf",
        "xmn",
        "xmr",
        "xna",
        "xnr",
        "xog",
        "xon",
        "xpr",
        "xrb",
        "xsa",
        "xsi",
        "xsm",
        "xsr",
        "xwe",
        "yam",
        "yao",
        "yap",
        "yas",
        "yat",
        "yav",
        "yay",
        "yaz",
        "yba",
        "ybb",
        "yby",
        "yer",
        "ygr",
        "ygw",
        "yi",
        "yko",
        "yle",
        "ylg",
        "yll",
        "yml",
        "yo",
        "yon",
        "yrb",
        "yre",
        "yrl",
        "yss",
        "yua",
        "yue",
        "yue-CN",
        "yue-Hans",
        "yuj",
        "yut",
        "yuw",
        "za",
        "zag",
        "zdj",
        "zea",
        "zgh",
        "zh",
        "zh-AU",
        "zh-BN",
        "zh-Bopo",
        "zh-GB",
        "zh-GF",
        "zh-Hanb",
        "zh-Hant",
        "zh-HK",
        "zh-ID",
        "zh-MO",
        "zh-PA",
        "zh-PF",
        "zh-PH",
        "zh-SR",
        "zh-TH",
        "zh-TW",
        "zh-US",
        "zh-VN",
        "zhx",
        "zia",
        "zkt",
        "zlm",
        "zmi",
        "zne",
        "zu",
        "zza",
    ];
}

#[allow(dead_code)]
pub mod short_subtags_10pct {
    pub static STRINGS: &[&str] = &[
        "aa",
        "acd",
        "aeb",
        "ahl",
        "amm",
        "aom",
        "arc-Nbat",
        "asa",
        "avl",
        "az",
        "bas",
        "bcf",
        "bef",
        "bft",
        "bho",
        "bjh",
        "bkq",
        "bmu",
        "bqc",
        "bsj",
        "bug",
        "bye",
        "bzw",
        "cgg",
        "cjv",
        "cop",
        "csw",
        "dah",
        "den",
        "dnj",
        "dtp",
        "dyo",
        "eky",
        "es",
        "ext",
        "ffi",
        "fod",
        "fub",
        "fy",
        "gay",
        "gdr",
        "gjk",
        "gnd",
        "grb",
        "gur",
        "gwt",
        "hbb",
        "hil",
        "ho",
        "hur",
        "ich",
        "ijj",
        "iou",
        "ja",
        "jib",
        "kac",
        "kbq",
        "kdt",
        "kgp",
        "kij",
        "kk-Arab",
        "klx",
        "knp",
        "kpr",
        "krl",
        "ktb",
        "kue",
        "kw",
        "kxp",
        "kzh",
        "las",
        "lem",
        "lif",
        "lle",
        "lok",
        "luo",
        "mai",
        "mbq",
        "mdf",
        "met",
        "mgo",
        "miw",
        "mmo",
        "mo",
        "mql",
        "mti",
        "mwv",
        "myz",
        "nak",
        "nco",
        "ng",
        "nin",
        "nn",
        "nop",
        "ntm",
        "nxr",
        "okr",
        "os",
        "pal",
        "ped",
        "pl",
        "pra",
        "quc",
        "rhg",
        "rmu",
        "ru",
        "saq",
        "scl",
        "se",
        "shi",
        "sk",
        "sma",
        "snx",
        "sps",
        "srx",
        "sue",
        "swv",
        "taq",
        "tdd",
        "tg",
        "ti",
        "tkt",
        "tof",
        "trw",
        "ttj",
        "tvu",
        "udi",
        "und",
        "und-018",
        "und-057",
        "und-419",
        "und-AQ",
        "und-Arab-MN",
        "und-Armn",
        "und-Bass",
        "und-BJ",
        "und-Bugi",
        "und-CH",
        "und-CP",
        "und-Cyrl-AL",
        "und-DE",
        "und-Dogr",
        "und-Elym",
        "und-GA",
        "und-Goth",
        "und-Guru",
        "und-Hebr-SE",
        "und-HR",
        "und-IS",
        "und-Kawi",
        "und-Kore",
        "und-Latn-AM",
        "und-Latn-MM",
        "und-LI",
        "und-LY",
        "und-MD",
        "und-Mlym",
        "und-Mtei",
        "und-NA",
        "und-NL",
        "und-Osma",
        "und-Phag",
        "und-PS",
        "und-RS",
        "und-SE",
        "und-SM",
        "und-SV",
        "und-Tavt",
        "und-Thai-CN",
        "und-Tnsa",
        "und-UY",
        "und-WF",
        "und-ZW",
        "usa",
        "vai",
        "vmw",
        "wan",
        "wiu",
        "wob",
        "xbi",
        "xmr",
        "xsr",
        "yba",
        "yll",
        "yue-CN",
        "zh",
        "zh-MO",
        "zia",
    ];
}
