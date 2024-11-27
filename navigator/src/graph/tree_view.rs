use std::{cell::RefCell, rc::Rc};

use ncurses::KEY_DOWN;
use ncurses::KEY_UP;

use super::display::*;
use crate::common::*;
use crate::tree::*;

pub struct TreeView {
    tree: Rc<RefCell<Tree>>,
    lines: Vec<ViewLine>,
    pub modif_flags: ModifFlags,
}

impl TreeView {
    pub fn new(tree: Rc<RefCell<Tree>>) -> TreeView {
        TreeView {
            tree,
            lines: Vec::new(),
            modif_flags: ModifFlags::new(),
        }
    }

    fn list_node(&mut self, node: &TreeNodeRef, level: usize) {
        let n = node.borrow();
        if n.sys_node.typ != NodeType::Dir {
            return;
        }
        let name_as_str = n.sys_node.name.to_string_lossy().to_string();
        self.lines.push(ViewLine::new(
            &format!("{}{}", "--".repeat(level), &name_as_str),
            (2 * level) as i32,
            (2 * level + name_as_str.chars().count()) as i32,
            &node,
        ));
        for sn in &n.subnodes {
            self.list_node(sn, level + 1);
        }
    }

    fn list_tree(&mut self) {
        self.lines.clear();
        let root = &self.tree.borrow().root.clone(); // TODO: clone? - przyjrzeć się temu
        self.list_node(root, 0);
    }

    fn find_cursor(&self) -> i32 {
        let cd = self.tree.borrow().curr_dir();
        for (i, line) in self.lines.iter().enumerate() {
            if Rc::ptr_eq(&line.src_node, &cd) {
                return i as i32;
            }
        }
        -1
    }
}

impl DisplContent for TreeView {
    fn modified(&self) -> bool {
        self.modif_flags.print
    }

    fn reset_modified(&mut self) {
        self.modif_flags.print = false;
    }

    fn prepare(&mut self, info: &mut DisplInfo) -> Result<(), AppError> {
        if self.modif_flags.render {
            self.list_tree();
            self.modif_flags.render = false;
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
            None => Err(AppError::StrError("TreeView index out of range".to_owned())),
        }
    }

    fn process_key(&mut self, key: i32) -> Result<(), AppError> {
        match key {
            //KEY_DOWN => self.modif_flags = self.tree.borrow_mut().tmv_next()?,
            KEY_DOWN => {
                let tree = self.tree.clone();
                tree.borrow_mut().tv_move_next(self);
            }
            _ => {}
        };
        Ok(())
    }
}
