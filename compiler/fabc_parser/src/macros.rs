#[macro_export]
macro_rules! expect_token {
    ($parser:expr, $variant:path, $expected:expr) => {{
        if let $variant(value) = $parser.advance() {
            Ok(value.to_string())
        } else {
            Err($crate::error::Error::ExpectedFound {
                expected: $expected.to_string(),
                found: $parser.previous().to_string(),
            })
        }
    }};
}
