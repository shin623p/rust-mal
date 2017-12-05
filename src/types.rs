
pub enum MalType {
    Number(isize),
    Symbol(String),
    MalList(Vec<MalType>),
}
