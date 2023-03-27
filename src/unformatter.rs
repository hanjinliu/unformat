use std::{error::Error, fmt, iter};
use super::vars::{Var};

pub struct FormatPattern {
    pub consts: Vec<String>,
    pub vars: Vec<Var>,
}


#[derive(Debug)]
pub struct UnformatError;

impl Error for UnformatError {}

impl fmt::Display for UnformatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}

impl FormatPattern {
    pub fn from_string(s: String) -> FormatPattern {
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
                    panic!("Invalid format string");
                };
                vars.push(Var{ name, fmt });
                cur_var.clear();
            } else if active {
                cur_var.push(c);
            } else {
                cur_const.push(c);
            }
        }
        consts.push(cur_const);
        FormatPattern { consts, vars }
    }

    pub fn parse(&self, s: String) -> Result<Vec<Var>, UnformatError> {
        let mut idx = 0;
        let mut vars: Vec<Var> = Vec::new();
        match s.split_once(&self.consts[0]) {
            Some((_, _s)) => {
                idx += _s.len();
            },
            None => return Err(UnformatError),
        }
        for (cst, var) in iter::zip(self.consts.iter(), self.vars.iter()) {
            match s[idx..].split_once(cst) {
                Some((_, _s)) => {
                    idx += _s.len();
                    vars.push(Var{ name: _s.to_string(), fmt: var.fmt.clone() });
                },
                None => return Err(UnformatError),
            }
        }
        if s.len() > idx {
            return Err(UnformatError);
        }
        Ok(vars)
    }
}

#[cfg(test)]
mod test_from_string {
    #[test]
    fn basic() {
        let s = "aa{}bb{}cc".to_string();
        let model = super::FormatPattern::from_string(s);
        assert_eq!(model.consts, vec!["aa", "bb", "cc"]);
        assert_eq!(model.vars, vec![
            super::Var{ name: "".to_string(), fmt: None },
            super::Var{ name: "".to_string(), fmt: None },
        ]);
    }

    #[test]
    fn with_name() {
        let s = "aa{x}bb{y}cc".to_string();
        let model = super::FormatPattern::from_string(s);
        assert_eq!(model.consts, vec!["aa", "bb", "cc"]);
        assert_eq!(model.vars, vec![
            super::Var{ name: "x".to_string(), fmt: None },
            super::Var{ name: "y".to_string(), fmt: None },
        ]);
    }

    #[test]
    fn with_fmt() {
        let s = "aa{:str}bb{:int}cc".to_string();
        let model = super::FormatPattern::from_string(s);
        assert_eq!(model.consts, vec!["aa", "bb", "cc"]);
        assert_eq!(model.vars, vec![
            super::Var{ name: "".to_string(), fmt: Some("str".to_string()) },
            super::Var{ name: "".to_string(), fmt: Some("int".to_string()) },
        ]);
    }

    #[test]
    fn with_name_and_fmt() {
        let s = "aa{x:str}bb{y:int}cc".to_string();
        let model = super::FormatPattern::from_string(s);
        assert_eq!(model.consts, vec!["aa", "bb", "cc"]);
        assert_eq!(model.vars, vec![
            super::Var{ name: "x".to_string(), fmt: Some("str".to_string()) },
            super::Var{ name: "y".to_string(), fmt: Some("int".to_string()) },
        ]);
    }
}

#[cfg(test)]
mod test_parse {
    #[test]
    fn basic() {
        let model = super::FormatPattern::from_string("aa{}bb{}cc".to_string());
        let result = model.parse("aa{1}bb{2}cc".to_string()).unwrap();
        assert_eq!(result, vec![
            super::Var{ name: "1".to_string(), fmt: None },
            super::Var{ name: "2".to_string(), fmt: None },
        ]);
    }
}
