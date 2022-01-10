use std::collections::HashMap;

mod unicode_blocks;

pub mod models;

/// DEFAULT_THRESHOLD default threshold for splitting a sentence.
pub const DEFAULT_THRESHOLD: i32 = 1000;

/// Model trained machine learning model.
/// key (String) is feature of character, value (i32) is score of feature.
pub type Model = HashMap<String, i32>;

/// parse returns splitted string slice from input.
/// it is shorthand for budoux::parse_with_threshold(model, input, budoux::DEFAULT_THRESHOLD).
pub fn parse(model: &Model, input: &str) -> Vec<String> {
    parse_with_threshold(model, input, DEFAULT_THRESHOLD)
}

/// parse_with_threshold returns splitted string slice from input.
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

/// get_unicode_block_and_feature returns unicode character and block feature from rune slice.
fn get_unicode_block_and_feature(chars: &[char], index: usize) -> (String, String) {
    if chars.len() <= index {
        return (String::from(""), String::from("999")); // out of index.
    }

    let v = chars[index];
    let c = v as u32;

    let pos = match unicode_blocks::UNICODE_BLOCKS.binary_search(&c) {
        Ok(v) => v,
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
        String::from("UP1:") + p1,
        String::from("UP2:") + p2,
        String::from("UP3:") + p3,
        // BP is means bigram of previous results.
        String::from("BP1:") + p1 + p2,
        String::from("BP2:") + p2 + p3,
        // UW is means unigram of words.
        String::from("UW1:") + w1,
        String::from("UW2:") + w2,
        String::from("UW3:") + w3,
        String::from("UW4:") + w4,
        String::from("UW5:") + w5,
        String::from("UW6:") + w6,
        // BW is means bigram of words.
        String::from("BW1:") + w2 + w3,
        String::from("BW2:") + w3 + w4,
        String::from("BW3:") + w4 + w5,
        // TW is means trigram of words.
        String::from("TW1:") + w1 + w2 + w3,
        String::from("TW2:") + w2 + w3 + w4,
        String::from("TW3:") + w3 + w4 + w5,
        String::from("TW4:") + w4 + w5 + w6,
        // UB is means unigram of unicode blocks.
        String::from("UB1:") + b1,
        String::from("UB2:") + b2,
        String::from("UB3:") + b3,
        String::from("UB4:") + b4,
        String::from("UB5:") + b5,
        String::from("UB6:") + b6,
        // BB is means bigram of unicode blocks.
        String::from("BB1:") + b2 + b3,
        String::from("BB2:") + b3 + b4,
        String::from("BB3:") + b4 + b5,
        // TB is means trigram of unicode blocks.
        String::from("TB1:") + b1 + b2 + b3,
        String::from("TB2:") + b2 + b3 + b4,
        String::from("TB3:") + b3 + b4 + b5,
        String::from("TB4:") + b4 + b5 + b6,
        // UQ is combination of UP and UB.
        String::from("UQ1:") + p1 + b1,
        String::from("UQ2:") + p2 + b2,
        String::from("UQ3:") + p3 + b3,
        // BQ is combination of UP and BB.
        String::from("BQ1:") + p2 + b2 + b3,
        String::from("BQ2:") + p2 + b3 + b4,
        String::from("BQ3:") + p3 + b2 + b3,
        String::from("BQ4:") + p3 + b3 + b4,
        // TQ is combination of UP and TB.
        String::from("TQ1:") + p2 + b1 + b2 + b3,
        String::from("TQ2:") + p2 + b2 + b3 + b4,
        String::from("TQ3:") + p3 + b1 + b2 + b3,
        String::from("TQ4:") + p3 + b2 + b3 + b4,
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
