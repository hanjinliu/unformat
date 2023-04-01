use std::collections::HashMap;

use pyo3::{prelude::*, exceptions::*};
mod unformatter;
mod vars;

#[derive(PartialEq, Eq)]
enum PatternType {
    Anonymous,
    Named,
    Undef,
}

#[pyfunction]
fn is_named_pattern(ptn: &str) -> PyResult<bool> {
    // check the string pattern type
    let mut active = false;
    let mut ptntype = PatternType::Undef;
    let mut cur = String::new();
    for c in ptn.chars() {
        if c == '{' {
            if active {
                return Err(PyErr::new::<PyValueError, _>("Invalid format pattern: repeated `{`."));
            }
            active = true;
        } else if c == '}' {
            if !active {
                return Err(PyErr::new::<PyValueError, _>("Invalid format pattern: `}` closed before `{`."));
            }
            active = false;
            if cur.len() > 0 {
                if ptntype == PatternType::Anonymous {
                    return Err(PyErr::new::<PyValueError, _>("Uneven format pattern"));
                }
                ptntype = PatternType::Named;
            } else {
                if ptntype == PatternType::Named {
                    return Err(PyErr::new::<PyValueError, _>("Uneven format pattern"));
                }
                ptntype = PatternType::Anonymous;
            }
            cur.clear();
        } else if active {
            cur.push(c);
        } else {
            // do nothing
        }
    }

    if active {
        return Err(PyErr::new::<PyValueError, _>("Invalid format pattern: `{` not closed."));
    }
    if ptntype == PatternType::Named {
        Ok(true)
    } else if ptntype == PatternType::Anonymous {
        Ok(false)
    } else {
        Err(PyErr::new::<PyValueError, _>("Invalid format pattern: Did not encounter `{...}`."))
    }
}

#[pyfunction]
fn unformat(ptn: &str, text: &str) -> PyResult<(HashMap<String, usize>, Vec<String>)> {
    if is_named_pattern(ptn)? {
        let ptn = unformatter::NamedFormatPattern::new(ptn)?;
        let (vars, text) = ptn.unformat(text)?;
        Ok((vars, text))
    } else {
        let ptn = unformatter::FormatPattern::new(ptn)?;
        let (vars, text) = ptn.unformat(text)?;
        Ok((vars, text))
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn _unformat_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    // Add __version__
    let mut version = env!("CARGO_PKG_VERSION").to_string();
    version = version.replace("-alpha", "a").replace("-beta", "b");
    m.add("__version__", version)?;

    m.add_class::<unformatter::FormatPattern>()?;
    m.add_class::<unformatter::NamedFormatPattern>()?;
    m.add_function(wrap_pyfunction!(is_named_pattern, m)?)?;
    m.add_function(wrap_pyfunction!(unformat, m)?)?;
    Ok(())
}
