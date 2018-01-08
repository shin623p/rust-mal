use std::collections::HashMap;
use std::rc::Rc;

use types::MalType;

struct Env {
    outer: HashMap<String, Rc<MalType>>,
}

impl Env {
    fn set(&mut self, sym: Rc<MalType>, val: Rc<MalType>) {
        if let MalType::Symbol(ref s) = *sym {
            self.outer.insert(s.clone(), val);
        }
    }
}
