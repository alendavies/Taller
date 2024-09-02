use std::{fs, path::Path};

pub fn find_file_in_folder(folder_path: &str, file_name: &str) -> bool {
    let path = Path::new(folder_path);
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() && entry.file_name() == file_name {
                    return true;
                }
            }
        }
    }
    false
}
