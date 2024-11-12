use std::{cell::RefCell, rc::Rc};

use crate::common::*;
use super::display::*;
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
        for i in 0..15 {
            self.lines.push(ViewLine {
                content: i.to_string(),
            });
        }
        info.lines_count = 15;
        Ok(())
    }

    fn get_line(&self, y: usize) -> Result<&ViewLine, AppError> {
        match self.lines.get(y) {
            Some(line) => Ok(line),
            None => Err(AppError::StrError("TreeView index out of range".to_owned())),
        }
    }
}