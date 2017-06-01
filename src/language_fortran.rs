// External module
use nom;

// Grammer for Fortran code
named!(fortran_code<&str, &str>, alt!(program | module));

named!(program<&str, &str>, do_parse!(
    ws!(tag!("program")) >>
    ws!(identifier) >>

    statements >>

    ws!(tag!("end program")) >>
    ws!(identifier)
));

named!(module<&str, &str>, do_parse!(
    ws!(tag!("module")) >>
    ws!(identifier) >>

    statements >>

    ws!(tag!("end module")) >>
    ws!(identifier)
));

named!(identifier<&str, &str>, do_parse!(
    alpha >>
    many0!(alt!(alplanumeric | tag!("_")))
));

named!(statement<&str, &str>, do_parse!(
    many0!(stmt_use) >>
    many0!(stmt_implicit) >>
    many0!(declaration)
));

named!(stmt_use<&str, &str>, do_parse!(
    ws!(tag!("use")) >>
    ws!(identifier)
));

named!(stmt_implicit<&str, &str>, do_parse!(
    ws!(tag!("implicit none"))
));

named!(declaration<&str, &str>, do_parse!(
    ws!(type) >>
    ws!(tag!("::")) >>
    ws!(identifier)
));

pub fn process_language_fortran(file_name: &str) {
    println!("processing fortran source file: '{}'", file_name);
}
