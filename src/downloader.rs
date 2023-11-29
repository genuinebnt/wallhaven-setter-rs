use std::collections::HashMap;
use std::path::PathBuf;

struct Downloader {
    links: HashMap<String, String>,
    path: PathBuf,
}

impl Downloader {
    fn new() -> Downloader {
        Downloader {
            links: HashMap::new(),
            path: PathBuf::new(),
        }
    }
}
