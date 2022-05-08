// Code generated by gen/generate.go. DO NOT EDIT.
// Generate from https://raw.githubusercontent.com/google/budoux/v0.1.0/budoux/unicode_blocks.json
// This file is contains the deliverables of the [BudouX](https://github.com/google/budoux) project.
//
// BudouX | Apache License 2.0 | https://github.com/google/budoux/blob/main/LICENSE
//

/// UNICODE_BLOCKS range of code points block.
pub const UNICODE_BLOCKS: [u32; 308] = [
    0, 128, 256, 384, 592, 688, 768, 880, 1024, 1280, 1328, 1424, 1536, 1792, 1872, 1920, 1984,
    2048, 2112, 2144, 2208, 2304, 2432, 2560, 2688, 2816, 2944, 3072, 3200, 3328, 3456, 3584, 3712,
    3840, 4096, 4256, 4352, 4608, 4992, 5024, 5120, 5760, 5792, 5888, 5920, 5952, 5984, 6016, 6144,
    6320, 6400, 6480, 6528, 6624, 6656, 6688, 6832, 6912, 7040, 7104, 7168, 7248, 7296, 7312, 7360,
    7376, 7424, 7552, 7616, 7680, 7936, 8192, 8304, 8352, 8400, 8448, 8528, 8592, 8704, 8960, 9216,
    9280, 9312, 9472, 9600, 9632, 9728, 9984, 10176, 10224, 10240, 10496, 10624, 10752, 11008,
    11264, 11360, 11392, 11520, 11568, 11648, 11744, 11776, 11904, 12032, 12272, 12288, 12352,
    12448, 12544, 12592, 12688, 12704, 12736, 12784, 12800, 13056, 13312, 19904, 19968, 40960,
    42128, 42192, 42240, 42560, 42656, 42752, 42784, 43008, 43056, 43072, 43136, 43232, 43264,
    43312, 43360, 43392, 43488, 43520, 43616, 43648, 43744, 43776, 43824, 43888, 43968, 44032,
    55216, 55296, 56192, 56320, 57344, 63744, 64256, 64336, 65024, 65040, 65056, 65072, 65104,
    65136, 65280, 65520, 65536, 65664, 65792, 65856, 65936, 66000, 66176, 66208, 66272, 66304,
    66352, 66384, 66432, 66464, 66560, 66640, 66688, 66736, 66816, 66864, 67072, 67584, 67648,
    67680, 67712, 67808, 67840, 67872, 67968, 68000, 68096, 68192, 68224, 68288, 68352, 68416,
    68448, 68480, 68608, 68736, 68864, 69216, 69248, 69376, 69424, 69552, 69600, 69632, 69760,
    69840, 69888, 69968, 70016, 70112, 70144, 70272, 70320, 70400, 70656, 70784, 71040, 71168,
    71264, 71296, 71424, 71680, 71840, 71936, 72096, 72192, 72272, 72384, 72704, 72816, 72960,
    73056, 73440, 73648, 73664, 73728, 74752, 74880, 77824, 78896, 82944, 92160, 92736, 92880,
    92928, 93760, 93952, 94176, 94208, 100352, 101120, 101632, 110592, 110848, 110896, 110960,
    113664, 113824, 118784, 119040, 119296, 119520, 119552, 119648, 119808, 120832, 122880, 123136,
    123584, 124928, 125184, 126064, 126208, 126464, 126976, 127024, 127136, 127232, 127488, 127744,
    128512, 128592, 128640, 128768, 128896, 129024, 129280, 129536, 129648, 129792, 131072, 173824,
    177984, 178208, 183984, 194560, 196608, 917504, 917760, 983040, 1048576,
];

/// BLOCK_FEATURES feature of unicode block.
pub const BLOCK_FEATURES: [&str; 308] = [
    "000", "001", "002", "003", "004", "005", "006", "007", "008", "009", "010", "011", "012",
    "013", "014", "015", "016", "017", "018", "019", "020", "021", "022", "023", "024", "025",
    "026", "027", "028", "029", "030", "031", "032", "033", "034", "035", "036", "037", "038",
    "039", "040", "041", "042", "043", "044", "045", "046", "047", "048", "049", "050", "051",
    "052", "053", "054", "055", "056", "057", "058", "059", "060", "061", "062", "063", "064",
    "065", "066", "067", "068", "069", "070", "071", "072", "073", "074", "075", "076", "077",
    "078", "079", "080", "081", "082", "083", "084", "085", "086", "087", "088", "089", "090",
    "091", "092", "093", "094", "095", "096", "097", "098", "099", "100", "101", "102", "103",
    "104", "105", "106", "107", "108", "109", "110", "111", "112", "113", "114", "115", "116",
    "117", "118", "119", "120", "121", "122", "123", "124", "125", "126", "127", "128", "129",
    "130", "131", "132", "133", "134", "135", "136", "137", "138", "139", "140", "141", "142",
    "143", "144", "145", "146", "147", "148", "149", "150", "151", "152", "153", "154", "155",
    "156", "157", "158", "159", "160", "161", "162", "163", "164", "165", "166", "167", "168",
    "169", "170", "171", "172", "173", "174", "175", "176", "177", "178", "179", "180", "181",
    "182", "183", "184", "185", "186", "187", "188", "189", "190", "191", "192", "193", "194",
    "195", "196", "197", "198", "199", "200", "201", "202", "203", "204", "205", "206", "207",
    "208", "209", "210", "211", "212", "213", "214", "215", "216", "217", "218", "219", "220",
    "221", "222", "223", "224", "225", "226", "227", "228", "229", "230", "231", "232", "233",
    "234", "235", "236", "237", "238", "239", "240", "241", "242", "243", "244", "245", "246",
    "247", "248", "249", "250", "251", "252", "253", "254", "255", "256", "257", "258", "259",
    "260", "261", "262", "263", "264", "265", "266", "267", "268", "269", "270", "271", "272",
    "273", "274", "275", "276", "277", "278", "279", "280", "281", "282", "283", "284", "285",
    "286", "287", "288", "289", "290", "291", "292", "293", "294", "295", "296", "297", "298",
    "299", "300", "301", "302", "303", "304", "305", "306", "307",
];
