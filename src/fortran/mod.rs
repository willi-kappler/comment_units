#[cfg(test)] mod tests;

use std::fs::File;
use std::io::Read;

use nom::{alpha, alphanumeric};
use nom;

#[derive(PartialEq, Debug)]
enum FortranTokenType {
    Comment(String),            // Simgle line comment with !
    Identifier(String),         // distance, particle_id
}

#[derive(PartialEq, Debug)]
struct Token {
    line_number: u32,
    column: u32,
    token: FortranTokenType
}

named!(parse_comment<&str, FortranTokenType>, do_parse!(
    tag!("!") >>
    comment: take_while!(call!(|c| c != '\n')) >>
    (FortranTokenType::Comment(comment.to_owned()))
));

named!(parse_many_comment_lines<&str, Vec<FortranTokenType> >, many0!(ws!(parse_comment)));

named!(parse_identifier<&str, FortranTokenType>, do_parse!(
    head: alpha >>
    tail: many0!(alt!(alphanumeric | tag!("_"))) >>
    (FortranTokenType::Identifier(head.to_owned()))
));

named!(parse_program<&str, Vec<FortranTokenType> >, do_parse!(
    ws!(tag!("program")) >>
    ws!(parse_identifier) >>
    ws!(tag!("end")) >>
    ws!(tag!("program")) >>
    (Vec::new())
));

named!(parse_module<&str, Vec<FortranTokenType> >, do_parse!(
    ws!(tag!("module")) >>
    ws!(parse_identifier) >>
    ws!(tag!("end")) >>
    ws!(tag!("module")) >>
    (Vec::new())
));

named!(parse_code<&str, Vec<FortranTokenType> >, do_parse!(
    parse_many_comment_lines >>
    actual_code: alt!(parse_program | parse_module) >>
    (actual_code)
));


pub fn process_language(file_name: &str) {
    println!("processing fortran source file: '{}'", file_name);

    let mut buffer = String::new();

    let mut f = File::open(file_name).expect("unable to open file");

    f.read_to_string(&mut buffer).expect("unable to read to string");

    let result = parse_code(&buffer);
    println!("result: {:?}", result);
}
