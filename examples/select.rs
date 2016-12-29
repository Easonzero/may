#[macro_use]
extern crate coroutine;

use std::time::Duration;
use coroutine::sync::mpsc::channel;
use coroutine::net::TcpListener;


// general select example that use cqueue
fn main() {
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();
    let listener = TcpListener::bind(("0.0.0.0", 1234)).unwrap();

    coroutine::spawn(move || {
        tx2.send("hello").unwrap();
        coroutine::sleep(Duration::from_millis(100));
        tx1.send(42).unwrap();
    });

    let id = select!(
        _ = listener.accept() => println!("got connected"),
        _ = coroutine::sleep(Duration::from_millis(1000)) => {},
        _ = rx1.recv() => println!("rx1 received"),
        a = rx2.recv() => println!("rx2 received, a={:?}", a)
    );

    assert_eq!(id, 3);
    assert_eq!(rx1.recv(), Ok(42));
}
