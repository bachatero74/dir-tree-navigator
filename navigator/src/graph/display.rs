use std::{cell::RefCell, rc::Rc};

use crate::common::*;
use crate::filesystem::NodeType;
use crate::tree_node::*;

use ncurses::*;

#[derive(Default)]
pub struct DisplInfo {
    pub lines_count: i32,
    pub curs_line: Option<i32>, // TODO: optional usize?
    pub curs_x1: i32,
    pub curs_x2: i32,
}

pub struct ViewLine {
    pub content: String,
    pub x1: i32,
    pub x2: i32,
    pub src_node: TreeNodeRef,
}

impl ViewLine {
    pub fn new(content: &str, x1: i32, x2: i32, src_node: &TreeNodeRef) -> ViewLine {
        ViewLine {
            content: content.to_owned(),
            x1,
            x2,
            src_node: src_node.clone(),
        }
    }
}

pub trait DisplContent {
    fn prepare(&mut self, info: &mut DisplInfo) -> Result<(), AppError>;
    fn get_line(&self, y: usize) -> Result<&ViewLine, AppError>;
    fn process_key(&mut self, key: i32) -> Result<(), AppError>;
    fn modified(&self) -> bool;
    fn reset_modified(&mut self);
}

pub struct Display {
    content: Rc<RefCell<dyn DisplContent>>,
    window: WINDOW,
    size: Size,
    offset_y: i32,
    margin_y: i32,
    pub active: bool,
}

impl Display {
    pub fn new(content: Rc<RefCell<dyn DisplContent>>, window: &WINDOW, size: &Size) -> Display {
        let mut max_margin_y = (size.height - 1) / 2;
        if max_margin_y < 0 {
            max_margin_y = 0;
        }
        Display {
            content,
            window: *window,
            size: *size,
            offset_y: 0,
            margin_y: 3.clamp(0, max_margin_y),
            active: false,
        }
    }

    pub fn display(&mut self, center: bool) -> Result<(), AppError> {
        let mut info: DisplInfo = Default::default();
        if !self.content.borrow().modified() {
            return Ok(());
        }
        self.content.borrow_mut().prepare(&mut info)?;

        if let Some(curs_line) = info.curs_line {
            if !center {
                if curs_line - self.offset_y > self.size.height - 1 - self.margin_y {
                    self.offset_y = curs_line - self.size.height + 1 + self.margin_y;
                }
                if curs_line - self.offset_y < self.margin_y {
                    self.offset_y = curs_line - self.margin_y;
                }
            } else {
                self.offset_y = curs_line - self.size.height / 2;
            }
        }

        let mut mx = info.lines_count - self.size.height;
        if mx < 0 {
            mx = 0;
        }
        self.offset_y = self.offset_y.clamp(0, mx);

        let offset_x = fit_str(info.curs_x1, info.curs_x2, self.size.width);

        werase(self.window);
        for y in 0..self.size.height {
            let ln = y + self.offset_y;
            if ln >= info.lines_count {
                // TODO: nieoptymalne
                break;
            }
            let cont = self.content.borrow();
            let view_line = cont.get_line(ln as usize)?;

            let cursor = match info.curs_line {
                Some(curs_line) => (y + self.offset_y) == curs_line,
                None => false,
            };

            self.print_line(y as i32, 0, view_line, offset_x, cursor, self.active);
        }

        wrefresh(self.window);
        self.content.borrow_mut().reset_modified();
        Ok(())
    }

    pub fn process_key(&self, key: i32) -> Result<(), AppError> {
        self.content.borrow_mut().process_key(key)
    }

    fn print_line(
        &self,
        y: i32,
        x: i32,
        vline: &ViewLine,
        offs: i32,
        cursor: bool,
        container_active: bool,
    ) {
        let typ = &vline.src_node.borrow().sys_node.typ;
        let mut attributor = Attributor::new(self.window, container_active, typ);
        wmove(self.window, y, x);

        for (i, ch) in vline
            .content
            .chars()
            .enumerate()
            .skip(offs as usize)
            .take(self.size.width as usize)
        {
            if cursor {
                if i >= vline.x2 as usize {
                    attributor.sel_off();
                } else if i >= vline.x1 as usize {
                    attributor.sel_on();
                }
            }
            let ch32 = match ch {
                '├' => ACS_LTEE(),
                '└' => ACS_LLCORNER(),
                '│' => ACS_VLINE(),
                _ => ch as u32,
            };
            waddch(self.window, ch32);
        }
    }
}

fn fit_str(x1: i32, x2: i32, width: i32) -> i32 {
    (x2 - width).clamp(0, x1)
}

// -----------------------------------------------------------------------

pub struct Attributor {
    window: WINDOW,
    container_active: bool,
    color_pairs: (i16, i16),
    curr_color: Option<i16>,
    curr_reverse: bool,
}

impl Attributor {
    fn new(window: WINDOW, container_active: bool, node_type: &NodeType) -> Attributor {
        let cp = Attributor::get_color_pairs(node_type);
        let mut ret = Attributor {
            window,
            container_active,
            color_pairs: cp,
            curr_color: None,
            curr_reverse: false,
        };
        ret.set_curr_color(cp.0);
        ret
    }

    fn sel_on(&mut self) {
        if self.container_active {
            self.set_curr_reverse();
        } else {
            self.set_curr_color(self.color_pairs.1);
        }
    }

    fn sel_off(&mut self) {
        if self.container_active {
            self.reset_curr_reverse();
        } else {
            self.set_curr_color(self.color_pairs.0);
        }
    }

    fn set_curr_color(&mut self, color: i16) {
        self.reset_curr_color();
        wattron(self.window, COLOR_PAIR(color));
        self.curr_color = Some(color);
    }

    fn set_curr_reverse(&mut self) {
        if !self.curr_reverse {
            wattr_on(self.window, A_REVERSE);
            self.curr_reverse = true;
        }
    }

    fn reset_curr_reverse(&mut self) {
        if self.curr_reverse {
            wattr_off(self.window, A_REVERSE);
            self.curr_reverse = false;
        }
    }

    fn reset_curr_color(&mut self) {
        if let Some(col) = self.curr_color {
            wattroff(self.window, COLOR_PAIR(col));
            self.curr_color = None;
        }
    }

    pub fn init_color_pairs() {
        let p = Attributor::get_color_pairs(&NodeType::File);
        init_pair(p.0, COLOR_WHITE, -1);
        init_pair(p.1, COLOR_CYAN, -1);

        let p = Attributor::get_color_pairs(&NodeType::Dir);
        init_pair(p.0, COLOR_BLUE, -1);
        init_pair(p.1, COLOR_CYAN, -1);
    }

    fn get_color_pairs(node_type: &NodeType) -> (i16, i16) {
        match node_type {
            NodeType::Dir => (3, 4),
            _ => (1, 2),
        }
    }
}

impl Drop for Attributor {
    fn drop(&mut self) {
        self.reset_curr_color();
        self.reset_curr_reverse();
    }
}
