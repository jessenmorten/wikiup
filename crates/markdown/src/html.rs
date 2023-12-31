use crate::lexer::Token;

pub fn render_html(tokens: Vec<Token>) -> String {
    let mut html = String::new();
    let mut h1 = false;
    let mut h2 = false;
    let mut h3 = false;
    let mut h4 = false;
    let mut h5 = false;
    let mut h6 = false;
    let mut ul = 0;
    let mut li = false;
    let mut p = false;
    let mut bold = false;
    let mut italic = false;
    let mut should_start_with_space = false;

    for token in tokens {
        match token {
            Token::Text(text) => {
                if !li && !h1 && !h2 && !h3 && !h4 && !h5 && !h6 && !p {
                    html.push_str("<p>");
                    p = true;
                }
                if should_start_with_space {
                    html.push(' ');
                    should_start_with_space = false;
                }
                html.push_str(&text);
            }
            Token::Code(code) => {
                if !li && !h1 && !h2 && !h3 && !h4 && !h5 && !h6 && !p {
                    html.push_str("<p>");
                    p = true;
                }
                html.push_str("<code>");
                html.push_str(&code);
                html.push_str("</code>");
            }
            Token::CodeBlock(language, code) => {
                html.push_str("<pre>");
                if let Some(lang) = language {
                    html.push_str("<code class=\"language-");
                    html.push_str(&lang);
                    html.push_str("\">");
                } else {
                    html.push_str("<code>");
                }
                html.push_str(&code);
                html.push_str("</code></pre>");
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
            Token::UnorderedBullet(indent) => {
                while ul > indent + 1 {
                    html.push_str("</ul>");
                    ul -= 1;
                }

                while ul <= indent {
                    html.push_str("<ul>");
                    ul += 1;
                }

                html.push_str("<li>");
                li = true;
            }
            Token::Bold => {
                if !li && !h1 && !h2 && !h3 && !h4 && !h5 && !h6 && !p {
                    html.push_str("<p>");
                    p = true;
                }
                if !bold {
                    html.push_str("<b>");
                } else {
                    html.push_str("</b>");
                }
                bold = !bold;
            }
            Token::Italic => {
                if !li && !h1 && !h2 && !h3 && !h4 && !h5 && !h6 && !p {
                    html.push_str("<p>");
                    p = true;
                }
                if !italic {
                    html.push_str("<i>");
                } else {
                    html.push_str("</i>");
                }
                italic = !bold;
            }
            Token::EndOfFile | Token::Newline | Token::DoubleNewline => {
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
                if p && token == Token::Newline {
                    should_start_with_space = true;
                }
                if p && (token == Token::DoubleNewline || token == Token::EndOfFile) {
                    html.push_str("</p>");
                    p = false;
                }
                if li {
                    html.push_str("</li>");
                    li = false;
                }
                if token == Token::DoubleNewline || token == Token::EndOfFile {
                    while ul > 0 {
                        html.push_str("</ul>");
                        ul -= 1;
                    }
                }
            }
            _ => todo!("{:?}", token),
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

    #[test]
    fn bold() {
        let tokens = vec![
            Token::Text("Hello ".into()),
            Token::Bold,
            Token::Text("World!".into()),
            Token::Bold,
            Token::Newline,
            Token::EndOfFile,
        ];
        assert_eq!(render_html(tokens), "<p>Hello <b>World!</b></p>");
    }

    #[test]
    fn italic() {
        let tokens = vec![
            Token::Text("Hello ".into()),
            Token::Italic,
            Token::Text("World!".into()),
            Token::Italic,
            Token::Newline,
            Token::EndOfFile,
        ];
        assert_eq!(render_html(tokens), "<p>Hello <i>World!</i></p>");
    }

    #[test]
    fn newlines() {
        let tokens = vec![
            Token::Text("Hi".into()),
            Token::Newline,
            Token::Text("Hey".into()),
            Token::DoubleNewline,
            Token::Text("Yo".into()),
            Token::Newline,
            Token::EndOfFile,
        ];
        assert_eq!(render_html(tokens), "<p>Hi Hey</p><p>Yo</p>");
    }

    #[test]
    fn inline_code() {
        let tokens = vec![
            Token::Code("Hello".into()),
            Token::Text(" World!".into()),
            Token::Newline,
            Token::EndOfFile,
        ];
        assert_eq!(render_html(tokens), "<p><code>Hello</code> World!</p>");
    }

    #[test]
    fn code_block() {
        let tokens = vec![
            Token::CodeBlock(
                Some("rust".into()),
                "fn main() {\n    println!(\"Hello World!\");\n}".into(),
            ),
            Token::Newline,
            Token::EndOfFile,
        ];
        assert_eq!(render_html(tokens), "<pre><code class=\"language-rust\">fn main() {\n    println!(\"Hello World!\");\n}</code></pre>");
    }

    #[test]
    fn code_block_no_language() {
        let tokens = vec![
            Token::CodeBlock(None, "Hello\nWorld!".into()),
            Token::Newline,
            Token::EndOfFile,
        ];
        assert_eq!(render_html(tokens), "<pre><code>Hello\nWorld!</code></pre>");
    }

    #[test]
    fn unordered_list() {
        let tokens = vec![
            Token::UnorderedBullet(0),
            Token::Text("Hello".into()),
            Token::Newline,
            Token::UnorderedBullet(0),
            Token::Text("Hi ".into()),
            Token::Italic,
            Token::Text("there".into()),
            Token::Italic,
            Token::Newline,
            Token::UnorderedBullet(1),
            Token::Text("Hello".into()),
            Token::Newline,
            Token::UnorderedBullet(0),
            Token::Text("Hola".into()),
            Token::EndOfFile,
        ];
        assert_eq!(
            render_html(tokens),
            "<ul><li>Hello</li><li>Hi <i>there</i></li><ul><li>Hello</li></ul><li>Hola</li></ul>"
        );
    }
}
