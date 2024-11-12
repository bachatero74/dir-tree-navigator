mod common;
mod tree;
mod graph{
    pub mod display;
    pub mod tree_view;
    pub mod list_view;
}

use std::process::ExitCode;
use std::{cell::RefCell, rc::Rc};

use tree::*;
use graph::display::*;
use graph::tree_view::*;
use graph::list_view::*;
use common::*;
use ncurses::*;

struct Screen {
    left_pane: WINDOW,
    right_pane: WINDOW,
    tree_win: WINDOW,
    list_win: WINDOW,

    tw_size: Size,
    lw_size: Size,
}

impl Screen {
    fn create() -> Screen {
        initscr();
        cbreak();
        noecho();
        keypad(stdscr(), true);

        refresh();

        let mut scr_height: i32 = 0;
        let mut scr_width: i32 = 0;

        getmaxyx(ncurses::stdscr(), &mut scr_height, &mut scr_width);

        let l_width = scr_width / 4;
        let r_width = scr_width - l_width;

        // Tree
        let left_pane = newwin(scr_height, l_width, 0, 0);
        box_(left_pane, 0, 0);
        wrefresh(left_pane);

        let tree_win: WINDOW = newwin(scr_height - 2, l_width - 2, 1, 1);
        wrefresh(tree_win);

        // List
        let right_pane = newwin(scr_height, r_width, 0, l_width);
        box_(right_pane, 0, 0);
        wrefresh(right_pane);

        let list_win: WINDOW = newwin(scr_height - 2, r_width - 2, 1, l_width + 1);
        wrefresh(list_win);

        Screen {
            left_pane,
            right_pane,
            tree_win,
            list_win,

            tw_size: Size::new(l_width - 2, scr_height - 2),
            lw_size: Size::new(r_width - 2, scr_height - 2),
        }
    }

    fn close(&self) {
        delwin(self.tree_win);
        delwin(self.list_win);
        delwin(self.left_pane);
        delwin(self.right_pane);
        endwin();
    }
}

fn run(screen: &Screen) -> Result<(), AppError> {
    let tree: Rc<RefCell<Tree>> = Rc::new(RefCell::new(Tree {}));

    let tree_view: Box<TreeView> = Box::new(TreeView::new(tree.clone()));
    let list_view: Box<ListView> = Box::new(ListView::new(tree.clone()));

    let mut left_displ = Display::new(tree_view, &screen.tree_win, &screen.tw_size);
    let mut right_displ = Display::new(list_view, &screen.list_win, &screen.lw_size);

    loop {
        left_displ.display()?;
        right_displ.display()?;

        let ch = getch();

        if ch == KEY_F(10) || ch == 27 {
            break;
        }
    }

    Ok(())
}

fn main()->ExitCode {
    let screen = Screen::create();

    let result = run(&screen);

    screen.close();
    if let Err(err) = result {
        eprintln!("{}", err);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
