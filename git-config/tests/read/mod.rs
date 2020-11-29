use git_config::Parse;

mod http {
    use crate::parse::{assert_url_roundtrip, url};
    use git_url::Scheme;
    use std::io;
    use git_config::parse::ParsedConfig;

    #[test]
    fn read_test_file() -> crate::Result {
        const CONFIG: &str = include_str!("./tests/assets/config");

        assert_eq!(ParsedConfig::from(CONFIG.to_string()), vec![]);
        Ok(())
    }
}