use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub enum NodeType {
    File,
    Dir,
    UpDir,
}
pub struct SysNode {
    pub name: String,
    pub typ: NodeType,
}

pub type TreeNodeRef = Rc<RefCell<TreeNode>>;
type TreeNodeWeak = Weak<RefCell<TreeNode>>;

// -----------------------------------------------------------------------------

pub struct TreeNode {
    pub sys_node: SysNode,
    pub subnodes: Vec<TreeNodeRef>,
    pub parent: TreeNodeWeak,
}

impl TreeNode {
    pub fn new(sys_node: SysNode) -> TreeNodeRef {
        Rc::new(RefCell::new(Self {
            sys_node,
            subnodes: Vec::new(),
            parent: Weak::new(),
        }))
    }

    pub fn create(name: &str, typ: NodeType) -> TreeNodeRef {
        Rc::new(RefCell::new(Self {
            sys_node: SysNode {
                name: name.to_owned(),
                typ,
            },
            subnodes: Vec::new(),
            parent: Weak::new(),
        }))
    }

    pub fn append(this: &mut TreeNodeRef, subn: TreeNodeRef) {
        subn.borrow_mut().parent = Rc::downgrade(this);
        this.borrow_mut().subnodes.push(subn);
    }
}

// -----------------------------------------------------------------------------

struct TreeCursor {
    node: Option<TreeNodeRef>, // Refers to parent
    pos: usize,                // Index in parent vec
}

struct ListCursor {
    node: TreeNodeRef, // Refers to parent
    pos: usize,        // Index in parent vec
}

pub struct Tree {
    pub root: TreeNodeRef,
    tree_cursor: TreeCursor,
    list_cursor: ListCursor,
}

impl Tree {
    pub fn new() -> Tree {
        let root = TreeNode::create("/", NodeType::Dir);
        Tree {
            root: root.clone(),
            tree_cursor: TreeCursor { node: None, pos: 0 },
            list_cursor: ListCursor { node: root, pos: 0 },
        }
    }

    pub fn curr_dir(&self) -> TreeNodeRef {
        if let Some(n) = &self.tree_cursor.node {
            let nb = n.borrow();
            if let Some(sn) = nb.subnodes.get(self.tree_cursor.pos) {
                return sn.clone();
            }
        }
        self.root.clone()
    }

    pub fn tmv_next(&self) {}

    pub fn tmv_subdir(&mut self) {
        let cd: TreeNodeRef = self.curr_dir();
        if cd.borrow().subnodes.len() > 0 {
            self.tree_cursor.node = Some(cd);
            self.tree_cursor.pos = 0;
        }
    }

    pub fn tmv_updir(&mut self) {
        if let Some(n) = &self.tree_cursor.node {
            let nc:TreeCursor;
            if let Some(p) = n.borrow().parent.upgrade() {
                nc=TreeCursor{ node: Some(p.clone()), pos: 0 };
            }
            else{
                nc=TreeCursor{ node:None, pos: 0 };
            }
            self.tree_cursor=nc;
        }
    }
}

fn list_node(node: &TreeNodeRef, level: usize) {
    let n = node.borrow();
    println!("{}{}", "-".repeat(level), n.sys_node.name);
    for sn in &n.subnodes {
        list_node(sn, level + 1);
    }
}

fn list_tree(tree: &Tree) {
    list_node(&tree.root, 0);
}