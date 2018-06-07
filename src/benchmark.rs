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
fn bench_single_vs_multi_threaded_message_passing() {
    let mut start = Instant::now();
    single_threaded_processor();
    let mut elapsed = Instant::now().duration_since(start);
    println!("single: {:?} seconds passed", elapsed);
    start = Instant::now();
    multi_threaded_processor();
    elapsed = Instant::now().duration_since(start);
    println!("multi: {:?} seconds passed", elapsed);
}

fn single_threaded_processor() -> Result<(), Error> {
    for i in 0..NUM_ITER {
        let u: User = serde_json::from_str(BENCH_STRING)?;
        let target_string = format!("user.{}:{}", u.name, u.age);
    }
    Ok(())
}

fn multi_threaded_processor() -> Result<(), Error> {
    let (tx, rx) = spmc::channel::<User>();
    let mut handles = Vec::new();
    let THREAD_COUNT = 100;
    for n in 0..THREAD_COUNT {
        let rx = rx.clone();
        handles.push(thread::spawn(move || {
            match rx.recv() {
                Ok(u) => {
                    let target_string = format!("user.{}:{}", u.name, u.age);
                },
                // this only happens if the
                // sender has disconnected,
                // effectively means stop processing messages.
                _ => {}
            }
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
