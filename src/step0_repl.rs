use std::io;
use std::io::stdout;
use std::io::Write;

fn read() -> Option<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) => None,
        Ok(_) => Some(input),
        _ => None,
    }
}

fn eval(eval_str: String) -> Option<String> {
    Some(eval_str)
}

fn print(print_str: String) {
    println!("{}", print_str);
}

fn rep() {
    print!("user> ");
    stdout().flush();
    match (read().and_then(|s| eval(s))) {
        Some(s) => {
            print(s);
            rep()
        }
        _ => return,
    };
}

fn main() {
    rep();
}
