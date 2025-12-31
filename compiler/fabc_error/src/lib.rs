use anstyle::{AnsiColor, Color, Style};

#[derive(Debug, Clone)]
pub enum ErrorKind {
    ExpectedType { expected: String, found: String },
}

impl ErrorKind {
    pub fn name(&self) -> String {
        match self {
            ErrorKind::ExpectedType { .. } => "Type mismatch".to_string(),
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
        let (start_line, end_line) = (self.span.start().line(), self.span.end().line());
        let lines = source
            .lines()
            .enumerate()
            .filter(|(i, _)| (start_line..=end_line).contains(&(i + 1)))
            .map(|(_, str)| str)
            .collect::<Vec<&str>>();

        let caption = format!(
            "{}Error: {}{}",
            Self::ERROR_COLOR,
            self.kind.name(),
            anstyle::Reset
        );
        let formatted_lines = if lines.len() == 1 {
            format!(
                "{}\n{} {}",
                Self::format_line_header(None),
                Self::format_line(
                    start_line,
                    lines[0],
                    self.span.start().col(),
                    self.span.end().col()
                ),
                Self::format_annotation(&self.kind.message())
            )
        } else {
            let first_line = Self::format_line(
                start_line,
                lines[0],
                self.span.start().col(),
                lines[0].len() + 1,
            );
            let last_line_leading_whitespace_len = lines[lines.len() - 1]
                .chars()
                .take_while(|c| c.is_whitespace())
                .count();
            let last_line = Self::format_line(
                end_line,
                lines[lines.len() - 1],
                last_line_leading_whitespace_len + 1,
                self.span.end().col(),
            );

            format!(
                "{} \n{} \n{} {}",
                first_line,
                Self::format_line_continuation(),
                last_line,
                Self::format_annotation(&self.kind.message())
            )
        };

        format!("{}\n{}", caption, formatted_lines)
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
