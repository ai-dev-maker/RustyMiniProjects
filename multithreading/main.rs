use std::thread;    
use std::sync::mpsc;
use std::time::Instant;

fn main() {
    let (sender, receiver) = mpsc::channel();

    let ranges = [(1, 250_000), (250_001, 500_000), (500_001, 1_000_000)];
    let mut handles = Vec::new();

    let now = Instant::now();
    for (start, end) in ranges {
        let sender_clone = sender.clone();
        let mut value = 0;
        let handle = thread::spawn(move || {
            for _ in start..=end {
                value += 1;
            }
            sender_clone.send(value).unwrap();
        });
        handles.push(handle);
    }
    let elapsed_time = now.elapsed();

    drop(sender);

    let mut total_val = 0;
    for received in receiver {
        total_val += received;
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let as_secs = elapsed_time.as_secs_f64();

    println!("Total value: {}\nWhole time: {} seconds", total_val, as_secs);
}
