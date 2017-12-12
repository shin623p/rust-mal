use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum MalType {
    Number(isize),
    Symbol(String),
    List(Vec<Rc<MalType>>),
    Func(fn(Vec<Rc<MalType>>) -> Result<Rc<MalType>, String>),
}
