//! # Overview
//!
//! BudouX-rs is a rust port of [BudouX](https://github.com/google/budoux) (machine learning powered line break organizer tool).
//!
//! Note:
//! This project contains the deliverables of the [BudouX](https://github.com/google/budoux) project.
//!
//! Note:
//! BudouX-rs supported plain text only, not supports html inputs.

use std::collections::HashMap;

mod unicode_blocks;

/// models provides trained machine learning model.
pub mod models;

/// DEFAULT_THRESHOLD is default threshold for splitting a sentences.
pub const DEFAULT_THRESHOLD: i32 = 1000;

/// Model is type of trained machine learning model.
/// key (String) is feature of character, value (i32) is score of feature.
pub type Model = HashMap<String, i32>;

/// INVALID_FEATURE is indicate for invalid feature.
const INVALID_FEATURE: &str = "▔";

/// parse returns splitted string slice from input.
/// It is shorthand for budoux::parse_with_threshold(model, input, budoux::DEFAULT_THRESHOLD).
///
/// * `model` - trained machine learning model.
/// * `input` - input sentences.
///
/// # Examples
///
/// Split sentences with internal model.
///
/// ```
/// let model = budoux::models::default_japanese_model();
/// let words = budoux::parse(model, "これはテストです。");
///
/// assert_eq!(words, vec!["これは", "テストです。"]);
/// ```
///
/// Load model from json file and split sentences using the loaded model.
///
/// ```ignore
/// let file = File::open(path_to_json).unwrap();
/// let reader = BufReader::new(file);
/// let model: budoux::Model = serde_json::from_reader(reader).unwrap();
/// let words = budoux::parse(&model, "これはテストです。");
///
/// assert_eq!(words, vec!["これは", "テストです。"]);
/// ```
pub fn parse(model: &Model, input: &str) -> Vec<String> {
    parse_with_threshold(model, input, DEFAULT_THRESHOLD)
}

/// parse_with_threshold returns splitted string slice from input.
///
/// * `model` - trained machine learning model.
/// * `input` - input sentences.
/// * `threshold` - threshold for splitting a sentences.
///
/// # Examples
///
/// Split sentences with internal model.
///
/// ```
/// let model = budoux::models::default_japanese_model();
/// let words = budoux::parse_with_threshold(model, "これはテストです。", budoux::DEFAULT_THRESHOLD);
///
/// assert_eq!(words, vec!["これは", "テストです。"]);
/// ```
///
/// If you use a large threshold, will not be split.
///
/// ```
/// let model = budoux::models::default_japanese_model();
/// let words = budoux::parse_with_threshold(model, "これはテストです。", 100000000);
///
/// assert_eq!(words, vec!["これはテストです。"]);
/// ```
pub fn parse_with_threshold(model: &Model, input: &str, threshold: i32) -> Vec<String> {
    if input.is_empty() {
        return vec![String::default()];
    }

    let mut out: Vec<String> = Vec::new();

    let mut p1 = "U"; // unknown
    let mut p2 = "U"; // unknown
    let mut p3 = "U"; // unknown

    let mut chars = input.char_indices();

    let (mut w1, mut b1) = ("", INVALID_FEATURE); // i - 3
    let (mut w2, mut b2) = ("", INVALID_FEATURE); // i - 2
    let (mut w3, mut s3, mut b3) = get_unicode_block_and_feature(input, &mut chars); // i - 1
    let (mut w4, mut s4, mut b4) = get_unicode_block_and_feature(input, &mut chars); // i
    let (mut w5, mut s5, mut b5) = get_unicode_block_and_feature(input, &mut chars); // i + 1

    let mut start: usize = 0;
    let mut end = s3;

    let mut wb = String::with_capacity(20); // working buffer

    while s3 != 0 {
        let (w6, s6, b6) = get_unicode_block_and_feature(input, &mut chars);

        let score = get_feature(
            model, &mut wb, w1, w2, w3, w4, w5, w6, b1, b2, b3, b4, b5, b6, p1, p2, p3,
        );

        if score > threshold {
            out.push(input[start..end].to_string());
            start = end;
        }

        p1 = p2;
        p2 = p3;

        if score > 0 {
            p3 = "B"; // positive
        } else {
            p3 = "O"; // negative
        }

        w1 = w2;
        w2 = w3;
        w3 = w4;
        w4 = w5;
        w5 = w6;

        b1 = b2;
        b2 = b3;
        b3 = b4;
        b4 = b5;
        b5 = b6;

        s3 = s4;
        s4 = s5;
        s5 = s6;

        end += s3;
    }

    if start < input.len() {
        out.push(input[start..].to_string());
    }

    out
}

/// get_unicode_block_and_feature returns unicode character and block feature from char slice.
fn get_unicode_block_and_feature<'a>(
    input: &'a str,
    chars: &mut std::str::CharIndices,
) -> (&'a str, usize, &'a str) {
    let v = chars.next();
    if v.is_none() {
        return ("", 0, INVALID_FEATURE);
    }

    let (index, c) = v.unwrap();
    let size = c.len_utf8();

    let pos = match unicode_blocks::UNICODE_BLOCKS.binary_search(&(c as u32)) {
        Ok(v) => v + 1,
        Err(e) => e,
    };

    (
        &input[index..index + size],
        size,
        unicode_blocks::BLOCK_FEATURES[pos],
    )
}

/// get_feature returns feature list.
#[allow(clippy::too_many_arguments)]
fn get_feature(
    model: &Model,
    buf: &mut String, // working buffer
    w1: &str,
    w2: &str,
    w3: &str,
    w4: &str,
    w5: &str,
    w6: &str,
    b1: &str,
    b2: &str,
    b3: &str,
    b4: &str,
    b5: &str,
    b6: &str,
    p1: &str,
    p2: &str,
    p3: &str,
) -> i32 {
    let mut score: i32 = 0;

    // UP is means unigram of previous results.
    score += model.get(key(buf, &["UP1:", p1])).unwrap_or(&0);
    score += model.get(key(buf, &["UP2:", p2])).unwrap_or(&0);
    score += model.get(key(buf, &["UP3:", p3])).unwrap_or(&0);
    // BP is means bigram of previous results.
    score += model.get(key(buf, &["BP1:", p1, p2])).unwrap_or(&0);
    score += model.get(key(buf, &["BP2:", p2, p3])).unwrap_or(&0);
    // UW is means unigram of words.
    score += model.get(key(buf, &["UW1:", w1])).unwrap_or(&0);
    score += model.get(key(buf, &["UW2:", w2])).unwrap_or(&0);
    score += model.get(key(buf, &["UW3:", w3])).unwrap_or(&0);
    score += model.get(key(buf, &["UW4:", w4])).unwrap_or(&0);
    score += model.get(key(buf, &["UW5:", w5])).unwrap_or(&0);
    score += model.get(key(buf, &["UW6:", w6])).unwrap_or(&0);
    // BW is means bigram of words.
    score += model.get(key(buf, &["BW1:", w2, w3])).unwrap_or(&0);
    score += model.get(key(buf, &["BW2:", w3, w4])).unwrap_or(&0);
    score += model.get(key(buf, &["BW3:", w4, w5])).unwrap_or(&0);
    // TW is means trigram of words.
    score += model.get(key(buf, &["TW1:", w1, w2, w3])).unwrap_or(&0);
    score += model.get(key(buf, &["TW2:", w2, w3, w4])).unwrap_or(&0);
    score += model.get(key(buf, &["TW3:", w3, w4, w5])).unwrap_or(&0);
    score += model.get(key(buf, &["TW4:", w4, w5, w6])).unwrap_or(&0);
    // UB is means unigram of unicode blocks.
    score += model.get(key(buf, &["UB1:", b1])).unwrap_or(&0);
    score += model.get(key(buf, &["UB2:", b2])).unwrap_or(&0);
    score += model.get(key(buf, &["UB3:", b3])).unwrap_or(&0);
    score += model.get(key(buf, &["UB4:", b4])).unwrap_or(&0);
    score += model.get(key(buf, &["UB5:", b5])).unwrap_or(&0);
    score += model.get(key(buf, &["UB6:", b6])).unwrap_or(&0);
    // BB is means bigram of unicode blocks.
    score += model.get(key(buf, &["BB1:", b2, b3])).unwrap_or(&0);
    score += model.get(key(buf, &["BB2:", b3, b4])).unwrap_or(&0);
    score += model.get(key(buf, &["BB3:", b4, b5])).unwrap_or(&0);
    // TB is means trigram of unicode blocks.
    score += model.get(key(buf, &["TB1:", b1, b2, b3])).unwrap_or(&0);
    score += model.get(key(buf, &["TB2:", b2, b3, b4])).unwrap_or(&0);
    score += model.get(key(buf, &["TB3:", b3, b4, b5])).unwrap_or(&0);
    score += model.get(key(buf, &["TB4:", b4, b5, b6])).unwrap_or(&0);
    // UQ is combination of UP and UB.
    score += model.get(key(buf, &["UQ1:", p1, b1])).unwrap_or(&0);
    score += model.get(key(buf, &["UQ2:", p2, b2])).unwrap_or(&0);
    score += model.get(key(buf, &["UQ3:", p3, b3])).unwrap_or(&0);
    // BQ is combination of UP and BB.
    score += model.get(key(buf, &["BQ1:", p2, b2, b3])).unwrap_or(&0);
    score += model.get(key(buf, &["BQ2:", p2, b3, b4])).unwrap_or(&0);
    score += model.get(key(buf, &["BQ3:", p3, b2, b3])).unwrap_or(&0);
    score += model.get(key(buf, &["BQ4:", p3, b3, b4])).unwrap_or(&0);
    // TQ is combination of UP and TB.
    score += model.get(key(buf, &["TQ1:", p2, b1, b2, b3])).unwrap_or(&0);
    score += model.get(key(buf, &["TQ2:", p2, b2, b3, b4])).unwrap_or(&0);
    score += model.get(key(buf, &["TQ3:", p3, b1, b2, b3])).unwrap_or(&0);
    score += model.get(key(buf, &["TQ4:", p3, b2, b3, b4])).unwrap_or(&0);

    score
}

/// key returns feature key.
fn key<'a>(buf: &'a mut String, params: &[&str]) -> &'a str {
    buf.clear();
    for param in params {
        buf.push_str(param);
    }

    buf
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse() {
        let m = super::models::default_japanese_model();

        assert_eq!(super::parse(m, ""), vec![""]);
        assert_eq!(super::parse(m, "日本語"), vec!["日本語"]);
        assert_eq!(super::parse(m, "水と油"), vec!["水と", "油"]);
        assert_eq!(
            super::parse(m, "水道水とミネラルウォーター"),
            vec!["水道水と", "ミネラルウォーター"]
        );
        assert_eq!(
            super::parse(m, "PythonとJavaScriptとGolang"),
            vec!["Pythonと", "JavaScriptと", "Golang"]
        );
        assert_eq!(
            super::parse(
                m,
                "日本語の文章において語の区切りに空白を挟んで記述すること"
            ),
            vec![
                "日本語の",
                "文章に",
                "おいて",
                "語の",
                "区切りに",
                "空白を",
                "挟んで",
                "記述する",
                "こと"
            ]
        );
        assert_eq!(
            super::parse(m, "これはテストです。"),
            vec!["これは", "テストです。"]
        );
        assert_eq!(
            super::parse(m, "これは美しいペンです。"),
            vec!["これは", "美しい", "ペンです。"]
        );
        assert_eq!(
            super::parse(m, "今日は天気です。"),
            vec!["今日は", "天気です。"]
        );
        assert_eq!(
            super::parse(m, "今日はとても天気です。"),
            vec!["今日は", "とても", "天気です。"]
        );
        assert_eq!(
            super::parse(m, "あなたに寄り添う最先端のテクノロジー。"),
            vec!["あなたに", "寄り添う", "最先端の", "テクノロジー。"]
        );
        assert_eq!(
            super::parse(m, "これはテストです。今日は晴天です。"),
            vec!["これは", "テストです。", "今日は", "晴天です。"]
        );
        assert_eq!(
            super::parse(m, "これはテストです。\n今日は晴天です。"),
            vec!["これは", "テストです。", "\n今日は", "晴天です。"]
        );
    }

    #[test]
    fn test_parse_zh_hans() {
        let m = super::models::default_simplified_chinese_model();

        assert_eq!(super::parse(m, ""), vec![""]);
        assert_eq!(
            super::parse(m, "今天是晴天。"),
            vec!["今天", "是", "晴天。"]
        );
    }

    #[test]
    fn test_get_unicode_block_and_feature() {
        let seek = |chars: &mut std::str::CharIndices, offset: usize| {
            for _ in 0..offset {
                chars.next();
            }
        };

        let input = "abc";
        let mut chars = input.char_indices();
        seek(&mut chars, 0);
        assert_eq!(
            super::get_unicode_block_and_feature(input, &mut chars),
            ("a", 1, "001",)
        );

        let input = "xyz";
        let mut chars = input.char_indices();
        seek(&mut chars, 2);
        assert_eq!(
            super::get_unicode_block_and_feature(input, &mut chars),
            ("z", 1, "001",)
        );

        let input = "out of index";
        let mut chars = input.char_indices();
        seek(&mut chars, 12);
        assert_eq!(
            super::get_unicode_block_and_feature(input, &mut chars),
            ("", 0, super::INVALID_FEATURE,)
        );

        let input = "あいうえお";
        let mut chars = input.char_indices();
        seek(&mut chars, 0);
        assert_eq!(
            super::get_unicode_block_and_feature(input, &mut chars),
            ("あ", 3, "108",)
        );

        let input = "わをん";
        let mut chars = input.char_indices();
        seek(&mut chars, 2);
        assert_eq!(
            super::get_unicode_block_and_feature(input, &mut chars),
            ("ん", 3, "108",)
        );

        let input = "安";
        let mut chars = input.char_indices();
        seek(&mut chars, 0);
        assert_eq!(
            super::get_unicode_block_and_feature(input, &mut chars),
            ("安", 3, "120",)
        );

        let input = "範囲外アクセス";
        let mut chars = input.char_indices();
        seek(&mut chars, 7);
        assert_eq!(
            super::get_unicode_block_and_feature(input, &mut chars),
            ("", 0, super::INVALID_FEATURE,)
        );
    }

    #[test]
    fn test_key() {
        let mut wb = String::with_capacity(20);

        assert_eq!(super::key(&mut wb, &[""]), "");
        assert_eq!(super::key(&mut wb, &["AAA", "BBB"]), "AAABBB");
        assert_eq!(super::key(&mut wb, &["AAA", "BBB", "CCC"]), "AAABBBCCC");
        assert_eq!(
            super::key(&mut wb, &["TW4:", "日", "本", "語"]),
            "TW4:日本語"
        );
        assert_eq!(
            super::key(&mut wb, &["TQ4:", "O", "120", "120", "120"]),
            "TQ4:O120120120"
        );

        assert_eq!(wb.capacity(), 20);
    }
}
