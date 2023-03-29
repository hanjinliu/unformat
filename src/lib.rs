use pyo3::prelude::*;
mod unformatter;
mod vars;

#[pyfunction]
fn parse(ptn: String, s: String) -> PyResult<Vec<String>> {
    let pattern = unformatter::FormatPattern::from_string(ptn);
    Ok(pattern.parse_as_vec(s).unwrap())
}

/// A Python module implemented in Rust.
#[pymodule]
fn _unformat_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<unformatter::FormatPattern>()?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}
