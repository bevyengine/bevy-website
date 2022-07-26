use std::{
    ffi::OsStr,
    fs::{self, DirEntry},
    io,
    path::Path,
};

pub fn visit_dir_md_files(dir: &Path, cb: &dyn Fn(&DirEntry) -> io::Result<()>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit_dir_md_files(&path, cb)?;
            } else if let Some(ext) = path.extension().and_then(OsStr::to_str) {
                if ext.to_lowercase() == "md" {
                    cb(&entry)?;
                }
            }
        }
    }

    Ok(())
}
