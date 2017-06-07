use nom::{IResult};

use super::super::{FortranTokenType, custom_nom_error_option, custom_nom_error, parse_comment};

#[test]
fn parse_comment1() {
    let input = "";
    let expected_output = IResult::Done("", None);

    let result = parse_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment2() {
    let input = " ";
    let expected_output = IResult::Done(" ", None);

    let result = parse_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment3() {
    let input = "!";
    let expected_output = IResult::Done("", Some(FortranTokenType::Comment("".to_owned())));

    let result = parse_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment4() {
    let input = " !";
    let expected_output = IResult::Done(" ", Some(FortranTokenType::Comment("".to_owned())));

    let result = parse_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment5() {
    let input = " ! ";
    let expected_output = IResult::Done(" ", Some(FortranTokenType::Comment(" ".to_owned())));

    let result = parse_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment6() {
    let input = "!Test";
    let expected_output = IResult::Done("", Some(FortranTokenType::Comment("Test".to_owned())));

    let result = parse_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment7() {
    let input = "! Test";
    let expected_output = IResult::Done("", Some(FortranTokenType::Comment(" Test".to_owned())));

    let result = parse_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment8() {
    let input = " ! Test";
    let expected_output = IResult::Done(" ", Some(FortranTokenType::Comment(" Test".to_owned())));

    let result = parse_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment9() {
    let input = "x";
    let expected_output = custom_nom_error_option(1);

    let result = parse_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment10() {
    let input = " x";
    let expected_output = custom_nom_error_option(1);

    let result = parse_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment11() {
    let input = " x ";
    let expected_output = custom_nom_error_option(1);

    let result = parse_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment12() {
    let input = " x!";
    let expected_output = custom_nom_error_option(1);

    let result = parse_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment13() {
    let input = " x !";
    let expected_output = custom_nom_error_option(1);

    let result = parse_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment14() {
    let input = " x ! ";
    let expected_output = custom_nom_error_option(1);

    let result = parse_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment15() {
    let input = " x !Test";
    let expected_output = custom_nom_error_option(1);

    let result = parse_comment(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment16() {
    let input = " x ! Test";
    let expected_output = custom_nom_error_option(1);

    let result = parse_comment(input);

    assert_eq!(result, expected_output);
}
