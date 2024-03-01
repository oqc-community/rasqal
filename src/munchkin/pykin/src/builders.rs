// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2024 Oxford Quantum Circuits Ltd

use std::borrow::Borrow;
use std::f64::consts::PI;
use std::ops::{Deref, DerefMut};
use pyo3::{PyAny, PyResult};
use crate::analysis::{AnalysisResult};
use crate::blueprints::QuantumBlueprint;
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
    python_methods!(self.runtime.create_builder());
    python_methods!(self.runtime.can_run_blueprint(blueprint: &PyAny));
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

pub struct PythonRuntime {
    wrapped: PyRuntimeAdaptor
}

impl PythonRuntime {
    pub fn new(backend: &PyAny) -> PythonRuntime {
        PythonRuntime { wrapped: PyRuntimeAdaptor::new(backend) }
    }

    /// Returns whether this runtime can be actively used.
    pub fn is_usable(&self) -> bool {
        !self.wrapped.is_adaptor_empty()
    }

    pub fn execute(&self, builder: &Ptr<PythonBuilder>) -> AnalysisResult {
        let result = self.wrapped.execute(builder.wrapped.deref())
          .expect("Engine doesn't have an execute method.").expect("QPU didn't return a result.");

        AnalysisResult::new(
            result.extract().expect("Object returned from 'execute' isn't a distribution dictionary."))
    }

    pub fn create_builder(&self) -> Ptr<PythonBuilder> {
        Ptr::from(PythonBuilder::new(self.wrapped.create_builder()
          .expect("Runtime doesn't have a 'create_builder' method.").expect("Couldn't create a builder from runtime.")))
    }

    // TODO: Hook up to Python objects, make blueprint Python/Rust compatible.
    pub fn can_run_blueprint(&self, blueprint: &QuantumBlueprint) -> bool {
        true
    }
}

impl Default for PythonRuntime {
    fn default() -> Self {
        PythonRuntime { wrapped: PyRuntimeAdaptor::default() }
    }
}

pub(crate) struct PythonBuilder {
    wrapped: PyBuilderAdaptor
}

impl Default for PythonBuilder {
    fn default() -> Self {
        PythonBuilder { wrapped: PyBuilderAdaptor::default() }
    }
}

impl PythonBuilder {
    pub fn new(builder: &PyAny) -> PythonBuilder {
        PythonBuilder { wrapped: PyBuilderAdaptor::new(builder) }
    }

    /// Returns whether this builder can be actively used.
    pub fn is_usable(&self) -> bool {
        !self.wrapped.is_adaptor_empty()
    }
}

// TODO: Make sure we propagate Python exceptions for easy debugging.
impl InstructionBuilder for PythonBuilder {
    fn measure(&self, qb: &Qubit) -> &Self {
        self.wrapped.measure(qb.index);
        self
    }

    fn x(&self, qb: &Qubit, radians: f64) -> &Self {
        self.wrapped.x(qb.index, radians);
        self
    }

    fn y(&self, qb: &Qubit, radians: f64) -> &Self {
        self.wrapped.y(qb.index, radians);
        self
    }

    fn z(&self, qb: &Qubit, radians: f64) -> &Self {
        self.wrapped.z(qb.index, radians);
        self
    }

    fn cx(&self, controls: &Vec<Qubit>, target: &Qubit, radians: f64) -> &Self {
        let controls: Vec<i64> = controls.iter().map(|val| val.index).collect::<Vec<_>>();
        self.wrapped.cx(controls, target.index, radians);
        self
    }

    fn cy(&self, controls: &Vec<Qubit>, target: &Qubit, radians: f64) -> &Self {
        let controls = controls.iter().map(|val| val.index).collect::<Vec<_>>();
        self.wrapped.cy(controls, target.index, radians);
        self
    }

    fn cz(&self, controls: &Vec<Qubit>, target: &Qubit, radians: f64) -> &Self {
        let controls: Vec<i64> = controls.iter().map(|val| val.index).collect::<Vec<_>>();
        self.wrapped.cz(controls, target.index, radians);
        self
    }

    fn reset(&self, qb: &Qubit) -> &Self {
        self.wrapped.reset(qb.index);
        self
    }
}

pub trait InstructionBuilder {
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