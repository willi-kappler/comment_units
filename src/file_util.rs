use walkdir::WalkDir;

pub fn extract_language_extensions(language_list: &str) -> Vec<&str> {
    language_list.split(",").collect::<Vec<&str>>()
}

pub fn process_folder(folder: &str, lang_filter: Vec<&str>) {
    println!("processing top folder: '{}' with filter: '{}'", folder, lang_filter.join(","));

    for entry in WalkDir::new(folder) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let entry = entry.file_name().to_str().unwrap();
            if consider_file(entry, &lang_filter) {
                process_file(entry)
            }
        }
    }
}

pub fn consider_file(file: &str, lang_filter: &Vec<&str>) -> bool {
    for pattern in lang_filter {
        if file.ends_with(pattern) {
            return true;
        }
    }

    false
}

pub fn process_file(file_name: &str) {
    println!("processing file: {}", file_name);
}
