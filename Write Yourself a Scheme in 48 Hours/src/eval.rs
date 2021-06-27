use crate::{Integer, LispVal};
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

impl LispVal {
    pub fn eval(&self) -> Self {
        match self {
            LispVal::Atom(_) => todo!(),
            LispVal::List(list) => match list.as_slice() {
                [LispVal::Atom(atom), val] if atom == "quote" => val.clone(),
                &[LispVal::Atom(ref func), ref args @ ..] => {
                    apply(func, args.iter().map(|arg| arg.eval()))
                }
                _ => todo!(),
            },
            LispVal::DottedList(_, _) => todo!(),
            LispVal::Number(_) => self.clone(),
            LispVal::String(_) => self.clone(),
            LispVal::Bool(_) => self.clone(),
        }
    }
}

fn apply(func: &str, args: impl Iterator<Item = LispVal>) -> LispVal {
    match primitives(func, args) {
        Some(val) => val,
        None => LispVal::Bool(false),
    }
}

fn primitives(op: &str, args: impl Iterator<Item = LispVal>) -> Option<LispVal> {
    match op {
        "+" => Some(numeric_bin_op(Integer::add, args)),
        "-" => Some(numeric_bin_op(Integer::sub, args)),
        "*" => Some(numeric_bin_op(Integer::mul, args)),
        "/" => Some(numeric_bin_op(Integer::div, args)),
        _ => {
            dbg!(op);
            None
        }
    }
}

fn numeric_bin_op(
    op: fn(Integer, Integer) -> Integer,
    params: impl Iterator<Item = LispVal>,
) -> LispVal {
    params
        .map(|param| unpack_num(&param))
        .fold1(op)
        .map(LispVal::Number)
        .unwrap()
}

fn unpack_num(val: &LispVal) -> Integer {
    match val {
        LispVal::Number(num) => *num,
        LispVal::String(s) => Integer::from_str(&s).unwrap_or(0),
        LispVal::List(list) => {
            match list.as_slice() {
                [n] => unpack_num(n),
                _ => dbg!(0), // the book says this will be an error later
            }
        }
        _ => {
            dbg!(val);
            0 // really?
        }
    }
}

trait IteratorExt: Iterator {
    fn fold1<F>(mut self, f: F) -> Option<Self::Item>
    where
        F: FnMut(Self::Item, Self::Item) -> Self::Item,
        Self: Sized,
    {
        self.next().map(move |x| self.fold(x, f))
    }
}

impl<T: ?Sized> IteratorExt for T where T: Iterator {}
