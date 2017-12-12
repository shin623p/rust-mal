use std::io::stdout;
use std::io::Write;
use types::MalType;
use std::rc::Rc;

pub fn pr_str(mal: Rc<MalType>) {
    match *mal {
        MalType::Symbol(ref sym) => print!("{}", sym),
        MalType::Number(ref num) => print!("{}", num),
        MalType::List(ref lst) => {
            print!("(");
            for (index, elem) in lst.iter().enumerate() {
                //pr_str(lst[index].clone());
                pr_str(elem.clone());
                if index < lst.len() - 1 {
                    print!(" ");
                }
            }
            print!(")");
        }
        MalType::Func(_) => print!("func"),
        //_ => print!("Err"),
    }
    let _ = stdout().flush();
}
