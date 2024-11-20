// Test of new Tree concept

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

type TreeNodeRef = Rc<RefCell<TreeNode>>;
type WeakTreeNodeRef = Weak<RefCell<TreeNode>>;

enum SysNodeType {
    File,
    Directory,
}

struct SysNode {
    name: String,
    typ: SysNodeType,
}

// ------------------------------------------------------------------------

struct TreeNode {
    sys_node: SysNode,
    subnodes: TreeNodeList,
    next: Option<TreeNodeRef>,
    prev: WeakTreeNodeRef,
}

impl TreeNode {
    fn new(sys_node: SysNode) -> TreeNodeRef {
        Rc::new(RefCell::new(TreeNode {
            sys_node,
            subnodes: TreeNodeList::new(),
            next: None,
            prev: Weak::new(),
        }))
    }
}

// ------------------------------------------------------------------------

struct TreeNodeList {
    first: Option<TreeNodeRef>,
    last: Option<TreeNodeRef>,
}

impl TreeNodeList {
    fn new() -> Self {
        Self {
            first: None,
            last: None,
        }
    }

    fn append(&mut self, next: &TreeNodeRef) {
        if let Some(last) = &self.last {
            next.borrow_mut().prev = Rc::downgrade(&last);
            last.borrow_mut().next = Some(next.clone());
        } else {
            self.first = Some(next.clone());
            self.last = Some(next.clone());
        }
    }
}

// ------------------------------------------------------------------------

struct Tree {
    root: TreeNodeRef,
    current: TreeNodeRef,
}

fn test_main() {
    let sys_node = SysNode {
        name: "/".to_owned(),
        typ: SysNodeType::Directory,
    };

    let root = TreeNode::new(sys_node);
    let mut tree = Tree {
        root: root.clone(),
        current: root,
    };

    let sys_node = SysNode {
        name: "bin".to_owned(),
        typ: SysNodeType::Directory,
    };

    tree.root
        .borrow_mut()
        .subnodes
        .append(&TreeNode::new(sys_node));

    let sys_node = SysNode {
        name: "home".to_owned(),
        typ: SysNodeType::Directory,
    };

    tree.root
        .borrow_mut()
        .subnodes
        .append(&TreeNode::new(sys_node));

    if let Some(f) = &tree.root.borrow().subnodes.first {
        if let Some(n) = &f.borrow().next {
            println!("name={}", n.borrow().sys_node.name);
            tree.current = n.clone();
        }
    }

    println!("pwd={}", tree.current.borrow().sys_node.name);

    let mut nc: Option<TreeNodeRef> = None;
    {
        let current = tree.current.borrow();

        if let Some(prev) = &current.prev.upgrade() {
            nc = Some(prev.clone());
        }
    }

    if let Some(tn)=nc{
        tree.current=tn;
    }
    println!("pwd={}", tree.current.borrow().sys_node.name);

    let sys_node = SysNode {
        name: "config.sh".to_owned(),
        typ: SysNodeType::File,
    };
    tree.current.borrow_mut().subnodes.append(&TreeNode::new(sys_node));

    
}
