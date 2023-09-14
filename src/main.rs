use markdown::to_html;

fn main() {
    let html = to_html("
# Wikiup
## Wikiup
### Wikiup
#### Wikiup
##### Wikiup
###### Wikiup
");
    println!("{}", html);
}
