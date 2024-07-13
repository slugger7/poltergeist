pub mod extensions;
pub mod ffmpeg;

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
                    .unwrap_or(OsStr::new(""))
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

#[cfg(test)]
mod test {
    use super::*;

    fn test_data_path() -> String {
        "./src/test_data".to_string()
    }

    #[test]
    fn list_all_directories() {
        let mut dirs: Vec<String> = get_directories(&test_data_path())
            .into_iter()
            .map(|dir| dir.file_name().into_string().unwrap())
            .collect();

        dirs.sort();

        assert_eq!(2, dirs.len());
        assert_eq!("folder_1", dirs[0]);
        assert_eq!("folder_2", dirs[1]);
    }

    #[test]
    fn list_files_of_extension_rs() {
        let files: Vec<String> = get_files_by_extension(&test_data_path(), &vec!["rs"])
            .into_iter()
            .map(|dir| dir.file_name().into_string().unwrap())
            .collect();

        assert_eq!(1, files.len());
        assert_eq!("root_file.rs", files[0]);
    }

    #[test]
    fn list_files_recursively_by_extension() {
        let mut files: Vec<String> =
            get_files_by_extensions_recursive(&test_data_path(), &vec!["rs", "json"])
                .into_iter()
                .map(|dir| dir.file_name().into_string().unwrap())
                .collect();

        files.sort();

        assert_eq!(2, files.len());
        assert_eq!("folder_2_file.json", files[0]);
        assert_eq!("root_file.rs", files[1]);
    }
}
