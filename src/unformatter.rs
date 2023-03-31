use std::{iter, collections::HashMap};
use super::vars::{Var, NamedVar};
use pyo3::{prelude::*, exceptions::PyValueError};

pub trait FormatPatternTrait {
    fn from_string(s: String) -> Self;
    fn parse(&self, s: String) -> PyResult<Vec<Var>>;
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

impl FormatPattern {
    pub fn parse_string(&self, s: String) -> PyResult<Vec<Var>> {
        let mut idx = 0;
        let mut vars: Vec<Var> = Vec::new();
        if !s.starts_with(&self.consts[0]) {
            return Err(PyErr::new::<PyValueError, _>(""));
        }
        idx += self.consts[0].len();
        
        for (cst, var) in iter::zip(self.consts[1..].iter(), self.vars.iter()) {
            match s[idx..].split_once(cst) {
                Some((_s0, _s1)) => {
                    idx += _s0.len() + cst.len();
                    vars.push(Var{ value: _s0.to_string(), fmt: var.fmt.clone() });
                },
                None => return Err(PyErr::new::<PyValueError, _>("")),
            }
        }
        if s.len() != idx {
            return Err(PyErr::new::<PyValueError, _>(""));
        }
        Ok(vars)
    }
}

#[pymethods]
impl FormatPattern {
    #[new]
    pub fn new(s: String) -> Self {
        let mut vars: Vec<Var> = Vec::new();
        let mut consts: Vec<String> = Vec::new();
        let mut active = false;
        let mut cur_var = String::new();
        let mut cur_const: String = String::new();
        for c in s.chars() {
            if c == '{' {
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
                    let fmt = Some(split[1].to_string());
                    (name, fmt)
                } else {
                    // Err(PyErr::new::<PyValueError, _>("Invalid format string"));
                    panic!("Invalid format string");
                };
                vars.push(Var{ value: name, fmt });
                cur_var.clear();
            } else if active {
                cur_var.push(c);
            } else {
                cur_const.push(c);
            }
        }
        consts.push(cur_const);
        Self { consts, vars }
    }

    pub fn unformat(&self, s: String) -> PyResult<(HashMap<String, usize>, Vec<String>)> {
        let vars = self.parse_string(s)?;
        let mut values = Vec::new();
        for var in vars {
            values.push(var.value);
        }
        let keys: HashMap<String, usize> = HashMap::new();
        Ok((keys, values))
    }

    pub fn formats(&self) -> Vec<String> {
        let mut formats = Vec::new();
        for var in self.vars.iter() {
            if let Some(fmt) = &var.fmt {
                formats.push(fmt.clone());
            } else {
                formats.push(String::new());
            }
        }
        formats
    }
}

impl NamedFormatPattern {
    pub fn parse_string(&self, s: String) -> PyResult<Vec<NamedVar>> {
        let mut idx = 0;
        let mut vars: Vec<NamedVar> = Vec::new();
        if !s.starts_with(&self.consts[0]) {
            return Err(PyErr::new::<PyValueError, _>(""));
        }
        idx += self.consts[0].len();
        
        for (cst, var) in iter::zip(self.consts[1..].iter(), self.vars.iter()) {
            match s[idx..].split_once(cst) {
                Some((_s0, _s1)) => {
                    idx += _s0.len() + cst.len();
                    vars.push(NamedVar{ value: _s0.to_string(), fmt: var.fmt.clone(), name: var.value.clone() });
                },
                None => return Err(PyErr::new::<PyValueError, _>("")),
            }
        }
        if s.len() != idx {
            return Err(PyErr::new::<PyValueError, _>(""));
        }
        Ok(vars)
    }

}

#[pymethods]
impl NamedFormatPattern {
    #[new]
    pub fn new(s: String) -> Self {
        let mut vars: Vec<Var> = Vec::new();
        let mut consts: Vec<String> = Vec::new();
        let mut active = false;
        let mut cur_var = String::new();
        let mut cur_const: String = String::new();
        for c in s.chars() {
            if c == '{' {
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
                    let fmt = Some(split[1].to_string());
                    (name, fmt)
                } else {
                    // Err(PyErr::new::<PyValueError, _>("Invalid format string"));
                    panic!("Invalid format string");
                };
                vars.push(Var{ value: name, fmt });
                cur_var.clear();
            } else if active {
                cur_var.push(c);
            } else {
                cur_const.push(c);
            }
        }
        consts.push(cur_const);
        Self { consts, vars }
    }

    pub fn unformat(&self, s: String) -> PyResult<(HashMap<String, usize>, Vec<String>)> {
        let vars = self.parse_string(s)?;
        let mut keys = HashMap::new();
        let mut values = Vec::new();
        for (idx, var) in vars.iter().enumerate() {
            keys.insert(var.name.clone(), idx);
            values.push(var.value.clone());
        }
        Ok((keys, values))
    }

    pub fn formats(&self) -> Vec<String> {
        let mut formats = Vec::new();
        for var in self.vars.iter() {
            if let Some(fmt) = &var.fmt {
                formats.push(fmt.clone());
            } else {
                formats.push(String::new());
            }
        }
        formats
    }
}

// --- test ---------------------------------------------------------------
#[cfg(test)]
mod test_from_string {
    #[test]
    fn basic() {
        let s = "aa{}bb{}cc".to_string();
        let model = super::FormatPattern::new(s);
        assert_eq!(model.consts, vec!["aa", "bb", "cc"]);
        assert_eq!(model.vars, vec![
            super::Var{ value: "".to_string(), fmt: None },
            super::Var{ value: "".to_string(), fmt: None },
        ]);
    }

    #[test]
    fn with_name() {
        let s = "aa{x}bb{y}cc".to_string();
        let model = super::FormatPattern::new(s);
        assert_eq!(model.consts, vec!["aa", "bb", "cc"]);
        assert_eq!(model.vars, vec![
            super::Var{ value: "x".to_string(), fmt: None },
            super::Var{ value: "y".to_string(), fmt: None },
        ]);
    }

    #[test]
    fn with_fmt() {
        let s = "aa{:str}bb{:int}cc".to_string();
        let model = super::FormatPattern::new(s);
        assert_eq!(model.consts, vec!["aa", "bb", "cc"]);
        assert_eq!(model.vars, vec![
            super::Var{ value: "".to_string(), fmt: Some("str".to_string()) },
            super::Var{ value: "".to_string(), fmt: Some("int".to_string()) },
        ]);
    }

    #[test]
    fn with_name_and_fmt() {
        let s = "aa{x:str}bb{y:int}cc".to_string();
        let model = super::FormatPattern::new(s);
        assert_eq!(model.consts, vec!["aa", "bb", "cc"]);
        assert_eq!(model.vars, vec![
            super::Var{ value: "x".to_string(), fmt: Some("str".to_string()) },
            super::Var{ value: "y".to_string(), fmt: Some("int".to_string()) },
        ]);
    }
}

#[cfg(test)]
mod test_parse {
    #[test]
    fn basic() {
        let model = super::FormatPattern::new("aa{}bbb{}cccc".to_string());
        let result = model.parse_string("aa1bbb2cccc".to_string()).unwrap();
        assert_eq!(result, vec![
            super::Var{ value: "1".to_string(), fmt: None },
            super::Var{ value: "2".to_string(), fmt: None },
        ]);
    }
}
