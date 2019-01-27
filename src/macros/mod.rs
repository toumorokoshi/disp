use super::{FunctionMap, UnparsedFunction};
use std::{collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct Macro {}

pub type MacroMap = HashMap<String, Macro>;

/// modify function map in place, using macros to expand.
pub fn apply_macros_to_function_map(macros: &MacroMap, functions: &mut FunctionMap) {
    for value in functions.values_mut() {
        apply_macros_to_function(macros, value);
    }
}

fn apply_macros_to_function(macros: &MacroMap, function: &mut Rc<UnparsedFunction>) {}
