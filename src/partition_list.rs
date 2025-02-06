use pyo3::prelude::*;

#[pyfunction]
#[pyo3(signature = (iterable, predicate))]
pub fn partition<'py>(
    py: Python<'py>,
    iterable: &Bound<'py, PyAny>,
    predicate: PyObject,
) -> PyResult<(Vec<PyObject>, Vec<PyObject>)> {
    let mut true_items = Vec::new();
    let mut false_items = Vec::new();

    let iter = iterable.try_iter()?;

    for item in iter {
        let item = item?;
        let result = predicate.bind(py).call1((item.clone(),))?;
        let is_true = result.extract::<bool>()?;

        if is_true {
            true_items.push(item.into_pyobject(py)?.into());
        } else {
            false_items.push(item.into_pyobject(py)?.into());
        }
    }

    Ok((true_items, false_items))
}
