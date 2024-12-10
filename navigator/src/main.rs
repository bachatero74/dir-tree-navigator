mod common;
mod filesystem;
mod screen;
mod tree;
mod tree_node;
mod graph {
    pub mod display;
    pub mod list_view;
    pub mod tree_view;
}

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::{cell::RefCell, rc::Rc};
use std::{env, iter};

use common::*;
use graph::{display::*, list_view::*, tree_view::*};
use ncurses::*;
use screen::*;
use tree::*;

fn run(screen: &Screen) -> Result<PathBuf, AppError> {
    let tree = Rc::new(RefCell::new(Tree::new()));

    let args: Vec<String> = env::args().collect();
    if let Some(path) = args.get(1) {
        let _ = tree.borrow_mut().go_to_path(&PathBuf::from(path));
    } else {
        if let Ok(path) = env::current_dir() {
            let _ = tree.borrow_mut().go_to_path(&path);
        }
    }

    let tree_view = Rc::new(RefCell::new(TreeView::new(tree.clone())));
    let list_view = Rc::new(RefCell::new(ListView::new(tree.clone())));

    tree.borrow_mut().tree_view = Rc::downgrade(&tree_view);
    tree.borrow_mut().list_view = Rc::downgrade(&list_view);

    let left_displ = Rc::new(RefCell::new(Display::new(
        tree_view.clone(),
        &screen.tree_win,
        &screen.tw_size,
    )));
    let right_displ = Rc::new(RefCell::new(Display::new(
        list_view.clone(),
        &screen.list_win,
        &screen.lw_size,
    )));

    let mut focused_displ = left_displ.clone();
    focused_displ.borrow_mut().active = true;
    left_displ.borrow_mut().display(true)?;
    loop {
        left_displ.borrow_mut().display(false)?;
        right_displ.borrow_mut().display(false)?;
        display_status(&screen, &tree.borrow());

        let ch: i32 = getch();

        if ch == KEY_F(10) {
            break;
        }
        if ch == 27 {
            return Err(AppError::StrError("Abandoned.".to_owned()));
        }
        if ch == '\t' as i32 {
            focused_displ.borrow_mut().active = false;
            focused_displ = if Rc::ptr_eq(&focused_displ, &left_displ) {
                right_displ.clone()
            } else {
                left_displ.clone()
            };
            focused_displ.borrow_mut().active = true;
            tree_view.borrow_mut().modif_flags.print = true;
            list_view.borrow_mut().modif_flags.print = true;
            continue;
        }

        focused_displ.borrow().process_key(ch)?;
    }
    let x = Ok(tree.borrow().curr_path());
    x
}

fn main() -> ExitCode {
    let screen = Screen::create();
    init_app_colors();
    let result = run(&screen);
    screen.close();

    match result {
        Ok(path) => {
            if let Err(err) = write_output(&path) {
                eprintln!("{}", err);
                return ExitCode::FAILURE;
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}

fn write_output(path: &Path) -> Result<(), AppError> {
    let mut file = File::create("/tmp/navigator.dir")?;
    file.write_all(path.to_string_lossy().as_bytes())?;
    Ok(())
}

fn display_status(screen: &Screen, tree: &Tree) {
    let win = screen.status_win;
    wmove(win, 0, 0);
    wattr_on(win, A_REVERSE);
    for ch in tree
        .curr_path()
        .to_string_lossy()
        .to_string()
        .chars()
        .chain(iter::repeat(' '))
        .take(screen.sw_size.width as usize)
    {
        waddch(win, ch as u32);
    }
    wattr_off(win, A_REVERSE);
    wrefresh(screen.status_win);
}
