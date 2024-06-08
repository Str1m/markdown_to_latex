use crate::tokenizer::Token;

pub struct LatexConverter {
    in_list: bool,
    list_type: Option<bool>,
}

impl LatexConverter {
    pub fn convert(tokens: Vec<Token>) -> String {
        let mut converter = LatexConverter {
            in_list: false,
            list_type: None,
        };
        let mut latex = String::new();
        for token in tokens {
            latex.push_str(&match token {
                Token::Header(text, level) => {
                    converter.close_list_if_needed() + &Self::convert_header(&text, level)
                }
                Token::Bold(text) => converter.close_list_if_needed() + &Self::convert_bold(&text),
                Token::Italic(text) => {
                    converter.close_list_if_needed() + &Self::convert_italic(&text)
                }
                Token::Link(text, url) => {
                    converter.close_list_if_needed() + &Self::convert_link(&text, &url)
                }
                Token::ListItem(text, is_numbered) => {
                    converter.convert_list_item(&text, is_numbered)
                }
                Token::Text(text) => converter.close_list_if_needed() + &text,
                Token::Newline => "\n".to_string(),
            });
        }
        latex + &converter.close_list_if_needed()
    }

    fn convert_header(text: &str, level: u8) -> String {
        match level {
            1 => format!("\\section{{{}}}\n", text),
            2 => format!("\\subsection{{{}}}\n", text),
            3 => format!("\\subsubsection{{{}}}\n", text),
            4 => format!("\\paragraph{{{}}}\n", text),
            5 => format!("\\subparagraph{{{}}}\n", text),
            _ => format!("\\textbf{{{}}}\n", text),
        }
    }

    fn convert_bold(text: &str) -> String {
        format!("\\textbf{{{}}}", text)
    }

    fn convert_italic(text: &str) -> String {
        format!("\\textit{{{}}}", text)
    }

    fn convert_link(text: &str, url: &str) -> String {
        format!("\\href{{{}}}{{{}}}", url, text)
    }

    fn convert_list_item(&mut self, text: &str, is_numbered: bool) -> String {
        if !self.in_list {
            self.in_list = true;
            self.list_type = Some(is_numbered);
            let env = if is_numbered { "enumerate" } else { "itemize" };
            format!("\\begin{{{}}}\n\\item {}", env, text)
        } else if self.list_type == Some(is_numbered) {
            format!("\\item {}", text)
        } else {
            let close = self.close_list_if_needed();
            self.in_list = true;
            self.list_type = Some(is_numbered);
            let env = if is_numbered { "enumerate" } else { "itemize" };
            format!("{}\\begin{{{}}}\n\\item {}", close, env, text)
        }
    }

    fn close_list_if_needed(&mut self) -> String {
        if self.in_list {
            self.in_list = false;
            let env = if self.list_type.unwrap_or(false) {
                "enumerate"
            } else {
                "itemize"
            };
            format!("\\end{{{}}}\n", env)
        } else {
            String::new()
        }
    }
}
