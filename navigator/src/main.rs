mod common;
mod filesystem;
mod screen;
mod tree;
mod graph {
    pub mod display;
    pub mod list_view;
    pub mod tree_view;
}

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::{cell::RefCell, rc::Rc};

use common::*;
use graph::{display::*, list_view::*, tree_view::*};
use ncurses::*;
use screen::*;
use tree::*;

fn run(screen: &Screen) -> Result<PathBuf, AppError> {
    let tree = Rc::new(RefCell::new(Tree::new()));

    // {
    //     let mut tree = tree.borrow_mut();
    //     let mut etc = TreeNode::create(&OsString::from("etc"), NodeType::Dir);
    //     TreeNode::append(&mut tree.root, etc.clone());

    //     let mut fstab = TreeNode::create(&OsString::from("fstab"), NodeType::File);
    //     TreeNode::append(&mut etc, fstab.clone());

    //     let mut mtab = TreeNode::create(&OsString::from("mtab"), NodeType::File);
    //     TreeNode::append(&mut etc, mtab.clone());

    //     let mut mnt = TreeNode::create(&OsString::from("mnt"), NodeType::Dir);
    //     TreeNode::append(&mut tree.root, mnt.clone());

    //     let mut cdrom = TreeNode::create(&OsString::from("cdrom"), NodeType::File);
    //     TreeNode::append(&mut mnt, cdrom.clone());

    //     let mut wd = TreeNode::create(&OsString::from("wd"), NodeType::File);
    //     TreeNode::append(&mut mnt, wd.clone());

    //     //tree.tmv_subdir();
    //     //tree.lmv_next();

    //     // Uwaga:
    //     // let xr = Rc::new(RefCell::new(X { v: 58 }));
    //     // let x:&mut X = &mut xr.borrow_mut(); wtedy możnaby się obejść bez fn go_to_path
    //     // i zrobić to w jednej linijce bez błędów z pożyczkami
    //     if let Err(err) = tree.go_to_path(&PathBuf::from("/etc")) {
    //         eprintln!("{}", err);
    //     }
    // }

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
    let x = Ok(tree.borrow().curr_path());
    x
}

fn main() -> ExitCode {
    let screen = Screen::create();
    let result = run(&screen);
    screen.close();

    match result {
        Ok(path) => {
            println!("{}", path.to_string_lossy().to_string());
        }
        Err(err) => {
            eprintln!("{}", err);
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
