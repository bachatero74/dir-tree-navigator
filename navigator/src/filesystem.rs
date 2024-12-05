use std::ffi::{OsStr, OsString};
use std::fs::DirEntry;
use std::os::unix::fs::MetadataExt;
use std::time::{SystemTime, UNIX_EPOCH};
use users::get_user_by_uid;

#[derive(PartialEq)]
pub enum NodeType {
    File,
    Dir,
    SymLink,
    //UpDir,
}

impl NodeType {
    pub fn priority(&self) -> i32 {
        match self {
            //NodeType::UpDir => 0,
            NodeType::Dir => 1,
            NodeType::File => 2,
            NodeType::SymLink => 2,
        }
    }
}

pub struct SysNode {
    pub name: OsString,
    pub typ: NodeType,
    pub mode: u32,
    pub user: OsString,
    pub group: OsString,
    pub size: u64,
    pub modified: chrono::DateTime<chrono::Local>,
}

// TODO: zrobić to ładniej, bo jest łopatologicznie?
// unwrap_or_default - sprawdzić to
impl SysNode {
    pub fn from(entry: &DirEntry) -> Self {
        let name = entry.file_name();
        let mut typ = NodeType::File;
        let mut mode = 0;
        let mut user = OsString::from("");
        let mut group = OsString::from("");
        let mut size: u64 = 0;
        let mut modified: chrono::DateTime<chrono::Local> = SystemTime::now().into();

        if let Ok(md) = entry.metadata() {
            typ = if md.is_dir() {
                NodeType::Dir
            } else if md.is_symlink() {
                NodeType::SymLink
            } else {
                NodeType::File
            };
            mode = md.mode();
            size = md.len();
            user = match get_user_by_uid(md.uid()) {
                Some(usr) => usr.name().to_os_string(),
                None => md.uid().to_string().into(),
            };
            group = match get_user_by_uid(md.gid()) {
                Some(grp) => grp.name().to_os_string(),
                None => md.gid().to_string().into(),
            };
            if let Ok(m) = md.modified() {
                let m = m.duration_since(UNIX_EPOCH).unwrap_or_default();
                let modified_time = SystemTime::UNIX_EPOCH + m;
                modified = modified_time.into();
            };
        }

        Self {
            name,
            typ,
            mode,
            user,
            group,
            size,
            modified,
        }
    }

    pub fn new(name: &OsStr, typ: NodeType) -> Self {
        Self {
            name: name.to_os_string(),
            typ,
            mode: 0,
            user: OsString::from(""),
            group: OsString::from(""),
            size: 0,
            modified: SystemTime::now().into(),
        }
    }
}

pub fn file_type_to_str(typ: &NodeType) -> &str {
    match typ {
        NodeType::File => "-",
        NodeType::Dir => "d",
        NodeType::SymLink => "l",
        //NodeType::UpDir => "d",
    }
}

pub fn permissions_to_str(mode: u32) -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}",
        if mode & 0o400 != 0 { 'r' } else { '-' },
        if mode & 0o200 != 0 { 'w' } else { '-' },
        if mode & 0o100 != 0 { 'x' } else { '-' },
        if mode & 0o040 != 0 { 'r' } else { '-' },
        if mode & 0o020 != 0 { 'w' } else { '-' },
        if mode & 0o010 != 0 { 'x' } else { '-' },
        if mode & 0o004 != 0 { 'r' } else { '-' },
        if mode & 0o002 != 0 { 'w' } else { '-' },
        if mode & 0o001 != 0 { 'x' } else { '-' },
    )
}

pub fn datetime_to_str(datetime: chrono::DateTime<chrono::Local>) -> String {
    datetime.format("%b %d %H:%M").to_string()
}
