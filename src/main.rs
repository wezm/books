use scheme::parse_expr;

fn main() {
    for arg in std::env::args().skip(1) {
        println!("{}", read_expr(&arg));
    }
}

fn read_expr(input: &str) -> String {
    match parse_expr(input) {
        Ok((_, val)) => format!("Found {}", val),
        Err(err) => format!("No match: {}", err),
    }
}
