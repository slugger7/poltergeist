pub fn create_relative_path(base_path: String, absolute_path: String) -> Result<String, String> {
    if !absolute_path.contains(&base_path) {
        return Err(String::from("Base path was not found in absolute path"));
    }

    let relative_path = absolute_path.replace(base_path.as_str(), "");

    Ok(relative_path[1..].to_string())
}

pub fn file_name_without_extension(file_name: &str) -> String {
    if let Some(last_index) = file_name.rfind('.') {
        file_name[..last_index].to_string()
    } else {
        file_name.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn file_name_without_extension_with_extension() {
        let file_name = String::from("something.mp4");

        let title = file_name_without_extension(file_name.as_str());

        assert_eq!("something", title)
    }

    #[test]
    fn file_name_without_extension_with_no_extension() {
        let file_name = String::from("something");

        let title = file_name_without_extension(file_name.as_str());

        assert_eq!(file_name, title)
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
}
