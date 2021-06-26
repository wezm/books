use nom::branch::alt;
use nom::bytes::complete::{tag, take_while, take_while1};
use nom::character::complete::{char, digit1, one_of};
use nom::combinator::{map, map_res};
use nom::multi::{many0, separated_list0, separated_list1};
use nom::sequence::{delimited, tuple};
use nom::IResult;
use std::str::FromStr;

use crate::LispVal;

fn symbol(input: &str) -> IResult<&str, char> {
    one_of("!$%&|*+-/:<=?>@^_~#")(input)
}

fn letter(input: &str) -> IResult<&str, char> {
    one_of("abcdefghijklmnopqrstuvwxyz")(input)
}

fn digit(input: &str) -> IResult<&str, char> {
    one_of("0123456789")(input)
}

fn spaces(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_ascii_whitespace())(input)
}

fn parse_string(input: &str) -> IResult<&str, LispVal> {
    map(
        delimited(tag("\""), take_while(|c| c != '"'), tag("\"")),
        |o: &str| LispVal::String(o.to_string()),
    )(input)
}

fn parse_atom(input: &str) -> IResult<&str, LispVal> {
    map(
        tuple((alt((letter, symbol)), many0(alt((letter, digit, symbol))))),
        |(first, rest)| {
            let mut out = first.to_string();
            out.extend(rest.iter());
            let val = match out.as_str() {
                "#t" => LispVal::Bool(true),
                "#f" => LispVal::Bool(false),
                _ => LispVal::Atom(out),
            };
            val
        },
    )(input)
}

fn parse_number(input: &str) -> IResult<&str, LispVal> {
    map_res(digit1, |out| {
        isize::from_str(out).map(|val| LispVal::Number(val))
    })(input)
}

fn parse_list_inner(input: &str) -> IResult<&str, LispVal> {
    map(separated_list0(spaces, parse_expr), |list| {
        LispVal::List(list)
    })(input)
}

fn parse_dotted_list(input: &str) -> IResult<&str, LispVal> {
    let (input, head) = separated_list1(spaces, parse_expr)(input)?;
    let (input, _) = tuple((spaces, char('.'), spaces))(input)?;
    parse_expr(input).map(|(i, tail)| (i, LispVal::DottedList(head, Box::from(tail))))
}

fn parse_list(input: &str) -> IResult<&str, LispVal> {
    alt((
        delimited(tag("("), parse_list_inner, tag(")")),
        delimited(tag("("), parse_dotted_list, tag(")")),
    ))(input)
}

fn parse_quoted(input: &str) -> IResult<&str, LispVal> {
    use LispVal::*;
    map(tuple((char('\''), parse_expr)), |(_, x)| {
        List(vec![Atom("quote".to_string()), x])
    })(input)
}

pub fn parse_expr(input: &str) -> IResult<&str, LispVal> {
    alt((
        parse_atom,
        parse_string,
        parse_number,
        parse_quoted,
        parse_list,
    ))(input)
}
