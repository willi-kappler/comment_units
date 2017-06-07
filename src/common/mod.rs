// External module
use walkdir::WalkDir;

// Std modules
use std::fmt;

// Local modules
use rust;
use fortran;
use matlab;

/// Currently supported languages:
pub fn supported_languages<'a>() -> Vec<&'a str> {
    vec![// Rust:
         "rs",
         // Fortran:
         "f90",
         // Matlab:
         "m"
        ]
}


pub enum SupportedLanguage {
    Rust,
    Fortran,
    Matlab
}

impl fmt::Display for SupportedLanguage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SupportedLanguage::Rust => write!(f, "Rust"),
            SupportedLanguage::Fortran => write!(f, "Fortran"),
            SupportedLanguage::Matlab => write!(f, "Matlab"),
            _ => write!(f, "unknown"),
        }
    }
}

pub fn extract_language_extensions(language_list: &str) -> Vec<&str> {
    language_list.split(",").collect::<Vec<&str>>()
}

pub fn process_folder(folder: &str, lang_filter: Vec<&str>) {
    println!("processing top folder: '{}' with filter: '{}'", folder, lang_filter.join(","));

    for entry in WalkDir::new(folder) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let entry = entry.file_name().to_str().unwrap();
            if let Some(language) = consider_file(entry, &lang_filter) {
                process_file(entry, language)
            }
        }
    }
}

pub fn consider_file(file: &str, lang_filter: &Vec<&str>) -> Option<SupportedLanguage> {
    for pattern in lang_filter {
        if file.ends_with(pattern) {
            return file_extension_to_language(pattern);
        }
    }

    None
}

pub fn process_file(file_name: &str, language: SupportedLanguage) {
    println!("processing file: '{}' written in language: '{}'", file_name, language);

    match language {
        SupportedLanguage::Rust => rust::process_language(file_name),
        SupportedLanguage::Fortran => fortran::process_language(file_name),
        SupportedLanguage::Matlab => matlab::process_language(file_name),
        _ => println!("language '{}' currently not supported!", language)
    }
}

fn file_extension_to_language(file_extension: &str) -> Option<SupportedLanguage> {
    match file_extension {
        "rs" => Some(SupportedLanguage::Rust),
        "f90" => Some(SupportedLanguage::Fortran),
        "m" => Some(SupportedLanguage::Matlab),
        _ => None
    }
}
