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
}

pub struct Display {
    content: Box<dyn DisplContent>,
    window: WINDOW,
    size: Size,
    offset_x: i32,
    offset_y: i32,
}

impl Display {
    pub fn new(content: Box<dyn DisplContent>, window: &WINDOW, size: &Size) -> Display {
        Display {
            content,
            window: *window,
            size: *size,
            offset_x: 0,
            offset_y: 0,
        }
    }

    pub fn display(&mut self) -> Result<(), AppError> {
        let mut info: DisplInfo = Default::default();
        self.content.prepare(&mut info)?;

        let l_cnt = std::cmp::min(info.lines_count, self.size.height);

        for y in 0..l_cnt {
            let v_line = self.content.get_line(y as usize)?;
            mvwprintw(self.window, y as i32, 0, &v_line.content);
        }

        wrefresh(self.window);
        Ok(())
    }
}
