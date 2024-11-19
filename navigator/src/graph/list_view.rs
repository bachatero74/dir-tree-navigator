use std::{cell::RefCell, rc::Rc};

use super::display::*;
use crate::common::*;
use crate::tree::*;

pub struct ListView {
    tree: Rc<RefCell<Tree>>,
    test_line: String,
    needs_render: bool,
    modified: bool,
}

impl ListView {
    pub fn new(tree: Rc<RefCell<Tree>>) -> ListView {
        ListView {
            tree,
            test_line: "ListView".to_owned(),
            needs_render: true,
            modified: true,
        }
    }

    pub fn set_flags(&mut self, needs_render: bool, modified: bool) {
        self.needs_render = needs_render;
        self.modified = modified;
    }
}

impl DisplContent for ListView {
    fn modified(&self) -> bool {
        self.modified || self.needs_render
    }

    fn prepare(&mut self, info: &mut DisplInfo) -> Result<(), AppError> {
        //todo!()
        Ok(())
    }

    fn get_line(&self, y: usize) -> Result<&ViewLine, AppError> {
        todo!()
    }

    fn process_key(&self, key: i32) -> Result<(), AppError> {
        Ok(())
    }
}
