use nom::{IResult, Needed, Err, ErrorKind};

use super::super::{FortranTokenType, parse_maybe_comment};

#[test]
fn parse_maybe_comment1() {
    let input = "";
    let expected_output = IResult::Done("", None);

    let result = parse_maybe_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_maybe_comment2() {
    let input = " ";
    let expected_output = IResult::Done(" ", None);

    let result = parse_maybe_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_maybe_comment3() {
    let input = "x";
    let expected_output = IResult::Done("x", None);

    let result = parse_maybe_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_maybe_comment4() {
    let input = "!";
    let expected_output = IResult::Done("", Some(FortranTokenType::Comment("".to_owned())));

    let result = parse_maybe_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_maybe_comment5() {
    let input = "! Test1";
    let expected_output = IResult::Done("", Some(FortranTokenType::Comment(" Test1".to_owned())));

    let result = parse_maybe_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_maybe_comment6() {
    let input = "! Test2\n";
    let expected_output = IResult::Done("\n", Some(FortranTokenType::Comment(" Test2".to_owned())));

    let result = parse_maybe_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_maybe_comment7() {
    let input = "! Test3\n\n";
    let expected_output = IResult::Done("\n\n", Some(FortranTokenType::Comment(" Test3".to_owned())));

    let result = parse_maybe_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_maybe_comment8() {
    let input = "! Test4\n x = 5 ! comment";
    let expected_output = IResult::Done("\n x = 5 ! comment", Some(FortranTokenType::Comment(" Test4".to_owned())));

    let result = parse_maybe_comment(input);

    assert_eq!(result, expected_output);
}
