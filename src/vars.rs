// unformat "a1b" with "a{:str}b" -> Var { value: "1", fmt: "str" }
#[derive(Debug, PartialEq, Eq)]
pub struct Var {
    pub value: String,
    pub fmt: Option<String>,
}

// unformat "a1b" with "a{x:str}b" -> Var { value: "1", fmt: "str", name: "x"}
#[derive(Debug, PartialEq, Eq)]
pub struct NamedVar {
    pub value: String,
    pub fmt: Option<String>,
    pub name: String,
}
