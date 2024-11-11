
pub struct Node{

}

pub struct FileNode {
    pub node:Node,
}

pub struct DirNode {
    pub node:Node,
}

pub enum TreeNode {
    File(FileNode),
    Dir(DirNode),
}

pub struct Tree {}
