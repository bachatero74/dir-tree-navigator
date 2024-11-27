use std::{ffi::{OsStr, OsString}, fs::{DirEntry, Permissions}};

use thiserror;

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

#[derive(PartialEq)]
pub enum NodeType {
    File,
    Dir,
    UpDir,
}

pub struct SysNode {
    pub name: OsString,
    pub typ: NodeType,
    //pub permissions:Permissions,
}

impl SysNode {
    pub fn from(entry: &DirEntry) -> Self {
        let name=entry.file_name();
        let typ=NodeType::Dir;

        Self { name, typ, }
    }

    pub fn new(name: &OsStr, typ: NodeType) -> Self {
        Self { name: name.to_os_string(), typ, }
    }
}
