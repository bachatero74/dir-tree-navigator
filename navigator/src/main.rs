mod common;
mod screen;
mod tree;
mod filesystem;
mod graph {
    pub mod display;
    pub mod list_view;
    pub mod tree_view;
}

use std::process::ExitCode;
use std::{cell::RefCell, rc::Rc};

use common::*;
use graph::{display::*, list_view::*, tree_view::*};
use ncurses::*;
use screen::*;
use tree::*;

fn run(screen: &Screen) -> Result<(), AppError> {
    let tree = Rc::new(RefCell::new(Tree::new()));

    {
        let mut tree = tree.borrow_mut();
        let mut etc = TreeNode::create("etc", NodeType::Dir);
        TreeNode::append(&mut tree.root, etc.clone());

        let mut fstab = TreeNode::create("fstab", NodeType::File);
        TreeNode::append(&mut etc, fstab.clone());

        let mut mtab = TreeNode::create("mtab", NodeType::File);
        TreeNode::append(&mut etc, mtab.clone());

        let mut mnt = TreeNode::create("mnt", NodeType::Dir);
        TreeNode::append(&mut tree.root, mnt.clone());

        let mut cdrom = TreeNode::create("cdrom", NodeType::File);
        TreeNode::append(&mut mnt, cdrom.clone());

        let mut wd = TreeNode::create("wd", NodeType::File);
        TreeNode::append(&mut mnt, wd.clone());

        tree.tmv_subdir();
        tree.lmv_next();
    }

    let tree_view = Rc::new(RefCell::new(TreeView::new(tree.clone())));
    let list_view = Rc::new(RefCell::new(ListView::new(tree.clone())));

    tree.borrow_mut().tree_view = Rc::downgrade(&tree_view);
    tree.borrow_mut().list_view = Rc::downgrade(&list_view);

    let left_displ = Rc::new(RefCell::new(Display::new(
        tree_view,
        &screen.tree_win,
        &screen.tw_size,
    )));
    let right_displ = Rc::new(RefCell::new(Display::new(
        list_view,
        &screen.list_win,
        &screen.lw_size,
    )));

    let mut focused_displ = left_displ.clone();
    loop {
        left_displ.borrow_mut().display()?;
        right_displ.borrow_mut().display()?;

        let ch: i32 = getch();

        if ch == KEY_F(10) || ch == 27 {
            break;
        }

        focused_displ.borrow().process_key(ch)?;
        assert!(Rc::ptr_eq(&focused_displ, &left_displ))
    }

    Ok(())
}

fn main() -> ExitCode {
    let screen = Screen::create();
    let result = run(&screen);
    screen.close();

    if let Err(err) = result {
        eprintln!("{}", err);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
