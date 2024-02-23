#![deny(clippy::all, clippy::pedantic)]

use inkwell::{
    context::Context,
    memory_buffer::MemoryBuffer,
    module::Module,
    OptimizationLevel,
    passes::{PassManager, PassManagerBuilder},
    targets::{InitializationConfig, Target},
};
use std::{ffi::OsStr, path::Path};
use std::borrow::{Borrow};
use inkwell::values::FunctionValue;
use inkwell::attributes::AttributeLoc;
use crate::builders::PythonEngine;
use crate::evaluator::QIREvaluator;
use crate::graphs::ExecutableAnalysisGraph;
use crate::instructions::Value;
use crate::runtime::{ActiveTracers, QuantumRuntime, TracingModule};
use crate::smart_pointers::Ptr;

pub fn run_file(path: impl AsRef<Path>, args: &Vec<Value>, engine: &Ptr<PythonEngine>,
                entry_point: Option<&str>, tracer: ActiveTracers) -> Result<Option<Ptr<Value>>, String> {
    run_graph(&parse_file(path, entry_point)?, args, engine, tracer)
}

pub fn parse_file(path: impl AsRef<Path>, entry_point: Option<&str>) -> Result<Ptr<ExecutableAnalysisGraph>, String> {
    let context = Context::create();
    let module = file_to_module(path, &context)?;
    build_graph_from_module(&module, entry_point)
}

pub fn file_to_module(path: impl AsRef<Path>, context: &Context) -> Result<Module, String> {
    let path = path.as_ref();
    let extension = path.extension().and_then(OsStr::to_str);

    match extension {
        Some("ll") => MemoryBuffer::create_from_file(path)
            .and_then(|buffer| context.create_module_from_ir(buffer))
            .map_err(|e| e.to_string()),
        Some("bc") => Module::parse_bitcode_from_path(path, context).map_err(|e| e.to_string()),
        _ => Err(format!("Unsupported file extension '{:?}'.", extension)),
    }
}

pub fn build_graph_from_module(module: &Module, entry_point: Option<&str>) -> Result<Ptr<ExecutableAnalysisGraph>, String> {
    module.verify()
      .map_err(|e| format!("Failed to verify module: {}", e.to_string()))?;

    let pass_manager_builder = PassManagerBuilder::create();
    pass_manager_builder.set_optimization_level(OptimizationLevel::None);

    let fpm = PassManager::create(());
    fpm.add_global_dce_pass();
    fpm.add_strip_dead_prototypes_pass();
    pass_manager_builder.populate_module_pass_manager(&fpm);
    fpm.run_on(module);

    Target::initialize_native(&InitializationConfig::default())?;
    inkwell::support::load_library_permanently(&Path::new(""));

    let evaluator = QIREvaluator::new();
    evaluator.evaluate(
        &choose_entry_point(module_functions(module.borrow()), entry_point)?,
        &Ptr::from(module))
}

pub fn run_graph(graph: &Ptr<ExecutableAnalysisGraph>, arguments: &Vec<Value>, engine: &Ptr<PythonEngine>, tracer: ActiveTracers) -> Result<Option<Ptr<Value>>, String> {
    let engines = Ptr::from(EngineCollection::from(engine));
    let mut runtime = QuantumRuntime::new(engines.borrow(), tracer);
    runtime.execute(graph.borrow(), arguments)
}

/// Top-level collection item that holds information about target runtimes and engines for graphs.
pub struct EngineCollection {
    python_engine: Ptr<PythonEngine>
}

/// We don't have a 'new' because later on this will be a proper collection, but will have a
/// helper for creating from a single engine instance.
impl EngineCollection {
    pub fn from(python_engine: &Ptr<PythonEngine>) -> EngineCollection {
        EngineCollection {python_engine: python_engine.clone()}
    }

    pub fn get_available_QPU(&self) -> Ptr<PythonEngine> {
        self.python_engine.clone()
    }
}

impl Default for EngineCollection {
    fn default() -> Self {
        EngineCollection { python_engine: Ptr::default() }
    }
}

/// Returns all functions from a module.
pub fn module_functions<'ctx>(module: &Module<'ctx>) -> impl Iterator<Item = FunctionValue<'ctx>> {
    struct FunctionValueIter<'ctx>(Option<FunctionValue<'ctx>>);

    impl<'ctx> Iterator for FunctionValueIter<'ctx> {
        type Item = FunctionValue<'ctx>;

        fn next(&mut self) -> Option<Self::Item> {
            let function = self.0;
            self.0 = function.and_then(FunctionValue::get_next_function);
            function
        }
    }

    FunctionValueIter(module.get_first_function())
}

/// Checks if this function is a QIR entry-point.
pub fn is_entry_point(function: FunctionValue) -> bool {
    function
        .get_string_attribute(AttributeLoc::Function, "entry_point")
        .is_some()
        || function
            .get_string_attribute(AttributeLoc::Function, "EntryPoint")
            .is_some()
}

/// Looks through the entry-points available and either picks the method that matches the name
/// passed-in.
pub fn choose_entry_point<'ctx>(
    functions: impl Iterator<Item = FunctionValue<'ctx>>,
    name: Option<&str>,
) -> Result<FunctionValue<'ctx>, String> {
    if name.is_some() {
        functions.filter(|f| name.unwrap() == f.get_name().to_str().unwrap()).next().ok_or("Can't find a method with this nane.".to_string())
    } else {
        let eps: Vec<FunctionValue> = functions.filter(|f| is_entry_point(*f)).collect();
        if eps.is_empty() {
            return Err("Can't find any entry-points.".to_string());
        }

        if eps.len() > 1 {
            return Err("No specified method and more than one entry-point. Can't auto-detect.".to_string());
        }
        Ok(*(eps.first().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use std::fs::canonicalize;
    use bitflags::Flags;
    use crate::builders::{PythonEngine};
    use crate::execution::run_file;
    use crate::instructions::Value;
    use crate::runtime::ActiveTracers;
    use crate::smart_pointers::Ptr;

    #[test]
    fn execute_qaoa() {
        let relative_path = canonicalize("../tests/qsharp/qaoa/qir/qaoa.ll").unwrap();
        let path = relative_path.to_str().unwrap();

        let py_builder = Ptr::from(PythonEngine::default());
        run_file(path, &Vec::new(), py_builder.borrow(), None, ActiveTracers::empty());
    }

    #[test]
    fn execute_simplified_oracle_generator() {
        let relative_path = canonicalize("../tests/qsharp/simplified-oracle-generator/qir/simplified-oracle-generator.ll").unwrap();
        let path = relative_path.to_str().unwrap();

        let py_builder = Ptr::from(PythonEngine::default());
        run_file(path, &Vec::new(), py_builder.borrow(), None, ActiveTracers::empty());
    }

    #[test]
    fn execute_oracle_generator() {
        let relative_path = canonicalize("../tests/qsharp/oracle-generator/qir/oracle-generator.ll").unwrap();
        let path = relative_path.to_str().unwrap();

        let py_builder = Ptr::from(PythonEngine::default());
        run_file(path, &Vec::new(), py_builder.borrow(), None, ActiveTracers::empty());
    }

    #[test]
    fn execute_minified_oracle_generator() {
        let relative_path = canonicalize("../tests/qsharp/minified-oracle-generator/qir/minified-oracle-generator.ll").unwrap();
        let path = relative_path.to_str().unwrap();

        let py_builder = Ptr::from(PythonEngine::default());
        run_file(path, &vec![Value::Bool(true)], py_builder.borrow(), None, ActiveTracers::Graphs);
    }
}
