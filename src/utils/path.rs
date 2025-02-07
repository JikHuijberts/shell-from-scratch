use std::env;
use std::path::{Path,PathBuf};

pub fn find_path<P>(exe_name:P) -> Option<PathBuf> 
    where P: AsRef<Path>,
    {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).filter_map(|dir| {
            let full_path = dir.join(&exe_name);

            if full_path.is_file() {
                Some(full_path)
            } else {
                None
            }
        }).next()
    })
}
