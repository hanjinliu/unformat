use std::collections::HashMap;
// "{a:str}" -> Var { name: "a", fmt: "str" }
#[derive(Debug, PartialEq, Eq)]
pub struct Var {
    pub name: String,
    pub fmt: Option<String>,
}

pub struct VarHashmap {
    pub vars: HashMap<String, Var>,
}

impl VarHashmap {
    pub fn new() -> VarHashmap {
        VarHashmap { vars: HashMap::new() }
    }

    pub fn add(&mut self, var: Var) {
        self.vars.insert(var.name.clone(), var);
    }

    pub fn get(&self, name: &str) -> Option<&Var> {
        self.vars.get(name)
    }
}
