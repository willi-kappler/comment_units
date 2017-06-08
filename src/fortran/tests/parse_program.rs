use nom::{IResult, Needed, Err, ErrorKind};

use super::super::{FortranTokenType, parse_program};

#[test]
fn parse_program1() {
    let input = "";
    let expected_output = IResult::Incomplete(Needed::Size(7));

    let result = parse_program(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_program2() {
    let input = "x";
    let expected_output = IResult::Error(Err::Position(ErrorKind::Tag, "x"));

    let result = parse_program(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_program3() {
    let input = "program";
    let expected_output = IResult::Incomplete(Needed::Unknown);

    let result = parse_program(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_program4() {
    let input = "program test_comment";
    let expected_output = IResult::Incomplete(Needed::Size(28));

    let result = parse_program(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_program5() {
    let input = "program test_comment\nend";
    let expected_output = IResult::Incomplete(Needed::Size(31));

    let result = parse_program(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_program6() {
    let input = "program test_comment\nend program";
    let expected_output = IResult::Incomplete(Needed::Unknown);

    let result = parse_program(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_program7() {
    let input = "program test_comment\nend program test_comment";
    let expected_output = IResult::Done("", Vec::new());

    let result = parse_program(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_program8() {
    let input = "program test_comment ! Test \nend program test_comment";
    let expected_output = IResult::Done("", Vec::new());

    let result = parse_program(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_program9() {
    let input = "program test_comment ! Test \nend program test_comment ! Test";
    let expected_output = IResult::Done("", Vec::new());

    let result = parse_program(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_program10() {
    let input = "program test_comment ! Test
        ! Some comments
        use util
        use particle ! Has struct definition

        end program test_comment ! Test";
    let expected_output = IResult::Done("", Vec::new());

    let result = parse_program(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_program11() {
    let input = "program test_comment ! Test
        ! Some comments
        implicit none

        end program test_comment ! Test";
    let expected_output = IResult::Done("", Vec::new());

    let result = parse_program(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_program12() {
    let input = "program test_comment ! Test
        ! Some comments
        use util
        implicit none ! Good practise

        ! Code goes here

        end program test_comment ! Test";
    let expected_output = IResult::Done("", Vec::new());

    let result = parse_program(input);

    assert_eq!(result, expected_output);
}
