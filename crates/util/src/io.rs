use std::{
    fs::read_dir,
    path::PathBuf,
    sync::mpsc::{channel, Receiver},
    thread::spawn,
};

pub fn get_all_markdown_files(root: PathBuf) -> Receiver<PathBuf> {
    let files_rx = get_all_files(root);
    let (tx, rx) = channel::<PathBuf>();

    spawn(move || {
        while let Ok(path) = files_rx.recv() {
            let is_markdown = path
                .extension()
                .map(|ext| ext == "md" || ext == "markdown")
                .unwrap_or(false);

            if is_markdown {
                if let Err(err) = tx.send(path) {
                    eprintln!("Failed to send path: {}", err);
                    break;
                }
            }
        }
    });

    rx
}

pub fn get_all_files(root: PathBuf) -> Receiver<PathBuf> {
    let (tx, rx) = std::sync::mpsc::channel();

    spawn(move || {
        let mut to_walk = vec![root];

        while let Some(path_to_walk) = to_walk.pop() {
            let mut dir = match read_dir(path_to_walk) {
                Ok(dir) => dir,
                Err(err) => {
                    eprintln!("Failed to read dir: {}", err);
                    continue;
                }
            };

            while let Some(entry) = dir.next() {
                let entry = match entry {
                    Ok(entry) => entry,
                    Err(err) => {
                        eprintln!("Failed to read entry: {}", err);
                        continue;
                    }
                };

                let new_path = entry.path();
                if new_path.is_dir() {
                    to_walk.push(new_path);
                } else {
                    if let Err(err) = tx.send(new_path) {
                        eprintln!("Failed to send path: {}", err);
                        break;
                    }
                }
            }
        }
    });

    rx
}
