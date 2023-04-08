use calendar::{Month, MonthContainer};
use std::sync::{Arc, Mutex};
use std::thread;

mod calendar;

fn test_schedule_concurrency() {
    // Create a shared MonthContainer object wrapped in a Mutex and an Arc.
    let month_container = Arc::new(Mutex::new(MonthContainer::new(Month::new(4).unwrap())));

    // Clone the Arc so we can pass a copy to each thread.
    let mc1 = month_container.clone();
    let mc2 = month_container.clone();

    // Define a closure that will be executed in each thread.
    let thread_closure = move |mc: Arc<Mutex<MonthContainer>>| {
        // Lock the Mutex and call the schedule method.
        let mut month = mc.lock().unwrap();
        println!("Using Thread number");
        let result = month.schedule(4, 5);

        // Print the result of the schedule method.
        println!("Thread result: {}", result);
    };

    // Spawn two threads and pass a copy of the Arc to each one.
    let handle1 = thread::spawn(move || thread_closure(mc1));
    let handle2 = thread::spawn(move || thread_closure(mc2));

    // Wait for the threads to complete.
    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn main() {
    test_schedule_concurrency();
    loop {}
}
