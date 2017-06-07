use nom::{IResult, Needed, Err, ErrorKind};

use super::super::{FortranTokenType, parse_identifier};

#[test]
fn parse_identifier1() {
    let input = "";
    let expected_output = IResult::Incomplete(Needed::Unknown);

    let result = parse_identifier(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_identifier2() {
    let input = " ";
    let expected_output = IResult::Error(Err::Position(ErrorKind::Alpha, " "));

    let result = parse_identifier(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_identifier3() {
    let input = "5";
    let expected_output = IResult::Error(Err::Position(ErrorKind::Alpha, "5"));

    let result = parse_identifier(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_identifier4() {
    let input = "g";
    let expected_output = IResult::Done("", FortranTokenType::Identifier("g".to_owned()));

    let result = parse_identifier(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_identifier5() {
    let input = "abc";
    let expected_output = IResult::Done("", FortranTokenType::Identifier("abc".to_owned()));

    let result = parse_identifier(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_identifier6() {
    let input = "abc_123";
    let expected_output = IResult::Done("", FortranTokenType::Identifier("abc_123".to_owned()));

    let result = parse_identifier(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_identifier7() {
    let input = "abc_123.";
    let expected_output = IResult::Done(".", FortranTokenType::Identifier("abc_123".to_owned()));

    let result = parse_identifier(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_identifier8() {
    let input = "abc_123.xyz.";
    let expected_output = IResult::Done(".xyz.", FortranTokenType::Identifier("abc_123".to_owned()));

    let result = parse_identifier(input);

    assert_eq!(result, expected_output);
}
