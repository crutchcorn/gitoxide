mod read {
    use git_config::parse::parse_config;

    #[test]
    fn read_test_file() -> crate::Result {
        const CONFIG: &str = include_str!("../assets/config");

        assert_eq!(parse_config(CONFIG.to_string()), vec![]);
        Ok(())
    }
}