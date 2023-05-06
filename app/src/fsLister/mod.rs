use crate::resources::Node;




pub struct fsLister {
    file_struct: Node,
    folder_path: String,
    sort: bool,
}

impl fsLister {
    pub fn new() -> Self {
        let file_struct = Node::new();
        let folder_path = "".to_string();
        let sort = false;
        Self {
        
            file_struct,
            folder_path,
            sort,
        }
    }

    pub fn get_sort_setting(&self) -> bool {
        self.sort
    }

    pub fn set_sort_setting(&mut self, sort: bool) {
        self.sort = sort
    }

    pub fn get_folder_path(&self) -> &String {
        &self.folder_path
    }

    pub fn set_folder_path(&mut self, path: String) {
        self.folder_path = path;
    }

    pub fn get_file_struct(&self) -> &Node {
        &self.file_struct
    }

    pub fn set_file_struct(&mut self, tree: Node) {
        self.file_struct = tree;
    }


}


