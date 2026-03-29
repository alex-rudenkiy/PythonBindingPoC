use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::exceptions::PyRuntimeError;

#[pyclass]
#[derive(Clone)]
pub struct LintWarning {
    #[pyo3(get)]
    pub message: String,
}

#[pyclass]
pub struct WdlDocument {
    #[pyo3(get)]
    pub version: String,
    #[pyo3(get)]
    pub tasks_count: usize,
    #[pyo3(get)]
    pub warnings: Py<PyList>,
}

#[pyfunction]
fn parse_wdl(py: Python<'_>, source: String) -> PyResult<WdlDocument> {
    if source.is_empty() {
        return Err(PyRuntimeError::new_err("WDL source is empty"));
    }

    let version = if source.contains("version 1.0") { "1.0" } else { "unknown" };
    let tasks_count = source.matches("task ").count();
    
    let mut warnings_vec = Vec::new();
    if version == "unknown" {
        let warning = Py::new(py, LintWarning {
            message: "Missing WDL version declaration".to_string(),
        })?;
        warnings_vec.push(warning);
    }

    let warnings_list = PyList::new(py, warnings_vec)?;

    Ok(WdlDocument {
        version: version.to_string(),
        tasks_count,
        warnings: warnings_list.into(),
    })
}

#[pymodule]
fn sprocket_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<LintWarning>()?;
    m.add_class::<WdlDocument>()?;
    m.add_function(wrap_pyfunction!(parse_wdl, m)?)?;
    Ok(())
}