use ncurses::*;

use crate::app_sys::*;
use crate::tree::Tree;
use std::{cell::RefCell, rc::Rc};

#[derive(Default)]
struct DisplInfo {
    lines_count: i32,
}

pub trait DisplContent {
    fn prepare(&self, info: &mut DisplInfo);
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

        mvwprintw(
            self.window,
            3,
            3,
            &format!("{} x {}", self.size.width, self.size.height),
        );

        wrefresh(self.window);
    }
}

// ------ TreeView

pub struct TreeView {
    tree: Rc<RefCell<Tree>>,
}

impl TreeView {
    pub fn new(tree: Rc<RefCell<Tree>>) -> TreeView {
        TreeView { tree }
    }
}

impl DisplContent for TreeView {
    fn prepare(&self, info: &mut DisplInfo) {
        //todo!()
    }
}

// ------ ListView

pub struct ListView {
    tree: Rc<RefCell<Tree>>,
}

impl ListView {
    pub fn new(tree: Rc<RefCell<Tree>>) -> ListView {
        ListView { tree }
    }
}

impl DisplContent for ListView {
    fn prepare(&self, info: &mut DisplInfo) {
        //todo!()
    }
}
