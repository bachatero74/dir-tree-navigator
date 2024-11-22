use std::{cell::RefCell, rc::Rc};

use crate::common::*;
use ncurses::*;

#[derive(Default)]
pub struct DisplInfo {
    pub lines_count: i32,
    pub curs_line: i32,
    pub curs_x1: i32,
    pub curs_x2: i32,
}

pub struct ViewLine {
    pub content: String,
    pub x1: i32,
    pub x2: i32,
}

impl ViewLine {
    pub fn new(content: &str, x1: i32, x2: i32) -> ViewLine {
        ViewLine {
            content: content.to_owned(),
            x1,
            x2,
        }
    }
}

pub trait DisplContent {
    fn prepare(&mut self, info: &mut DisplInfo) -> Result<(), AppError>;
    fn get_line(&self, y: usize) -> Result<&ViewLine, AppError>;
    fn process_key(&mut self, key: i32) -> Result<(), AppError>;
    fn modified(&self) -> bool;
}

pub struct Display {
    content: Rc<RefCell<dyn DisplContent>>,
    window: WINDOW,
    size: Size,
    //offset_x: i32,
    offset_y: i32,
}

impl Display {
    pub fn new(content: Rc<RefCell<dyn DisplContent>>, window: &WINDOW, size: &Size) -> Display {
        Display {
            content,
            window: *window,
            size: *size,
            //offset_x: 0,
            offset_y: 0,
        }
    }

    pub fn display(&mut self) -> Result<(), AppError> {
        let mut info: DisplInfo = Default::default();
        if !self.content.borrow().modified() {
            return Ok(());
        }
        self.content.borrow_mut().prepare(&mut info)?;

        if info.curs_line - self.offset_y > self.size.height - 1 {
            self.offset_y = info.curs_line - self.size.height + 1;
        }

        if info.curs_line - self.offset_y < 0 {
            self.offset_y = info.curs_line;
        }

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
            self.print_line(
                y as i32,
                0,
                view_line,
                offset_x,
                y + self.offset_y == info.curs_line,
            );
        }

        wrefresh(self.window);
        Ok(())
    }

    pub fn process_key(&self, key: i32) -> Result<(), AppError> {
        self.content.borrow_mut().process_key(key)
    }

    fn print_line(&self, y: i32, x: i32, vline: &ViewLine, offs: i32, cursor: bool) {
        wattr_off(self.window, A_REVERSE);
        wmove(self.window, y, x);
        for (i, ch) in vline.content.chars().enumerate() {
            if i < offs as usize {
                continue;
            }
            if i - (offs as usize) >= self.size.width as usize {
                break;
            }
            if cursor {
                if i >= vline.x2 as usize {
                    wattr_off(self.window, A_REVERSE);
                } else if i >= vline.x1 as usize {
                    wattr_on(self.window, A_REVERSE);
                }
            }
            waddch(self.window, ch as u32);
        }
        wattr_off(self.window, A_REVERSE);
    }
}

fn fit_str(x1: i32, x2: i32, width: i32) -> i32 {
    (x2 - width).clamp(0, x1)
}
