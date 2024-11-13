use std::{cell::RefCell, rc::Rc};

use ncurses::KEY_DOWN;
use ncurses::KEY_UP;

use super::display::*;
use crate::common::*;
use crate::tree;
use crate::tree::*;

pub struct TreeView {
    tree: Rc<RefCell<Tree>>,
    lines: Vec<ViewLine>,
}

impl TreeView {
    pub fn new(tree: Rc<RefCell<Tree>>) -> TreeView {
        TreeView {
            tree,
            lines: Vec::new(),
        }
    }
}

impl DisplContent for TreeView {
    fn prepare(&mut self, info: &mut DisplInfo) -> Result<(), AppError> {
        self.lines.clear();
        let tree = self.tree.borrow();
        info.lines_count = tree.tmp_lines.len() as i32;
        info.curs_line = tree.tmp_cursor;
        for i in 0..info.lines_count {
            let src: &ViewLine = &tree.tmp_lines[i as usize];
            self.lines
                .push(ViewLine::new(src.content.clone(), src.x1, src.x2));
        }
        Ok(())
    }

    fn get_line(&self, y: usize) -> Result<&ViewLine, AppError> {
        match self.lines.get(y) {
            Some(line) => Ok(line),
            None => Err(AppError::StrError("TreeView index out of range".to_owned())),
        }
    }

    fn process_key(&self, key: i32) -> Result<(), AppError> {
        match key {
            KEY_UP => self.tree.borrow_mut().move_to_prev_dir(),
            KEY_DOWN => self.tree.borrow_mut().move_to_next_dir(),
            _ => Ok(()),
        }
    }
}
