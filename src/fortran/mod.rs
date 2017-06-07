#[cfg(test)] mod tests;

use std::fs::File;
use std::io::Read;

use nom::{alphanumeric, IResult};
use nom;

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

fn custom_nom_error_option<'a>(error_code: u32) -> nom::IResult<&'a str, Option<FortranTokenType>> {
    IResult::Error(nom::Err::Code(nom::ErrorKind::Custom(error_code)))
}

fn custom_nom_error<'a>(error_code: u32) -> nom::IResult<&'a str, FortranTokenType> {
    IResult::Error(nom::Err::Code(nom::ErrorKind::Custom(error_code)))
}

fn parse_comment(input: &str) -> IResult<&str, Option<FortranTokenType>> {
    if input.len() == 0 {
        return IResult::Done(&input, None)
    } else {
        for (i, c) in input.chars().enumerate() {
            match c {
                '!' => return IResult::Done(&input[0..i], Some(FortranTokenType::Comment(input[i+1..].to_owned()))),
                ' ' | '\t' => continue,
                _ => return custom_nom_error_option(1)
            }
        }
    }
    IResult::Done(&input, None)
}

fn parse_comment_line(input: &str) -> IResult<&str, FortranTokenType> {
    if input.len() == 0 {
        return custom_nom_error(2)
    } else {
        for (i, c) in input.chars().enumerate() {
            match c {
                '!' => return IResult::Done(&input[0..i], FortranTokenType::Comment(input[i+1..].to_owned())),
                ' ' | '\t' => continue,
                _ => return custom_nom_error(2)
            }
        }
    }
    custom_nom_error(2)
}

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


pub fn process_language(file_name: &str) {
    println!("processing fortran source file: '{}'", file_name);

    let mut buffer = String::new();

    let mut f = File::open(file_name).expect("unable to open file");

    f.read_to_string(&mut buffer).expect("unable to read to string");

    let result = parse_code(&buffer);
    println!("result: {:?}", result);
}
