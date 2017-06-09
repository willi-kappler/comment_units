use nom::{IResult, Needed, Err, ErrorKind};

use super::super::{FortranTokenType, parse_type};

#[test]
fn parse_type1() {
    let input = "";
    let expected_output = IResult::Incomplete(Needed::Size(4));

    let result = parse_type(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_type2() {
    let input = " ";
    let expected_output = IResult::Incomplete(Needed::Size(5));

    let result = parse_type(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_type3() {
    let input = "x";
    let expected_output = IResult::Error(Err::Position(ErrorKind::Alt, "x"));

    let result = parse_type(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_type4() {
    let input = "real";
    let expected_output = IResult::Done("", ());

    let result = parse_type(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_type5() {
    let input = "  integer ( 4  )  ";
    let expected_output = IResult::Done("", ());

    let result = parse_type(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_type6() {
    let input = "real(8)";
    let expected_output = IResult::Done("", ());

    let result = parse_type(input);

    assert_eq!(result, expected_output);
}
