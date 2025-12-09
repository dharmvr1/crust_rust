use std::{thread, time};

fn main() {
    for n in 1..1001 {
        let start = time::Instant::now();
        let pause = time::Duration::from_millis(20);
        let mut handler_vec = Vec::with_capacity(n);

        for m in 0..n {
            let handle = thread::spawn(move || {
                thread::sleep(pause);
            });
            handler_vec.push(handle);
        }
        // while let Some(handle) = handler_vec.pop() {
        //     handle.join();
        // }

        for handle in handler_vec {
            handle.join();
        }

        let finish = time::Instant::now();

        println!("{}\t{:02?}", n, finish.duration_since(start));
    }
}
