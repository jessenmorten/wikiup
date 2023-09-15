use crate::lexer::Token;

pub fn render_html(tokens: Vec<Token>) -> String {
    let mut html = String::new();
    let mut h1 = false;
    let mut h2 = false;
    let mut h3 = false;
    let mut h4 = false;
    let mut h5 = false;
    let mut h6 = false;
    let mut p = false;

    for token in tokens {
        match token {
            Token::Text(text) => {
                if !h1 && !h2 && !h3 && !h4 && !h5 && !h6 && !p {
                    html.push_str("<p>");
                    p = true;
                }
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
                    h1 = false;
                }
                if h2 {
                    html.push_str("</h2>");
                    h2 = false;
                }
                if h3 {
                    html.push_str("</h3>");
                    h3 = false;
                }
                if h4 {
                    html.push_str("</h4>");
                    h4 = false;
                }
                if h5 {
                    html.push_str("</h5>");
                    h5 = false;
                }
                if h6 {
                    html.push_str("</h6>");
                    h6 = false;
                }
                if p {
                    html.push_str("</p>");
                    p = false;
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
            Token::Newline,
            Token::EndOfFile,
        ];
        assert_eq!(render_html(tokens), "<h1>Hello World</h1>");
    }

    #[test]
    fn heading_2() {
        let tokens = vec![
            Token::Heading2,
            Token::Text("Hello World".into()),
            Token::Newline,
            Token::EndOfFile,
        ];
        assert_eq!(render_html(tokens), "<h2>Hello World</h2>");
    }

    #[test]
    fn heading_3() {
        let tokens = vec![
            Token::Heading3,
            Token::Text("Hello World".into()),
            Token::Newline,
            Token::EndOfFile,
        ];
        assert_eq!(render_html(tokens), "<h3>Hello World</h3>");
    }

    #[test]
    fn heading_4() {
        let tokens = vec![
            Token::Heading4,
            Token::Text("Hello World".into()),
            Token::Newline,
            Token::EndOfFile,
        ];
        assert_eq!(render_html(tokens), "<h4>Hello World</h4>");
    }

    #[test]
    fn heading_5() {
        let tokens = vec![
            Token::Heading5,
            Token::Text("Hello World".into()),
            Token::Newline,
            Token::EndOfFile,
        ];
        assert_eq!(render_html(tokens), "<h5>Hello World</h5>");
    }

    #[test]
    fn heading_6() {
        let tokens = vec![
            Token::Heading6,
            Token::Text("Hello World".into()),
            Token::Newline,
            Token::EndOfFile,
        ];
        assert_eq!(render_html(tokens), "<h6>Hello World</h6>");
    }

    #[test]
    fn p() {
        let tokens = vec![
            Token::Text("Hello World".into()),
            Token::Newline,
            Token::EndOfFile,
        ];
        assert_eq!(render_html(tokens), "<p>Hello World</p>");
    }
}
