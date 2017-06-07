use nom::{IResult, Needed, Err, ErrorKind};

use super::super::{FortranTokenType, parse_module};

#[test]
fn parse_module1() {
    let input = "";
    let expected_output = IResult::Incomplete(Needed::Size(6));

    let result = parse_module(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_module2() {
    let input = "module";
    let expected_output = IResult::Incomplete(Needed::Unknown);

    let result = parse_module(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_module3() {
    let input = "module test_comment";
    let expected_output = IResult::Incomplete(Needed::Size(20));

    let result = parse_module(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_module4() {
    let input = "module test_comment\nend";
    let expected_output = IResult::Incomplete(Needed::Size(29));

    let result = parse_module(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_module5() {
    let input = "module test_comment\nend module";
    let expected_output = IResult::Incomplete(Needed::Unknown);

    let result = parse_module(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_module6() {
    let input = "module test_comment\nend module test_comment";
    let expected_output = IResult::Done("", Vec::new());

    let result = parse_module(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_module7() {
    let input = "module test_comment ! Test \nend module test_comment";
    let expected_output = IResult::Done("", Vec::new());

    let result = parse_module(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_module8() {
    let input = "module test_comment ! Test \nend module test_comment ! Test";
    let expected_output = IResult::Done("", Vec::new());

    let result = parse_module(input);

    assert_eq!(result, expected_output);
}
