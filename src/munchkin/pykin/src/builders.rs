use std::borrow::Borrow;
use std::f64::consts::PI;
use std::ops::{Deref, DerefMut};
use pyo3::{PyAny, PyResult};
use crate::analysis::{AnalysisResult};
use crate::hardware::{Qubit};
use crate::smart_pointers::{Ptr};

macro_rules! python_methods {
    (self.$wrapped_obj:ident.$python_gate:ident()) => {
        pub fn $python_gate(&self) -> Option<PyResult<&PyAny>> {
            if Ptr::is_not_null(&self.$wrapped_obj) {
                let pyobj: &PyAny = self.$wrapped_obj.borrow();
                let has_gate = pyobj.hasattr(stringify!($python_gate)).unwrap_or(false);
                if has_gate {
                    let func = pyobj.getattr(stringify!($python_gate)).unwrap();
                    Some(func.call0())
                } else { None }
            } else { None }
        }
    };
    (self.$wrapped_obj:ident.$python_gate:ident($($var:ident: $ty:ty),*)) => {
        pub fn $python_gate(&self, $($var: $ty),*) -> Option<PyResult<&PyAny>> {
            if Ptr::is_not_null(&self.$wrapped_obj) {
                let pyobj: &PyAny = self.$wrapped_obj.borrow();
                let has_gate = pyobj.hasattr(stringify!($python_gate)).unwrap_or(false);
                if has_gate {
                    let func = pyobj.getattr(stringify!($python_gate)).unwrap();
                    Some(func.call1(($($var),*,)))
                } else { None }
            } else { None }
        }
    }
}

struct PyBuilderAdaptor {
    builder: Ptr<PyAny>
}

impl PyBuilderAdaptor {
    fn new(builder: &PyAny) -> PyBuilderAdaptor {
        PyBuilderAdaptor { builder: Ptr::from(builder) }
    }

    pub fn is_adaptor_empty(&self) -> bool {
        return Ptr::is_null(self.builder.borrow()) || self.builder.is_none()
    }

    python_methods!(self.builder.x(qubit: i64, radians: f64));
    python_methods!(self.builder.y(qubit: i64, radians: f64));
    python_methods!(self.builder.z(qubit: i64, radians: f64));
    python_methods!(self.builder.cx(controls: Vec<i64>, target: i64, radian: f64));
    python_methods!(self.builder.cy(controls: Vec<i64>, target: i64, radian: f64));
    python_methods!(self.builder.cz(controls: Vec<i64>, target: i64, radian: f64));
    python_methods!(self.builder.reset(qubit: i64));
    python_methods!(self.builder.measure(qubit: i64));
    python_methods!(self.builder.clear());
}

impl Deref for PyBuilderAdaptor {
    type Target = PyAny;

    fn deref(&self) -> &Self::Target {
        self.builder.deref()
    }
}

impl DerefMut for PyBuilderAdaptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.builder.deref_mut()
    }
}

impl Default for PyBuilderAdaptor {
    fn default() -> Self {
        PyBuilderAdaptor { builder: Ptr::default() }
    }
}

struct PyRuntimeAdaptor {
    runtime: Ptr<PyAny>
}

impl PyRuntimeAdaptor {
    fn new(runtime: &PyAny) -> PyRuntimeAdaptor {
        PyRuntimeAdaptor { runtime: Ptr::from(runtime) }
    }

    pub fn is_adaptor_empty(&self) -> bool {
        return Ptr::is_null(self.runtime.borrow()) || self.runtime.is_none()
    }

    python_methods!(self.runtime.execute(builder: &PyAny));
}

impl Deref for PyRuntimeAdaptor {
    type Target = PyAny;

    fn deref(&self) -> &Self::Target {
        self.runtime.deref()
    }
}

impl DerefMut for PyRuntimeAdaptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.runtime.deref_mut()
    }
}

impl Default for PyRuntimeAdaptor {
    fn default() -> Self {
        PyRuntimeAdaptor { runtime: Ptr::default() }
    }
}

pub struct PythonEngine {
    builder: PyBuilderAdaptor,
    runtime: PyRuntimeAdaptor
}

impl PythonEngine {
    pub fn new(builder: &PyAny, backend: &PyAny) -> PythonEngine {
        PythonEngine { builder: PyBuilderAdaptor::new(builder), runtime: PyRuntimeAdaptor::new(backend) }
    }

    pub fn execute(&self) -> AnalysisResult {
        if self.runtime.is_adaptor_empty() || self.builder.is_adaptor_empty() {
            return AnalysisResult::empty();
        }

        let result = self.runtime.execute(self.builder.deref())
          .expect("Engine doesn't have an execute method.").expect("QPU didn't return a result.");

        AnalysisResult::new(
            result.extract().expect("Object returned from 'execute' isn't a distribution dictionary."))
    }
}

impl Default for PythonEngine {
    fn default() -> Self {
        PythonEngine { builder: PyBuilderAdaptor::default(), runtime: PyRuntimeAdaptor::default() }
    }
}

impl Builder for PythonEngine {
    fn clear(&self) -> &Self {
        self.builder.clear();
        self
    }
}

// TODO: Make sure we propagate Python exceptions for easy debugging.
impl InstructionBuilder for PythonEngine {
    fn measure(&self, qb: &Qubit) -> &Self {
        self.builder.measure(qb.index);
        self
    }

    fn x(&self, qb: &Qubit, radians: f64) -> &Self {
        self.builder.x(qb.index, radians);
        self
    }

    fn y(&self, qb: &Qubit, radians: f64) -> &Self {
        self.builder.y(qb.index, radians);
        self
    }

    fn z(&self, qb: &Qubit, radians: f64) -> &Self {
        self.builder.z(qb.index, radians);
        self
    }

    fn cx(&self, controls: &Vec<Qubit>, target: &Qubit, radians: f64) -> &Self {
        let controls: Vec<i64> = controls.iter().map(|val| val.index).collect::<Vec<_>>();
        self.builder.cx(controls, target.index, radians);
        self
    }

    fn cy(&self, controls: &Vec<Qubit>, target: &Qubit, radians: f64) -> &Self {
        let controls = controls.iter().map(|val| val.index).collect::<Vec<_>>();
        self.builder.cy(controls, target.index, radians);
        self
    }

    fn cz(&self, controls: &Vec<Qubit>, target: &Qubit, radians: f64) -> &Self {
        let controls: Vec<i64> = controls.iter().map(|val| val.index).collect::<Vec<_>>();
        self.builder.cz(controls, target.index, radians);
        self
    }

    fn reset(&self, qb: &Qubit) -> &Self {
        self.builder.reset(qb.index);
        self
    }
}

pub trait Builder {
    fn clear(&self) -> &Self { self }
}

pub trait InstructionBuilder: Builder {
    fn measure(&self, qb: &Qubit) -> &Self { self }

    fn had(&self, qb: &Qubit) -> &Self {
        self.z(qb, PI);
        self.y(qb, PI / 2.0)
    }

    fn i(&self, qb: &Qubit) -> &Self {
        self
    }

    fn x(&self, qb: &Qubit, radii: f64) -> &Self { self }

    fn y(&self, qb: &Qubit, radii: f64) -> &Self { self }

    fn z(&self, qb: &Qubit, radii: f64) -> &Self { self }

    fn u(&self, qb: &Qubit, theta: f64, phi: f64, lambda: f64) -> &Self {
        self.z(qb, lambda).y(qb, phi).z(qb, theta)
    }

    fn swap(&self, first: &Qubit, second: &Qubit) -> &Self { self }

    fn sx(&self, qb: &Qubit) -> &Self {
        self.x(qb, PI / 2.0)
    }

    fn sx_dgr(&self, qb: &Qubit) -> &Self {
        self.x(qb, -(PI / 2.0))
    }

    fn s(&self, qb: &Qubit) -> &Self {
        self.z(qb, PI / 2.0)
    }

    fn s_dgr(&self, qb: &Qubit) -> &Self {
        self.z(qb, -(PI / 2.0))
    }

    fn t(&self, qb: &Qubit) -> &Self {
        self.z(qb, PI / 4.0)
    }

    fn t_dgr(&self, qb: &Qubit) -> &Self {
        self.z(qb, -(PI / 4.0))
    }

    fn cx(&self, controls: &Vec<Qubit>, target: &Qubit, radii: f64) -> &Self { self }

    fn cy(&self, controls: &Vec<Qubit>, target: &Qubit, radii: f64) -> &Self { self }

    fn cz(&self, controls: &Vec<Qubit>, target: &Qubit, radii: f64) -> &Self { self }

    fn cnot(&self, control: &Qubit, target: &Qubit, radii: f64) -> &Self {
        self.cx(&vec!(control.clone()), target, radii)
    }

    fn ccnot(&self, c1: &Qubit, c2: &Qubit, target: &Qubit, radii: f64) -> &Self {
        self.cx(&vec!(c1.clone(), c2.clone()), target, radii)
    }

    fn reset(&self, qb: &Qubit) -> &Self { self }
}