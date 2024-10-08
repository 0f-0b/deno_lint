// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.
//
// This file was generated with ECMAScript specifications.
// Originally from: https://github.com/mysticatea/regexpp

use super::{EcmaVersion, UnicodeChar};

use once_cell::sync::Lazy;
use std::collections::HashSet;

struct PatternVersions {
  es2018: HashSet<&'static str>,
  es2019: HashSet<&'static str>,
  es2020: HashSet<&'static str>,
}

static GC_NAME_PATTERN: Lazy<HashSet<&'static str>> =
  Lazy::new(|| ["General_Category", "gc"].iter().copied().collect());
static SC_NAME_PATTERN: Lazy<HashSet<&'static str>> = Lazy::new(|| {
  ["Script", "Script_Extensions", "sc", "scx"]
    .iter()
    .copied()
    .collect()
});
static GC_VALUE_PATTERNS: Lazy<PatternVersions> =
  Lazy::new(|| PatternVersions {
    es2018: [
      "C",
      "Cased_Letter",
      "Cc",
      "Cf",
      "Close_Punctuation",
      "Cn",
      "Co",
      "Combining_Mark",
      "Connector_Punctuation",
      "Control",
      "Cs",
      "Currency_Symbol",
      "Dash_Punctuation",
      "Decimal_Number",
      "Enclosing_Mark",
      "Final_Punctuation",
      "Format",
      "Initial_Punctuation",
      "L",
      "LC",
      "Letter",
      "Letter_Number",
      "Line_Separator",
      "Ll",
      "Lm",
      "Lo",
      "Lowercase_Letter",
      "Lt",
      "Lu",
      "M",
      "Mark",
      "Math_Symbol",
      "Mc",
      "Me",
      "Mn",
      "Modifier_Letter",
      "Modifier_Symbol",
      "N",
      "Nd",
      "Nl",
      "No",
      "Nonspacing_Mark",
      "Number",
      "Open_Punctuation",
      "Other",
      "Other_Letter",
      "Other_Number",
      "Other_Punctuation",
      "Other_Symbol",
      "P",
      "Paragraph_Separator",
      "Pc",
      "Pd",
      "Pe",
      "Pf",
      "Pi",
      "Po",
      "Private_Use",
      "Ps",
      "Punctuation",
      "S",
      "Sc",
      "Separator",
      "Sk",
      "Sm",
      "So",
      "Space_Separator",
      "Spacing_Mark",
      "Surrogate",
      "Symbol",
      "Titlecase_Letter",
      "Unassigned",
      "Uppercase_Letter",
      "Z",
      "Zl",
      "Zp",
      "Zs",
      "cntrl",
      "digit",
      "punct",
    ]
    .iter()
    .copied()
    .collect(),

    es2019: HashSet::new(),
    es2020: HashSet::new(),
  });
static SC_VALUE_PATTERNS: Lazy<PatternVersions> =
  Lazy::new(|| PatternVersions {
    es2018: [
      "Adlam",
      "Adlm",
      "Aghb",
      "Ahom",
      "Anatolian_Hieroglyphs",
      "Arab",
      "Arabic",
      "Armenian",
      "Armi",
      "Armn",
      "Avestan",
      "Avst",
      "Bali",
      "Balinese",
      "Bamu",
      "Bamum",
      "Bass",
      "Bassa_Vah",
      "Batak",
      "Batk",
      "Beng",
      "Bengali",
      "Bhaiksuki",
      "Bhks",
      "Bopo",
      "Bopomofo",
      "Brah",
      "Brahmi",
      "Brai",
      "Braille",
      "Bugi",
      "Buginese",
      "Buhd",
      "Buhid",
      "Cakm",
      "Canadian_Aboriginal",
      "Cans",
      "Cari",
      "Carian",
      "Caucasian_Albanian",
      "Chakma",
      "Cham",
      "Cher",
      "Cherokee",
      "Common",
      "Copt",
      "Coptic",
      "Cprt",
      "Cuneiform",
      "Cypriot",
      "Cyrillic",
      "Cyrl",
      "Deseret",
      "Deva",
      "Devanagari",
      "Dsrt",
      "Dupl",
      "Duployan",
      "Egyp",
      "Egyptian_Hieroglyphs",
      "Elba",
      "Elbasan",
      "Ethi",
      "Ethiopic",
      "Geor",
      "Georgian",
      "Glag",
      "Glagolitic",
      "Gonm",
      "Goth",
      "Gothic",
      "Gran",
      "Grantha",
      "Greek",
      "Grek",
      "Gujarati",
      "Gujr",
      "Gurmukhi",
      "Guru",
      "Han",
      "Hang",
      "Hangul",
      "Hani",
      "Hano",
      "Hanunoo",
      "Hatr",
      "Hatran",
      "Hebr",
      "Hebrew",
      "Hira",
      "Hiragana",
      "Hluw",
      "Hmng",
      "Hung",
      "Imperial_Aramaic",
      "Inherited",
      "Inscriptional_Pahlavi",
      "Inscriptional_Parthian",
      "Ital",
      "Java",
      "Javanese",
      "Kaithi",
      "Kali",
      "Kana",
      "Kannada",
      "Katakana",
      "Kayah_Li",
      "Khar",
      "Kharoshthi",
      "Khmer",
      "Khmr",
      "Khoj",
      "Khojki",
      "Khudawadi",
      "Knda",
      "Kthi",
      "Lana",
      "Lao",
      "Laoo",
      "Latin",
      "Latn",
      "Lepc",
      "Lepcha",
      "Limb",
      "Limbu",
      "Lina",
      "Linb",
      "Linear_A",
      "Linear_B",
      "Lisu",
      "Lyci",
      "Lycian",
      "Lydi",
      "Lydian",
      "Mahajani",
      "Mahj",
      "Malayalam",
      "Mand",
      "Mandaic",
      "Mani",
      "Manichaean",
      "Marc",
      "Marchen",
      "Masaram_Gondi",
      "Meetei_Mayek",
      "Mend",
      "Mende_Kikakui",
      "Merc",
      "Mero",
      "Meroitic_Cursive",
      "Meroitic_Hieroglyphs",
      "Miao",
      "Mlym",
      "Modi",
      "Mong",
      "Mongolian",
      "Mro",
      "Mroo",
      "Mtei",
      "Mult",
      "Multani",
      "Myanmar",
      "Mymr",
      "Nabataean",
      "Narb",
      "Nbat",
      "New_Tai_Lue",
      "Newa",
      "Nko",
      "Nkoo",
      "Nshu",
      "Nushu",
      "Ogam",
      "Ogham",
      "Ol_Chiki",
      "Olck",
      "Old_Hungarian",
      "Old_Italic",
      "Old_North_Arabian",
      "Old_Permic",
      "Old_Persian",
      "Old_South_Arabian",
      "Old_Turkic",
      "Oriya",
      "Orkh",
      "Orya",
      "Osage",
      "Osge",
      "Osma",
      "Osmanya",
      "Pahawh_Hmong",
      "Palm",
      "Palmyrene",
      "Pau_Cin_Hau",
      "Pauc",
      "Perm",
      "Phag",
      "Phags_Pa",
      "Phli",
      "Phlp",
      "Phnx",
      "Phoenician",
      "Plrd",
      "Prti",
      "Psalter_Pahlavi",
      "Qaac",
      "Qaai",
      "Rejang",
      "Rjng",
      "Runic",
      "Runr",
      "Samaritan",
      "Samr",
      "Sarb",
      "Saur",
      "Saurashtra",
      "Sgnw",
      "Sharada",
      "Shavian",
      "Shaw",
      "Shrd",
      "Sidd",
      "Siddham",
      "SignWriting",
      "Sind",
      "Sinh",
      "Sinhala",
      "Sora",
      "Sora_Sompeng",
      "Soyo",
      "Soyombo",
      "Sund",
      "Sundanese",
      "Sylo",
      "Syloti_Nagri",
      "Syrc",
      "Syriac",
      "Tagalog",
      "Tagb",
      "Tagbanwa",
      "Tai_Le",
      "Tai_Tham",
      "Tai_Viet",
      "Takr",
      "Takri",
      "Tale",
      "Talu",
      "Tamil",
      "Taml",
      "Tang",
      "Tangut",
      "Tavt",
      "Telu",
      "Telugu",
      "Tfng",
      "Tglg",
      "Thaa",
      "Thaana",
      "Thai",
      "Tibetan",
      "Tibt",
      "Tifinagh",
      "Tirh",
      "Tirhuta",
      "Ugar",
      "Ugaritic",
      "Vai",
      "Vaii",
      "Wara",
      "Warang_Citi",
      "Xpeo",
      "Xsux",
      "Yi",
      "Yiii",
      "Zanabazar_Square",
      "Zanb",
      "Zinh",
      "Zyyy",
    ]
    .iter()
    .copied()
    .collect(),

    es2019: [
      "Dogr",
      "Dogra",
      "Gong",
      "Gunjala_Gondi",
      "Hanifi_Rohingya",
      "Maka",
      "Makasar",
      "Medefaidrin",
      "Medf",
      "Old_Sogdian",
      "Rohg",
      "Sogd",
      "Sogdian",
      "Sogo",
    ]
    .iter()
    .copied()
    .collect(),

    es2020: [
      "Elym",
      "Elymaic",
      "Hmnp",
      "Nand",
      "Nandinagari",
      "Nyiakeng_Puachue_Hmong",
      "Wancho",
      "Wcho",
    ]
    .iter()
    .copied()
    .collect(),
  });
static BIN_PROPERTY_PATTERNS: Lazy<PatternVersions> =
  Lazy::new(|| PatternVersions {
    es2018: [
      "AHex",
      "ASCII",
      "ASCII_Hex_Digit",
      "Alpha",
      "Alphabetic",
      "Any",
      "Assigned",
      "Bidi_C",
      "Bidi_Control",
      "Bidi_M",
      "Bidi_Mirrored",
      "CI",
      "CWCF",
      "CWCM",
      "CWKCF",
      "CWL",
      "CWT",
      "CWU",
      "Case_Ignorable",
      "Cased",
      "Changes_When_Casefolded",
      "Changes_When_Casemapped",
      "Changes_When_Lowercased",
      "Changes_When_NFKC_Casefolded",
      "Changes_When_Titlecased",
      "Changes_When_Uppercased",
      "DI",
      "Dash",
      "Default_Ignorable_Code_Point",
      "Dep",
      "Deprecated",
      "Dia",
      "Diacritic",
      "Emoji",
      "Emoji_Component",
      "Emoji_Modifier",
      "Emoji_Modifier_Base",
      "Emoji_Presentation",
      "Ext",
      "Extender",
      "Gr_Base",
      "Gr_Ext",
      "Grapheme_Base",
      "Grapheme_Extend",
      "Hex",
      "Hex_Digit",
      "IDC",
      "IDS",
      "IDSB",
      "IDST",
      "IDS_Binary_Operator",
      "IDS_Trinary_Operator",
      "ID_Continue",
      "ID_Start",
      "Ideo",
      "Ideographic",
      "Join_C",
      "Join_Control",
      "LOE",
      "Logical_Order_Exception",
      "Lower",
      "Lowercase",
      "Math",
      "NChar",
      "Noncharacter_Code_Point",
      "Pat_Syn",
      "Pat_WS",
      "Pattern_Syntax",
      "Pattern_White_Space",
      "QMark",
      "Quotation_Mark",
      "RI",
      "Radical",
      "Regional_Indicator",
      "SD",
      "STerm",
      "Sentence_Terminal",
      "Soft_Dotted",
      "Term",
      "Terminal_Punctuation",
      "UIdeo",
      "Unified_Ideograph",
      "Upper",
      "Uppercase",
      "VS",
      "Variation_Selector",
      "White_Space",
      "XIDC",
      "XIDS",
      "XID_Continue",
      "XID_Start",
      "space",
    ]
    .iter()
    .copied()
    .collect(),

    es2019: ["Extended_Pictographic"].iter().copied().collect(),

    es2020: HashSet::new(),
  });
static LARGE_ID_START_RANGES: Lazy<Vec<u32>> = Lazy::new(|| {
  restore_ranges(
        "4q 0 b 0 5 0 6 m 2 u 2 cp 5 b f 4 8 0 2 0 3m 4 2 1 3 3 2 0 7 0 2 2 2 0 2 j 2 2a 2 3u 9 4l 2 11 3 0 7 14 20 q 5 3 1a 16 10 1 2 2q 2 0 g 1 8 1 b 2 3 0 h 0 2 t u 2g c 0 p w a 1 5 0 6 l 5 0 a 0 4 0 o o 8 a 1i k 2 h 1p 1h 4 0 j 0 8 9 g f 5 7 3 1 3 l 2 6 2 0 4 3 4 0 h 0 e 1 2 2 f 1 b 0 9 5 5 1 3 l 2 6 2 1 2 1 2 1 w 3 2 0 k 2 h 8 2 2 2 l 2 6 2 1 2 4 4 0 j 0 g 1 o 0 c 7 3 1 3 l 2 6 2 1 2 4 4 0 v 1 2 2 g 0 i 0 2 5 4 2 2 3 4 1 2 0 2 1 4 1 4 2 4 b n 0 1h 7 2 2 2 m 2 f 4 0 r 2 6 1 v 0 5 7 2 2 2 m 2 9 2 4 4 0 x 0 2 1 g 1 i 8 2 2 2 14 3 0 h 0 6 2 9 2 p 5 6 h 4 n 2 8 2 0 3 6 1n 1b 2 1 d 6 1n 1 2 0 2 4 2 n 2 0 2 9 2 1 a 0 3 4 2 0 m 3 x 0 1s 7 2 z s 4 38 16 l 0 h 5 5 3 4 0 4 1 8 2 5 c d 0 i 11 2 0 6 0 3 16 2 98 2 3 3 6 2 0 2 3 3 14 2 3 3 w 2 3 3 6 2 0 2 3 3 e 2 1k 2 3 3 1u 12 f h 2d 3 5 4 h7 3 g 2 p 6 22 4 a 8 c 2 3 f h f h f c 2 2 g 1f 10 0 5 0 1w 2g 8 14 2 0 6 1x b u 1e t 3 4 c 17 5 p 1j m a 1g 2b 0 2m 1a i 6 1k t e 1 b 17 r z 16 2 b z 3 8 8 16 3 2 16 3 2 5 2 1 4 0 6 5b 1t 7p 3 5 3 11 3 5 3 7 2 0 2 0 2 0 2 u 3 1g 2 6 2 0 4 2 2 6 4 3 3 5 5 c 6 2 2 6 39 0 e 0 h c 2u 0 5 0 3 9 2 0 3 5 7 0 2 0 2 0 2 f 3 3 6 4 5 0 i 14 22g 1a 2 1a 2 3o 7 3 4 1 d 11 2 0 6 0 3 1j 8 0 h m a 6 2 6 2 6 2 6 2 6 2 6 2 6 2 6 fb 2 q 8 8 4 3 4 5 2d 5 4 2 2h 2 3 6 16 2 2l i v 1d f e9 533 1t g70 4 wc 1w 19 3 7g 4 f b 1 l 1a h u 3 27 14 8 3 2u 3 1g 3 8 17 c 2 2 2 3 2 m u 1f f 1d 1r 5 4 0 2 1 c r b m q s 8 1a t 0 h 4 2 9 b 4 2 14 o 2 2 7 l m 4 0 4 1d 2 0 4 1 3 4 3 0 2 0 p 2 3 a 8 2 d 5 3 5 3 5 a 6 2 6 2 16 2 d 7 36 u 8mb d m 5 1c 6it a5 3 2x 13 6 d 4 6 0 2 9 2 c 2 4 2 0 2 1 2 1 2 2z y a2 j 1r 3 1h 15 b 39 4 2 3q 11 p 7 p c 2g 4 5 3 5 3 5 3 2 10 b 2 p 2 i 2 1 2 e 3 d z 3e 1y 1g 7g s 4 1c 1c v e t 6 11 b t 3 z 5 7 2 4 17 4d j z 5 z 5 13 9 1f 4d 8m a l b 7 49 5 3 0 2 17 2 1 4 0 3 m b m a u 1u i 2 1 b l b p 1z 1j 7 1 1t 0 g 3 2 2 2 s 17 s 4 s 10 7 2 r s 1h b l b i e h 33 20 1k 1e e 1e e z 9p 15 7 1 27 s b 0 9 l 2z k s m d 1g 24 18 x o r z u 0 3 0 9 y 4 0 d 1b f 3 m 0 2 0 10 h 2 o 2d 6 2 0 2 3 2 e 2 9 8 1a 13 7 3 1 3 l 2 6 2 1 2 4 4 0 j 0 d 4 4f 1g j 3 l 2 v 1b l 1 2 0 55 1a 16 3 11 1b l 0 1o 16 e 0 20 q 6e 17 39 1r w 7 3 0 3 7 2 1 2 n g 0 2 0 2n 7 3 12 h 0 2 0 t 0 b 13 8 0 m 0 c 19 k 0 z 1k 7c 8 2 10 i 0 1e t 35 6 2 1 2 11 m 0 q 5 2 1 2 v f 0 94 i 5a 0 28 pl 2v 32 i 5f 24d tq 34i g6 6nu fs 8 u 36 t j 1b h 3 w k 6 i j5 1r 3l 22 6 0 1v c 1t 1 2 0 t 4qf 9 yd 17 8 6wo 7y 1e 2 i 3 9 az 1s5 2y 6 c 4 8 8 9 4mf 2c 2 1y 2 1 3 0 3 1 3 3 2 b 2 0 2 6 2 1s 2 3 3 7 2 6 2 r 2 3 2 4 2 0 4 6 2 9f 3 o 2 o 2 u 2 o 2 u 2 o 2 u 2 o 2 u 2 o 2 7 1th 18 b 6 h 0 aa 17 105 5g 1o 1v 8 0 xh 3 2 q 2 1 2 0 3 0 2 9 2 3 2 0 2 0 7 0 5 0 2 0 2 0 2 2 2 1 2 0 3 0 2 0 2 0 2 0 2 0 2 1 2 0 3 3 2 6 2 3 2 3 2 0 2 9 2 g 6 2 2 4 2 g 3et wyl z 378 c 65 3 4g1 f 5rk 2e8 f1 15v 3t6",
    )
});
static LARGE_ID_CONTINUE_RANGES: Lazy<Vec<u32>> = Lazy::new(|| {
  restore_ranges(
        "53 0 g9 33 o 0 70 4 7e 18 2 0 2 1 2 1 2 0 21 a 1d u 7 0 2u 6 3 5 3 1 2 3 3 9 o 0 v q 2k a g 9 y 8 a 0 p 3 2 8 2 2 2 4 18 2 3c e 2 w 1j 2 2 h 2 6 b 1 3 9 i 2 1l 0 2 6 3 1 3 2 a 0 b 1 3 9 f 0 3 2 1l 0 2 4 5 1 3 2 4 0 l b 4 0 c 2 1l 0 2 7 2 2 2 2 l 1 3 9 b 5 2 2 1l 0 2 6 3 1 3 2 8 2 b 1 3 9 j 0 1o 4 4 2 2 3 a 0 f 9 h 4 1m 6 2 2 2 3 8 1 c 1 3 9 i 2 1l 0 2 6 2 2 2 3 8 1 c 1 3 9 h 3 1k 1 2 6 2 2 2 3 a 0 b 1 3 9 i 2 1z 0 5 5 2 0 2 7 7 9 3 1 1q 0 3 6 d 7 2 9 2g 0 3 8 c 5 3 9 1r 1 7 9 c 0 2 0 2 0 5 1 1e j 2 1 6 a 2 z a 0 2t j 2 9 d 3 5 2 2 2 3 6 4 3 e b 2 e jk 2 a 8 pt 2 u 2 u 1 v 1 1t v a 0 3 9 y 2 3 9 40 0 3b b 5 b b 9 3l a 1p 4 1m 9 2 s 3 a 7 9 n d 2 1 1s 4 1c g c 9 i 8 d 2 v c 3 9 19 d 1d j 9 9 7 9 3b 2 2 k 5 0 7 0 3 2 5j 1l 2 4 g0 1 k 0 3g c 5 0 4 b 2db 2 3y 0 2p v ff 5 2y 1 n7q 9 1y 0 5 9 x 1 29 1 7l 0 4 0 5 0 o 4 5 0 2c 1 1f h b 9 7 h e a t 7 q c 19 3 1c d g 9 c 0 b 9 1c d d 0 9 1 3 9 y 2 1f 0 2 2 3 1 6 1 2 0 16 4 6 1 6l 7 2 1 3 9 fmt 0 ki f h f 4 1 p 2 5d 9 12 0 ji 0 6b 0 46 4 86 9 120 2 2 1 6 3 15 2 5 0 4m 1 fy 3 9 9 aa 1 4a a 4w 2 1i e w 9 g 3 1a a 1i 9 7 2 11 d 2 9 6 1 19 0 d 2 1d d 9 3 2 b 2b b 7 0 4h b 6 9 7 3 1k 1 2 6 3 1 3 2 a 0 b 1 3 6 4 4 5d h a 9 5 0 2a j d 9 5y 6 3 8 s 1 2b g g 9 2a c 9 9 2c e 5 9 6r e 4m 9 1z 5 2 1 3 3 2 0 2 1 d 9 3c 6 3 6 4 0 t 9 15 6 2 3 9 0 a a 1b f ba 7 2 7 h 9 1l l 2 d 3f 5 4 0 2 1 2 6 2 0 9 9 1d 4 2 1 2 4 9 9 96 3 ewa 9 3r 4 1o 6 q 9 s6 0 2 1i 8 3 2a 0 c 1 f58 1 43r 4 4 5 9 7 3 6 v 3 45 2 13e 1d e9 1i 5 1d 9 0 f 0 n 4 2 e 11t 6 2 g 3 6 2 1 2 4 7a 6 a 9 bn d 15j 6 32 6 6 9 3o7 9 gvt3 6n",
    )
});

pub fn is_valid_unicode_property(
  version: EcmaVersion,
  name: &str,
  value: &str,
) -> bool {
  if GC_NAME_PATTERN.contains(name)
    && version >= EcmaVersion::Es2018
    && GC_VALUE_PATTERNS.es2018.contains(value)
  {
    true
  } else if SC_NAME_PATTERN.contains(name) {
    (version >= EcmaVersion::Es2018 && SC_VALUE_PATTERNS.es2018.contains(value))
      || (version >= EcmaVersion::Es2019
        && SC_VALUE_PATTERNS.es2019.contains(value))
      || (version >= EcmaVersion::Es2020
        && SC_VALUE_PATTERNS.es2020.contains(value))
  } else {
    false
  }
}

pub fn is_valid_lone_unicode_property(
  version: EcmaVersion,
  value: &str,
) -> bool {
  (version >= EcmaVersion::Es2018
    && BIN_PROPERTY_PATTERNS.es2018.contains(value))
    || (version >= EcmaVersion::Es2019
      && BIN_PROPERTY_PATTERNS.es2019.contains(value))
}

pub fn is_large_id_start(cp: UnicodeChar) -> bool {
  is_in_range(cp, &LARGE_ID_START_RANGES)
}

pub fn is_large_id_continue(cp: UnicodeChar) -> bool {
  is_in_range(cp, &LARGE_ID_CONTINUE_RANGES)
}

fn is_in_range(cp: UnicodeChar, ranges: &[u32]) -> bool {
  let mut l = 0;
  let mut r = ranges.len() / 2;
  while l < r {
    let i = (l + r) / 2;
    let min = ranges[2 * i];
    let max = ranges[2 * i + 1];
    if cp < min {
      r = i;
    } else if cp > max {
      l = i + 1;
    } else {
      return true;
    }
  }
  false
}

fn restore_ranges(data: &str) -> Vec<u32> {
  let mut last = 0;
  data
    .split(' ')
    .map(|s| {
      last += u32::from_str_radix(s, 36).unwrap();
      last
    })
    .collect()
}
