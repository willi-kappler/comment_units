

pub fn extract_language_extensions(language_list: &str) -> Vec<&str> {
    language_list.split(",").collect::<Vec<&str>>()
}

pub fn process_folder(folder: &str, lang_filter: Vec<&str>) {
    println!("processing folder: '{}' with filter: '{}'", folder, lang_filter.join(","))


}
