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

pub trait DisplContent {
    fn prepare(&self, info: &mut DisplInfo);
    fn get_line(&self, y: i32) -> &str;
}

pub struct Display {
    content: Box<dyn DisplContent>,
    window: WINDOW,
    size: Size,
}

impl Display {
    pub fn new(content: Box<dyn DisplContent>, window: &WINDOW, size: &Size) -> Display {
        Display {
            content,
            window: *window,
            size: *size,
        }
    }

    pub fn display(&self) {
        let mut info: DisplInfo = Default::default();
        self.content.prepare(&mut info);

        mvwprintw(self.window, 3, 3, self.content.get_line(0));

        wrefresh(self.window);
    }
}

// ------ TreeView

pub struct TreeView {
    tree: Rc<RefCell<Tree>>,
    test_line: String,
}

impl TreeView {
    pub fn new(tree: Rc<RefCell<Tree>>) -> TreeView {
        TreeView {
            tree,
            test_line: "TreeView".to_owned(),
        }
    }
}

impl DisplContent for TreeView {
    fn prepare(&self, info: &mut DisplInfo) {
        //todo!()
    }

    fn get_line(&self, y: i32) -> &str {
        &self.test_line
    }
}

// ------ ListView

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
    fn prepare(&self, info: &mut DisplInfo) {
        //todo!()
    }

    fn get_line(&self, y: i32) -> &str {
        &self.test_line
    }
}
