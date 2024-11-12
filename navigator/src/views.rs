use ncurses::*;

use crate::app_sys::*;
use crate::tree::Tree;
use std::{cell::RefCell, rc::Rc};

pub struct TreeView {
    tree: Rc<RefCell<Tree>>,
}

impl TreeView {
    pub fn new(tree: Rc<RefCell<Tree>>) -> TreeView {
        TreeView { tree }
    }
}

pub struct ListView {
    tree: Rc<RefCell<Tree>>,
}

impl ListView {
    pub fn new(tree: Rc<RefCell<Tree>>) -> ListView {
        ListView { tree }
    }
}

pub trait DisplContent {}

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
}

impl DisplContent for ListView {}

impl DisplContent for TreeView {}
