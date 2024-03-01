// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2024 Oxford Quantum Circuits Ltd

use std::borrow::Borrow;
use bitflags::Flags;
use log::{Level, log, log_enabled};
use pyo3::prelude::*;
use pyo3::exceptions::{PyValueError};
use pyo3::types::{PyBool, PyFloat, PyInt, PyList, PyString};
use crate::builders::PythonRuntime;
use crate::execution::{parse_file, run_file, run_graph, RuntimeCollection};
use crate::graphs::ExecutableAnalysisGraph;
use crate::{DEFAULT_LOG_FILE, initialize_loggers};
use crate::instructions::Value;
use crate::runtime::ActiveTracers;
use crate::smart_pointers::{Ptr};

#[pymodule]
fn _native(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Executor>()?;
    m.add_function(wrap_pyfunction!(initialize_file_logger, m)?);
    m.add_function(wrap_pyfunction!(initialize_commandline_logger, m)?);
    m.add("DEFAULT_LOG_FILE", DEFAULT_LOG_FILE);
    Ok(())
}

/// Proxy for initializing Munchkin loggers. Pass in path for file logger initialization.
#[pyfunction]
fn initialize_file_logger(file_path: &str) {
    initialize_loggers(Some(file_path.to_string()));
}

#[pyfunction]
fn initialize_commandline_logger() {
    initialize_loggers(None);
}

impl ToPyObject for Value {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        match self {
            Value::Empty => py.None(),
            Value::Byte(nested) => nested.to_object(py),
            Value::Short(nested) => nested.to_object(py),
            Value::Int(nested) => nested.to_object(py),
            Value::Long(nested) => nested.to_object(py),
            Value::Bool(nested) => nested.to_object(py),
            Value::Float(nested) => nested.to_object(py),
            Value::String(nested) => nested.to_object(py),
            Value::AnalysisResult(nested) => nested.distribution.to_object(py),
            Value::Array(nested) => nested.iter().map(|val| val.to_object(py)).collect::<Vec<_>>().to_object(py),
            _ => panic!("Can't return this type.")
        }
    }
}

impl FromPyObject<'_> for Value {
    fn extract(ob: &PyAny) -> PyResult<Self> {
        let transformed = if ob.is_instance_of::<PyInt>().is_ok_and(|val| val) {
            let value: i128 = ob.extract().expect(format!("Can't map {} to Munchkin value.", ob.to_string()).as_str());
            Value::Long(value)
        } else if ob.is_instance_of::<PyFloat>().is_ok_and(|val| val) {
            let value: f64 = ob.extract().expect(format!("Can't map {} to Munchkin value.", ob.to_string()).as_str());
            Value::Float(value)
        } else if ob.is_instance_of::<PyBool>().is_ok_and(|val| val) {
            let value: bool = ob.extract().expect(format!("Can't map {} to Munchkin value.", ob.to_string()).as_str());
            Value::Bool(value)
        } else if ob.is_instance_of::<PyString>().is_ok_and(|val| val) {
            let value: String = ob.extract().expect(format!("Can't map {} to Munchkin value.", ob.to_string()).as_str());
            Value::String(value)
        } else {
            return Err(PyValueError::new_err("Can't resolve Python value to Munchkin value."));
        };

        Ok(transformed)
    }
}

/// Python wrapper around an execution graph. Currently used for simply passing things around for
/// the APIs. Later it'll expose more internal operations for the graph itself for
/// mutations/changes from Python.
#[pyclass]
#[derive(Clone)]
pub(crate) struct Graph {
    pub wrapped: Ptr<ExecutableAnalysisGraph>
}

impl Graph {
    pub fn new(graph: &Ptr<ExecutableAnalysisGraph>) -> Graph {
        activate_fallback_logger();
        Graph { wrapped: graph.clone() }
    }
}

/// People should set up loggers before they call our Python bindings, but if they don't we want
/// to make sure our execution chain still outputs things correctly.
///
/// This call should be the first line in any Rust/Python boundary. Mostly constructors and
/// free methods.
fn activate_fallback_logger() {
    if !log_enabled!(Level::Error) {
        initialize_commandline_logger();
        log!(Level::Info, "Logger not initialized, defaulting to commandline.");
     }
}

#[pyclass]
pub(crate) struct Executor {
    tracing: ActiveTracers,
}

/// Python binding for allowing consumes to call into the Rust code.
#[pymethods]
impl Executor {
    #[new]
    fn new() -> Self {
        // Activate fallback logging if we don't have any.
        activate_fallback_logger();
        Executor { tracing: ActiveTracers::empty() }
    }

    fn trace_runtime(&mut self) {
        self.tracing.insert(ActiveTracers::Runtime);
    }

    fn trace_projections(&mut self) {
        self.tracing.insert(ActiveTracers::Projections);
    }

    fn trace_graphs(&mut self) {
        self.tracing.insert(ActiveTracers::Graphs);
    }

    #[allow(clippy::unused_self)]
    fn parse_file(
        &self, file: &str, entry_point: Option<&str>) -> PyResult<Py<Graph>> {
        Python::with_gil(|py| -> PyResult<Py<Graph>> {
            parse_file(file, entry_point)
              .map_err(PyValueError::new_err)
              .map(|value| {
                  let result: Py<Graph> = Py::new(py, Graph::new(value.borrow()))
                    .ok().expect("Unable to build Python graph representation.");
                  result
            })
        })
    }

    fn run_graph(&self, graph: Py<Graph>, arguments: &PyAny,
                 runtime_adaptor: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| -> Result<PyObject, PyErr> {
            // We just build a reference directly here so our smart-pointer doesn't auto-drop.
            let runtimes: Vec<&PyAny> = runtime_adaptor.extract().expect("Unable to transform runtimes to Rust objects.");
            let mut collection = Ptr::from(RuntimeCollection::default());
            for runtime in runtimes {
                collection.add(&Ptr::from(PythonRuntime::new(runtime)))
            }

            let graph: Graph = graph.extract(py).expect("Unable to extract graph.");
            let args: Vec<Value> = arguments.extract().expect("Unable to transform arguments to Rust objects.");

            run_graph(graph.wrapped.borrow(), args.as_ref(), collection.borrow(), self.tracing.clone())
              .map_err(PyValueError::new_err)
              .map(|value| {
                value.map_or(py.None(), |val| val.to_object(py))
            })
        })
    }

    #[allow(clippy::unused_self)]
    fn run(
        &self,
        file: &str,
        runtime_adaptor: &PyAny,
    ) -> PyResult<PyObject> {
        Python::with_gil(|py| -> Result<PyObject, PyErr> {
            self.run_with_args(file, PyList::empty(py), runtime_adaptor)
        })
    }

    #[allow(clippy::unused_self)]
    fn run_with_args(
        &self,
        file: &str,
        arguments: &PyAny,
        runtime_adaptor: &PyAny,
    ) -> PyResult<PyObject> {
        Python::with_gil(|py| -> Result<PyObject, PyErr> {
            let runtimes: Vec<&PyAny> = runtime_adaptor.extract()?;
            let mut collection = Ptr::from(RuntimeCollection::default());
            for runtime in runtimes {
                collection.add(&Ptr::from(PythonRuntime::new(runtime)))
            }

            let args: Vec<Value> = arguments.extract()?;
            run_file(file, &args, collection.borrow(), None, self.tracing.clone())
              .map_err(PyValueError::new_err)
              .map(|value| {
                value.map_or(py.None(), |val| val.to_object(py))
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::{canonicalize, File, remove_file};
    use std::io::{BufRead, BufReader, Read};
    use std::path::PathBuf;
    use pyo3::{PyAny, PyObject, PyResult, Python};
    use pyo3::types::{PyList, PyModule};
    use crate::python::{_native, Executor};

    fn python_from<'a>(py: Python<'a>, file: &str, name: &str) -> &'a PyAny {
        PyModule::from_code(py, file, "", "").unwrap()
          .getattr(name).unwrap().call0().expect("Unable to call Python method/constructor.")
    }

    fn assert_default_results(py: Python, results: PyResult<PyObject>) {
        let rust_results: HashMap<String, i64> = results.expect("Results need to exist.")
          .extract(py).expect("Results aren't the right type.");

        assert_eq!(rust_results.get("00").expect("Key should exist"), &250);
        assert_eq!(rust_results.get("01").expect("Key should exist"), &250);
        assert_eq!(rust_results.get("10").expect("Key should exist"), &250);
        assert_eq!(rust_results.get("11").expect("Key should exist"), &251);
    }

    #[test]
    fn no_args() {
        Python::with_gil(|py| {
            let relative_path = canonicalize("../tests/files/qir/generator-bell.ll").unwrap();
            let path = relative_path.to_str().unwrap();

            let adaptor_file = include_str!("../../tests/rust_python_integration.py");
            let builder = python_from(py, adaptor_file, "BuilderAdaptor");
            let runtime = python_from(py, adaptor_file, "RuntimeAdaptor");

            let walker = Executor::new();
            let results = walker.run(path, runtime);
            assert_default_results(py, results);
        });
    }

    #[test]
    fn invalid_args() {
        Python::with_gil(|py| {
            let relative_path = canonicalize("../tests/files/qir/generator-bell.ll").unwrap();
            let path = relative_path.to_str().unwrap();

            let adaptor_file = include_str!("../../tests/rust_python_integration.py");
            let runtime = python_from(py, adaptor_file, "RuntimeAdaptor");
            let args = python_from(py, adaptor_file, "build_invalid_args");

            let walker = Executor::new();
            walker.run_with_args(path, args, runtime)
              .expect_err("Invalid args passed, should error.");
        });
    }

    #[test]
    fn parse_graph() {
        Python::with_gil(|py| {
            let relative_path = canonicalize("../tests/files/qir/generator-bell.ll").unwrap();
            let path = relative_path.to_str().unwrap();

            let walker = Executor::new();
            let parsed_graph = walker.parse_file(path, None).expect("Unable to parse graph.");
        });
    }

    #[test]
    fn parse_and_execute() {
        Python::with_gil(|py| {
            let relative_path = canonicalize("../tests/files/qir/generator-bell.ll").unwrap();
            let path = relative_path.to_str().unwrap();

            let adaptor_file = include_str!("../../tests/rust_python_integration.py");
            let runtime = python_from(py, adaptor_file, "RuntimeAdaptor");

            let walker = Executor::new();
            let parsed_graph = walker.parse_file(path, None);
            let results = walker.run_graph(
                parsed_graph.expect("Graph should be parsable"), PyList::empty(py), runtime);

            assert_default_results(py, results);
        });
    }
}
