use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// LSP text position (zero-based line and character offsets).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

/// A range of text identified by start and end positions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

/// A diagnostic message from the language server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub range: Range,
    pub message: String,
    pub severity: DiagnosticSeverity,
}

/// Severity level for a diagnostic.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

/// Manages diagnostics and completion suggestions from a language server.
pub struct LspClient {
    /// URI of the currently open file.
    pub document_uri: Option<PathBuf>,
    /// Active diagnostics for the current file.
    pub diagnostics: Vec<Diagnostic>,
}

impl LspClient {
    pub fn new() -> Self {
        LspClient {
            document_uri: None,
            diagnostics: Vec::new(),
        }
    }

    /// Open a document and clear previous diagnostics.
    pub fn open_document(&mut self, path: impl Into<PathBuf>) {
        self.document_uri = Some(path.into());
        self.diagnostics.clear();
    }

    /// Update the diagnostics list (typically received from the language server).
    pub fn update_diagnostics(&mut self, diagnostics: Vec<Diagnostic>) {
        self.diagnostics = diagnostics;
    }

    /// Return diagnostics that overlap the given line.
    pub fn diagnostics_at_line(&self, line: u32) -> Vec<&Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.range.start.line <= line && line <= d.range.end.line)
            .collect()
    }

    /// Return the count of errors in the current diagnostics.
    pub fn error_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == DiagnosticSeverity::Error)
            .count()
    }
}

impl Default for LspClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_diagnostic(line: u32, severity: DiagnosticSeverity, msg: &str) -> Diagnostic {
        Diagnostic {
            range: Range {
                start: Position { line, character: 0 },
                end: Position { line, character: 10 },
            },
            message: msg.into(),
            severity,
        }
    }

    #[test]
    fn test_open_document_clears_diagnostics() {
        let mut lsp = LspClient::new();
        lsp.diagnostics.push(make_diagnostic(0, DiagnosticSeverity::Error, "old error"));
        lsp.open_document("/src/main.rs");
        assert!(lsp.diagnostics.is_empty());
    }

    #[test]
    fn test_diagnostics_at_line() {
        let mut lsp = LspClient::new();
        lsp.open_document("/src/main.rs");
        lsp.update_diagnostics(vec![
            make_diagnostic(5, DiagnosticSeverity::Error, "error on line 5"),
            make_diagnostic(10, DiagnosticSeverity::Warning, "warning on line 10"),
        ]);
        let diags = lsp.diagnostics_at_line(5);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].message, "error on line 5");
    }

    #[test]
    fn test_error_count() {
        let mut lsp = LspClient::new();
        lsp.update_diagnostics(vec![
            make_diagnostic(1, DiagnosticSeverity::Error, "e1"),
            make_diagnostic(2, DiagnosticSeverity::Error, "e2"),
            make_diagnostic(3, DiagnosticSeverity::Warning, "w1"),
        ]);
        assert_eq!(lsp.error_count(), 2);
    }
}
