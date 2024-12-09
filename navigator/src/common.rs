use thiserror;
use ncurses::*;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    StrError(String),

    #[error("Path error: {0} ('{1}')")]
    PathError(String, String),

    #[error("Błąd IO: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Copy, Clone)]
pub struct Size {
    pub height: i32,
    pub width: i32,
}

impl Size {
    pub fn new(width: i32, height: i32) -> Size {
        Size { width, height }
    }
}

enum AppColorTypes {
    Dir=10,
    Exec=11,
}

pub fn init_app_colors(){
    init_pair(AppColorTypes::Dir as i16, COLOR_BLUE, -1);
    init_pair(AppColorTypes::Exec as i16, COLOR_CYAN, -1);
}

