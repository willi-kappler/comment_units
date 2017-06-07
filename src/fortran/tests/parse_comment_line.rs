use nom::{IResult};

use super::super::{FortranTokenType, custom_nom_error_option, custom_nom_error, parse_comment_line};

#[test]
fn parse_comment_line1() {
    let input = "";
    let expected_output = custom_nom_error(2);

    let result = parse_comment_line(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment_line2() {
    let input = " ";
    let expected_output = custom_nom_error(2);

    let result = parse_comment_line(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment_line3() {
    let input = "!";
    let expected_output = IResult::Done("", FortranTokenType::Comment("".to_owned()));

    let result = parse_comment_line(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment_line4() {
    let input = " !";
    let expected_output = IResult::Done(" ", FortranTokenType::Comment("".to_owned()));

    let result = parse_comment_line(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment_line5() {
    let input = " ! ";
    let expected_output = IResult::Done(" ", FortranTokenType::Comment(" ".to_owned()));

    let result = parse_comment_line(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment_line6() {
    let input = "! ";
    let expected_output = IResult::Done("", FortranTokenType::Comment(" ".to_owned()));

    let result = parse_comment_line(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_comment_line7() {
    let input = "! Test";
    let expected_output = IResult::Done("", FortranTokenType::Comment(" Test".to_owned()));

    let result = parse_comment_line(input);

    assert_eq!(result, expected_output);
}
