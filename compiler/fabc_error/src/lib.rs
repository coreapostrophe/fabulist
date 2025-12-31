#[derive(Debug)]
pub enum ErrorKind {}

impl ErrorKind {
    pub fn to_template(&self) -> &str {
        todo!()
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
    pub fn new(kind: ErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}
