use nom::{IResult, Needed, Err, ErrorKind};

use super::super::{FortranTokenType, parse_implicit};

#[test]
fn parse_implicit1() {
    let input = "";
    let expected_output = IResult::Incomplete(Needed::Size(8));

    let result = parse_implicit(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_implicit2() {
    let input = " ";
    let expected_output = IResult::Incomplete(Needed::Size(9));

    let result = parse_implicit(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_implicit3() {
    let input = "x";
    let expected_output = IResult::Error(Err::Position(ErrorKind::Tag, "x"));

    let result = parse_implicit(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_implicit4() {
    let input = "implicit";
    let expected_output = IResult::Incomplete(Needed::Size(12));

    let result = parse_implicit(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_implicit5() {
    let input = "  implicit   none   ";
    let expected_output = IResult::Done("", ());

    let result = parse_implicit(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_implicit6() {
    let input = "implicit   none     ! Some stuff";
    let expected_output = IResult::Done("", ());

    let result = parse_implicit(input);

    assert_eq!(result, expected_output);
}
