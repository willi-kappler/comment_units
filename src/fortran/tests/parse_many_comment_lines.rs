use nom::{IResult, Needed};

use super::super::{FortranTokenType, parse_many_comment_lines};

#[test]
fn parse_many_comment_lines1() {
    let input = "";
    let expected_output = IResult::Done("", Vec::new());

    let result = parse_many_comment_lines(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_many_comment_lines2() {
    let input = " ";
    let expected_output = IResult::Incomplete(Needed::Size(2));

    let result = parse_many_comment_lines(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_many_comment_lines3() {
    let input = "x";
    let expected_output = IResult::Done("x", Vec::new());

    let result = parse_many_comment_lines(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_many_comment_lines4() {
    let input = "!";
    let expected_output = IResult::Done("", vec![FortranTokenType::Comment("".to_owned())]);

    let result = parse_many_comment_lines(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_many_comment_lines5() {
    let input = " ! ";
    let expected_output = IResult::Done("", vec![FortranTokenType::Comment(" ".to_owned())]);

    let result = parse_many_comment_lines(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_many_comment_lines6() {
    let input = " ! Test";
    let expected_output = IResult::Done("", vec![FortranTokenType::Comment(" Test".to_owned())]);

    let result = parse_many_comment_lines(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_many_comment_lines7() {
    let input = " ! Test\n";
    let expected_output = IResult::Done("", vec![FortranTokenType::Comment(" Test".to_owned())]);

    let result = parse_many_comment_lines(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_many_comment_lines8() {
    let input = " ! Test1\n ! Test2";
    let expected_output = IResult::Done("", vec![
        FortranTokenType::Comment(" Test1".to_owned()),
        FortranTokenType::Comment(" Test2".to_owned())
    ]);

    let result = parse_many_comment_lines(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_many_comment_lines9() {
    let input = " ! Test1\n \n     \n  \n \n\n\n  ! Test2\n \n \n\n    \n";
    let expected_output = IResult::Done("", vec![
        FortranTokenType::Comment(" Test1".to_owned()),
        FortranTokenType::Comment(" Test2".to_owned())
    ]);

    let result = parse_many_comment_lines(input);

    assert_eq!(result, expected_output);
}
