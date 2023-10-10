use markdown::to_html;
use std::{
    env::temp_dir,
    fs::{create_dir_all, read_to_string, write},
    path::PathBuf,
};
use util::{io::get_all_markdown_files, threads::set_max_threads};

fn main() {
    set_max_threads(20);

    let start = std::time::Instant::now();
    let root = PathBuf::from(".");
    let out = temp_dir().join("wikiup");
    let file_rx = get_all_markdown_files(root.clone());

    while let Ok(path) = file_rx.recv() {
        if let Ok(markdown) = read_to_string(&path) {
            let html = to_html(&markdown);

            let path = match path.strip_prefix(&root) {
                Ok(path) => path,
                Err(_) => {
                    eprintln!("Failed to strip prefix from {:?}", path);
                    continue;
                }
            };

            let out_path = out.join(path).with_extension("html");

            let out_parent_dir = match out_path.parent() {
                Some(dir) => dir,
                None => {
                    eprintln!("Failed to get parent dir of {:?}", out_path);
                    continue;
                }
            };

            match create_dir_all(out_parent_dir) {
                Ok(_) => (),
                Err(_) => {
                    eprintln!("Failed to create dir {:?}", out_parent_dir);
                    continue;
                }
            }

            match write(&out_path, html) {
                Ok(_) => println!("Wrote {:?}", out_path),
                Err(_) => {
                    eprintln!("Failed to write to {:?}", out_path);
                    continue;
                }
            }
        }
    }

    println!("Time taken: {:?}", start.elapsed());
}
