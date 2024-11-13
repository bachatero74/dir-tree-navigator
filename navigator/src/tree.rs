use crate::graph::display::*; // Tymczasowe

pub struct Node {}

pub struct FileNode {
    pub node: Node,
}

pub struct DirNode {
    pub node: Node,
}

pub enum TreeNode {
    File(FileNode),
    Dir(DirNode),
}

pub struct Tree {
    pub tmp_lines: Vec<ViewLine>,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            tmp_lines: vec![
                ViewLine::new("12345abcde".to_owned(), 5, 9),
                ViewLine::new("12345abcde".to_owned(), 5, 9),
                ViewLine::new("12345abcde".to_owned(), 5, 9),
                ViewLine::new("12345abcde".to_owned(), 5, 9),
            ]
        }
    }
}
