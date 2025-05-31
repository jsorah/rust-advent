use std::sync::atomic::AtomicBool;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use std::thread;

use log::info;

pub struct Day4;
impl Day4 {
    // brute force impl
    pub fn run_single(prefix: &String) {
        let mut n: u64 = 0;
        loop {
            if Self::is_target_md5(n, prefix) {
                info!("{}", n);
                break;
            }

            n += 1;
        }
    }

    fn is_target_md5(n: u64, prefix: &String) -> bool {
        let val = format!("iwrupvqb{}", n);

        let merr = md5::compute(val.as_bytes());
        let output = format!("{:x}", merr);

        output.starts_with(prefix)
    }

    pub fn run_threaded(prefix: &String) {
        let mut handles = Vec::new();

        let num_threads: u8 = 8;
        let block_size = 25_000;

        let result = Arc::new(AtomicU64::new(u64::MAX));
        let counter = Arc::new(AtomicU64::new(0));
        let found = Arc::new(AtomicBool::new(false));

        for _i in 0..num_threads {
            let counter = Arc::clone(&counter);
            let found = Arc::clone(&found);
            let result = Arc::clone(&result);

            let prefix = prefix.clone();
            let handle = thread::spawn(move || {
                while !found.load(Ordering::Relaxed) {
                    let start = counter.fetch_add(block_size, Ordering::Relaxed);
                    let end = start + block_size;
                    for n in start..end {
                        if Self::is_target_md5(n, &prefix) {
                            let prev = result.fetch_min(n, Ordering::SeqCst);
                            if n < prev {
                                found.store(true, Ordering::SeqCst);
                            }
                            return;
                        }
                    }
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let final_result = result.load(Ordering::SeqCst);

        if final_result != u64::MAX {
            info!("Lowest matching number: {}", final_result);
        } else {
            info!("No match found.");
        }
    }
}
