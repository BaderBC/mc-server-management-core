use std::env;
use std::path::Path;

pub fn path_to_string<T>(path: T) -> String
    where T: AsRef<Path> {
    let path = path.as_ref();

    if path.is_absolute() {
        path.to_path_buf().to_str().unwrap().to_string()
    } else {
        env::current_dir().unwrap().join(path).to_str().unwrap().to_string()
    }
}