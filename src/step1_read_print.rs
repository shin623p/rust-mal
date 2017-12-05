use std::io;
use std::io::stdout;
use std::io::Write;
use types::MalType;
use reader::read_str;
use printer::pr_str;

mod reader;
mod types;
mod printer;

fn read() -> Option<MalType> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) => None,
        Ok(_) => read_str(input),
        _ => None,
    }
}

fn eval(eval_mal: MalType) -> Option<MalType> {
    Some(eval_mal)
}

fn print(print_mal: MalType) {
    pr_str(&print_mal);
}

fn rep() {
    print!("user> ");
    let _ = stdout().flush();
    match read().and_then(|s| eval(s)) {
        Some(s) => {
            print(s);
            println!("");
            rep()
        }
        _ => return,
    };
}

fn main() {
    rep();
}
