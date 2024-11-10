mod app_sys;
use ncurses::*;

struct Screen {
    left_pane: WINDOW,
    right_pane: WINDOW,
    tree_win: WINDOW,
    list_win: WINDOW,
}

impl Screen {
    fn initialize() -> Screen {
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

// test

fn main() {
    let screen = Screen::initialize();
    getch();
    
    mvwprintw(screen.tree_win, 0, 0, "123456789");
    mvwprintw(screen.list_win, 0, 0, "123456789");
    wrefresh(screen.tree_win);
	wrefresh(screen.list_win);

    getch();
    screen.close();
}
