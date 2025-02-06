use pyo3::prelude::*;
use pyo3::types::PyDict;

fn merge_dicts<'py>(
    py: Python<'py>,
    dict1: &Bound<'py, PyDict>,
    dict2: &Bound<'py, PyDict>,
) -> PyResult<Py<PyDict>> {
    let result = PyDict::new(py);

    // Bulk copy dict1
    result.update(dict1.as_mapping())?;

    // Only iterate dict2 to handle nested dicts
    for item in dict2.iter() {
        let (key, value2) = item;
        if let Some(value1) = result.get_item(key.clone())? {
            if value1.is_instance_of::<PyDict>() && value2.is_instance_of::<PyDict>() {
                let merged = merge_dicts(
                    py,
                    value1.downcast::<PyDict>()?,
                    value2.downcast::<PyDict>()?,
                )?;
                result.set_item(key, merged)?;
                continue;
            }
        }
        result.set_item(key, value2)?;
    }

    Ok(result.into())
}

#[pyfunction]
#[pyo3(signature = (*dicts))]
pub fn deep_merge_dicts(py: Python, dicts: Vec<PyObject>) -> PyResult<Py<PyDict>> {
    let mut result: Py<PyDict> = PyDict::new(py).into();

    for dict in dicts {
        let dict = dict.downcast_bound::<PyDict>(py)?;
        result = merge_dicts(py, result.bind(py), dict)?;
    }

    Ok(result)
}
