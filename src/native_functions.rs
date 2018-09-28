/// Native functions that are available as functions within disp.
/// Functions within this module must be publicly exported in the main.rs
/// file, or else LLVM will be unable to discover the externs.

// no_mangle is required, to ensure that
// it resolves the name that's specified by the method
// signature.
#[no_mangle]
pub extern "C" fn print(value: i64) {
    println!("{}", value);
}
