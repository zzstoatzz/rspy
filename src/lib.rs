use pyo3::prelude::*;

mod distinct;
mod merge;
mod partition_list;

#[pymodule]
fn _rspy_utilities(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(distinct::distinct, m)?)?;
    m.add_function(wrap_pyfunction!(merge::deep_merge_dicts, m)?)?;
    m.add_function(wrap_pyfunction!(partition_list::partition, m)?)?;
    Ok(())
}
