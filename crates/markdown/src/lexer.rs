#[derive(Debug, PartialEq)]
pub enum Token {
    Text(String),
    Heading1,      // #
    Heading2,      // ##
    Heading3,      // ###
    Heading4,      // ####
    Heading5,      // #####
    Heading6,      // ######
    Bold,          // **
    Italic,        // *
    Newline,       // \n
    DoubleNewline, // \n\n
    EndOfFile,     // 0
    Illegal,       // ?
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

    pub fn next_token(&mut self) -> Token {
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
                    1 => Token::Heading1,
                    2 => Token::Heading2,
                    3 => Token::Heading3,
                    4 => Token::Heading4,
                    5 => Token::Heading5,
                    6 => Token::Heading6,
                    _ => Token::Illegal,
                }
            }
            b'*' => {
                if self.is_italic {
                    self.is_italic = false;
                    Token::Italic
                } else if self.peek() == b'*' {
                    self.read_char();
                    Token::Bold
                } else {
                    self.is_italic = true;
                    Token::Italic
                }
            }
            b'\n' => {
                if self.peek() == b'\n' {
                    self.read_char();
                    Token::DoubleNewline
                } else {
                    Token::Newline
                }
            }
            0 => Token::EndOfFile,
            _ => Token::Text(self.read_text()),
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
        assert_eq!(lexer.next_token(), Token::Newline);
        assert_eq!(lexer.next_token(), Token::Heading1);
        assert_eq!(lexer.next_token(), Token::Text(text.clone()));
        assert_eq!(lexer.next_token(), Token::Newline);
        assert_eq!(lexer.next_token(), Token::Heading2);
        assert_eq!(lexer.next_token(), Token::Text(text.clone()));
        assert_eq!(lexer.next_token(), Token::Newline);
        assert_eq!(lexer.next_token(), Token::Heading3);
        assert_eq!(lexer.next_token(), Token::Text(text.clone()));
        assert_eq!(lexer.next_token(), Token::Newline);
        assert_eq!(lexer.next_token(), Token::Heading4);
        assert_eq!(lexer.next_token(), Token::Text(text.clone()));
        assert_eq!(lexer.next_token(), Token::Newline);
        assert_eq!(lexer.next_token(), Token::Heading5);
        assert_eq!(lexer.next_token(), Token::Text(text.clone()));
        assert_eq!(lexer.next_token(), Token::Newline);
        assert_eq!(lexer.next_token(), Token::Heading6);
        assert_eq!(lexer.next_token(), Token::Text(text.clone()));
        assert_eq!(lexer.next_token(), Token::Newline);
        assert_eq!(lexer.next_token(), Token::EndOfFile);
    }

    #[test]
    fn bold_and_italic() {
        let text = String::from("hi");
        let input = format!("{text} **{text}** *{text}* ***{text}***");
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::Text("hi ".into()));
        assert_eq!(lexer.next_token(), Token::Bold);
        assert_eq!(lexer.next_token(), Token::Text("hi".into()));
        assert_eq!(lexer.next_token(), Token::Bold);
        assert_eq!(lexer.next_token(), Token::Text(" ".into()));
        assert_eq!(lexer.next_token(), Token::Italic);
        assert_eq!(lexer.next_token(), Token::Text("hi".into()));
        assert_eq!(lexer.next_token(), Token::Italic);
        assert_eq!(lexer.next_token(), Token::Text(" ".into()));
        assert_eq!(lexer.next_token(), Token::Bold);
        assert_eq!(lexer.next_token(), Token::Italic);
        assert_eq!(lexer.next_token(), Token::Text("hi".into()));
        assert_eq!(lexer.next_token(), Token::Italic);
        assert_eq!(lexer.next_token(), Token::Bold);
        assert_eq!(lexer.next_token(), Token::EndOfFile);
    }

    #[test]
    fn newlines() {
        let input = format!("Hello\n\nWorld\n");
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::Text("Hello".into()));
        assert_eq!(lexer.next_token(), Token::DoubleNewline);
        assert_eq!(lexer.next_token(), Token::Text("World".into()));
        assert_eq!(lexer.next_token(), Token::Newline);
        assert_eq!(lexer.next_token(), Token::EndOfFile);
    }
}
