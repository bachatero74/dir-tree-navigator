use std::{cell::RefCell, rc::Rc};

use super::display::*;
use crate::common::*;
use crate::tree::*;

pub struct ListView {
    tree: Rc<RefCell<Tree>>,
    lines: Vec<ViewLine>,
    pub modif_flags: ModifFlags,
}

impl ListView {
    pub fn new(tree: Rc<RefCell<Tree>>) -> ListView {
        ListView {
            tree,
            lines: Vec::new(),
            modif_flags: ModifFlags::new(),
        }
    }

    fn list_curr_node(&mut self) {
        self.lines.clear();
        let cd = self.tree.borrow().curr_dir();
        for node in &cd.borrow().subnodes {
            let n = node.borrow();
            self.lines.push(ViewLine::new(
                &n.sys_node.name.to_string_lossy().to_string(),
                0,
                n.sys_node.name.len() as i32,
                &node,
            ));
        }
    }

    fn find_cursor(&self) -> i32 {
        if let Some(cf) = self.tree.borrow().curr_file() {
            for (i, line) in self.lines.iter().enumerate() {
                if Rc::ptr_eq(&line.src_node, &cf) {
                    return i as i32;
                }
            }
        }
        -1
    }
}

impl DisplContent for ListView {
    fn modified(&self) -> bool {
        self.modif_flags.print
    }

    fn prepare(&mut self, info: &mut DisplInfo) -> Result<(), AppError> {
        if self.modif_flags.render {
            self.list_curr_node();
        }
        info.lines_count = self.lines.len() as i32;
        info.curs_line = self.find_cursor();
        info.curs_x1 = 0;
        info.curs_x2 = 0;
        Ok(())
    }

    fn get_line(&self, y: usize) -> Result<&ViewLine, AppError> {
        match self.lines.get(y) {
            Some(line) => Ok(line),
            None => Err(AppError::StrError("ListView index out of range".to_owned())),
        }
    }

    fn process_key(&mut self, key: i32) -> Result<(), AppError> {
        Ok(())
    }
}
