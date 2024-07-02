use std::{
    ffi::OsStr,
    fs::{read_dir, DirEntry},
};

pub fn get_directories(path: &str) -> Vec<DirEntry> {
    let Ok(contents) = read_dir(path) else {
        eprintln!("Could not read contents of directory");
        return vec![];
    };

    let dirs = contents
        .filter(|dir_ent| dir_ent.is_ok())
        .map(|dir| dir.unwrap())
        .filter(|dir| dir.file_type().unwrap().is_dir())
        .collect();

    return dirs;
}

pub fn get_files_by_extension(path: &str, extensions: &Vec<&str>) -> Vec<DirEntry> {
    let Ok(contents) = read_dir(path) else {
        eprintln!("Could not read contents of directory");
        return vec![];
    };

    let files = contents
        .filter(|dir_ent| dir_ent.is_ok())
        .map(|file| file.unwrap())
        .filter(|file| file.file_type().unwrap().is_file())
        .filter(|file| {
            extensions.contains(
                &file
                    .path()
                    .extension()
                    .unwrap_or(OsStr::new("s"))
                    .to_str()
                    .unwrap(),
            )
        })
        .collect();

    return files;
}

pub fn get_files_by_extensions_recursive(path: &str, extensions: &Vec<&str>) -> Vec<DirEntry> {
    let dirs = get_directories(path);

    let mut files = get_files_by_extension(path, extensions);

    for dir in dirs {
        files.append(&mut get_files_by_extensions_recursive(
            dir.path().as_path().to_str().unwrap(),
            extensions,
        ));
    }

    return files;
}
