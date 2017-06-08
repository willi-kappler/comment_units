use nom::{IResult, Needed, Err, ErrorKind};

use super::super::{FortranTokenType, parse_use};

#[test]
fn parse_use1() {
    let input = "";
    let expected_output = IResult::Incomplete(Needed::Size(3));

    let result = parse_use(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_use2() {
    let input = " ";
    let expected_output = IResult::Incomplete(Needed::Size(4));

    let result = parse_use(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_use3() {
    let input = "x";
    let expected_output = IResult::Error(Err::Position(ErrorKind::Tag, "x"));

    let result = parse_use(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_use4() {
    let input = "use";
    let expected_output = IResult::Incomplete(Needed::Unknown);

    let result = parse_use(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_use5() {
    let input = "use velo";
    let expected_output = IResult::Done("", ());

    let result = parse_use(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_use6() {
    let input = "use velo  ! Some stuff";
    let expected_output = IResult::Done("", ());

    let result = parse_use(input);

    assert_eq!(result, expected_output);
}
