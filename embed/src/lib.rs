use std::thread;

#[no_mangle]
pub extern fn process() -> i64 {
    let handles: Vec<_> = (0..10).map(|_| {
        thread::spawn(|| {
            let mut x = 0;
            for _ in (0..20_000_001) {
                x += 1
            }
        })
    }).collect();

    for h in handles {
        h.join().ok().expect("Could not join a thread!");
    }

    return 65;
}
