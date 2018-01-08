use std::io;
use std::io::stdout;
use std::io::Write;
use types::MalType;
use reader::read_str;
use printer::pr_str;
use std::collections::HashMap;
use std::rc::Rc;

mod reader;
mod types;
mod printer;
mod env;

fn read() -> Option<MalType> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) => None,
        Ok(_) => read_str(input),
        _ => None,
    }
}

fn eval(ast: Rc<MalType>, env: &HashMap<String, Rc<MalType>>) -> Result<Rc<MalType>, String> {
    match *ast {
        MalType::List(ref lst) => {
            if lst.len() == 0 {
                Ok(Rc::new(MalType::List(lst.clone())))
            } else {
                let mut new_list: Vec<Rc<MalType>> = Vec::new();
                for mal in lst.clone() {
                    match eval_ast(mal, env) {
                        Ok(new_ast) => new_list.push(new_ast),
                        Err(e) => return Err(e),
                    }
                }
                match *new_list[0] {
                    MalType::Func(f) => f(new_list[1..].to_vec()),
                    _ => Err("1st value is not Function.".to_string()),
                }
            }
        }
        ref mal => eval_ast(Rc::new(mal.clone()), env),
    }
}

fn eval_ast(ast: Rc<MalType>, env: &HashMap<String, Rc<MalType>>) -> Result<Rc<MalType>, String> {
    match *ast {
        MalType::List(ref lst) => eval(Rc::new(MalType::List(lst.clone())), env),
        MalType::Symbol(ref sym) => {
            match env.get(sym) {
                Some(f) => Ok(f.clone()),
                None => Err(format!("{} is no value in the environment structurewrong.", sym)),
            }
        }
        ref mal => Ok(Rc::new(mal.clone())),
    }
}

fn print(print_mal: Rc<MalType>) {
    pr_str(print_mal);
}

fn rep() {
    let mut repl_env: HashMap<String, Rc<MalType>> = HashMap::new();
    repl_env.insert("+".to_string(), Rc::new(MalType::Func(add)));
    repl_env.insert("-".to_string(), Rc::new(MalType::Func(sub)));
    repl_env.insert("*".to_string(), Rc::new(MalType::Func(mul)));
    repl_env.insert("/".to_string(), Rc::new(MalType::Func(div)));

    loop {
        print!("user> ");
        let _ = stdout().flush();
        let read_ast = read().unwrap();
        match eval_ast(Rc::new(read_ast), &repl_env) {
            Ok(ret) => print(ret),
            Err(e) => print!("Error: {}", e),
        }
        println!("");
    }
}

fn add(args: Vec<Rc<MalType>>) -> Result<Rc<MalType>, String> {
    if let MalType::Number(ref x) = *args[0] {
        if let MalType::Number(ref y) = *args[1] {
            Ok(Rc::new(MalType::Number(*x + *y)))
        } else {
            Err("2nd value is not number!!".to_string())
        }
    } else {
        Err("1st value is not number!!".to_string())
    }
}

fn sub(args: Vec<Rc<MalType>>) -> Result<Rc<MalType>, String> {
    if let MalType::Number(ref x) = *args[0] {
        if let MalType::Number(ref y) = *args[1] {
            Ok(Rc::new(MalType::Number(*x - *y)))
        } else {
            Err("2nd value is not number!!".to_string())
        }
    } else {
        Err("1st value is not number!!".to_string())
    }
}

fn mul(args: Vec<Rc<MalType>>) -> Result<Rc<MalType>, String> {
    if let MalType::Number(ref x) = *args[0] {
        if let MalType::Number(ref y) = *args[1] {
            Ok(Rc::new(MalType::Number(*x * *y)))
        } else {
            Err("2nd value is not number!!".to_string())
        }
    } else {
        Err("1st value is not number!!".to_string())
    }
}

fn div(args: Vec<Rc<MalType>>) -> Result<Rc<MalType>, String> {
    if let MalType::Number(ref x) = *args[0] {
        if let MalType::Number(ref y) = *args[1] {
            Ok(Rc::new(MalType::Number(*x / *y)))
        } else {
            Err("2nd value is not number!!".to_string())
        }
    } else {
        Err("1st value is not number!!".to_string())
    }
}

fn main() {
    rep();
}
