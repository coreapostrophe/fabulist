use anstyle::{AnsiColor, Color, Style};

#[derive(Debug, Clone)]
pub enum ErrorKind {
    ExpectedType { expected: String, found: String },
}

impl ErrorKind {
    pub fn name(&self) -> &'static str {
        match self {
            ErrorKind::ExpectedType { .. } => "Type mismatch",
        }
    }
    pub fn message(&self) -> String {
        match self {
            ErrorKind::ExpectedType { expected, found } => {
                format!("Expected type '{}', found '{}'", expected, found)
            }
        }
    }
}

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
}

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

pub struct Error {
    pub kind: ErrorKind,
    pub span: Span,
}

impl Error {
    const LINE_OFFSET: usize = 5;
    const ERROR_COLOR: Style = Style::new()
        .fg_color(Some(Color::Ansi(AnsiColor::Red)))
        .bold();
    const LINE_NUMBER_COLOR: Style = Style::new()
        .fg_color(Some(Color::Ansi(AnsiColor::Blue)))
        .bold();
    const INFO_COLOR: Style = Style::new()
        .fg_color(Some(Color::Ansi(AnsiColor::Blue)))
        .bold();

    pub fn new(kind: ErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
    pub fn format(&self, source: &str) -> String {
        let start_line = self.span.start().line();
        let end_line = self.span.end().line();
        let (first_line, last_line) = Self::span_lines(source, start_line, end_line);
        let message = self.kind.message();

        let formatted_lines = if start_line == end_line {
            format!(
                "{} {}",
                Self::format_line(
                    start_line,
                    first_line,
                    self.span.start().col(),
                    self.span.end().col()
                ),
                Self::format_annotation(&message)
            )
        } else {
            let first_line = Self::format_line(
                start_line,
                first_line,
                self.span.start().col(),
                first_line.len() + 1,
            );
            let last_line_leading_whitespace_len =
                last_line.chars().take_while(|c| c.is_whitespace()).count();
            let last_line = Self::format_line(
                end_line,
                last_line,
                last_line_leading_whitespace_len + 1,
                self.span.end().col(),
            );

            format!(
                "{} \n{} \n{} {}",
                first_line,
                Self::format_line_continuation(),
                last_line,
                Self::format_annotation(&message)
            )
        };

        format!(
            "{}Error: {}{}\n{}",
            Self::ERROR_COLOR,
            self.kind.name(),
            anstyle::Reset,
            formatted_lines
        )
    }
    fn format_annotation(content: &str) -> String {
        format!("{}{}{}", Self::INFO_COLOR, content, anstyle::Reset)
    }
    fn format_line(
        line_number: usize,
        content: &str,
        annotation_start: usize,
        annotation_end: usize,
    ) -> String {
        let annotation_length = annotation_end - annotation_start;
        let anotation_offset = annotation_start - 1 + annotation_length;
        format!(
            "{}\n{}{}\n{}{}{:>offset$}{}",
            Self::format_line_header(None),
            Self::format_line_header(Some(line_number)),
            content,
            Self::format_line_header(None),
            Self::INFO_COLOR,
            "^".repeat(annotation_length),
            anstyle::Reset,
            offset = anotation_offset
        )
    }
    fn format_line_header(line_number: Option<usize>) -> String {
        format!(
            "{}{:>offset$} |{}",
            Self::LINE_NUMBER_COLOR,
            line_number
                .map(|num| num.to_string())
                .unwrap_or_else(|| " ".to_string()),
            anstyle::Reset,
            offset = Self::LINE_OFFSET
        )
    }
    fn format_line_continuation() -> String {
        format!(
            "{}{:>offset$}{}",
            Self::LINE_NUMBER_COLOR,
            "...",
            anstyle::Reset,
            offset = Self::LINE_OFFSET + 2
        )
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

    #[test]
    fn formats_single_line_error() {
        let source = r#"
            fn main() {
                let x: i32 = "hello";
            }
        "#;
        let kind = ErrorKind::ExpectedType {
            expected: "i32".to_string(),
            found: "string".to_string(),
        };
        let span = Span::new(LineCol::new(3, 30), LineCol::new(3, 37));
        let error = Error::new(kind, span);
        let formatted = error.format(source);

        assert_eq!(formatted, "\u{1b}[1m\u{1b}[31mError: Type mismatch\u{1b}[0m\n\u{1b}[1m\u{1b}[34m      |\u{1b}[0m\n\u{1b}[1m\u{1b}[34m    3 |\u{1b}[0m                let x: i32 = \"hello\";\n\u{1b}[1m\u{1b}[34m      |\u{1b}[0m\u{1b}[1m\u{1b}[34m                             ^^^^^^^\u{1b}[0m \u{1b}[1m\u{1b}[34mExpected type 'i32', found 'string'\u{1b}[0m")
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
        let kind = ErrorKind::ExpectedType {
            expected: "i32".to_string(),
            found: "string".to_string(),
        };
        let span = Span::new(LineCol::new(3, 30), LineCol::new(6, 19));
        let error = Error::new(kind, span);
        let formatted = error.format(source);

        assert_eq!(formatted, "\u{1b}[1m\u{1b}[31mError: Type mismatch\u{1b}[0m\n\u{1b}[1m\u{1b}[34m      |\u{1b}[0m\n\u{1b}[1m\u{1b}[34m    3 |\u{1b}[0m                let x: i32 = some_function(\n\u{1b}[1m\u{1b}[34m      |\u{1b}[0m\u{1b}[1m\u{1b}[34m                             ^^^^^^^^^^^^^^\u{1b}[0m \n\u{1b}[1m\u{1b}[34m    ...\u{1b}[0m \n\u{1b}[1m\u{1b}[34m      |\u{1b}[0m\n\u{1b}[1m\u{1b}[34m    6 |\u{1b}[0m                );\n\u{1b}[1m\u{1b}[34m      |\u{1b}[0m\u{1b}[1m\u{1b}[34m                ^^\u{1b}[0m \u{1b}[1m\u{1b}[34mExpected type 'i32', found 'string'\u{1b}[0m")
    }
}
