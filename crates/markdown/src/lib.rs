pub fn to_html(markdown: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heading_1() {
        let html = to_html("# Hello");
        assert_eq!(html, "<h1>Hello</h1>");
    }
}
