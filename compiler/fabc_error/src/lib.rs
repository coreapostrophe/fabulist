use anstyle::{Ansi256Color, AnsiColor, Color, Style};
use fabc_lexer::tokens::Token;
use std::fmt::Write;

use crate::kind::ErrorKind;

pub mod kind;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct LineCol(usize, usize);

impl LineCol {
    pub fn new(line: usize, col: usize) -> Self {
        Self(line, col)
    }
    pub fn line(&self) -> usize {
        self.0
    }
    pub fn col(&self) -> usize {
        self.1
    }
    pub fn from_token(token: &Token<'_>) -> Self {
        Self::new(token.line, token.column)
    }
    pub fn from_token_end(token: &Token<'_>) -> Self {
        Self::new(token.line, token.column + token.length - 1)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    start: LineCol,
    end: LineCol,
}

impl Span {
    pub fn new(start: LineCol, end: LineCol) -> Self {
        Self { start, end }
    }
    pub fn start(&self) -> &LineCol {
        &self.start
    }
    pub fn end(&self) -> &LineCol {
        &self.end
    }
}

impl From<(LineCol, LineCol)> for Span {
    fn from((start, end): (LineCol, LineCol)) -> Self {
        Self::new(start, end)
    }
}

impl From<&Token<'_>> for Span {
    fn from(token: &Token<'_>) -> Self {
        Self::new(LineCol::from_token(token), LineCol::from_token_end(token))
    }
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub span: Span,
}

impl Error {
    const LINE_OFFSET: usize = 5;
    const ERROR_COLOR: Style = Style::new()
        .fg_color(Some(Color::Ansi256(Ansi256Color(160))))
        .bold();
    const LINE_NUMBER_COLOR: Style = Style::new()
        .fg_color(Some(Color::Ansi(AnsiColor::BrightBlack)))
        .bold();
    const INFO_COLOR: Style = Style::new()
        .fg_color(Some(Color::Ansi(AnsiColor::BrightBlue)))
        .bold();
    const CODE_COLOR: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::White)));

    pub fn new(kind: impl Into<ErrorKind>, span: impl Into<Span>) -> Self {
        Self {
            kind: kind.into(),
            span: span.into(),
        }
    }

    pub fn format(&self, source: &str) -> String {
        let start_line = self.span.start().line();
        let end_line = self.span.end().line();
        let (first_line, last_line) = Self::span_lines(source, start_line, end_line);
        let message = self.kind.message();

        let mut formatted_error = String::new();

        let _ = writeln!(
            formatted_error,
            "{}error: {}{}",
            Self::ERROR_COLOR,
            self.kind.name(),
            anstyle::Reset
        );

        if start_line == end_line {
            Self::format_line(
                &mut formatted_error,
                start_line,
                first_line,
                self.span.start().col(),
                self.span.end().col(),
            );
            Self::format_annotation(&mut formatted_error, &message);
        } else {
            Self::format_line(
                &mut formatted_error,
                start_line,
                first_line,
                self.span.start().col(),
                first_line.len() + 1,
            );
            formatted_error.push('\n');

            Self::format_line_continuation(&mut formatted_error);

            let last_line_leading_whitespace_len =
                last_line.chars().take_while(|c| c.is_whitespace()).count();

            Self::format_line(
                &mut formatted_error,
                end_line,
                last_line,
                last_line_leading_whitespace_len + 1,
                self.span.end().col(),
            );

            Self::format_annotation(&mut formatted_error, &message);
        };

        formatted_error
    }
    fn format_annotation(string_buf: &mut String, content: &str) {
        let _ = write!(
            string_buf,
            " {}{}{}",
            Self::INFO_COLOR,
            content,
            anstyle::Reset
        );
    }
    fn format_line(
        string_buf: &mut String,
        line_number: usize,
        content: &str,
        annotation_start: usize,
        annotation_end: usize,
    ) {
        let annotation_length = annotation_end - annotation_start;
        let anotation_offset = annotation_start - 1 + annotation_length;

        Self::format_line_header(string_buf, None);
        string_buf.push('\n');

        Self::format_line_header(string_buf, Some(line_number));

        let _ = writeln!(
            string_buf,
            "{}{}{}",
            Self::CODE_COLOR,
            content,
            anstyle::Reset
        );

        Self::format_line_header(string_buf, None);

        let _ = write!(
            string_buf,
            "{}{:>offset$}{}",
            Self::INFO_COLOR,
            "^".repeat(annotation_length),
            anstyle::Reset,
            offset = anotation_offset
        );
    }
    fn format_line_header(string_buf: &mut String, line_number: Option<usize>) {
        let _ = write!(
            string_buf,
            "{}{:>offset$} |{}",
            Self::LINE_NUMBER_COLOR,
            line_number
                .map(|num| num.to_string())
                .unwrap_or_else(|| " ".to_string()),
            anstyle::Reset,
            offset = Self::LINE_OFFSET
        );
    }
    fn format_line_continuation(string_buf: &mut String) {
        let _ = writeln!(
            string_buf,
            "{}{:>offset$}{}",
            Self::LINE_NUMBER_COLOR,
            "...",
            anstyle::Reset,
            offset = Self::LINE_OFFSET + 2
        );
    }
    fn span_lines(source: &str, start_line: usize, end_line: usize) -> (&str, &str) {
        let mut first = "";
        let mut last = "";

        for (idx, line) in source.lines().enumerate() {
            let line_num = idx + 1;
            if line_num == start_line {
                first = line;
            }
            if line_num == end_line {
                last = line;
                break;
            }
            if line_num > end_line {
                break;
            }
        }

        (first, last)
    }
}

#[cfg(test)]
mod error_tests {
    use super::*;
    use crate::kind::CompileErrorKind;

    #[test]
    fn formats_single_line_error() {
        let source = r#"
            fn main() {
                let x: i32 = "hello";
            }
        "#;
        let kind = ErrorKind::Compile(CompileErrorKind::ExpectedSymbol {
            expected: "i32".to_string(),
            found: "string".to_string(),
        });
        let span = Span::new(LineCol::new(3, 30), LineCol::new(3, 37));
        let error = Error::new(kind, span);
        let formatted = error.format(source);

        assert_eq!(
            formatted,
            concat!(
                "\u{1b}[1m\u{1b}[38;5;160merror: Unexpected symbol\u{1b}[0m\n\u{1b}[1m\u{1b}[90m",
                "      |\u{1b}[0m\n\u{1b}[1m\u{1b}[90m    3 |\u{1b}[0m\u{1b}[37m            ",
                "    let x: i32 = \"hello\";\u{1b}[0m\n\u{1b}[1m\u{1b}[90m      ",
                "|\u{1b}[0m\u{1b}[1m\u{1b}[94m                             ",
                "^^^^^^^\u{1b}[0m \u{1b}[1m\u{1b}[94mExpected 'i32', found 'string'\u{1b}[0m"
            )
        )
    }

    #[test]
    fn formats_multi_line_error() {
        let source = r#"
            fn main() {
                let x: i32 = some_function(
                    "hello",
                    42,
                );
            }
        "#;
        let kind = ErrorKind::Compile(CompileErrorKind::ExpectedSymbol {
            expected: "i32".to_string(),
            found: "string".to_string(),
        });
        let span = Span::new(LineCol::new(3, 30), LineCol::new(6, 19));
        let error = Error::new(kind, span);
        let formatted = error.format(source);

        assert_eq!(
            formatted,
            concat!(
            "\u{1b}[1m\u{1b}[38;5;160merror: Unexpected symbol\u{1b}[0m\n\u{1b}[1m\u{1b}[90m",
            "      |\u{1b}[0m\n\u{1b}[1m\u{1b}[90m    3 |\u{1b}[0m\u{1b}[37m            ",
            "    let x: i32 = some_function(\u{1b}[0m\n\u{1b}[1m\u{1b}[90m      ",
            "|\u{1b}[0m\u{1b}[1m\u{1b}[94m                             ^^^^^^^^^^^^^^",
            "\u{1b}[0m\n\u{1b}[1m\u{1b}[90m    ...\u{1b}[0m\n\u{1b}[1m\u{1b}[90m      ",
            "|\u{1b}[0m\n\u{1b}[1m\u{1b}[90m    6 |\u{1b}[0m\u{1b}[37m                ",
            ");\u{1b}[0m\n\u{1b}[1m\u{1b}[90m      |\u{1b}[0m\u{1b}[1m\u{1b}[94m                ",
            "^^\u{1b}[0m \u{1b}[1m\u{1b}[94mExpected 'i32', found 'string'\u{1b}[0m")
        )
    }
}
