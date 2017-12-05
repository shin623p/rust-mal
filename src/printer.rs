use std::io::stdout;
use std::io::Write;
use types::MalType;

pub fn pr_str(mal: &MalType) {
    match mal {
        &MalType::Symbol(ref sym) => print!("{}", sym),
        &MalType::Number(ref num) => print!("{}", num),
        &MalType::MalList(ref lst) => {
            print!("(");
            for (index, elem) in lst.iter().enumerate() {
                pr_str(elem);
                if index < lst.len() - 1 {
                    print!(" ");
                }
            }
            print!(")");
        }
    }
    let _ = stdout().flush();
}
