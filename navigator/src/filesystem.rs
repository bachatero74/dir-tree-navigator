use std::fs;
use std::os::unix::fs::MetadataExt;
use std::time::{UNIX_EPOCH, SystemTime};
use users::{get_group_by_gid, get_user_by_uid};

fn print_dir() -> std::io::Result<()> {
    // Ścieżka katalogu
    let path = "/home/jacek";

    // Iterujemy przez zawartość katalogu
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        // Nazwa pliku
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();

        // Uprawnienia (np. -rw-r--r--)
        let permissions = metadata.permissions();
        let file_type = if metadata.is_dir() {
            'd'
        } else if metadata.is_symlink() {
            'l'
        } else {
            '-'
        };
        let mode = metadata.mode();
        let permissions_str = format!(
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
        );

        // Właściciel i grupa
        let uid = metadata.uid();
        let gid = metadata.gid();
        let owner = get_user_by_uid(uid).map(|u| u.name().to_string_lossy().to_string());
        let group = get_group_by_gid(gid).map(|g| g.name().to_string_lossy().to_string());

        // Rozmiar pliku
        let size = metadata.len();

        // Data modyfikacji
        let modified = metadata.modified()?.duration_since(UNIX_EPOCH).unwrap_or_default();
        let modified_time = SystemTime::UNIX_EPOCH + modified;
        let datetime: chrono::DateTime<chrono::Local> = modified_time.into();
        let date_str = datetime.format("%b %d %H:%M").to_string();

        // Wyświetlenie wyniku
        println!(
            "{}{} {:>8} {:>8} {:>10} {} {}",
            file_type,
            permissions_str,
            owner.unwrap_or_else(|| uid.to_string()),
            group.unwrap_or_else(|| gid.to_string()),
            size,
            date_str,
            file_name
        );
    }

    Ok(())
}