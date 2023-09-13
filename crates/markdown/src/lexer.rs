#[derive(Debug, PartialEq)]
pub enum MarkdownToken {
    Text(String),
    Heading1,  // #
    Heading2,  // ##
    Heading3,  // ###
    Heading4,  // ####
    Heading5,  // #####
    Heading6,  // ######
    Bold,      // **
    Italic,    // *
    Newline,   // \n
    EndOfFile, // 0
    Illegal,   // ?
}

pub struct Lexer {
    position: usize,
    read_position: usize,
    ch: u8,
    input: Vec<u8>,
    is_italic: bool,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
            is_italic: false,
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> MarkdownToken {
        let token = match self.ch {
            b'#' => {
                let mut heading_level = 1;

                while self.peek() == b'#' {
                    heading_level += 1;
                    self.read_char();
                }

                while self.peek() == b' ' {
                    self.read_char();
                }

                match heading_level {
                    1 => MarkdownToken::Heading1,
                    2 => MarkdownToken::Heading2,
                    3 => MarkdownToken::Heading3,
                    4 => MarkdownToken::Heading4,
                    5 => MarkdownToken::Heading5,
                    6 => MarkdownToken::Heading6,
                    _ => MarkdownToken::Illegal,
                }
            }
            b'*' => {
                if self.is_italic {
                    self.is_italic = false;
                    MarkdownToken::Italic
                } else if self.peek() == b'*' {
                    self.read_char();
                    MarkdownToken::Bold
                } else {
                    self.is_italic = true;
                    MarkdownToken::Italic
                }
            }
            b'\n' => MarkdownToken::Newline,
            0 => MarkdownToken::EndOfFile,
            _ => MarkdownToken::Text(self.read_text()),
        };

        self.read_char();
        return token;
    }

    fn peek(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return self.input[self.read_position];
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_text(&mut self) -> String {
        let mut text = String::new();
        text.push(self.ch as char);
        while self.is_peek_text() {
            self.read_char();
            text.push(self.ch as char);
        }
        text
    }

    fn is_peek_text(&mut self) -> bool {
        let peek = self.peek();
        if peek == b'#' || peek == b'*' || peek == b'\n' || peek == 0 {
            return false;
        } else {
            return true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headings() {
        let text = String::from("Hello World");
        let input = format!(
            "
# {text}
## {text}
### {text}
#### {text}
##### {text}
###### {text}
"
        );
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), MarkdownToken::Newline);
        assert_eq!(lexer.next_token(), MarkdownToken::Heading1);
        assert_eq!(lexer.next_token(), MarkdownToken::Text(text.clone()));
        assert_eq!(lexer.next_token(), MarkdownToken::Newline);
        assert_eq!(lexer.next_token(), MarkdownToken::Heading2);
        assert_eq!(lexer.next_token(), MarkdownToken::Text(text.clone()));
        assert_eq!(lexer.next_token(), MarkdownToken::Newline);
        assert_eq!(lexer.next_token(), MarkdownToken::Heading3);
        assert_eq!(lexer.next_token(), MarkdownToken::Text(text.clone()));
        assert_eq!(lexer.next_token(), MarkdownToken::Newline);
        assert_eq!(lexer.next_token(), MarkdownToken::Heading4);
        assert_eq!(lexer.next_token(), MarkdownToken::Text(text.clone()));
        assert_eq!(lexer.next_token(), MarkdownToken::Newline);
        assert_eq!(lexer.next_token(), MarkdownToken::Heading5);
        assert_eq!(lexer.next_token(), MarkdownToken::Text(text.clone()));
        assert_eq!(lexer.next_token(), MarkdownToken::Newline);
        assert_eq!(lexer.next_token(), MarkdownToken::Heading6);
        assert_eq!(lexer.next_token(), MarkdownToken::Text(text.clone()));
        assert_eq!(lexer.next_token(), MarkdownToken::Newline);
        assert_eq!(lexer.next_token(), MarkdownToken::EndOfFile);
    }

    #[test]
    fn bold_and_italic() {
        let text = String::from("hi");
        let input = format!("{text} **{text}** *{text}* ***{text}***");
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), MarkdownToken::Text("hi ".into()));
        assert_eq!(lexer.next_token(), MarkdownToken::Bold);
        assert_eq!(lexer.next_token(), MarkdownToken::Text("hi".into()));
        assert_eq!(lexer.next_token(), MarkdownToken::Bold);
        assert_eq!(lexer.next_token(), MarkdownToken::Text(" ".into()));
        assert_eq!(lexer.next_token(), MarkdownToken::Italic);
        assert_eq!(lexer.next_token(), MarkdownToken::Text("hi".into()));
        assert_eq!(lexer.next_token(), MarkdownToken::Italic);
        assert_eq!(lexer.next_token(), MarkdownToken::Text(" ".into()));
        assert_eq!(lexer.next_token(), MarkdownToken::Bold);
        assert_eq!(lexer.next_token(), MarkdownToken::Italic);
        assert_eq!(lexer.next_token(), MarkdownToken::Text("hi".into()));
        assert_eq!(lexer.next_token(), MarkdownToken::Italic);
        assert_eq!(lexer.next_token(), MarkdownToken::Bold);
        assert_eq!(lexer.next_token(), MarkdownToken::EndOfFile);
    }
}
