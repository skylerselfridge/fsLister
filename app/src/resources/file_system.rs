use std::{fs, path::Path};

use walkdir::WalkDir;

#[derive(Debug)]
pub struct Node {
    name: String,
    files: i32,
    children: Vec<Node>,
}

impl Node {
    pub fn from_path(path: &std::path::Path) -> Node {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "".to_string());
        let mut children = Vec::new();
        let mut files = 0;
        if path.is_dir() {
            if let Ok(entries) = fs::read_dir(path) {
                files = get_files(path.clone());
                for entry in entries {
                    if let Ok(entry) = entry {
                        children.push(Node::from_path(&entry.path()));
                    }
                }
            }
        }
        Node { name, files, children }
    }
    pub fn new() -> Self{
        let name = "".to_string();
        let children = Vec::new();
        let files = 0;
        Self {
           name,
           files,
           children,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_children(&self) -> &Vec<Node> {
        &self.children
    }

    pub fn get_files(&self) -> i32 {
        self.files.clone()
    }
}

pub fn get_files<P: AsRef<Path>>(path: P) -> i32 {
    let mut count = 0;
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() && is_cr2_or_jpg_file(entry.path()) {
            count += 1;
        }
    }
    count
}

fn is_cr2_or_jpg_file<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    match path.extension() {
        Some(ext) => {
            let ext_str = ext.to_string_lossy().to_lowercase();
            ext_str == "cr2" || ext_str == "jpg"
        },
        None => false
    }
}





pub fn is_dir<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    path.is_dir()
}
