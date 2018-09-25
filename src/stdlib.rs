use super::{
};
use warpspeed::VM;

// for now, we can emulate a standard library
// by just loading a lib file before executing the file in question.
const LIB_FILE: &'static str = "lib.ds";


// load the standard library
pub fn load_stdlib(vm: &mut VM) {
}
