mod rust_core;

use pyo3::prelude::*;
use rust_core::perm::Perm;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn test(perm: Vec<usize>) -> PyResult<String> {
    let pi = Perm::new(perm.to_owned().into_iter());

    let result = pi.pattern_details();

    Ok(format!("{:?}", result))
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn permuta_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test, m)?)?;
    Ok(())
}
