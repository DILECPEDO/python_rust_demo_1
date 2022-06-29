use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn doublestring(str: String) -> PyResult<String> {
    let result = str
        .chars()
        .into_iter()
        .map(|c| c.to_string().repeat(2))
        .collect::<String>();
    Ok(result)
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_demo_1(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(doublestring, m)?)?;
    Ok(())
}
