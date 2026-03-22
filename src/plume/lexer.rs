//! Lexer for MTSS (MachTUI Style Sheets).

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Ident(String),
    OpenBrace,
    CloseBrace,
    Colon,
    Semicolon,
    Dot,
    Hash,
    Whitespace,
    Unknown(char),
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let chars = self.input[self.pos..].chars();
        let mut chars_peek = chars.clone();
        let c = chars_peek.next()?;
        let char_len = c.len_utf8();

        let token = match c {
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            ':' => Token::Colon,
            ';' => Token::Semicolon,
            '.' => Token::Dot,
            '#' => Token::Hash,
            c if c.is_whitespace() => {
                let mut end = self.pos + char_len;
                while let Some(next_c) = chars_peek.next() {
                    if next_c.is_whitespace() {
                        end += next_c.len_utf8();
                    } else {
                        break;
                    }
                }
                self.pos = end;
                return Some(Token::Whitespace);
            }
            c if c.is_alphanumeric() || c == '-' || c == '_' => {
                let mut end = self.pos + char_len;
                while let Some(next_c) = chars_peek.next() {
                    if next_c.is_alphanumeric() || next_c == '-' || next_c == '_' {
                        end += next_c.len_utf8();
                    } else {
                        break;
                    }
                }
                let ident = self.input[self.pos..end].to_string();
                self.pos = end;
                return Some(Token::Ident(ident));
            }
            _ => Token::Unknown(c),
        };

        self.pos += char_len;
        Some(token)
    }

    pub fn tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            if token != Token::Whitespace {
                tokens.push(token);
            }
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("button { color: red; }");
        let tokens = lexer.tokens();
        assert_eq!(tokens, vec![
            Token::Ident("button".to_string()),
            Token::OpenBrace,
            Token::Ident("color".to_string()),
            Token::Colon,
            Token::Ident("red".to_string()),
            Token::Semicolon,
            Token::CloseBrace,
        ]);
    }
}
