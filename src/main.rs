#![feature(test)]
extern crate test;
extern crate futures;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

fn main() {
    println!("Hello, world!");
}


#[derive(Serialize, Deserialize)]
pub struct User {
    age: u8,
    name: String
}


mod tests {
    use super::*;
    use test::Bencher;
    use serde_json::{Value, Error};
    static BENCH_STRING: &'static str = "{\"name\": \"fred\", \"age\": 10}";

    /// This tests the theory of a single threaded
    /// application, with a tight loop, handling
    /// load faster than a multithreaded application.
    #[bench]
    fn bench_single_threaded_message_passing(b: &mut Bencher) {
        b.iter(|| single_threaded_message_passing());
    }

    fn single_threaded_message_passing() -> Result<(), Error> {
        let u: User = serde_json::from_str(BENCH_STRING)?;
        let target_string = format!("user.{}:{}", u.name, u.age);
        Ok(())
    }
}
