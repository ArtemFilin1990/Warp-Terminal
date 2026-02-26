/// Token types recognized by the syntax highlighter.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Keyword,
    String,
    Number,
    Comment,
    Operator,
    Identifier,
    Plain,
}

/// A highlighted span within a line.
#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
    pub kind: TokenKind,
}

/// A very simple line-based syntax highlighter for shell and Rust snippets.
pub struct Highlighter {
    keywords: Vec<&'static str>,
}

impl Highlighter {
    pub fn new() -> Self {
        Highlighter {
            keywords: vec![
                "let", "mut", "fn", "pub", "use", "mod", "struct", "enum", "impl",
                "if", "else", "for", "while", "loop", "match", "return", "in",
                "echo", "cd", "ls", "export", "source",
            ],
        }
    }

    /// Tokenize `line` into a sequence of highlighted tokens.
    pub fn tokenize(&self, line: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut chars = line.chars().peekable();

        while let Some(&ch) = chars.peek() {
            if ch == '#' {
                // Rest of line is a comment
                let comment: String = chars.collect();
                tokens.push(Token { text: comment, kind: TokenKind::Comment });
                break;
            } else if ch == '"' || ch == '\'' {
                // String literal
                let quote = ch;
                chars.next();
                let mut s = String::from(quote);
                while let Some(&c) = chars.peek() {
                    chars.next();
                    s.push(c);
                    if c == quote { break; }
                }
                tokens.push(Token { text: s, kind: TokenKind::String });
            } else if ch.is_ascii_digit() {
                // Number — consume digits and decimal points only via peek to avoid losing the delimiter
                let mut num = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        num.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token { text: num, kind: TokenKind::Number });
            } else if ch.is_alphabetic() || ch == '_' {
                // Keyword or identifier — consume via peek to preserve delimiter
                let mut word = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        word.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let kind = if self.keywords.contains(&word.as_str()) {
                    TokenKind::Keyword
                } else {
                    TokenKind::Identifier
                };
                tokens.push(Token { text: word, kind });
            } else if "+-*/<>=!&|".contains(ch) {
                chars.next();
                tokens.push(Token { text: ch.to_string(), kind: TokenKind::Operator });
            } else {
                chars.next();
                tokens.push(Token { text: ch.to_string(), kind: TokenKind::Plain });
            }
        }

        tokens
    }
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_highlighted() {
        let h = Highlighter::new();
        let tokens = h.tokenize("let x = 1");
        assert_eq!(tokens[0].kind, TokenKind::Keyword);
        assert_eq!(tokens[0].text, "let");
    }

    #[test]
    fn test_comment_highlighted() {
        let h = Highlighter::new();
        let tokens = h.tokenize("# this is a comment");
        assert_eq!(tokens[0].kind, TokenKind::Comment);
    }

    #[test]
    fn test_string_highlighted() {
        let h = Highlighter::new();
        let tokens = h.tokenize("\"hello\"");
        assert_eq!(tokens[0].kind, TokenKind::String);
    }

    #[test]
    fn test_number_highlighted() {
        let h = Highlighter::new();
        let tokens = h.tokenize("42");
        assert_eq!(tokens[0].kind, TokenKind::Number);
    }
}
