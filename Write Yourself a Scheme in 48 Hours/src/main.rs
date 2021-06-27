use scheme::{parse_expr, LispVal};

fn main() {
    for arg in std::env::args().skip(1) {
        println!("{}", read_expr(&arg).eval());
    }
}

fn read_expr(input: &str) -> LispVal {
    parse_expr(input)
        .map(|(_i, val)| val)
        .unwrap_or_else(|err| LispVal::String(format!("No match: {}", err)))
}
