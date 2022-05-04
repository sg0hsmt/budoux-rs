#[path = "ja_knbc.rs"]
mod ja_knbc;

#[path = "zh_hans.rs"]
mod zh_hans;

/// default_japanese_model returns trained machine learning model for japanese.
pub fn default_japanese_model() -> &'static crate::Model {
    &ja_knbc::MODEL
}

/// default_simplified_chinese_model returns trained machine learning model for simplified chinese.
pub fn default_simplified_chinese_model() -> &'static crate::Model {
    &zh_hans::MODEL
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_multiple_ref() {
        let m1 = super::default_japanese_model();
        let m2 = super::default_japanese_model();

        assert_eq!(m1, m2);
        assert!(m1.len() > 0);
        assert!(m2.len() > 0);
    }

    #[test]
    fn test_multiple_ref_zh_hans() {
        let m1 = super::default_simplified_chinese_model();
        let m2 = super::default_simplified_chinese_model();

        assert_eq!(m1, m2);
        assert!(m1.len() > 0);
        assert!(m2.len() > 0);
    }
}
