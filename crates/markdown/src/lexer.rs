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
    Code(String),  // `
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
            b'`' => {
                let mut bytes = Vec::new();
                self.read_char();

                while self.ch != b'`' {
                    bytes.push(self.ch);
                    self.read_char();
                }

                match String::from_utf8(bytes) {
                    Ok(s) => Token::Code(s),
                    Err(_) => {
                        eprintln!("Error: Invalid UTF-8 sequence");
                        Token::Code(String::new())
                    }
                }
            }
            b'\r' | b'\n' => {
                let mut newline_count = 0;

                if self.ch == b'\n' {
                    newline_count += 1;
                }

                while newline_count < 2 && (self.peek() == b'\r' || self.peek() == b'\n') {
                    self.read_char();
                    if self.ch == b'\n' {
                        newline_count += 1;
                    }
                }

                if newline_count == 2 {
                    Token::DoubleNewline
                } else {
                    Token::Newline
                }
            }
            0 => Token::EndOfFile,
            _ => Token::Text(self.read_text()),
        };

        self.read_char();
        token
    }

    fn peek(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position]
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
        let mut bytes = Vec::new();
        bytes.push(self.ch);

        while self.is_peek_text() {
            self.read_char();
            bytes.push(self.ch);
        }

        match String::from_utf8(bytes) {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Error: Invalid UTF-8 sequence");
                String::new()
            }
        }
    }

    fn is_peek_text(&mut self) -> bool {
        let peek = self.peek();
        !(peek == b'#'
            || peek == b'`'
            || peek == b'*'
            || peek == b'\n'
            || peek == b'\r'
            || peek == 0)
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
        assert_eq!(lexer.next_token(), Token::Text(text));
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
        let input = "Hello\n\nWorld\n".to_string();
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::Text("Hello".into()));
        assert_eq!(lexer.next_token(), Token::DoubleNewline);
        assert_eq!(lexer.next_token(), Token::Text("World".into()));
        assert_eq!(lexer.next_token(), Token::Newline);
        assert_eq!(lexer.next_token(), Token::EndOfFile);
    }

    #[test]
    fn code() {
        let input = "Example: `code`".to_string();
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::Text("Example: ".into()));
        assert_eq!(lexer.next_token(), Token::Code("code".into()));
        assert_eq!(lexer.next_token(), Token::EndOfFile);
    }

    #[test]
    fn handle_carriage_return() {
        let input = String::from("# Hello\r\n\r\nWorld\r\n");
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::Heading1);
        assert_eq!(lexer.next_token(), Token::Text("Hello".into()));
        assert_eq!(lexer.next_token(), Token::DoubleNewline);
        assert_eq!(lexer.next_token(), Token::Text("World".into()));
        assert_eq!(lexer.next_token(), Token::Newline);
        assert_eq!(lexer.next_token(), Token::EndOfFile);
    }
}
