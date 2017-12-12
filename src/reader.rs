extern crate regex;

use self::regex::Regex;
use types::MalType;
use std::rc::Rc;

struct Reader {
    tokens: Vec<String>,
    position: usize,
}

impl Reader {
    fn next(&mut self) -> Option<String> {
        self.position = self.position + 1;
        Some(self.tokens[self.position - 1].to_string())
    }
    fn peek(&self) -> Option<String> {
        Some(self.tokens[self.position].to_string())
    }
}

pub fn read_str(str: String) -> Option<MalType> {
    let mut reader = Reader {
        tokens: tokenizer(str),
        position: 0,
    };
    read_form(&mut reader)
}

fn tokenizer(str: String) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let re =
        Regex::new(r##"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"|;.*|[^\s\[\]{}('"`,;)]*)"##)
            .unwrap();
    for caps in re.captures_iter(str.as_str()) {
        tokens.push(caps.get(1).unwrap().as_str().to_string());
    }
    tokens
}

fn read_form(rd: &mut Reader) -> Option<MalType> {
    match &rd.peek().unwrap()[..] {
        "(" => read_list(rd),
        _ => read_atom(rd),
    }
}

fn read_list(rd: &mut Reader) -> Option<MalType> {
    let mut list: Vec<Rc<MalType>> = Vec::new();
    let _ = rd.next();
    loop {
        match &rd.peek().unwrap()[..] {
            ")" => return Some(MalType::List(list)),
            _ => list.push(Rc::new(read_form(rd).unwrap())),
        }
    }
}

fn read_atom(rd: &mut Reader) -> Option<MalType> {
    let token = rd.next().unwrap();
    let re = Regex::new(r"^-?[0-9]+$").unwrap();
    if re.is_match(&token[..]) {
        Some(MalType::Number(token.parse().unwrap()))
    } else {
        Some(MalType::Symbol(token))
    }
}
