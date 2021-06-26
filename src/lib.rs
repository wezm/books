mod eval;
mod parse;

use std::fmt::Write;
use std::fmt::{self, Display, Formatter};

pub use parse::parse_expr;

#[derive(Debug, Clone)]
pub enum LispVal {
    Atom(String),
    List(Vec<LispVal>),
    DottedList(Vec<LispVal>, Box<LispVal>),
    Number(isize),
    String(String),
    Bool(bool),
}

impl Display for LispVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LispVal::Atom(name) => name.fmt(f),
            LispVal::List(list) => {
                f.write_char('(')?;
                fmt_list(list, f)?;
                f.write_char(')')
            }
            LispVal::DottedList(head, tail) => {
                f.write_char('(')?;
                fmt_list(head, f)?;
                f.write_str(" . ")?;
                tail.fmt(f)?;
                f.write_char(')')
            }
            LispVal::Number(num) => num.fmt(f),
            LispVal::String(s) => {
                f.write_char('"')?;
                f.write_str(s)?;
                f.write_char('"')
            }
            LispVal::Bool(true) => f.write_str("#t"),
            LispVal::Bool(false) => f.write_str("#f"),
        }
    }
}

fn fmt_list(list: &[LispVal], f: &mut Formatter<'_>) -> fmt::Result {
    list.iter().enumerate().try_for_each(|(i, val)| {
        if i > 0 {
            f.write_char(' ')?;
        }
        val.fmt(f)
    })
}
