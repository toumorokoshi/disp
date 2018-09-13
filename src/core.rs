/// core VM data structures
use std::collections::HashMap;

pub type Register = i64;
pub type RegisterList = Vec<Register>;
pub type Value = i64;
pub type ValueList = Vec<Value>;
pub type Map = HashMap<Value, Value>;
