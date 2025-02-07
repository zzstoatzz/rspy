use pyo3::prelude::*;
use std::sync::OnceLock;

mod distinct;
mod merge;
mod partition_list;

pub fn get_version() -> &'static str {
    // thank you pydantic-core team!
    static VERSION: OnceLock<String> = OnceLock::new();
    VERSION.get_or_init(|| {
        let version = env!("CARGO_PKG_VERSION");
        // Convert from Cargo's version format to Python's if needed
        // e.g. "1.0-alpha1" -> "1.0.0a1"
        version.replace("-alpha", "a").replace("-beta", "b")
    })
}

#[pymodule]
fn _rspy_utilities(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(distinct::distinct, m)?)?;
    m.add_function(wrap_pyfunction!(merge::deep_merge_dicts, m)?)?;
    m.add_function(wrap_pyfunction!(partition_list::partition, m)?)?;
    m.add("__version__", get_version())?;
    Ok(())
}
