use std::time::{Duration, Instant};

#[test]
fn single_thread_test() {
    let mut limiter = ratelimit::Builder::new()
        .capacity(10)
        .quantum(10)
        .interval(Duration::from_secs(1))
        .single_thread();
    let begin = Instant::now();
    for _ in 0..20 {
        limiter.wait_for(1);
    }
    assert_eq!(begin.elapsed().as_secs(), 1);
}

#[test]
fn multi_thread_test() {
    let begin = Instant::now();
    let inner = ratelimit::Builder::new()
        .capacity(10)
        .quantum(5)
        .interval(Duration::from_secs(1))
        .multi_thread();
    let outer = inner.clone();
    let worker = std::thread::spawn(move || {
        for _ in 0..10 {
            inner.wait_for(1);
        }
    });
    for _ in 0..10 {
        outer.wait_for(1);
    }
    worker.join().unwrap();
    assert_eq!(begin.elapsed().as_secs(), 2);
}
