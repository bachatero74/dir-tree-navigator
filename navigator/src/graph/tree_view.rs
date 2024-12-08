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

    fn list_node(&mut self, node: &TreeNodeRef, prevs_stack: &mut Vec<bool>, tbc: Option<bool>) {
        let n = node.borrow();
        if n.sys_node.typ != NodeType::Dir {
            return;
        }
        let lead: String = prevs_stack
            .iter()
            .map(|b| if *b { "│" } else { " " })
            .collect();

        let link = match tbc {
            Some(tbc) => match tbc {
                true => "├",
                false => "└",
            },
            None => "",
        };
        let link_len = link.chars().count();
        let exp_stat = if node.borrow().expanded { "-" } else { "+" };
        let name_as_str = n.sys_node.name.to_string_lossy().to_string();
        let s = &format!("{}{}{}{}", lead, link, exp_stat, &name_as_str);
        let vline = ViewLine::new(
            s,
            (prevs_stack.len() + link_len + 1) as i32,
            (prevs_stack.len() + link_len + 1 + name_as_str.chars().count()) as i32,
            &node,
        );
        self.lines.push(vline);
        if n.expanded {
            let subnodes: Vec<_> = n
                .subnodes
                .iter()
                .filter(|sn| sn.borrow().sys_node.typ == NodeType::Dir)
                .collect();
            if let Some(tbc) = tbc {
                prevs_stack.push(tbc);
            }
            let len = subnodes.len();
            for (i, sn) in subnodes.iter().enumerate() {
                self.list_node(sn, prevs_stack, Some(i < len - 1));
            }
            if let Some(_) = tbc {
                prevs_stack.pop();
            }
        }
    }

    fn list_tree(&mut self) {
        self.lines.clear();
        let root = &self.tree.borrow().root.clone(); // TODO: clone? - przyjrzeć się temu
        let mut prevs_stack: Vec<bool> = Vec::new();
        self.list_node(root, &mut prevs_stack, None);
    }

    // TODO: to ma zwracać Option(i32 lub usize) i tegoż typu ma być DisplInfo::curs_line
    fn find_cursor(&self) -> Option<i32> {
        let cd = self.tree.borrow().curr_dir();
        if let Some(idx) = self
            .lines
            .iter()
            .position(|vl| Rc::ptr_eq(&vl.src_node, &cd))
        {
            return Some(idx as i32);
        }
        None
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
        match info.curs_line {
            Some(curs_line) => match self.lines.get(curs_line as usize) {
                Some(ln) => {
                    info.curs_x1 = ln.x1;
                    info.curs_x2 = ln.x2;
                }
                None => {
                    info.curs_x1 = 0;
                    info.curs_x2 = 0;
                }
            },
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
                if let Some(curs_y) = self.find_cursor() {
                    if let Some(line) = self.lines.get((curs_y - 1) as usize) {
                        let tree = self.tree.clone();
                        let dest = line.src_node.clone();
                        tree.borrow_mut().tv_goto(&dest, self)?;
                    }
                }
            }
            KEY_DOWN => {
                if let Some(curs_y) = self.find_cursor() {
                    if let Some(line) = self.lines.get((curs_y + 1) as usize) {
                        let tree = self.tree.clone();
                        let dest = line.src_node.clone();
                        tree.borrow_mut().tv_goto(&dest, self)?;
                    }
                }
            }
            KEY_RIGHT => {
                let tree = self.tree.clone();
                tree.borrow_mut().tv_expand(true, self)?;
            }
            KEY_LEFT => {
                let tree = self.tree.clone();
                tree.borrow_mut().tv_expand(false, self)?;
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
