/// contains built in functions
use warpspeed::{Value, ValueList};


pub fn print(args: &mut ValueList) -> Value {
    println!("{0}", args[0]);
    0
}
