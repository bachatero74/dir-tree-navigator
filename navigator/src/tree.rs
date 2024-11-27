use std::{
    cell::RefCell,
    ffi::{OsStr, OsString},
    ops::Index,
    path::{Component, Components, Path, PathBuf},
    rc::{Rc, Weak},
};

use crate::common::AppError;
use crate::graph::list_view::ListView;
use crate::graph::tree_view::TreeView;

#[derive(PartialEq)]
pub enum NodeType {
    File,
    Dir,
    UpDir,
}
pub struct SysNode {
    pub name: OsString,
    pub typ: NodeType,
}

pub type TreeNodeRef = Rc<RefCell<TreeNode>>;
pub type TreeNodeWeak = Weak<RefCell<TreeNode>>;

// -----------------------------------------------------------------------------

pub struct TreeNode {
    pub sys_node: SysNode,
    pub subnodes: Vec<TreeNodeRef>,
    pub parent: TreeNodeWeak,
    pub loaded: bool,
}

impl TreeNode {
    pub fn new(sys_node: SysNode) -> TreeNodeRef {
        Rc::new(RefCell::new(Self {
            sys_node,
            subnodes: Vec::new(),
            parent: Weak::new(),
            loaded: false,
        }))
    }

    pub fn create(name: &OsStr, typ: NodeType) -> TreeNodeRef {
        Rc::new(RefCell::new(Self {
            sys_node: SysNode {
                name: name.to_os_string(),
                typ,
            },
            subnodes: Vec::new(),
            parent: Weak::new(),
            loaded: false,
        }))
    }

    pub fn append(this: &mut TreeNodeRef, subn: TreeNodeRef) {
        subn.borrow_mut().parent = Rc::downgrade(this);
        this.borrow_mut().subnodes.push(subn);
    }

    pub fn fill_path(&self, p: &mut PathBuf) {
        if let Some(parent) = self.parent.upgrade() {
            parent.borrow().fill_path(p);
        }
        p.push(&self.sys_node.name);
    }

    pub fn load(&mut self) {
        if !self.loaded {
            self.loaded = true;
        }
    }

    pub fn unload(&mut self) {
        self.subnodes.clear();
        self.loaded = false;
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
        let root = TreeNode::create(&OsString::from("/"), NodeType::Dir);
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

    pub fn lmv_next(&mut self) {
        let cd = self.curr_dir();
        if self.cursor.lpos < cd.borrow().subnodes.len() - 1 {
            self.cursor.lpos += 1;
        }
    }

    pub fn tv_move_next(&mut self, tv: &mut TreeView) {
        if let Some(n) = &self.cursor.node {
            if self.cursor.tpos < n.borrow().subnodes.len() - 1 {
                self.curr_dir().borrow_mut().unload();
                self.cursor.tpos += 1;
                self.cursor.lpos = 0;
                self.curr_dir().borrow_mut().load();

                if let Some(lv) = self.list_view.upgrade() {
                    lv.borrow_mut().modif_flags.render = true;
                    lv.borrow_mut().modif_flags.print = true;
                }

                tv.modif_flags.print = true;
            }
        }
    }

    // pub fn tmv_next(&mut self) -> Result<ModifFlags, AppError> {
    //     if let Some(n) = &self.cursor.node {
    //         if self.cursor.tpos < n.borrow().subnodes.len() - 1 {
    //             self.cursor.tpos += 1;
    //             self.cursor.lpos = 0;

    //             if let Some(lv) = self.list_view.upgrade() {
    //                 lv.borrow_mut().modif_flags.render = true;
    //                 lv.borrow_mut().modif_flags.print = true;
    //             }

    //             return Ok(ModifFlags {
    //                 render: false,
    //                 print: true,
    //             });
    //         }
    //     }
    //     Ok(ModifFlags {
    //         render: false,
    //         print: false,
    //     })
    // }

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

    pub fn curr_path(&self) -> PathBuf {
        let mut path = PathBuf::new();
        self.curr_dir().borrow().fill_path(&mut path);
        path
    }

    pub fn go_to_path(&mut self, path: &Path) -> Result<(), AppError> {
        self.goto(&self.find(path)?)
    }

    pub fn goto(&mut self, node: &TreeNodeRef) -> Result<(), AppError> {
        match node.borrow().parent.upgrade() {
            Some(parent) => {
                if let Some(idx) = parent
                    .borrow()
                    .subnodes
                    .iter()
                    .position(|n| Rc::ptr_eq(n, &node))
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

    pub fn find(&self, path: &Path) -> Result<TreeNodeRef, AppError> {
        let mut it = path.components();
        let oc: Option<Component> = it.next();
        match oc {
            // some component exists
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
        this_node.borrow_mut().load();
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
