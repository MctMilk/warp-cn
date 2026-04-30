use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Severity {
    Soft,
    Hard,
}

#[derive(Clone, Debug)]
pub struct Violation {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub callsite: String,
    pub literal: String,
    pub severity: Severity,
    pub message: String,
}

impl Violation {
    pub fn format(&self) -> String {
        let tag = match self.severity {
            Severity::Soft => "warning",
            Severity::Hard => "error",
        };
        format!(
            "{}:{}:{}: {tag}: {} at {} — literal {:?}",
            self.file.display(),
            self.line,
            self.column,
            self.message,
            self.callsite,
            self.literal,
        )
    }
}

pub fn render(violations: &[Violation]) {
    for v in violations {
        eprintln!("{}", v.format());
    }
}
