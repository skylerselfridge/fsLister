use std::{fs, path::Path};
#[derive(Debug)]
pub struct Node {
    name: String,
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
        if path.is_dir() {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        children.push(Node::from_path(&entry.path()));
                    }
                }
            }
        }
        Node { name, children }
    }
    pub fn new() -> Self{
        let name = "".to_string();
        let children = Vec::new();
        Self {
           name,
           children,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_children(&self) -> &Vec<Node> {
        &self.children
    }
}

pub fn get_files<P: AsRef<Path>>(path: P, paths: &mut Vec<String>) {

    // Traverse the folder and its subfolders recursively
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        // If the entry is a directory, count the files in the directory recursively
        if path.is_dir() {
            get_files(&path.clone(), paths);
        } else {
            // If the entry is a file, increment the count
            if is_cr2_or_jpg_file(path.clone()) {paths.push(path.to_string_lossy().to_string());}
        }
    }
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
