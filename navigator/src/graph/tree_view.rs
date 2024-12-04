use std::{cell::RefCell, rc::Rc};

use ncurses::KEY_BACKSPACE;
use ncurses::KEY_DOWN;
use ncurses::KEY_LEFT;
use ncurses::KEY_RIGHT;
use ncurses::KEY_UP;

use super::display::*;
use crate::{common::*, filesystem::NodeType, tree::*, tree_node::*};

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
        let exp_stat = if node.borrow().expanded { "-" } else { "+" };
        self.lines.push(ViewLine::new(
            &format!("{}{}{}", " ".repeat(level), exp_stat, &name_as_str),
            (1 * level + 1) as i32,
            (1 * level + 1 + name_as_str.chars().count()) as i32,
            &node,
        ));
        if n.expanded {
            for sn in &n.subnodes {
                self.list_node(sn, level + 1);
            }
        }
    }

    fn list_tree(&mut self) {
        self.lines.clear();
        let root = &self.tree.borrow().root.clone(); // TODO: clone? - przyjrzeć się temu
        self.list_node(root, 0);
    }

    // TODO: to ma zwracać Option(i32 lub usize) i tegoż typu ma być DisplInfo::curs_line
    // a w ogóle to wywalić tą funkcję bo zbyt prosta
    fn find_cursor(&self) -> i32 {
        let cd = self.tree.borrow().curr_dir();
        if let Some(idx) = self
            .lines
            .iter()
            .position(|vl| Rc::ptr_eq(&vl.src_node, &cd))
        {
            return idx as i32;
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
        match self.lines.get(info.curs_line as usize) {
            Some(ln) => {
                info.curs_x1 = ln.x1;
                info.curs_x2 = ln.x2;
            }
            None => {
                info.curs_x1 = 0;
                info.curs_x2 = 0;
            }
        }
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
            KEY_UP => {
                let curs_y = self.find_cursor();
                if curs_y >= 0 {
                    if let Some(line) = self.lines.get((curs_y - 1) as usize) {
                        let tree = self.tree.clone();
                        let dest = line.src_node.clone();
                        tree.borrow_mut().tv_goto(&dest, self)?;
                    }
                }
            }
            KEY_DOWN => {
                let curs_y = self.find_cursor();
                if curs_y >= 0 {
                    if let Some(line) = self.lines.get((curs_y + 1) as usize) {
                        let tree = self.tree.clone();
                        let dest = line.src_node.clone();
                        tree.borrow_mut().tv_goto(&dest, self)?;
                    }
                }
            }
            KEY_RIGHT => {
                let tree = self.tree.clone();
                tree.borrow_mut().tv_expand(true, self);
            }
            KEY_LEFT => {
                let tree = self.tree.clone();
                tree.borrow_mut().tv_expand(false, self);
            }
            KEY_BACKSPACE => {
                let tree = self.tree.clone();
                tree.borrow_mut().tv_move_up(self)?;
            }
            _ => {}
        };
        Ok(())
    }
}
