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

pub fn create_relative_path(base_path: String, absolute_path: String) -> Result<String, String> {
    if !absolute_path.contains(&base_path) {
        return Err(String::from("Base path was not found in absolute path"));
    }

    let relative_path = absolute_path.replace(base_path.as_str(), "");

    Ok(relative_path[1..].to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_data_path() -> String {
        "./src/test_data".to_string()
    }

    #[test]
    fn create_relative_path_test() {
        let base_path = String::from("/root/sub");
        let absolute_path = String::from("/root/sub/sub_sub/filename");

        if let Ok(relative_path) = create_relative_path(base_path, absolute_path) {
            assert_eq!("sub_sub/filename", relative_path);
        } else {
            assert!(false, "an error was thrown when it was not expected")
        }
    }

    #[test]
    fn create_relative_path_base_path_not_in_absolute_path() {
        let base_path = String::from("/root/sub");
        let absolute_path = String::from("/something/else/filename");

        if let Err(message) = create_relative_path(base_path, absolute_path) {
            assert_eq!("Base path was not found in absolute path", message);
        } else {
            assert!(false, "no error was thrown when we were expecting it")
        }
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
