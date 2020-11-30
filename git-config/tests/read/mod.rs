mod read {
    use git_config::parse::parse_config;
    use git_config::ParsedConfig;

    #[test]
    fn read_test_file() -> crate::Result {
        const CONFIG: &str = include_str!("../assets/config");

        assert_eq!(parse_config(CONFIG.to_string()), vec![
            ParsedConfig { is_section: false, section: "".to_string(), subsection: "".to_string(), name: "".to_string(), value: "".to_string(), path: "".to_string() },
            ParsedConfig { is_section: false, section: "".to_string(), subsection: "".to_string(), name: "repositoryformatversion".to_string(), value: "0".to_string(), path: "repositoryformatversion".to_string() },
            ParsedConfig { is_section: false, section: "".to_string(), subsection: "".to_string(), name: "filemode".to_string(), value: "false".to_string(), path: "filemode".to_string() },
            ParsedConfig { is_section: false, section: "".to_string(), subsection: "".to_string(), name: "bare".to_string(), value: "false".to_string(), path: "bare".to_string() },
            ParsedConfig { is_section: false, section: "".to_string(), subsection: "".to_string(), name: "logallrefupdates".to_string(), value: "true".to_string(), path: "logallrefupdates".to_string() },
            ParsedConfig { is_section: false, section: "".to_string(), subsection: "".to_string(), name: "symlinks".to_string(), value: "false".to_string(), path: "symlinks".to_string() },
            ParsedConfig { is_section: false, section: "".to_string(), subsection: "".to_string(), name: "ignorecase".to_string(), value: "true".to_string(), path: "ignorecase".to_string() },
            ParsedConfig { is_section: true, section: "remote".to_string(), subsection: "origin".to_string(), name: "".to_string(), value: "".to_string(), path: "remote.origin".to_string() },
            ParsedConfig { is_section: false, section: "remote".to_string(), subsection: "origin".to_string(), name: "url".to_string(), value: "https://github.com/Byron/gitoxide.git".to_string(), path: "remote.origin.url".to_string() },
            ParsedConfig { is_section: false, section: "remote".to_string(), subsection: "origin".to_string(), name: "fetch".to_string(), value: "+refs/heads/*:refs/remotes/origin/*".to_string(), path: "remote.origin.fetch".to_string() },
            ParsedConfig { is_section: true, section: "branch".to_string(), subsection: "main".to_string(), name: "".to_string(), value: "".to_string(), path: "branch.main".to_string() },
            ParsedConfig { is_section: false, section: "branch".to_string(), subsection: "main".to_string(), name: "remote".to_string(), value: "origin".to_string(), path: "branch.main.remote".to_string() },
            ParsedConfig { is_section: false, section: "branch".to_string(), subsection: "main".to_string(), name: "merge".to_string(), value: "refs/heads/main".to_string(), path: "branch.main.merge".to_string() },
            ParsedConfig { is_section: false, section: "branch".to_string(), subsection: "main".to_string(), name: "".to_string(), value: "".to_string(), path: "branch.main".to_string() }
        ]);
        Ok(())
    }
}