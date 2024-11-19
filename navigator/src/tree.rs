use std::rc::Weak;

use crate::common::AppError;
use crate::graph::display::*; // Tymczasowe
use crate::graph::tree_view::TreeView;

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
    pub tmp_cursor: i32,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
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

    pub fn move_to_prev_dir(&mut self) -> Result<(), AppError> {
        if self.tmp_cursor > 0 {
            self.tmp_cursor -= 1;
        }
        Ok(())
    }

    pub fn move_to_next_dir(&mut self) -> Result<(), AppError> {
        if self.tmp_cursor < self.tmp_lines.len() as i32 - 1 {
            self.tmp_cursor += 1;
        }
        Ok(())
    }
}
