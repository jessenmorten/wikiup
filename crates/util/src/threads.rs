use std::{num::NonZeroUsize, thread::available_parallelism};

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
