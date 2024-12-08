use std::{
    cell::RefCell,
    ffi::OsString,
    path::{Component, Components, Path, PathBuf},
    rc::{Rc, Weak},
};

use crate::graph::{list_view::ListView, tree_view::TreeView};
use crate::{common::*, filesystem::*, tree_node::*};

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
        let _ = TreeNode::load(&root); // Error ignored
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

    /* #region Navigation */

    fn goto(&mut self, node: &TreeNodeRef) -> Result<(), AppError> {
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

    fn move_from_to(&mut self, prev: &TreeNodeRef, next: &TreeNodeRef) -> Result<bool, AppError> {
        let ul = TreeNode::try_unload(prev, next);
        self.goto(next)?;
        Ok(ul)
    }

    pub fn go_to_path(&mut self, path: &Path) -> Result<(), AppError> {
        let mut node = self.find(path)?;
        TreeNode::expand(&mut node);
        self.goto(&node)
    }

    fn move_to_list_node(&mut self, node: &TreeNodeRef) -> Result<(), AppError> {
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

    /* #endregion */

    /* #region Current Pos */

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

    pub fn curr_path(&self) -> PathBuf {
        self.curr_dir().borrow().get_path()
    }

    /* #endregion */

    /* #region TreeView Operations */
    pub fn tv_goto(&mut self, node: &TreeNodeRef, tv: &mut TreeView) -> Result<(), AppError> {
        let old_cd = self.curr_dir();
        let ul = self.move_from_to(&old_cd, node)?;

        let _ = TreeNode::load(node); // Error ignored

        if let Some(lv) = self.list_view.upgrade() {
            lv.borrow_mut().modif_flags.render = true;
            lv.borrow_mut().modif_flags.print = true;
        }
        tv.modif_flags.render = ul;
        tv.modif_flags.print = true;
        Ok(())
    }

    pub fn tv_move_up(&mut self, tv: &mut TreeView) -> Result<(), AppError> {
        let cd = self.curr_dir();
        let parent = cd.borrow().parent.upgrade();
        if let Some(parent) = parent {
            let ul = self.move_from_to(&cd, &parent)?;
            tv.modif_flags.render = ul;
            tv.modif_flags.print = true;
            if let Some(lv) = self.list_view.upgrade() {
                lv.borrow_mut().modif_flags.render = true;
                lv.borrow_mut().modif_flags.print = true;
            }
        }
        Ok(())
    }

    pub fn tv_expand(&mut self, b: bool, tv: &mut TreeView) -> Result<(), AppError> {
        let cd = self.curr_dir();
        if b {
            // expand
            if !cd.borrow().expanded {
                cd.borrow_mut().expanded = true;
                tv.modif_flags.render = true;
                tv.modif_flags.print = true;
            }
        } else {
            // collapse
            if cd.borrow().expanded {
                cd.borrow_mut().expanded = false;
                for sn in &cd.borrow().subnodes {
                    sn.borrow_mut().unload();
                }
                tv.modif_flags.render = true;
                tv.modif_flags.print = true;
            } else {
                // already collapsed
                let parent = cd.borrow().parent.upgrade();
                if let Some(parent) = parent {
                    let ul = self.move_from_to(&cd, &parent)?;
                    tv.modif_flags.render = ul;
                    tv.modif_flags.print = true;
                    if let Some(lv) = self.list_view.upgrade() {
                        lv.borrow_mut().modif_flags.render = true;
                        lv.borrow_mut().modif_flags.print = true;
                    }
                }
            }
        }
        Ok(())
    }
    /* #endregion */

    /* #region ListView Operations */

    pub fn lv_goto(&mut self, node: &TreeNodeRef, lv: &mut ListView) -> Result<(), AppError> {
        self.move_to_list_node(node)?;
        lv.modif_flags.print = true;
        Ok(())
    }

    pub fn lv_enter(&mut self, lv: &mut ListView) -> Result<(), AppError> {
        if let Some(file) = self.curr_file() {
            if file.borrow().sys_node.typ == NodeType::Dir {
                let cd = self.curr_dir();
                cd.borrow_mut().expanded = true;
                let _ul = self.move_from_to(&cd, &file)?;
                let _ = TreeNode::load(&file); // Error ignored
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
        let parent = cd.borrow().parent.upgrade();
        if let Some(parent) = parent {
            let ul = self.move_from_to(&cd, &parent)?;
            lv.modif_flags.render = true;
            lv.modif_flags.print = true;
            if let Some(tv) = self.tree_view.upgrade() {
                tv.borrow_mut().modif_flags.render = ul;
                tv.borrow_mut().modif_flags.print = true;
            }
        }
        Ok(())
    }
    /* #endregion */

    /* #region Searching */

    fn find(&self, path: &Path) -> Result<TreeNodeRef, AppError> {
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
        let _ = TreeNode::load(this_node); // Error ignored
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
    /* #endregion */
}
