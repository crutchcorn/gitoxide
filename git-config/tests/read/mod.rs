use git_config::File;

mod http {
    use crate::parse::{assert_url_roundtrip, url};
    use git_url::Scheme;
    use std::io;

    #[test]
    fn read_test_file() -> crate::Result {
        const CONFIG: &[u8] = include_bytes!("./tests/assets/config");

        let test = git_config::File { buf: vec![], tokens: vec![] };
        let editMode = test.edit();
        editMode.to_write(io::Write);
        Ok(())
        // assert_url_roundtrip(
        //     "http://example.com/~byron/hello",
        //     url(Scheme::Http, None, "example.com", None, b"/~byron/hello"),
        // )
    }
}