use nom::{IResult, Needed, Err, ErrorKind};

use super::super::{FortranTokenType, parse_identifier_list};

#[test]
fn parse_identifier_list1() {
    let input = "";
    let expected_output = IResult::Incomplete(Needed::Unknown);

    let result = parse_identifier_list(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_identifier_list2() {
    let input = " ";
    let expected_output = IResult::Error(Err::Position(ErrorKind::Alpha, " "));

    let result = parse_identifier_list(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_identifier_list3() {
    let input = "5";
    let expected_output = IResult::Error(Err::Position(ErrorKind::Alpha, "5"));

    let result = parse_identifier_list(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_identifier_list4() {
    let input = "g";
    let expected_output = IResult::Done("", vec![FortranTokenType::Identifier("g".to_owned())]);

    let result = parse_identifier_list(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_identifier_list5() {
    let input = "abc";
    let expected_output = IResult::Done("", vec![FortranTokenType::Identifier("abc".to_owned())]);

    let result = parse_identifier_list(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_identifier_list6() {
    let input = "abc_123";
    let expected_output = IResult::Done("", vec![FortranTokenType::Identifier("abc_123".to_owned())]);

    let result = parse_identifier_list(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_identifier_list7() {
    let input = "abc_123,";
    let expected_output = IResult::Incomplete(Needed::Unknown);

    let result = parse_identifier_list(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_identifier_list8() {
    let input = "abc_123,xyz";
    let expected_output = IResult::Done("", vec![
        FortranTokenType::Identifier("abc_123".to_owned()),
        FortranTokenType::Identifier("xyz".to_owned())
        ]);

    let result = parse_identifier_list(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_identifier_list9() {
    let input = "abc_123 ,   xyz    ,  person_id";
    let expected_output = IResult::Done("", vec![
        FortranTokenType::Identifier("abc_123".to_owned()),
        FortranTokenType::Identifier("xyz".to_owned()),
        FortranTokenType::Identifier("person_id".to_owned())
        ]);

    let result = parse_identifier_list(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_identifier_list10() {
    let input = "abc_123, xyz, person_id  ! These are very important";
    let expected_output = IResult::Done("! These are very important", vec![
        FortranTokenType::Identifier("abc_123".to_owned()),
        FortranTokenType::Identifier("xyz".to_owned()),
        FortranTokenType::Identifier("person_id".to_owned())
        ]);

    let result = parse_identifier_list(input);

    assert_eq!(result, expected_output);
}

#[test]
fn parse_identifier_list11() {
    let input = "abc_123, xyz, person_id\n next_item";
    let expected_output = IResult::Done("next_item", vec![
        FortranTokenType::Identifier("abc_123".to_owned()),
        FortranTokenType::Identifier("xyz".to_owned()),
        FortranTokenType::Identifier("person_id".to_owned())
        ]);

    let result = parse_identifier_list(input);

    assert_eq!(result, expected_output);
}
