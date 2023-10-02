use std::{
    num::NonZeroUsize,
    sync::atomic::{AtomicUsize, Ordering},
    thread::{available_parallelism, spawn},
};

static MAX_THREADS: AtomicUsize = AtomicUsize::new(1);
static RUNNING_THREADS: AtomicUsize = AtomicUsize::new(0);

pub fn maybe_spawn<F>(f: F)
where
    F: FnOnce() + Send + 'static,
{
    let max_threads = MAX_THREADS.load(Ordering::Relaxed);
    let running_threads = RUNNING_THREADS.load(Ordering::Relaxed);

    if running_threads < max_threads {
        RUNNING_THREADS.fetch_add(1, Ordering::Relaxed);
        spawn(move || {
            f();
            RUNNING_THREADS.fetch_sub(1, Ordering::Relaxed);
        });
    } else {
        f();
    }
}

pub fn get_max_threads() -> usize {
    MAX_THREADS.load(Ordering::Relaxed)
}

pub fn set_max_threads(max_threads: usize) {
    MAX_THREADS.store(max_threads, Ordering::Relaxed);
}

pub fn get_avaliable_parallelism() -> NonZeroUsize {
    match available_parallelism() {
        Ok(threads) => threads,
        Err(err) => {
            let default_parallelism = 4;
            eprintln!(
                "Failed to get available parallelism, using {default_parallelism} as default: {err}"
            );
            NonZeroUsize::new(default_parallelism).unwrap()
        }
    }
}
