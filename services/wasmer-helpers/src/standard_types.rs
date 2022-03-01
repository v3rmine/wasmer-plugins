use std::fs;

use helpers::FuncInfo;
use wasmer::{
    imports, ExportError, FromToNativeWasmType, Instance, Module, NativeFunc, RuntimeError, Store,
    WasmTypeList,
};

use crate::CallWithTuple;

pub fn get_func_simple<FuncType>(
    instance: &Instance,
) -> Result<impl Fn(FuncType::Inputs) -> Result<FuncType::Output, RuntimeError>, ExportError>
where
    FuncType: FuncInfo,
    NativeFunc<FuncType::Inputs, FuncType::Output>:
        CallWithTuple<FuncType::Inputs, FuncType::Output>,
    FuncType::Inputs: WasmTypeList,
    FuncType::Output: FromToNativeWasmType,
{
    (instance.exports.get_native_function(FuncType::FUNC_ID)
        as Result<NativeFunc<FuncType::Inputs, FuncType::Output>, ExportError>)
        .map(|native_func| move |args| CallWithTuple::call_with_tuple(&native_func, args))
}

pub fn instanciate_simple(path: &str) -> Result<Instance, Box<dyn std::error::Error>> {
    let module_wat = fs::read(path)?;
    let store = Store::default();
    let module = Module::new(&store, &module_wat)?;
    let import_object = imports! {};
    Ok(Instance::new(&module, &import_object)?)
}
