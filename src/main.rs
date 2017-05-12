// comment_units V0.1 (2017.05.12), written by Willi Kappler
//
// Licensed under the MIT License
//
// A tool to check the physical units described in comments
//

extern crate clap;

use clap::{App, Arg};

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
            .help(("The programming language(s) of the input source file(s).\n".to_owned() +
                  "Default is to check for all supported languages.\n" +
                  "Otherwise only the provided soure files will be checked, based on the file extension.\n" +
                  "You can provie multiple languages, these are currently supported:\n" +
                  "rust, fortran, matlab").as_str()
              )
            .takes_value(true)
            .required(false))
        .get_matches();

    match matches.value_of("file") {
        Some(input_file) => {
            println!("single input file: '{}''", input_file);
        },
        None => {
            match matches.value_of("directory") {
                Some(input_directory) => {
                    println!("input directory: '{}'", input_directory);
                }
                None => {
                    println!("current directory")
                }
            }
        }
    }

}
