use std::{cell::RefCell, rc::Rc};

use super::display::*;
use crate::common::*;
use crate::tree::*;

pub struct ListView {
    tree: Rc<RefCell<Tree>>,
    test_line: String,
    pub modif_flags: ModifFlags,
}

impl ListView {
    pub fn new(tree: Rc<RefCell<Tree>>) -> ListView {
        ListView {
            tree,
            test_line: "ListView".to_owned(),
            modif_flags: ModifFlags::new(),
        }
    }
}

impl DisplContent for ListView {
    fn modified(&self) -> bool {
        self.modif_flags.print
    }

    fn prepare(&mut self, info: &mut DisplInfo) -> Result<(), AppError> {
        //todo!()
        Ok(())
    }

    fn get_line(&self, y: usize) -> Result<&ViewLine, AppError> {
        todo!()
    }

    fn process_key(&mut self, key: i32) -> Result<(), AppError> {
        Ok(())
    }
}
