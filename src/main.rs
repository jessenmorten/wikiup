use markdown::to_html;
use std::{fs::read_to_string, path::PathBuf};
use util::{io::get_all_markdown_files, threads::set_max_threads};

fn main() {
    set_max_threads(20);

    let start = std::time::Instant::now();
    let root = PathBuf::from(".");
    let file_rx = get_all_markdown_files(root);

    while let Ok(path) = file_rx.recv() {
        if let Ok(markdown) = read_to_string(&path) {
            let html = to_html(&markdown);
            println!("{html}");
        }
    }

    println!("Time taken: {:?}", start.elapsed());
}
