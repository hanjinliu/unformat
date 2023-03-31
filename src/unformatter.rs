use std::{iter, collections::HashMap};
use super::vars::{Var, NamedVar};
use pyo3::{prelude::*, exceptions::PyValueError};


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
    pub fn parse_string(&self, s: &String) -> PyResult<Vec<NamedVar>> {
        parse_string(&self.consts, &self.vars, s)
    }
}

#[pymethods]
impl FormatPattern {
    #[new]
    pub fn new(s: String) -> PyResult<Self> {
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
                    let fmt = Some(split[1].to_string());
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

    pub fn unformat(&self, s: String) -> PyResult<(HashMap<String, usize>, Vec<String>)> {
        let vars = self.parse_string(&s)?;
        let mut values = Vec::new();
        for var in vars {
            values.push(var.value);
        }
        let keys: HashMap<String, usize> = HashMap::new();
        Ok((keys, values))
    }

    pub fn matches(&self, s: String) -> bool {
        return self.parse_string(&s).is_ok();
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
    pub fn parse_string(&self, s: &String) -> PyResult<Vec<NamedVar>> {
        parse_string(&self.consts, &self.vars, s)
    }
}

fn _str_repr(s: &String) -> String {
    if s.len() > 12 {
        s[..12].to_string() + "..."
    } else {
        s.clone()
    }
}

fn parse_string(consts: &Vec<String>, vars: &Vec<Var>, s: &String) -> PyResult<Vec<NamedVar>> {
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

#[pymethods]
impl NamedFormatPattern {
    #[new]
    pub fn new(s: String) -> PyResult<Self> {
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
                    let fmt = Some(split[1].to_string());
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

    pub fn unformat(&self, s: String) -> PyResult<(HashMap<String, usize>, Vec<String>)> {
        let vars = self.parse_string(&s)?;
        let mut keys = HashMap::new();
        let mut values = Vec::new();
        for (idx, var) in vars.iter().enumerate() {
            keys.insert(var.name.clone(), idx);
            values.push(var.value.clone());
        }
        Ok((keys, values))
    }

    pub fn matches(&self, s: String) -> bool {
        return self.parse_string(&s).is_ok();
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
        let model = super::FormatPattern::new(s).unwrap();
        assert_eq!(model.consts, vec!["aa", "bb", "cc"]);
        assert_eq!(model.vars, vec![
            super::Var{ value: "".to_string(), fmt: None },
            super::Var{ value: "".to_string(), fmt: None },
        ]);
    }

    #[test]
    fn with_name() {
        let s = "aa{x}bb{y}cc".to_string();
        let model = super::FormatPattern::new(s).unwrap();
        assert_eq!(model.consts, vec!["aa", "bb", "cc"]);
        assert_eq!(model.vars, vec![
            super::Var{ value: "x".to_string(), fmt: None },
            super::Var{ value: "y".to_string(), fmt: None },
        ]);
    }

    #[test]
    fn with_fmt() {
        let s = "aa{:str}bb{:int}cc".to_string();
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
        let s = "aa{x:str}bb{y:int}cc".to_string();
        let model = super::NamedFormatPattern::new(s).unwrap();
        assert_eq!(model.consts, vec!["aa", "bb", "cc"]);
        assert_eq!(model.vars, vec![
            super::Var{ value: "x".to_string(), fmt: Some("str".to_string()) },
            super::Var{ value: "y".to_string(), fmt: Some("int".to_string()) },
        ]);
    }

    #[test]
    fn err_invalid_format_string() {
        let s = "aa{:str}bb{:int:}cc".to_string();
        let model = super::FormatPattern::new(s);
        assert!(model.is_err());
    }

    #[test]
    fn err_tandem_brace() {
        let s = "aa{}{}c".to_string();
        let model = super::FormatPattern::new(s);
        assert!(model.is_err());
    }


}

#[cfg(test)]
mod test_parse {
    #[test]
    fn basic() {
        let model = super::FormatPattern::new("aa{}bbb{}cccc".to_string()).unwrap();
        let result = model.parse_string(&"aa1bbb2cccc".to_string()).unwrap();
        assert_eq!(result, vec![
            super::NamedVar{ value: "1".to_string(), fmt: None, name: "".to_string() },
            super::NamedVar{ value: "2".to_string(), fmt: None, name: "".to_string() },
        ]);
    }

    #[test]
    fn ends_with_brace() {
        let s = "{}bb{}".to_string();
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
        let s = "{x}bb{y}".to_string();
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
