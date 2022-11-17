mod unformatter;
pub mod vars;
use pyo3::prelude::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
