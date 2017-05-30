// comment_units V0.1 (2017.05.12), written by Willi Kappler
//
// Licensed under the MIT License
//
// A tool to check the physical units described in comments
//

// External crates:
extern crate clap;
extern crate walkdir;

// Std modules
use std::env;

// External modules:
use clap::{App, Arg};

// Local modules
mod file_util;

use file_util::{process_folder, extract_language_extensions, consider_file, process_file};

fn main() {
    let matches = App::new("My Super Program")
        .version("0.1")
        .author("Willi Kappler. <grandor@gmx.de>")
        .about("Check physical units in comments")
        .arg(Arg::with_name("directory")
            .short("d")
            .long("directory")
            .value_name("DIRECTORY")
            .help("The input directory to check for source files recursively. Default is current directory")
            .takes_value(true)
            .required(false))
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .value_name("FILE")
            .help("The input file to check, if you just want to check only one source file")
            .takes_value(true)
            .required(false))
        .arg(Arg::with_name("languages")
            .short("l")
            .long("languages")
            .value_name("LANGUAGES")
            .help("The programming language(s) of the input source file(s).\n\
                  Default is to check for all supported languages.\n\
                  Otherwise only the provided soure files will be checked, based on the file extension.\n\
                  You can provie multiple languages (separated by comma), these are currently supported:\n\
                  rust, fortran, matlab"
              )
            .takes_value(true)
            .required(false))
        .get_matches();

    let languages = match matches.value_of("languages") {
        Some(languages) => extract_language_extensions(languages),
        None => vec![// Rust:
                     "rs",
                     // Fortran:
                     "f90",
                     // Matlab:
                     "m"
                     ]
    };

    match matches.value_of("file") {
        Some(input_file) => {
            println!("single input file: '{}''", input_file);
            if consider_file(input_file, &languages) {
                process_file(input_file)
            }
        },
        None => {
            match matches.value_of("directory") {
                Some(input_directory) => {
                    println!("input directory: '{}'", input_directory);
                    process_folder(input_directory, languages);
                }
                None => {
                    println!("current directory");
                    let current_dir = env::current_dir().unwrap();
                    process_folder(current_dir.to_str().unwrap(), languages);
                }
            }
        }
    }

}
