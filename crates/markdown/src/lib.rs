use html::render_html;
use lexer::{Token, Lexer};

mod lexer;
mod html;

pub fn to_html(markdown: &str) -> String {
    render_html(get_tokens(markdown))
}

fn get_tokens(markdown: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(markdown.to_string());
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        let eof = token == Token::EndOfFile;
        tokens.push(token);
        if eof {
            break;
        }
    }
    tokens
}
