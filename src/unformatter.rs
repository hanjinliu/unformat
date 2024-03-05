use std::{iter, collections::{HashMap, HashSet}};
use super::vars::{Var, NamedVar};
use pyo3::{prelude::*, exceptions::PyValueError};

pub trait PatternTrait {
    fn consts(&self) -> &Vec<String>;
    fn vars(&self) -> &Vec<Var>;

    fn parse_string(&self, s: &str) -> PyResult<Vec<NamedVar>> {
        let consts = self.consts();
        let vars = self.vars();
        let mut idx = 0;
        let mut _vars: Vec<NamedVar> = Vec::new();
        if !s.starts_with(&consts[0]) {
            return Err(
                PyErr::new::<PyValueError, _>(
                    format!("Input should starts with {}", _str_repr(&consts[0]))
                )
            );
        }
        idx += consts[0].len();

        for (cst, var) in iter::zip(consts[1..].iter(), vars.iter()) {
            match s[idx..].split_once(cst) {
                Some((_s0, _s1)) => {
                    // NOTE: calling split_once with "" is an exceptional case.
                    // "a".split_once("") returns Some(("", "a")), not Somoe(("a", "")).
                    if cst.len() > 0 {
                        idx += _s0.len() + cst.len();
                        _vars.push(NamedVar{ value: _s0.to_string(), fmt: var.fmt.clone(), name: var.value.clone() });
                    } else {
                        idx += _s1.len();
                        _vars.push(NamedVar{ value: _s1.to_string(), fmt: var.fmt.clone(), name: var.value.clone() });
                    }
                },
                None => {
                    return Err(
                        PyErr::new::<PyValueError, _>(
                            format!("Input does not contain '{}'.", _str_repr(cst))
                        )
                    )
                }
            }
        }
        if s.len() != idx {
            return Err(
                PyErr::new::<PyValueError, _>(
                    format!("Input should ends with '{}'.", _str_repr(&consts[consts.len() - 1]))
                )
            );
        }
        Ok(_vars)
    }

    fn unformat(&self, s: &str) -> PyResult<(HashMap<String, usize>, Vec<String>)>;

    fn unformat_all(
        &self,
        s: Vec<&str>,
    ) -> PyResult<(HashMap<String, usize>, Vec<Vec<String>>)> {
        let mut keys: HashMap<String, usize> = HashMap::new();
        let mut values: Vec<Vec<String>> = Vec::new();
        for s in s {
            let (k, v) = self.unformat(s)?;
            keys.extend(k);
            values.push(v);
        }
        Ok((keys, values))
    }

    fn unformat_to_dict(
        &self,
        s: Vec<&str>,
    ) -> PyResult<(HashMap<String, usize>, HashMap<String, Vec<String>>)> {
        let mut keys: HashMap<String, usize> = HashMap::new();
        let mut values: HashMap<String, Vec<String>> = HashMap::new();
        for i in 0..self.vars().len() {
            values.insert(self.vars()[i].value.clone(), Vec::new());
        }
        for s in s {
            let (k, v) = self.unformat(s)?;
            keys.extend(k);
            for (key, value) in v.iter().enumerate() {
                values.get_mut(&self.vars()[key].value).unwrap().push(value.clone());
            }
        }
        Ok((keys, values))
    }

    fn matches(&self, s: String) -> bool {
        self.parse_string(&s).is_ok()
    }

    fn formats(&self) -> Vec<String> {
        let mut formats = Vec::new();
        for var in self.vars().iter() {
            if let Some(fmt) = &var.fmt {
                formats.push(fmt.clone());
            } else {
                formats.push(String::new());
            }
        }
        formats
    }

    fn variables(&self) -> Vec<String> {
        let mut variables = Vec::new();
        for var in self.vars().iter() {
            variables.push(var.value.clone());
        }
        variables
    }

    fn pattern(&self) -> String {
        let consts = self.consts();
        let vars = self.vars();
        let mut s = consts[0].clone();
        for (cst, var) in iter::zip(consts[1..].iter(), vars.iter()) {
            match var.fmt {
                Some(ref fmt) => {
                    s += &format!("{{{}:{}}}", var.value, fmt);
                },
                None => {
                    s += &format!("{{{}}}", var.value);
                }
            }
            s += cst;
        }
        s
    }

}

#[pyclass]
pub struct FormatPattern {
    pub consts: Vec<String>,
    pub vars: Vec<Var>,
}

#[pyclass]
pub struct NamedFormatPattern {
    pub consts: Vec<String>,
    pub vars: Vec<Var>,
}

impl PatternTrait for FormatPattern {
    fn consts(&self) -> &Vec<String> {
        &self.consts
    }

    fn vars(&self) -> &Vec<Var> {
        &self.vars
    }

    fn unformat(&self, s: &str) -> PyResult<(HashMap<String, usize>, Vec<String>)> {
        let vars = self.parse_string(s)?;
        let mut values = Vec::new();
        for var in vars {
            values.push(var.value);
        }
        let keys: HashMap<String, usize> = HashMap::new();
        Ok((keys, values))
    }
}

#[pymethods]
impl FormatPattern {
    #[new]
    pub fn new(s: &str) -> PyResult<Self> {
        let mut vars: Vec<Var> = Vec::new();
        let mut consts: Vec<String> = Vec::new();
        let mut active = false;
        let mut cur_var = String::new();
        let mut cur_const: String = String::new();
        let mut last_char = ' ';
        for c in s.chars() {
            if c == '{' {
                if last_char == '}' {
                    return Err(PyErr::new::<PyValueError, _>("'{{' cannot follow '}}'"));
                }
                active = true;
                consts.push(cur_const.clone());
                cur_const.clear();
            } else if c == '}' {
                active = false;
                let split: Vec<&str> = cur_var.split(':').collect();
                let (name, fmt) = if split.len() == 1 {
                    let name = split[0].to_string();
                    let fmt = None;
                    (name, fmt)
                } else if split.len() == 2 {
                    let name = split[0].to_string();
                    let fmt = Some(split[1].trim().to_string());
                    (name, fmt)
                } else {
                    let msg = format!("Invalid format string: {}", cur_var);
                    return Err(PyErr::new::<PyValueError, _>(msg));
                };
                vars.push(Var{ value: name, fmt });
                cur_var.clear();
            } else if active {
                cur_var.push(c);
            } else {
                cur_const.push(c);
            }
            last_char = c;
        }
        consts.push(cur_const);
        Ok(Self { consts, vars })
    }

    pub fn unformat(&self, s: &str) -> PyResult<(HashMap<String, usize>, Vec<String>)> {
        PatternTrait::unformat(self, s)
    }

    pub fn unformat_all(&self, s: Vec<&str>) -> PyResult<(HashMap<String, usize>, Vec<Vec<String>>)> {
        PatternTrait::unformat_all(self, s)
    }

    pub fn unformat_to_dict(&self, s: Vec<&str>) -> PyResult<(HashMap<String, usize>, HashMap<String, Vec<String>>)> {
        PatternTrait::unformat_to_dict(self, s)
    }

    pub fn matches(&self, s: String) -> bool {
        PatternTrait::matches(self, s)
    }

    pub fn formats(&self) -> Vec<String> {
        PatternTrait::formats(self)
    }

    pub fn variables(&self) -> Vec<String> {
        PatternTrait::variables(self)
    }

    pub fn pattern(&self) -> String {
        PatternTrait::pattern(self)
    }

    pub fn with_formats(&self, formats: Vec<String>) -> PyResult<FormatPattern> {
        let vars = update_format(&self.vars, &formats)?;
        Ok(FormatPattern{ consts: self.consts.clone(), vars })
    }
}

impl PatternTrait for NamedFormatPattern {
    fn consts(&self) -> &Vec<String> {
        &self.consts
    }

    fn vars(&self) -> &Vec<Var> {
        &self.vars
    }

    fn unformat(&self, s: &str) -> PyResult<(HashMap<String, usize>, Vec<String>)> {
        let vars = self.parse_string(s)?;
        let mut keys = HashMap::new();
        let mut values = Vec::new();
        for (idx, var) in vars.iter().enumerate() {
            keys.insert(var.name.clone(), idx);
            values.push(var.value.clone());
        }
        Ok((keys, values))
    }
}

fn _str_repr(s: &String) -> String {
    if s.len() > 12 {
        s[..12].to_string() + "..."
    } else {
        s.clone()
    }
}

fn update_format(vars: &Vec<Var>, formats: &Vec<String>) -> PyResult<Vec<Var>> {
    if formats.len() != vars.len() {
        return Err(PyErr::new::<PyValueError, _>("Length of formats must be same as length of variables."));
    }
    let mut out = Vec::new();
    for (var, fmt) in iter::zip(vars.iter(), formats.iter()) {
        out.push(Var{ value: var.value.clone(), fmt: Some(fmt.clone()) });
    }
    Ok(out)
}

#[pymethods]
impl NamedFormatPattern {
    #[new]
    pub fn new(s: &str) -> PyResult<Self> {
        let mut vars: Vec<Var> = Vec::new();
        let mut consts: Vec<String> = Vec::new();
        let mut active = false;
        let mut cur_var = String::new();
        let mut cur_const: String = String::new();
        let mut last_char = ' ';
        let mut existing: HashSet<String> = HashSet::new();
        for c in s.chars() {
            if c == '{' {
                if last_char == '}' {
                    return Err(PyErr::new::<PyValueError, _>("'{{' cannot follow '}}'"));
                }
                active = true;
                consts.push(cur_const.clone());
                cur_const.clear();
            } else if c == '}' {
                active = false;
                let split: Vec<&str> = cur_var.split(':').collect();
                let (name, fmt) = if split.len() == 1 {
                    let name = split[0].to_string();
                    let fmt = None;
                    (name, fmt)
                } else if split.len() == 2 {
                    let name = split[0].to_string();
                    let fmt = Some(split[1].trim().to_string());
                    (name, fmt)
                } else {
                    let msg = format!("Invalid format string: {}", cur_var);
                    return Err(PyErr::new::<PyValueError, _>(msg));
                };
                if existing.contains(&name) {
                    let msg = format!("Duplicate variable name: {}", name);
                    return Err(PyErr::new::<PyValueError, _>(msg));
                }
                vars.push(Var{ value: name.clone(), fmt });
                existing.insert(name);
                cur_var.clear();
            } else if active {
                cur_var.push(c);
            } else {
                cur_const.push(c);
            }
            last_char = c;
        }
        consts.push(cur_const);
        Ok(Self { consts, vars })
    }

    pub fn unformat(&self, s: &str) -> PyResult<(HashMap<String, usize>, Vec<String>)> {
        PatternTrait::unformat(self, s)
    }

    pub fn unformat_all(&self, s: Vec<&str>) -> PyResult<(HashMap<String, usize>, Vec<Vec<String>>)> {
        PatternTrait::unformat_all(self, s)
    }

    pub fn unformat_to_dict(&self, s: Vec<&str>) -> PyResult<(HashMap<String, usize>, HashMap<String, Vec<String>>)> {
        PatternTrait::unformat_to_dict(self, s)
    }

    pub fn matches(&self, s: String) -> bool {
        PatternTrait::matches(self, s)
    }

    pub fn formats(&self) -> Vec<String> {
        PatternTrait::formats(self)
    }

    pub fn variables(&self) -> Vec<String> {
        PatternTrait::variables(self)
    }

    pub fn pattern(&self) -> String {
        PatternTrait::pattern(self)
    }

    pub fn with_formats(&self, formats: Vec<String>) -> PyResult<NamedFormatPattern> {
        let vars = update_format(&self.vars, &formats)?;
        Ok(NamedFormatPattern{ consts: self.consts.clone(), vars })
    }
}

// --- test ---------------------------------------------------------------
#[cfg(test)]
mod test_from_string {
    #[test]
    fn basic() {
        let s = "aa{}bb{}cc";
        let model = super::FormatPattern::new(s).unwrap();
        assert_eq!(model.consts, vec!["aa", "bb", "cc"]);
        assert_eq!(model.vars, vec![
            super::Var{ value: "".to_string(), fmt: None },
            super::Var{ value: "".to_string(), fmt: None },
        ]);
    }

    #[test]
    fn with_name() {
        let s = "aa{x}bb{y}cc";
        let model = super::FormatPattern::new(s).unwrap();
        assert_eq!(model.consts, vec!["aa", "bb", "cc"]);
        assert_eq!(model.vars, vec![
            super::Var{ value: "x".to_string(), fmt: None },
            super::Var{ value: "y".to_string(), fmt: None },
        ]);
    }

    #[test]
    fn with_fmt() {
        let s = "aa{:str}bb{:int}cc";
        let model = super::FormatPattern::new(s).unwrap();
        assert_eq!(model.consts, vec!["aa", "bb", "cc"]);
        assert_eq!(model.vars, vec![
            super::Var{ value: "".to_string(), fmt: Some("str".to_string()) },
            super::Var{ value: "".to_string(), fmt: Some("int".to_string()) },
        ]);
        assert_eq!(model.formats(), vec!["str", "int"]);
    }

    #[test]
    fn with_name_and_fmt() {
        let s = "aa{x:str}bb{y:int}cc";
        let model = super::NamedFormatPattern::new(s).unwrap();
        assert_eq!(model.consts, vec!["aa", "bb", "cc"]);
        assert_eq!(model.vars, vec![
            super::Var{ value: "x".to_string(), fmt: Some("str".to_string()) },
            super::Var{ value: "y".to_string(), fmt: Some("int".to_string()) },
        ]);
    }

    #[test]
    fn err_invalid_format_string() {
        let s = "aa{:str}bb{:int:}cc";
        let model = super::FormatPattern::new(s);
        assert!(model.is_err());
    }

    #[test]
    fn err_tandem_brace() {
        let s = "aa{}{}c";
        let model = super::FormatPattern::new(s);
        assert!(model.is_err());
    }


}

#[cfg(test)]
mod test_parse {
    use crate::unformatter::PatternTrait;

    #[test]
    fn basic() {
        let model = super::FormatPattern::new("aa{}bbb{}cccc").unwrap();
        let result = model.parse_string(&"aa1bbb2cccc".to_string()).unwrap();
        assert_eq!(result, vec![
            super::NamedVar{ value: "1".to_string(), fmt: None, name: "".to_string() },
            super::NamedVar{ value: "2".to_string(), fmt: None, name: "".to_string() },
        ]);
    }

    #[test]
    fn ends_with_brace() {
        let s = "{}bb{}";
        let model = super::FormatPattern::new(s).unwrap();
        assert_eq!(model.consts, vec!["", "bb", ""]);
        assert_eq!(model.vars, vec![
            super::Var{ value: "".to_string(), fmt: None },
            super::Var{ value: "".to_string(), fmt: None },
        ]);
        let result = model.parse_string(&"1bb2".to_string());
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), vec![
            super::NamedVar{ value: "1".to_string(), fmt: None, name: "".to_string() },
            super::NamedVar{ value: "2".to_string(), fmt: None, name: "".to_string() },
        ]);
    }

    #[test]
    fn with_name_ends_with_brace() {
        let s = "{x}bb{y}";
        let model = super::FormatPattern::new(s).unwrap();
        assert_eq!(model.consts, vec!["", "bb", ""]);
        assert_eq!(model.vars, vec![
            super::Var{ value: "x".to_string(), fmt: None },
            super::Var{ value: "y".to_string(), fmt: None },
        ]);
        let result = model.parse_string(&"1bb2".to_string());
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), vec![
            super::NamedVar{ value: "1".to_string(), fmt: None, name: "x".to_string() },
            super::NamedVar{ value: "2".to_string(), fmt: None, name: "y".to_string() },
        ]);
    }

}
