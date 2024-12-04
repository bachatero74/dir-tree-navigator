use std::{cell::RefCell, rc::Rc};

use super::display::*;
use crate::common::*;
use crate::filesystem::*;
use crate::tree::*;

use ncurses::*;

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

            let line_str = format!(
                "{}{} {:>8} {:>8} {:>10} {} {}",
                file_type_to_str(&n.sys_node.typ),
                permissions_to_str(n.sys_node.mode),
                n.sys_node.user.to_string_lossy().to_string(),
                n.sys_node.group.to_string_lossy().to_string(),
                n.sys_node.size,
                datetime_to_str(n.sys_node.modified),
                n.sys_node.name.to_string_lossy().to_string()
            );

            self.lines.push(ViewLine::new(
                &line_str,
                53,
                line_str.chars().count() as i32,
                &node,
            ));
        }
    }

    // TODO: to ma zwracać Option(i32 lub usize) i tegoż typu ma być DisplInfo::curs_line
    // a w ogóle to wywalić tą funkcję bo zbyt prosta
    fn find_cursor(&self) -> i32 {
        if let Some(cf) = self.tree.borrow().curr_file() {
            if let Some(idx) = self
                .lines
                .iter()
                .position(|vl| Rc::ptr_eq(&vl.src_node, &cf))
            {
                return idx as i32;
            }
        }
        -1
    }
}

impl DisplContent for ListView {
    fn modified(&self) -> bool {
        self.modif_flags.print
    }

    fn reset_modified(&mut self) {
        self.modif_flags.print = false;
    }

    fn prepare(&mut self, info: &mut DisplInfo) -> Result<(), AppError> {
        if self.modif_flags.render {
            self.list_curr_node();
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
            None => Err(AppError::StrError("ListView index out of range".to_owned())),
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
                        tree.borrow_mut().lv_goto(&dest, self)?;
                    }
                }
            }
            KEY_DOWN => {
                let curs_y = self.find_cursor();
                if curs_y >= 0 {
                    if let Some(line) = self.lines.get((curs_y + 1) as usize) {
                        let tree = self.tree.clone();
                        let dest = line.src_node.clone();
                        tree.borrow_mut().lv_goto(&dest, self)?;
                    }
                }
            }
            10  => {
                let tree = self.tree.clone();
                tree.borrow_mut().lv_enter(self)?;
            }
            KEY_BACKSPACE => {
                let tree = self.tree.clone();
                tree.borrow_mut().lv_move_up(self)?;
            }
            _ => {}
        };
        Ok(())
    }
}
