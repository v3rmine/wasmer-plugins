pub use helpers::*;
use std::fs;
use typed_builder::TypedBuilder;
use wasmer::{ImportObject, Instance, Module, RuntimeError, Store};
pub use wasmer_helpers_macros::*;

mod standard_types;
pub use standard_types::*;

mod complex_types;
pub use complex_types::*;

const START_WASM_MEMORY: usize = 5;

pub trait CallWithTuple<Inputs, Output> {
    fn call_with_tuple(&self, t: Inputs) -> Result<Output, RuntimeError>;
}

#[derive(TypedBuilder)]
pub struct InstanceBuilder<'path> {
    wasm_path: &'path str,
    #[builder(default)]
    store: Store,
    #[builder(default = wasmer::imports! {})]
    import_object: ImportObject,
}

impl<'path> InstanceBuilder<'path> {
    pub fn finalize(&self) -> Result<Instance, Box<dyn std::error::Error>> {
        Ok(Instance::new(
            &Module::new(&self.store, fs::read(self.wasm_path)?)?,
            &self.import_object,
        )?)
    }
}

// A1..A20 to map https://docs.rs/wasmer/2.0.0/wasmer/struct.NativeFunc.html
impl_ident_from_to!(0, 20, impl_call_with_tuple);
