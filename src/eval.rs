use crate::LispVal;

impl LispVal {
    pub fn eval(&self) -> Self {
        match self {
            LispVal::Atom(_) => todo!(),
            LispVal::List(list) => match list.as_slice() {
                [LispVal::Atom(atom), val] if atom == "quote" => val.clone(),
                _ => todo!(),
            },
            LispVal::DottedList(_, _) => todo!(),
            LispVal::Number(_) => self.clone(),
            LispVal::String(_) => self.clone(),
            LispVal::Bool(_) => self.clone(),
        }
    }
}
