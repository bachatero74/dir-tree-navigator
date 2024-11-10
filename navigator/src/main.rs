mod app_sys;
mod tree;
mod views;

use std::{cell::RefCell, rc::Rc};

use crate::tree::*;
use crate::views::*;
use app_sys::AppError;
use ncurses::*;

struct Screen {
    left_pane: WINDOW,
    right_pane: WINDOW,
    tree_win: WINDOW,
    list_win: WINDOW,
}

impl Screen {
    fn create() -> Screen {
        initscr();
        cbreak();
        noecho();
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

    let tree_view: Rc<RefCell<TreeView>> = Rc::new(RefCell::new(TreeView { tree: tree.clone() }));
    let list_view: Rc<RefCell<ListView>> = Rc::new(RefCell::new(ListView { tree: tree.clone() }));

    let rightDispl=Display{content:tree_view.clone()};
    let leftDispl=Display{content:list_view.clone()};

    Ok(())
}

fn main() {
    let screen = Screen::create();

    mvwprintw(screen.tree_win, 0, 0, "123456789");
    mvwprintw(screen.list_win, 0, 0, "123456789");
    wrefresh(screen.tree_win);
    wrefresh(screen.list_win);

    getch();

    let result = run(&screen);

    screen.close();
    if let Err(err) = result {
        eprintln!("{}", err);
    }
}
