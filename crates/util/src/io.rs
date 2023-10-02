use std::{
    fs::read_dir,
    path::PathBuf,
    sync::{
        mpsc::{channel, Receiver},
        Arc, Mutex,
    },
};

use crate::threads::{get_max_threads, maybe_spawn};

pub fn get_all_markdown_files(root: PathBuf) -> Receiver<PathBuf> {
    let files_rx = get_all_files(root);
    let (tx, rx) = channel();

    maybe_spawn(move || {
        while let Ok(path) = files_rx.recv() {
            let is_markdown = path
                .extension()
                .map(|ext| ext == "md" || ext == "markdown")
                .unwrap_or(false);

            if is_markdown {
                if let Err(err) = tx.send(path) {
                    eprintln!("Failed to send path: {err}");
                    break;
                }
            }
        }
    });

    rx
}

pub fn get_all_files(root: PathBuf) -> Receiver<PathBuf> {
    let (tx, rx) = channel();
    let to_walk = Arc::new(Mutex::new(vec![root]));

    let max_threads = get_max_threads();
    for _ in 0..max_threads {
        let to_walk = to_walk.clone();
        let tx = tx.clone();

        maybe_spawn(move || loop {
            let mut p: Option<PathBuf> = None;
            {
                if let Ok(mut to_walk) = to_walk.lock() {
                    if let Some(hmm) = to_walk.pop() {
                        p = Some(hmm);
                    }
                }
            }

            let path_to_walk = match p {
                Some(p) => p,
                None => break,
            };

            let dir = match read_dir(path_to_walk) {
                Ok(dir) => dir,
                Err(err) => {
                    eprintln!("Failed to read dir: {err}");
                    continue;
                }
            };

            for entry in dir {
                let entry = match entry {
                    Ok(entry) => entry,
                    Err(err) => {
                        eprintln!("Failed to read entry: {err}");
                        continue;
                    }
                };

                let new_path = entry.path();
                if new_path.is_dir() {
                    {
                        if let Ok(mut to_walk) = to_walk.lock() {
                            to_walk.push(new_path);
                        }
                    }
                } else if let Err(err) = tx.send(new_path) {
                    eprintln!("Failed to send path: {err}");
                    break;
                }
            }
        });
    }

    rx
}
