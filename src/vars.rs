pub struct Var {
    pub name: String,
    pub value: String,
}

impl Var {
    fn new(name: String, value: String) -> Var {
        Var { name, value }
    }
}

pub struct VarVector {
    pub vars: Vec<Var>,
}

impl VarVector {
    fn new() -> VarVector {
        VarVector { vars: Vec::new() }
    }

    fn add(&mut self, name: String, value: String) {
        self.vars.push(Var::new(name, value));
    }
}
