use nom::{IResult, Needed, Err, ErrorKind};

use super::super::{FortranTokenType, parse_comment};

#[test]
fn parse_comment1() {
    let input = "";
    let expected_output = IResult::Incomplete(Needed::Size(1));

    let result = parse_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment2() {
    let input = " ";
    let expected_output = IResult::Error(Err::Position(ErrorKind::Tag, " "));

    let result = parse_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment3() {
    let input = "x";
    let expected_output = IResult::Error(Err::Position(ErrorKind::Tag, "x"));

    let result = parse_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment4() {
    let input = "!";
    let expected_output = IResult::Done("", FortranTokenType::Comment("".to_owned()));

    let result = parse_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment5() {
    let input = "! Test1";
    let expected_output = IResult::Done("", FortranTokenType::Comment(" Test1".to_owned()));

    let result = parse_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment6() {
    let input = "! Test2\n";
    let expected_output = IResult::Done("\n", FortranTokenType::Comment(" Test2".to_owned()));

    let result = parse_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment7() {
    let input = "! Test3\n\n";
    let expected_output = IResult::Done("\n\n", FortranTokenType::Comment(" Test3".to_owned()));

    let result = parse_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment8() {
    let input = "! Test4\n x = 5 ! comment";
    let expected_output = IResult::Done("\n x = 5 ! comment", FortranTokenType::Comment(" Test4".to_owned()));

    let result = parse_comment(input);

    assert_eq!(result, expected_output);
}
