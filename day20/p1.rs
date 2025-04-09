use std::thread;
use std::time;

fn main() {
    let mut handles = vec![];
    for i in 0..=18 {
        let handle = thread::spawn(move || {
            let now = time::Instant::now();
            let mut n: u64 = i * 50000;
            let mut total: u64 = 0;
            while total < 29_000_000 && n <= (i + 1) * 50000 {
                total = 0;
                n += 1;
                for d in 1..=n {
                    if n % d == 0 && n / d <= 50 {
                        total += d * 11;
                    }
                }
            }
            if total >= 29_000_000 {
                println!("{} {}", n, total);
            } else {
                println!("{} . {} secs", n, now.elapsed().as_secs());
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
