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
    let chars: Vec<char> = input.chars().collect();

    if chars.len() <= 3 {
        return vec![input.to_string()];
    }

    let mut out: Vec<String> = Vec::new();
    let mut buf: String = chars[0].to_string() + &chars[1].to_string() + &chars[2].to_string();

    let mut p1 = "U"; // unknown
    let mut p2 = "U"; // unknown
    let mut p3 = "U"; // unknown

    for i in 3..chars.len() {
        let (w1, b1) = get_unicode_block_and_feature(&chars, i - 3);
        let (w2, b2) = get_unicode_block_and_feature(&chars, i - 2);
        let (w3, b3) = get_unicode_block_and_feature(&chars, i - 1);
        let (w4, b4) = get_unicode_block_and_feature(&chars, i);
        let (w5, b5) = get_unicode_block_and_feature(&chars, i + 1);
        let (w6, b6) = get_unicode_block_and_feature(&chars, i + 2);

        let score: i32 = get_feature(
            &w1, &w2, &w3, &w4, &w5, &w6, &b1, &b2, &b3, &b4, &b5, &b6, p1, p2, p3,
        )
        .into_iter()
        .map(|x| model.get(&x).unwrap_or(&0))
        .sum();

        if score > threshold {
            out.push(buf);
            buf = w4;
        } else {
            buf += &w4;
        }

        p1 = p2;
        p2 = p3;

        if score > 0 {
            p3 = "B"; // positive
        } else {
            p3 = "O"; // negative
        }
    }

    if !buf.is_empty() {
        out.push(buf);
    }

    out
}

/// get_unicode_block_and_feature returns unicode character and block feature from char slice.
fn get_unicode_block_and_feature(chars: &[char], index: usize) -> (String, String) {
    if chars.len() <= index {
        return (String::from(""), String::from("999")); // out of index.
    }

    let v = chars[index];
    let c = v as u32;

    let pos = match unicode_blocks::UNICODE_BLOCKS.binary_search(&c) {
        Ok(v) => v + 1,
        Err(e) => e,
    };

    return (v.to_string(), format!("{:>03}", pos));
}

/// get_feature returns feature list.
#[allow(clippy::too_many_arguments)]
fn get_feature(
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
) -> Vec<String> {
    return vec![
        // UP is means unigram of previous results.
        format!("UP1:{}", p1),
        format!("UP2:{}", p2),
        format!("UP3:{}", p3),
        // BP is means bigram of previous results.
        format!("BP1:{}{}", p1, p2),
        format!("BP2:{}{}", p2, p3),
        // UW is means unigram of words.
        format!("UW1:{}", w1),
        format!("UW2:{}", w2),
        format!("UW3:{}", w3),
        format!("UW4:{}", w4),
        format!("UW5:{}", w5),
        format!("UW6:{}", w6),
        // BW is means bigram of words.
        format!("BW1:{}{}", w2, w3),
        format!("BW2:{}{}", w3, w4),
        format!("BW3:{}{}", w4, w5),
        // TW is means trigram of words.
        format!("TW1:{}{}{}", w1, w2, w3),
        format!("TW2:{}{}{}", w2, w3, w4),
        format!("TW3:{}{}{}", w3, w4, w5),
        format!("TW4:{}{}{}", w4, w5, w6),
        // UB is means unigram of unicode blocks.
        format!("UB1:{}", b1),
        format!("UB2:{}", b2),
        format!("UB3:{}", b3),
        format!("UB4:{}", b4),
        format!("UB5:{}", b5),
        format!("UB6:{}", b6),
        // BB is means bigram of unicode blocks.
        format!("BB1:{}{}", b2, b3),
        format!("BB2:{}{}", b3, b4),
        format!("BB3:{}{}", b4, b5),
        // TB is means trigram of unicode blocks.
        format!("TB1:{}{}{}", b1, b2, b3),
        format!("TB2:{}{}{}", b2, b3, b4),
        format!("TB3:{}{}{}", b3, b4, b5),
        format!("TB4:{}{}{}", b4, b5, b6),
        // UQ is combination of UP and UB.
        format!("UQ1:{}{}", p1, b1),
        format!("UQ2:{}{}", p2, b2),
        format!("UQ3:{}{}", p3, b3),
        // BQ is combination of UP and BB.
        format!("BQ1:{}{}{}", p2, b2, b3),
        format!("BQ2:{}{}{}", p2, b3, b4),
        format!("BQ3:{}{}{}", p3, b2, b3),
        format!("BQ4:{}{}{}", p3, b3, b4),
        // TQ is combination of UP and TB.
        format!("TQ1:{}{}{}{}", p2, b1, b2, b3),
        format!("TQ2:{}{}{}{}", p2, b2, b3, b4),
        format!("TQ3:{}{}{}{}", p3, b1, b2, b3),
        format!("TQ4:{}{}{}{}", p3, b2, b3, b4),
    ];
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse() {
        let m = super::models::default_japanese_model();

        assert_eq!(super::parse(m, ""), vec![""]);
        assert_eq!(super::parse(m, "日本語"), vec!["日本語"]);
        assert_eq!(super::parse(m, "水と油"), vec!["水と油"]);
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
    fn test_get_unicode_block_and_feature() {
        let to_chars = |x: &str| {
            let chars: Vec<char> = x.chars().collect();
            return chars;
        };

        assert_eq!(
            super::get_unicode_block_and_feature(&to_chars("abc"), 0),
            (String::from("a"), String::from("001"),)
        );
        assert_eq!(
            super::get_unicode_block_and_feature(&to_chars("xyz"), 2),
            (String::from("z"), String::from("001"),)
        );
        assert_eq!(
            super::get_unicode_block_and_feature(&to_chars("abc"), 0),
            (String::from("a"), String::from("001"),)
        );
        assert_eq!(
            super::get_unicode_block_and_feature(&to_chars("out of index"), 12),
            (String::from(""), String::from("999"),)
        );
        assert_eq!(
            super::get_unicode_block_and_feature(&to_chars("あいうえお"), 0),
            (String::from("あ"), String::from("108"),)
        );
        assert_eq!(
            super::get_unicode_block_and_feature(&to_chars("わをん"), 2),
            (String::from("ん"), String::from("108"),)
        );
        assert_eq!(
            super::get_unicode_block_and_feature(&to_chars("安"), 0),
            (String::from("安"), String::from("120"),)
        );
        assert_eq!(
            super::get_unicode_block_and_feature(&to_chars("範囲外アクセス"), 7),
            (String::from(""), String::from("999"),)
        );
    }
}
