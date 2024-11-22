use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::common::AppError;
use crate::graph::list_view::ListView;
use crate::graph::tree_view::TreeView;

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
pub type TreeNodeWeak = Weak<RefCell<TreeNode>>;

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

struct Cursor {
    node: Option<TreeNodeRef>,
    tpos: usize,
    lpos: usize,
}

pub struct ModifFlags {
    pub render: bool,
    pub print: bool,
}

impl ModifFlags {
    pub fn new() -> ModifFlags {
        ModifFlags {
            render: true,
            print: true,
        }
    }
    pub fn from(render: bool, print: bool) -> ModifFlags {
        ModifFlags { render, print }
    }
}

// -----------------------------------------------------------------------------

pub struct Tree {
    pub tree_view: Weak<RefCell<TreeView>>,
    pub list_view: Weak<RefCell<ListView>>,
    pub root: TreeNodeRef,
    cursor: Cursor,
}

impl Tree {
    pub fn new() -> Tree {
        let root = TreeNode::create("/", NodeType::Dir);
        Tree {
            tree_view: Weak::new(),
            list_view: Weak::new(),
            root: root.clone(),
            cursor: Cursor {
                node: None,
                tpos: 0,
                lpos: 0,
            },
        }
    }

    pub fn curr_dir(&self) -> TreeNodeRef {
        if let Some(n) = &self.cursor.node {
            let nb = n.borrow();
            if let Some(sn) = nb.subnodes.get(self.cursor.tpos) { // TODO: z tym błędem zrobić coś sensownego
                return sn.clone();
            }
        }
        self.root.clone()
    }

    pub fn curr_file(&self) -> Option<TreeNodeRef> {
        let cd = self.curr_dir();
        let result = match cd.borrow().subnodes.get(self.cursor.lpos) {
            Some(node) => Some(node.clone()),
            None => None,
        };
        result
    }

    pub fn lmv_next(&mut self) {
        let cd = self.curr_dir();
        if self.cursor.lpos < cd.borrow().subnodes.len() - 1 {
            self.cursor.lpos += 1;
        }
    }

    pub fn tmv_next(&mut self) {
        if let Some(n) = &self.cursor.node {
            if self.cursor.tpos < n.borrow().subnodes.len() - 1 {
                self.cursor.tpos += 1;
                self.cursor.lpos = 0;
            }
        }
    }

    pub fn tmv_subdir(&mut self) {
        let cd: TreeNodeRef = self.curr_dir();
        if cd.borrow().subnodes.len() > 0 {
            self.cursor.node = Some(cd);
            self.cursor.tpos = 0;
            self.cursor.lpos = 0;
        }
    }

    pub fn tmv_updir(&mut self) {
        if let Some(n) = &self.cursor.node {
            let nc: Cursor;
            if let Some(p) = n.borrow().parent.upgrade() {
                nc = Cursor {
                    node: Some(p.clone()),
                    tpos: 0,
                    lpos: 0,
                };
            } else {
                nc = Cursor {
                    node: None,
                    tpos: 0,
                    lpos: 0,
                };
            }
            self.cursor = nc;
        }
    }
}
