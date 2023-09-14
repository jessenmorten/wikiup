use crate::lexer::Token;

pub fn render_html(tokens: Vec<Token>) -> String {
    let mut html = String::new();
    let mut h1 = false;
    let mut h2 = false;
    let mut h3 = false;
    let mut h4 = false;
    let mut h5 = false;
    let mut h6 = false;

    for token in tokens {
        match token {
            Token::Text(text) => {
                html.push_str(&text);
            }
            Token::Heading1 => {
                html.push_str("<h1>");
                h1 = true;
            }
            Token::Heading2 => {
                html.push_str("<h2>");
                h2 = true;
            }
            Token::Heading3 => {
                html.push_str("<h3>");
                h3 = true;
            }
            Token::Heading4 => {
                html.push_str("<h4>");
                h4 = true;
            }
            Token::Heading5 => {
                html.push_str("<h5>");
                h5 = true;
            }
            Token::Heading6 => {
                html.push_str("<h6>");
                h6 = true;
            }
            Token::EndOfFile | Token::Newline => {
                if h1 {
                    html.push_str("</h1>");
                }
                if h2 {
                    html.push_str("</h2>");
                }
                if h3 {
                    html.push_str("</h3>");
                }
                if h4 {
                    html.push_str("</h4>");
                }
                if h5 {
                    html.push_str("</h5>");
                }
                if h6 {
                    html.push_str("</h6>");
                }
            }
            _ => todo!(),
        }
    }
    html
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heading_1() {
        let tokens = vec![
            Token::Heading1,
            Token::Text("Hello World".into()),
            Token::EndOfFile,
        ];
        assert_eq!(render_html(tokens), "<h1>Hello World</h1>");
    }

    #[test]
    fn heading_2() {
        let tokens = vec![
            Token::Heading2,
            Token::Text("Hello World".into()),
            Token::EndOfFile,
        ];
        assert_eq!(render_html(tokens), "<h2>Hello World</h2>");
    }

    #[test]
    fn heading_3() {
        let tokens = vec![
            Token::Heading3,
            Token::Text("Hello World".into()),
            Token::EndOfFile,
        ];
        assert_eq!(render_html(tokens), "<h3>Hello World</h3>");
    }

    #[test]
    fn heading_4() {
        let tokens = vec![
            Token::Heading4,
            Token::Text("Hello World".into()),
            Token::EndOfFile,
        ];
        assert_eq!(render_html(tokens), "<h4>Hello World</h4>");
    }

    #[test]
    fn heading_5() {
        let tokens = vec![
            Token::Heading5,
            Token::Text("Hello World".into()),
            Token::EndOfFile,
        ];
        assert_eq!(render_html(tokens), "<h5>Hello World</h5>");
    }

    #[test]
    fn heading_6() {
        let tokens = vec![
            Token::Heading6,
            Token::Text("Hello World".into()),
            Token::EndOfFile,
        ];
        assert_eq!(render_html(tokens), "<h6>Hello World</h6>");
    }
}
