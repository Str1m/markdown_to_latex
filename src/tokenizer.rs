use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Header(String, u8),
    Bold(String),
    Italic(String),
    Link(String, String), //(text, url)
    ListItem(String, bool),
    Text(String),
    Formula(String, bool),
    Newline,
}


//TODO: Формулы в LATEX
//TODO: * -- список
// Убрать двойные пробелы и - -> ~--

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
                '*' => tokens.push(self.tokenize_bold_or_italic()),
                '[' => tokens.push(self.tokenize_link()),
                '1'..='9' if self.is_numbered_list() => tokens.push(self.tokenize_list_item(true)),
                '-' => tokens.push(self.tokenize_list_item(false)),
                '\n' => {
                    tokens.push(Token::Newline);
                    self.advance();
                },
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
        self.advance(); // Skip initial '*'
        let is_bold = if self.current == Some('*'){
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
        self.input.clone().take_while(|ch| ch.is_digit(10)).any(|ch| ch == '.')
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
        // let text = text.replace("  ", " ");
        // let text = text.replace(" --", "~--");
        // let text = text.replace(" -", "~--");
        text
    }

    fn skip_whitespace(&mut self){
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
        assert_eq!(tokens, vec![Token::ListItem("List item".to_string(), false)]);
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
        assert_eq!(tokens, vec![Token::Text("Just some plain text.".to_string())]);
    }

    #[test]
    fn test_combined() {
        let input = "# Header 1\nSome *italic* and **bold** text.\n- List item 1\n1. Numbered list item\n[Link](http://example.com)\nAnother paragraph with a [different link](http://example2.com) and some **bold** text.\n## Subheader\n*italic* text and a - list item.\n3. Another numbered item";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Header("Header 1".to_string(), 1),
                Token::Newline,
                Token::Text("Some ".to_string()),
                Token::Italic("italic".to_string()),
                Token::Text(" and ".to_string()),
                Token::Bold("bold".to_string()),
                Token::Text(" text.".to_string()),
                Token::Newline,
                Token::ListItem("List item 1".to_string(), false),
                Token::Newline,
                Token::ListItem("Numbered list item".to_string(), true),
                Token::Newline,
                Token::Link("Link".to_string(), "http://example.com".to_string()),
                Token::Newline,
                Token::Text("Another paragraph with a ".to_string()),
                Token::Link("different link".to_string(), "http://example2.com".to_string()),
                Token::Text(" and some ".to_string()),
                Token::Bold("bold".to_string()),
                Token::Text(" text.".to_string()),
                Token::Newline,
                Token::Header("Subheader".to_string(), 2),
                Token::Newline,
                Token::Italic("italic".to_string()),
                Token::Text(" text and a ".to_string()),
                Token::ListItem("list item.".to_string(), false),
                Token::Newline,
                Token::ListItem("Another numbered item".to_string(), true)
            ]
        );
    }
}