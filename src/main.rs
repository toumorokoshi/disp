#![feature(test)]
extern crate test;
extern crate futures;
extern crate serde;
extern crate serde_json;
extern crate spmc;

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
    use std::thread;
    use std::time::{Instant};
    use test::Bencher;
    use serde_json::{Value, Error};
    static BENCH_STRING: &'static str = "{\"name\": \"fred\", \"age\": 10}";
    static NUM_ITER:u8 = 10000000;

    /// This tests the theory of a single threaded
    /// application, with a tight loop, handling
    /// load faster than a multithreaded application.
    #[test]
    fn bench_single_threaded_message_passing() {
        let start = Instant::now();
        single_threaded_processor();
        let elapsed = Instant::now().duration_since(start);
        println!("{:?} seconds passed", elapsed);
    }

    fn single_threaded_processor() -> Result<(), Error> {
        for i in 0..NUM_ITER {
            let u: User = serde_json::from_str(BENCH_STRING)?;
            let target_string = format!("user.{}:{}", u.name, u.age);
        }
        Ok(())
    }

    fn multi_threaded_processor() -> Result<(), Error> {
        let (tx, rx) = spmc::channel<User>();
        let mut handles = Vec::new();
        let THREAD_COUNT = 5;
        for n in 0..THREAD_COUNT {
            let rx = rx.clone();
            handles.push(thread::spawn(move || {
                let u: User = rx.recv();
                let target_string = format!("user.{}:{}", u.name, u.age);
            }));
        }

        for i in 0..NUM_ITER {
            let u: User = serde_json::from_str(BENCH_STRING)?;
            tx.send(u);
        }

        for handle in handles {
            handle.join();
        }
        Ok(())
    }
}
