#[cfg(test)] mod tests;

use std::fs::File;
use std::io::Read;

use nom::{alpha, alphanumeric, digit, ErrorKind};

#[derive(PartialEq, Debug)]
enum FortranTokenType {
     Comment(String),                           // Single line comment with !
     Identifier(String),                        // distance, particle_id
     IdentifierWithComment(String, String),     // velocity ! [m/s]
}

#[derive(PartialEq, Debug)]
struct Token {
    line_number: u32,
    column: u32,
    token: FortranTokenType
}

named!(parse_comment<&str, FortranTokenType>, do_parse!(
    tag!("!") >> comment: take_while!(call!(|c| c != '\n')) >>
    (FortranTokenType::Comment(comment.to_owned()))
));

named!(parse_many_comment_lines<&str, Vec<FortranTokenType> >, many0!(ws!(parse_comment)));

named!(parse_maybe_comment<&str, Option<FortranTokenType> >, opt!(complete!(parse_comment)));

named!(parse_identifier<&str, FortranTokenType>, do_parse!(
    head: alpha >>
    tail: fold_many0!(alt!(alphanumeric | tag!("_")), String::new(), |mut acc: String, item| {
        acc.push_str(item);
        acc
    }) >>
    (FortranTokenType::Identifier(format!("{}{}", head, tail)))
));

named!(parse_identifier_list<&str, Vec<FortranTokenType>>, do_parse!(
    first: parse_identifier >>
    rest: many0!(do_parse!(
        ws!(tag!(",")) >>
        content: ws!(parse_identifier) >>
        (content)
    )) >>
    ({
        let mut result = Vec::new();
        result.push(first);
        result.extend(rest);
        result
    })
));

named!(parse_use<&str, ()>, do_parse!(
    ws!(tag!("use")) >> ws!(parse_identifier) >> ws!(parse_maybe_comment) >> ()
));

named!(parse_implicit<&str, ()>, do_parse!(
    ws!(tag!("implicit")) >> ws!(tag!("none")) >> ws!(parse_maybe_comment) >> ()
));

named!(parse_type<&str, ()>, do_parse!(
    ws!(alt!(tag!("real") | tag!("integer") | tag!("character") | tag!("logical"))) >>
    opt!(complete!(do_parse!(ws!(tag!("(")) >> ws!(digit) >> ws!(tag!(")")) >> ()))) >>
    ()
));

named!(parse_declaration<&str, Option<FortranTokenType> >, do_parse!(
    ws!(parse_type) >> ws!(tag!("::")) >> ws!(parse_identifier_list) >>
    comment: parse_maybe_comment >>
    (comment)
));

named!(parse_program_body<&str, Vec<FortranTokenType> >, do_parse!(
    many0!(parse_use) >>
    parse_many_comment_lines >>
    opt!(parse_implicit) >>
    parse_many_comment_lines >>
    (Vec::new())
));

named!(parse_program<&str, Vec<FortranTokenType> >, do_parse!(
    ws!(tag!("program")) >> ws!(parse_identifier) >> ws!(parse_maybe_comment) >>
    parse_many_comment_lines >>
    result: parse_program_body >>
    parse_many_comment_lines >>
    ws!(tag!("end")) >> ws!(tag!("program")) >> ws!(parse_identifier) >> ws!(parse_maybe_comment) >>
    (result)
));

named!(parse_module<&str, Vec<FortranTokenType> >, do_parse!(
    ws!(tag!("module")) >> ws!(parse_identifier) >> ws!(parse_maybe_comment) >>
    parse_many_comment_lines >>

    ws!(tag!("end")) >> ws!(tag!("module")) >> ws!(parse_identifier) >> ws!(parse_maybe_comment) >>
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
