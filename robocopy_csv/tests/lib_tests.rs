#![allow(non_snake_case)]

#[cfg(test)]
mod lib_tests {
    use rstest::rstest;
    use std::str::FromStr;
    // use std::path::Path;
    use robocopy_csv::*;

    #[test]
    fn test_footer_detail_0101_N() {
        let result = FooterDetail::from_str("7.99 m         0    7.99 m         0         0    9.45 m").unwrap();
        assert_eq!(8378122, result.total);
        assert_eq!(0, result.copied);
        assert_eq!(8378122, result.skipped);
        assert_eq!(0, result.mismatch);
        assert_eq!(0, result.failed);
        assert_eq!(9909043, result.extras);
    }

    #[rstest]
    #[case(r"k:v", r"k", r"v")]
    #[case(r"   コピー元 : C:\_MIRROR\", r"コピー元", r"C:\_MIRROR\")]
    #[case(r"   ROBOCOPY     ::     Windows の堅牢性の高いファイル コピー                              ", r"ROBOCOPY", r":     Windows の堅牢性の高いファイル コピー")]
    #[case(r"  オプション: *.* /BYTES /S /E /DCOPY:DA /COPY:DAT /PURGE /MIR /NP /R:1 /W:3 ", r"オプション", r"*.* /BYTES /S /E /DCOPY:DA /COPY:DAT /PURGE /MIR /NP /R:1 /W:3")]
    #[case(r"k:", r"k", r"")]
    #[case(r":", r"", r"")]
    fn test_kv_split_0001_N(#[case] input: &str, #[case] key_expected: &str, #[case] value_expected: &str) {
        let result = kv_split(&input).unwrap();
        assert_eq!(key_expected, result.0);
        assert_eq!(value_expected, result.1);
    }

    #[rstest]
    #[case(r"a")]
    #[case(r"  ")]
    #[case(r"")]
    fn test_kv_split_0002_A(#[case] input: &str) {
        let result = kv_split(&input);
        assert!(result.is_none());
    }
}
