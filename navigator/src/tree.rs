use std::{
    cell::RefCell,
    ffi::{OsStr, OsString},
    fs::{self},
    path::{Component, Components, Path, PathBuf},
    rc::{Rc, Weak},
};

use crate::{common::*, filesystem::*};

use crate::graph::list_view::ListView;
use crate::graph::tree_view::TreeView;

pub type TreeNodeRef = Rc<RefCell<TreeNode>>;
pub type TreeNodeWeak = Weak<RefCell<TreeNode>>;

// -----------------------------------------------------------------------------

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

    pub fn is_child_of(parent: &TreeNodeRef, child: &TreeNodeRef) -> bool {
        if let Some(p) = child.borrow().parent.upgrade() {
            if Rc::ptr_eq(&p, parent) {
                return true;
            }
            return TreeNode::is_child_of(parent, &p);
        }
        return false;
    }

    pub fn unload(&mut self) {
        self.subnodes.clear();
        self.loaded = false;
    }

    pub fn expand(this: &mut TreeNodeRef) {
        if let Some(parent) = this.borrow().parent.upgrade() {
            let mut p = parent.clone();
            TreeNode::expand(&mut p);
        }
        this.borrow_mut().expanded = true;
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
        let root = TreeNode::from(SysNode::new(&OsString::from("/"), NodeType::Dir));
        root.borrow_mut().expanded = true;
        TreeNode::load(&root);
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
            if let Some(sn) = nb.subnodes.get(self.cursor.tpos) {
                // TODO: z tym błędem zrobić coś sensownego
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

    pub fn tv_goto(&mut self, node: &TreeNodeRef, tv: &mut TreeView) -> Result<(), AppError> {
        let old_cd = self.curr_dir();
        self.goto(node)?;

        TreeNode::load(node); // <--------------------------------------- !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

        if let Some(lv) = self.list_view.upgrade() {
            lv.borrow_mut().modif_flags.render = true;
            lv.borrow_mut().modif_flags.print = true;
        }
        tv.modif_flags.print = true;
        Ok(())
    }

    pub fn tv_move_up(&mut self, tv: &mut TreeView) -> Result<(), AppError> {
        let cd = self.curr_dir();
        if let Some(parent) = cd.borrow().parent.upgrade() {
            self.goto(&parent)?;
            tv.modif_flags.print = true;
            if let Some(lv) = self.list_view.upgrade() {
                lv.borrow_mut().modif_flags.render = true;
                lv.borrow_mut().modif_flags.print = true;
            }
        }
        Ok(())
    }

    pub fn tv_expand(&mut self, b: bool, tv: &mut TreeView) {
        let cd = self.curr_dir();
        let mut rcd = cd.borrow_mut();
        if b {
            // expand
            if !rcd.expanded {
                rcd.expanded = true;
                tv.modif_flags.render = true;
                tv.modif_flags.print = true;
            }
        } else {
            // collapse
            if rcd.expanded {
                rcd.expanded = false;
                tv.modif_flags.render = true;
                tv.modif_flags.print = true;
            } else {
                // already collapsed
                if let Some(parent) = rcd.parent.upgrade() {
                    //rcd.unload();
                    self.goto(&parent);
                    //parent.borrow_mut().expanded = false;
                    tv.modif_flags.render = false;
                    tv.modif_flags.print = true;
                    if let Some(lv) = self.list_view.upgrade() {
                        lv.borrow_mut().modif_flags.render = true;
                        lv.borrow_mut().modif_flags.print = true;
                    }
                }
            }
        }
    }

    pub fn lv_goto(&mut self, node: &TreeNodeRef, lv: &mut ListView) -> Result<(), AppError> {
        self.move_to_list_node(node)?;
        // if let Some(tv) = self.tree_view.upgrade() {
        //     tv.borrow_mut().modif_flags.render = false;
        //     tv.borrow_mut().modif_flags.print = false;
        // }
        lv.modif_flags.print = true;
        Ok(())
    }

    pub fn lv_enter(&mut self, lv: &mut ListView) -> Result<(), AppError> {
        if let Some(file) = self.curr_file() {
            if file.borrow().sys_node.typ == NodeType::Dir {
                let cd = self.curr_dir();
                cd.borrow_mut().expanded = true;
                self.goto(&file)?;
                TreeNode::load(&file);
                if let Some(tv) = self.tree_view.upgrade() {
                    tv.borrow_mut().modif_flags.render = true;
                    tv.borrow_mut().modif_flags.print = true;
                }
                lv.modif_flags.render = true;
                lv.modif_flags.print = true;
            }
        }
        Ok(())
    }

    pub fn lv_move_up(&mut self, lv: &mut ListView) -> Result<(), AppError> {
        let cd = self.curr_dir();
        if let Some(parent) = cd.borrow().parent.upgrade() {
            self.goto(&parent)?;
            lv.modif_flags.render = true;
            lv.modif_flags.print = true;
            if let Some(tv) = self.tree_view.upgrade() {
                tv.borrow_mut().modif_flags.render = false;
                tv.borrow_mut().modif_flags.print = true;
            }
        }
        Ok(())
    }

    pub fn curr_path(&self) -> PathBuf {
        self.curr_dir().borrow().get_path()
    }

    pub fn go_to_path(&mut self, path: &Path) -> Result<(), AppError> {
        let mut node = self.find(path)?;
        TreeNode::expand(&mut node);
        self.goto(&node)
    }

    pub fn goto(&mut self, node: &TreeNodeRef) -> Result<(), AppError> {
        match node.borrow().parent.upgrade() {
            Some(parent) => {
                if let Some(idx) = parent
                    .borrow()
                    .subnodes
                    .iter()
                    .position(|n| Rc::ptr_eq(n, node))
                {
                    self.cursor.tpos = idx;
                    self.cursor.lpos = 0;
                } else {
                    return Err(AppError::StrError("internal goto error".to_owned()));
                }
                self.cursor.node = Some(parent);
            }
            None => {
                self.cursor.node = None;
                self.cursor.tpos = 0;
                self.cursor.lpos = 0;
            }
        }
        Ok(())
    }

    pub fn move_to_list_node(&mut self, node: &TreeNodeRef) -> Result<(), AppError> {
        let cd = self.curr_dir();
        if let Some(idx) = cd
            .borrow()
            .subnodes
            .iter()
            .position(|n| Rc::ptr_eq(n, node))
        {
            self.cursor.lpos = idx;
        } else {
        }
        Ok(())
    }

    pub fn find(&self, path: &Path) -> Result<TreeNodeRef, AppError> {
        let mut it = path.components();
        let oc: Option<Component> = it.next();
        match oc {
            // some component exist
            Some(c) => match c {
                std::path::Component::RootDir => return Tree::inner_find(&self.root, &mut it),
                _ => {
                    return Err(AppError::PathError(
                        "absolute path expected".to_owned(),
                        path.to_string_lossy().to_string(),
                    ));
                }
            },
            None => {
                return Err(AppError::PathError("empty path".to_owned(), "".to_owned()));
            }
        }
    }

    fn inner_find(this_node: &TreeNodeRef, it: &mut Components) -> Result<TreeNodeRef, AppError> {
        TreeNode::load(this_node)?;
        let oc = it.next();
        if let Some(c) = oc {
            match this_node
                .borrow()
                .subnodes
                .iter()
                .find(|sn| sn.borrow().sys_node.name == c.as_os_str())
            {
                None => {
                    return Err(AppError::PathError(
                        "path not found".to_owned(),
                        c.as_os_str().to_string_lossy().to_string(),
                    ));
                }
                Some(subnode) => {
                    if subnode.borrow().sys_node.typ != NodeType::Dir {
                        return Err(AppError::PathError(
                            "not a directory".to_owned(),
                            c.as_os_str().to_string_lossy().to_string(),
                        ));
                    }
                    return Tree::inner_find(subnode, it);
                }
            }
        }

        Ok(this_node.clone())
    }
}
