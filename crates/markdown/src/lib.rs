use html::render_html;
use lexer::{Lexer, Token};

mod html;
mod lexer;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document() {
        let markdown = "
# Hello World
## Hello World
### Hello World
#### Hello World
##### Hello World
###### Hello World

*Hello* **World**!
**Hola** `Mundo`!

```rust
fn main() {
    println!(\"Hello World\");
}
```
";
        let expected = vec![
            "<h1>Hello World</h1>",
            "<h2>Hello World</h2>",
            "<h3>Hello World</h3>",
            "<h4>Hello World</h4>",
            "<h5>Hello World</h5>",
            "<h6>Hello World</h6>",
            "<br>",
            "<p><i>Hello</i> <b>World</b>!</p>",
            "<p><b>Hola</b> <code>Mundo</code>!</p>",
            "<br>",
            "<pre><code class=\"language-rust\">fn main() {\n    println!(\"Hello World\");\n}\n</code></pre>",
        ]
        .join("");
        assert_eq!(to_html(markdown), expected);
    }
}
