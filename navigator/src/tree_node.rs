use std::{
    cell::RefCell,
    fs,
    path::PathBuf,
    rc::{Rc, Weak},
};

use crate::{common::*, filesystem::*};

pub type TreeNodeRef = Rc<RefCell<TreeNode>>;
pub type TreeNodeWeak = Weak<RefCell<TreeNode>>;

pub struct TreeNode {
    pub sys_node: SysNode,
    pub subnodes: Vec<TreeNodeRef>,
    pub parent: TreeNodeWeak,
    pub loaded: bool,
    pub expanded: bool,
}

impl TreeNode {
    pub fn from(sys_node: SysNode) -> TreeNodeRef {
        Rc::new(RefCell::new(Self {
            sys_node,
            subnodes: Vec::new(),
            parent: Weak::new(),
            loaded: false,
            expanded: false,
        }))
    }

    pub fn append(this: &TreeNodeRef, subn: TreeNodeRef) {
        subn.borrow_mut().parent = Rc::downgrade(this);
        this.borrow_mut().subnodes.push(subn);
    }

    fn fill_path(&self, p: &mut PathBuf) {
        if let Some(parent) = self.parent.upgrade() {
            parent.borrow().fill_path(p);
        }
        p.push(&self.sys_node.name);
    }

    pub fn get_path(&self) -> PathBuf {
        let mut path = PathBuf::new();
        self.fill_path(&mut path);
        path
    }

    pub fn try_unload(this_node: &TreeNodeRef, next_node: &TreeNodeRef) -> bool {
        let mut unloaded = false;
        let mut dest_branch: Vec<TreeNodeRef> = Vec::new();
        TreeNode::get_branch(next_node, &mut dest_branch);
        TreeNode::inner_try_unload(this_node, &dest_branch, &mut unloaded);
        unloaded
    }

    fn inner_try_unload(
        this_node: &TreeNodeRef,
        dest_branch: &Vec<TreeNodeRef>,
        unloaded: &mut bool,
    ) {
        let on_branch = dest_branch.iter().any(|n| Rc::ptr_eq(n, this_node));
        if !on_branch {
            let mut this_node = this_node.borrow_mut();
            if !this_node.expanded {
                this_node.unload();
                *unloaded = true;
                if let Some(parent) = this_node.parent.upgrade() {
                    TreeNode::inner_try_unload(&parent, dest_branch, unloaded);
                }
            }
        }
    }

    fn get_branch(node: &TreeNodeRef, branch: &mut Vec<TreeNodeRef>) {
        branch.push(node.clone());
        if let Some(parent) = node.borrow().parent.upgrade() {
            TreeNode::get_branch(&parent, branch);
        }
    }

    pub fn load(this: &TreeNodeRef) -> Result<(), AppError> {
        if !this.borrow().loaded {
            this.borrow_mut().subnodes.clear();
            let nodes =
                fs::read_dir(this.borrow().get_path())?.map(|res| res.map(|e| SysNode::from(&e)));
            for on in nodes {
                if let Ok(n) = on {
                    TreeNode::append(this, TreeNode::from(n));
                }
            }
            this.borrow_mut().loaded = true;
        }
        Ok(())
    }

    pub fn unload(&mut self) {
        self.subnodes.clear();
        self.loaded = false;
    }

    // pub fn is_child_of(parent: &TreeNodeRef, child: &TreeNodeRef) -> bool {
    //     if let Some(p) = child.borrow().parent.upgrade() {
    //         if Rc::ptr_eq(&p, parent) {
    //             return true;
    //         }
    //         return TreeNode::is_child_of(parent, &p);
    //     }
    //     return false;
    // }

    pub fn expand(this: &mut TreeNodeRef) {
        if let Some(parent) = this.borrow().parent.upgrade() {
            let mut p = parent.clone();
            TreeNode::expand(&mut p);
        }
        this.borrow_mut().expanded = true;
    }
}
