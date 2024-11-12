use ncurses::*;

use crate::app_sys::*;
use crate::tree::Tree;
use std::{cell::RefCell, rc::Rc};

#[derive(Default)]
struct DisplInfo {
    lines_count: i32,
    curs_line: i32,
    curs_x1: i32,
    curs_x2: i32,
}

pub struct ViewLine {
    pub content: String,
}

pub trait DisplContent {
    fn prepare(&mut self, info: &mut DisplInfo) -> Result<(), AppError>;
    fn get_line(&self, y: usize) -> Result<&ViewLine, AppError>;
}

pub struct Display {
    content: Box<dyn DisplContent>,
    window: WINDOW,
    size: Size,
    offset_x: i32,
    offset_y: i32,
}

impl Display {
    pub fn new(content: Box<dyn DisplContent>, window: &WINDOW, size: &Size) -> Display {
        Display {
            content,
            window: *window,
            size: *size,
            offset_x: 0,
            offset_y: 0,
        }
    }

    pub fn display(&mut self) -> Result<(), AppError> {
        let mut info: DisplInfo = Default::default();
        self.content.prepare(&mut info)?;

        let l_cnt = std::cmp::min(info.lines_count, self.size.height);

        for y in 0..l_cnt {
            let v_line = self.content.get_line(y as usize)?;
            mvwprintw(self.window, y, 0, &v_line.content);
        }

        wrefresh(self.window);
        Ok(())
    }
}

// ------ TreeView -------------------------------------

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

// ------ ListView -------------------------------------

pub struct ListView {
    tree: Rc<RefCell<Tree>>,
    test_line: String,
}

impl ListView {
    pub fn new(tree: Rc<RefCell<Tree>>) -> ListView {
        ListView {
            tree,
            test_line: "ListView".to_owned(),
        }
    }
}

impl DisplContent for ListView {
    fn prepare(&mut self, info: &mut DisplInfo) -> Result<(), AppError> {
        //todo!()
        Ok(())
    }

    fn get_line(&self, y: usize) -> Result<&ViewLine, AppError> {
        todo!()
    }
}
