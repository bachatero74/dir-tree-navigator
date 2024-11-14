use crate::common::*;
use ncurses::*;

#[derive(Default)]
pub struct DisplInfo {
    pub lines_count: i32,
    pub curs_line: i32,
    pub curs_x1: i32,
    pub curs_x2: i32,
}

pub struct ViewLine {
    pub content: String,
    pub x1: i32,
    pub x2: i32,
}

impl ViewLine {
    pub fn new(content: String, x1: i32, x2: i32) -> ViewLine {
        ViewLine { content, x1, x2 }
    }
}

pub trait DisplContent {
    fn prepare(&mut self, info: &mut DisplInfo) -> Result<(), AppError>;
    fn get_line(&self, y: usize) -> Result<&ViewLine, AppError>;
    fn process_key(&self, key: i32) -> Result<(), AppError>;
}

pub struct Display {
    content: Box<dyn DisplContent>,
    window: WINDOW,
    size: Size,
    //offset_x: i32,
    offset_y: i32,
}

impl Display {
    pub fn new(content: Box<dyn DisplContent>, window: &WINDOW, size: &Size) -> Display {
        Display {
            content,
            window: *window,
            size: *size,
            //offset_x: 0,
            offset_y: 0,
        }
    }

    pub fn display(&mut self) -> Result<(), AppError> {
        let mut info: DisplInfo = Default::default();
        self.content.prepare(&mut info)?;

        if info.curs_line - self.offset_y > self.size.height - 1 {
            self.offset_y = info.curs_line - self.size.height + 1;
        }

        if info.curs_line - self.offset_y < 0 {
            self.offset_y = info.curs_line;
        }

        let offset_x = fit_str(info.curs_x1, info.curs_x2, self.size.width);

        werase(self.window);
        for y in 0..self.size.height {
            let ln = y + self.offset_y;
            if ln >= info.lines_count {
                // TODO: nieoptymalne
                break;
            }
            let view_line = self.content.get_line(ln as usize)?;
            mvwprintw_substr(
                self.window,
                y as i32,
                0,
                &view_line.content,
                offset_x,
                self.size.width,
            );
        }

        mvwprintw(self.window, info.curs_line as i32 - self.offset_y, 0, ">");

        wrefresh(self.window);
        Ok(())
    }

    pub fn process_key(&self, key: i32) -> Result<(), AppError> {
        self.content.process_key(key)
    }
}

fn fit_str(x1: i32, x2: i32, width: i32) -> i32 {
    (x2 - width).clamp(0, x1)
}

fn mvwprintw_substr(w: WINDOW, y: i32, x: i32, s: &str, offs: i32, len: i32) {
    wmove(w, y, x);
    for ch in s.chars().skip(offs as usize).take(len as usize) {
        waddch(w, ch as u32);
    }
}
