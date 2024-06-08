use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Header(String, u8),
    Bold(String),
    Italic(String),
    Link(String, String), //(text, url)
    ListItem(String, bool),
    Text(String),
    Newline,
}

pub struct Tokenizer<'a> {
    input: Chars<'a>,
    current: Option<char>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut tokenizer = Tokenizer {
            input: input.chars(),
            current: None,
        };
        tokenizer.advance();
        tokenizer
    }

    fn advance(&mut self) {
        self.current = self.input.next();
    }

    fn take_while<F>(&mut self, condition: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while let Some(ch) = self.current {
            if condition(ch) {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        result
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(ch) = self.current {
            match ch {
                '#' => tokens.push(self.tokenize_header()),
                '*' => {
                    if self.is_list_item() {
                        tokens.push(self.tokenize_list_item(false));
                    } else {
                        tokens.push(self.tokenize_bold_or_italic())
                    }
                }
                '[' => tokens.push(self.tokenize_link()),
                '1'..='9' => {
                    if self.is_numbered_list() {
                        self.advance();
                        self.advance();
                        tokens.push(self.tokenize_list_item(true));
                    } else {
                        tokens.push(self.tokenize_text())
                    }
                }
                '-' => tokens.push(self.tokenize_list_item(false)),
                '\n' => {
                    tokens.push(Token::Newline);
                    self.advance();
                }
                _ => tokens.push(self.tokenize_text()),
            }
        }
        tokens
    }

    fn tokenize_header(&mut self) -> Token {
        let level = self.take_while(|ch| ch == '#').len() as u8;
        self.skip_whitespace();
        let text = self.take_while(|ch| ch != '\n');
        Token::Header(text.trim().to_string(), level)
    }

    fn tokenize_bold_or_italic(&mut self) -> Token {
        self.advance();
        let is_bold = if self.current == Some('*') {
            self.advance();
            true
        } else {
            false
        };

        let text = self.take_while(|ch| ch != '*');
        self.advance();
        if is_bold {
            self.advance();
        }

        if is_bold {
            Token::Bold(self.clean_text(text))
        } else {
            Token::Italic(self.clean_text(text))
        }
    }

    fn tokenize_link(&mut self) -> Token {
        self.advance();
        let text = self.take_while(|ch| ch != ']');
        self.advance();
        self.advance();
        let url = self.take_while(|ch| ch != ')');
        self.advance();
        Token::Link(self.clean_text(text), url)
    }

    fn is_numbered_list(&self) -> bool {
        if self.current.map(|ch| ch.is_digit(10)).unwrap_or(false) {
            let mut lookahead = self.input.clone();
            if let Some('.') = lookahead.next() {
                if let Some(ch) = lookahead.next() {
                    return ch.is_whitespace();
                }
            }
        }
        false
    }

    fn tokenize_list_item(&mut self, is_numbered: bool) -> Token {
        if is_numbered {
            self.take_while(|ch| ch.is_digit(10));
        }
        self.advance();
        self.skip_whitespace();
        let text = self.take_while(|ch| ch != '\n');
        Token::ListItem(self.clean_text(text), is_numbered)
    }

    fn tokenize_text(&mut self) -> Token {
        let text = self.take_while(|ch| !matches!(ch, '#' | '*' | '[' | '\n'));
        Token::Text(self.clean_text(text))
    }

    fn clean_text(&self, text: String) -> String {
        let text = text.replace("  ", " ");
        let text = text.replace(" -", "~--");
        text
    }

    fn is_list_item(&self) -> bool {
        self.input.clone().next() == Some(' ')
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header() {
        let input = "# Header 1";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();
        assert_eq!(tokens, vec![Token::Header("Header 1".to_string(), 1)]);
    }

    #[test]
    fn test_bold() {
        let input = "This is **bold** text.";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Text("This is ".to_string()),
                Token::Bold("bold".to_string()),
                Token::Text(" text.".to_string())
            ]
        );
    }

    #[test]
    fn test_italic() {
        let input = "This is *italic* text.";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Text("This is ".to_string()),
                Token::Italic("italic".to_string()),
                Token::Text(" text.".to_string())
            ]
        );
    }

    #[test]
    fn test_link() {
        let input = "This is a [link](http://example.com).";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Text("This is a ".to_string()),
                Token::Link("link".to_string(), "http://example.com".to_string()),
                Token::Text(".".to_string())
            ]
        );
    }

    #[test]
    fn test_unordered_list_item() {
        let input = "- List item";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![Token::ListItem("List item".to_string(), false)]
        );
    }

    #[test]
    fn test_ordered_list_item() {
        let input = "1. List item";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();
        assert_eq!(tokens, vec![Token::ListItem("List item".to_string(), true)]);
    }

    #[test]
    fn test_text() {
        let input = "Just some plain text.";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![Token::Text("Just some plain text.".to_string())]
        );
    }
}
