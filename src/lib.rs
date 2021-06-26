mod parse;

#[derive(Debug)]
pub enum LispVal {
    Atom(String),
    List(Vec<LispVal>),
    DottedList(Vec<LispVal>, Box<LispVal>),
    Number(isize),
    String(String),
    Bool(bool),
}

pub use parse::parse_expr;
