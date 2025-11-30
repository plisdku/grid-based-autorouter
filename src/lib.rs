use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod short_stuff {
    use pyo3::prelude::*;

    #[pyfunction]
    fn get_name() -> String {
        "short_stuff".to_string()
    }

    // /// Formats the sum of two numbers as string.
    // #[pyfunction]
    // fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    //     Ok((a + b).to_string())
    // }
}
