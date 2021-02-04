extern crate dusk_api;
extern crate dusk_syntax;

mod text_preprocessing;

use std::any::{Any, TypeId};
use dusk_api::{Freight, export_plugin, Function, RuntimeError};
use dusk_syntax::{code_reference, warn, ElementReference, Element};

export_plugin!("precompiler", "0.0.0", Precompiler);

pub struct Precompiler;

impl Freight for Precompiler {
    fn call_function (
        self: &mut Self,
        function_number: u64,
        mut args: Vec<&mut Box<dyn Any>>
        ) -> Result<Box<dyn Any>, RuntimeError> {
        Ok(Box::new(0))
    }

    fn get_function_list (self: &mut Self) -> Vec<Function> {
        let mut result: Vec<Function> = Vec::new();
        Vec::new()
    }
}
