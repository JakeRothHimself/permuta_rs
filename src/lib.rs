mod rust_core;

use pyo3::prelude::*;
use rust_core::{patt::Patt, perm::Perm};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn test() -> PyResult<()> {
    let pi = Perm::new([5, 3, 0, 4, 2, 1].into_iter());
    let patt = Perm::new([2, 0, 1].into_iter());

    let iter = pi.patt_iter(&patt);

    let mut n = 0;
    for occ in iter {
        //println!("{:?}", occ);

        n += 1;
        if n == 50 {
            //print!("oops");
            break;
        }
    }

    Ok(())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn permuta_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test, m)?)?;
    Ok(())
}
