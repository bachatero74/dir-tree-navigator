use crate::tree::Tree;
use std::{cell::RefCell, rc::Rc};

pub struct TreeView {
    pub tree: Rc<RefCell<Tree>>,
}

pub struct ListView {
    pub tree: Rc<RefCell<Tree>>,
}

pub trait DisplContent {

}

pub struct Display{
    pub content:Rc<RefCell<dyn DisplContent>>,
}

impl DisplContent for ListView{
}

impl DisplContent for TreeView{
}