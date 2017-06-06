use std::fs::File;
use std::io::Read;

use nom::{alphanumeric};

#[derive(PartialEq, Debug)]
enum FortranTokenType {
    Comment(String),            // Simgle line comment with !
    Itentifier(String),         // distance, particle_id
}

#[derive(PartialEq, Debug)]
struct Token {
    line_number: u32,
    column: u32,
    token: FortranTokenType
}

named!(parse_comment<&str, Option<FortranTokenType>>, ws!(opt!(
    do_parse!(
        tag!("!") >>
        content: take_while!(call!(|c| c != '\n')) >>
        (FortranTokenType::Comment(content.to_string()))
    ))));

named!(parse_comment_line<&str, FortranTokenType>, ws!(
    do_parse!(
        tag!("!") >>
        content: take_while!(call!(|c| c != '\n')) >>
        (FortranTokenType::Comment(content.to_string()))
    )));

named!(parse_many_comment_lines<&str, Vec<FortranTokenType> >, many0!(parse_comment_line));

// fn parse_identifier(input: &str) -> IResult<&str, String> {
//     if input.len() == 0 {
//         IResult::Incomplete(Needed::Unknown)
//     } else {
//         if !input[0].is_alphabetic() {
//             IResult::Error
//         } else {
//             IResult::Done
//         }
//     }
// }

named!(parse_identifier<&str, String>, do_parse!(
    values: many1!(alt!(alphanumeric | tag!("_"))) >>
    (values.join(""))
));

named!(parse_program<&str, Vec<FortranTokenType> >, do_parse!(
    ws!(tag!("program")) >>
    ws!(parse_identifier) >>
    // parse_use_statements >>
    // parse_implicit_statements >>
    // variable_decls: parse_variable_declarations >>
    // statements: parse_statements >>
    // parse_many_comment_lines >>
    ws!(tag!("end")) >>
    ws!(tag!("program")) >>
    ws!(parse_identifier) >>
    parse_comment >>
    parse_many_comment_lines >>
    //(variable_decls.extend(&statements))

    parse_comment >>
    (Vec::new())
));

named!(parse_module<&str, Vec<FortranTokenType> >, do_parse!(
    ws!(tag!("program")) >>
    ws!(parse_identifier) >>
    parse_comment >>
    // TODO: add function and subroutine
    ws!(tag!("end")) >>
    ws!(tag!("program")) >>
    ws!(parse_identifier) >>
    parse_comment >>
    parse_many_comment_lines >>
    (Vec::new())
));

named!(parse_code<&str, Vec<FortranTokenType> >, do_parse!(
    parse_many_comment_lines >>
    actual_code: alt!(parse_program | parse_module) >>
    (actual_code)
));


pub fn process_language_fortran(file_name: &str) {
    println!("processing fortran source file: '{}'", file_name);

    let mut buffer = String::new();

    let mut f = File::open(file_name).expect("unable to open file");

    f.read_to_string(&mut buffer).expect("unable to read to string");

    let result = parse_code(&buffer);
    println!("result: {:?}", result);

}
