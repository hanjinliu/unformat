
// "{a:str}" -> Var { name: "a", fmt: "str" }
#[derive(Debug, PartialEq, Eq)]
pub struct Var {
    pub name: String,
    pub fmt: Option<String>,
}
