#[path = "ja_knbc.rs"]
mod ja_knbc;

pub fn default_japanese_model() -> &'static crate::Model {
    &ja_knbc::MODEL
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
}
