use std::fs::File;
use std::io::Read;

#[derive(PartialEq, Debug)]
enum FortranTokenType {
    Comment(String),         // Simgle line comment with !
    StringLiteral(String),   // Can be in " or '
    Boolean(bool),           // .true. or .false.
    IntegerI64(i64),         // 12, +67, 88
    IntegerU64(u64),         // -12, -67, -88
    FloatF64(f64),           // 1.4, -86.345, 2.6e3
    Identifier(String),      // distance, particle_id
    OperatorPlus,       // +
    OperatorMinus,      // -
    OperatorMul,        // *
    OperatorDiv,        // /
    OperatorEq,         // ==, .eq.
    OperatorNeq,        // /=, .neq.
    OperatorLT,         // <=, .leq.
    OperatorGT,         // >=, .geq.
    OperatorStruct,     // %
    Assign,             // =
    LParen,             // (
    RParen,             // )
    LBracket,           // [
    RBracket,           // ]
    Comma,              // ,
    ColonColon,         // ::

    // All keywords:
    KeywordEnd,
    KeywordProgram,
    KeywordUse,
    KeywordImplicit,
    KeywordNone,
    KeywordReal,
    KeywordPrint,
    KeywordIf,
    KeywordThen,
    KeywordElse,
    KeywordDo,
    KeywordSubroutine,

}

#[derive(PartialEq, Debug)]
struct Token {
    line_number: u32,
    column: u32,
    token: FortranTokenType
}

#[derive(PartialEq, Debug)]
enum ParseMode {
    NoMode,
    StringLiteral,
    Identifier,
    Number,
    Comment,
}

pub fn process_language_fortran(file_name: &str) {
    println!("processing fortran source file: '{}'", file_name);

    let mut buffer = String::new();

    let mut f = File::open(file_name).expect("unable to open file");

    f.read_to_string(&mut buffer).expect("unable to read to string");

    process_fortran_source_code(&mut buffer)
}


fn process_fortran_source_code(buffer: &str) {

    for (line_number, line) in (1..).zip(buffer.lines()) {
        println!("{}: {}", line_number, line);
        let tokens = split_into_fortran_tokens(line, line_number);
        println!("token: {:?}", tokens);
    }
}

fn split_into_fortran_tokens(line: &str, line_number: u32) -> Vec<Token> {
    let mut result = Vec::<Token>::new();
    let mut mode = ParseMode::NoMode;
    let mut content = String::new();
    let mut token_column: u32 = 1;

    for (column, c) in (1..).zip(line.chars()) {
        match mode {
            ParseMode::NoMode => {
                match c {
                    ' ' | '\n' => {},
                    '!' => {
                        mode = ParseMode::Comment;
                        token_column = column;
                    },
                    'a' ... 'z' | 'A' ... 'Z' => {
                        mode = ParseMode::Identifier;
                        token_column = column;
                        content.push(c)
                    }
                    '0' ... '9' | '+' | '-' => {
                        mode = ParseMode::Number;
                        token_column = column;
                        content.push(c)
                    }
                    '"' | '\'' => {
                        mode = ParseMode::StringLiteral;
                        token_column = column;
                    },
                    _ => {} // TODO
                }
            },
            ParseMode::StringLiteral => {

            },
            ParseMode::Identifier => {
                match c {
                    'a' ... 'z' | 'A' ... 'Z' | '0' ... '9' | '_' => {
                        content.push(c)
                    },
                    ' ' | '\n' => {
                        result.push(Token{
                            line_number: line_number,
                            column: token_column,
                            token: FortranTokenType::Identifier(content.clone())});
                        content = String::new();
                        mode = ParseMode::NoMode
                    },
                    _ => { // TODO
                        mode = ParseMode::NoMode
                    }
                }
            },
            ParseMode::Number => {

            },
            ParseMode::Comment => {
                content.push(c)
            }
        }
    }

    match mode {
        ParseMode::NoMode => {
        },
        ParseMode::StringLiteral => {

        },
        ParseMode::Identifier => {
            result.push(Token{
                line_number: line_number,
                column: token_column,
                token: FortranTokenType::Identifier(content)});
        },
        ParseMode::Number => {

        },
        ParseMode::Comment => {
            result.push(Token{
                line_number: line_number,
                column: token_column,
                token: FortranTokenType::Comment(content)})
        }
    }

    result
}
