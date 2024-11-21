use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::common::AppError;
use crate::graph::display::*; // Tymczasowe
use crate::graph::list_view::ListView;
use crate::graph::tree_view::TreeView;

pub struct Node {
    pub name: String,
}

pub struct FileNode {
    pub node: Node,
}

pub struct DirNode {
    pub node: Node,
}

pub enum TreeNode {
    File(FileNode),
    Dir(DirNode),
    UpDir,
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

pub struct Tree {
    pub tree_view: Weak<RefCell<TreeView>>,
    pub list_view: Weak<RefCell<ListView>>,
    pub tmp_lines: Vec<ViewLine>,
    pub tmp_cursor: i32,

    pub root: Rc<RefCell<DirNode>>,
    curr_dir: Rc<RefCell<DirNode>>,
}

impl Tree {
    pub fn new() -> Tree {
        let root = Rc::new(RefCell::new(DirNode {
            node: Node {
                name: "/".to_owned(),
            },
        }));
        Tree {
            root: root.clone(),
            curr_dir: root.clone(),
            tree_view: Weak::new(),
            list_view: Weak::new(),
            tmp_cursor: 0,
            tmp_lines: vec![
                ViewLine::new("-+---mnt".to_owned(), 5, 8),
                ViewLine::new(" +---home".to_owned(), 5, 9),
                ViewLine::new(" +---cdrom".to_owned(), 5, 10),
                ViewLine::new(" |  +----development12".to_owned(), 9, 20),
                ViewLine::new(" +---proc".to_owned(), 5, 9),
                ViewLine::new("-+---abcde".to_owned(), 5, 10),
                ViewLine::new(" +---a1cde".to_owned(), 5, 10),
                ViewLine::new(" +---a2cde".to_owned(), 5, 10),
                ViewLine::new(" |  +----development12".to_owned(), 9, 20),
                ViewLine::new(" +---abcde".to_owned(), 5, 10),
                ViewLine::new("-+---abcde".to_owned(), 5, 10),
                ViewLine::new(" +---a1cde".to_owned(), 5, 10),
                ViewLine::new(" +---a2cde".to_owned(), 5, 10),
                ViewLine::new(" +---abcde".to_owned(), 5, 10),
                ViewLine::new("-+---abcde".to_owned(), 5, 10),
                ViewLine::new(" +---a1cde".to_owned(), 5, 10),
                ViewLine::new(" +---a2cde".to_owned(), 5, 10),
                ViewLine::new(" +---abcde".to_owned(), 5, 10),
                ViewLine::new("-+---abcde".to_owned(), 5, 10),
                ViewLine::new(" +---a1cde".to_owned(), 5, 10),
                ViewLine::new(" +---a2cde".to_owned(), 5, 10),
                ViewLine::new(" |  +----development12".to_owned(), 9, 20),
                ViewLine::new(" +---abcde".to_owned(), 5, 10),
                ViewLine::new("-+---abcde".to_owned(), 5, 10),
                ViewLine::new(" +---a1cde".to_owned(), 5, 10),
                ViewLine::new(" +---a2cde".to_owned(), 5, 10),
                ViewLine::new(" |  +----development12".to_owned(), 9, 20),
                ViewLine::new(" +---abcde".to_owned(), 5, 10),
            ],
        }
    }

    pub fn move_to_prev_dir(&mut self) -> Result<(ModifFlags), AppError> {
        if self.tmp_cursor > 0 {
            self.tmp_cursor -= 1;
        }
        Ok(ModifFlags::from(false, true))
    }

    pub fn move_to_next_dir(&mut self) -> Result<(ModifFlags), AppError> {
        if self.tmp_cursor < self.tmp_lines.len() as i32 - 1 {
            self.tmp_cursor += 1;
        }
        if let Some(lv) = self.list_view.upgrade() {
            lv.borrow_mut().modif_flags = ModifFlags::from(true, true);
            return Ok(ModifFlags::from(false, true));
        }
        Err(AppError::StrError("".to_owned()))
    }
}
