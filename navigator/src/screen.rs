use crate::common::*;
use ncurses::*;

pub struct Screen {
    pub left_pane: WINDOW,
    pub right_pane: WINDOW,
    pub tree_win: WINDOW,
    pub list_win: WINDOW,

    pub tw_size: Size,
    pub lw_size: Size,
}

impl Screen {
    pub fn create() -> Screen {
        initscr();
        cbreak();
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
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

    pub fn close(&self) {
        delwin(self.tree_win);
        delwin(self.list_win);
        delwin(self.left_pane);
        delwin(self.right_pane);
        endwin();
    }
}
