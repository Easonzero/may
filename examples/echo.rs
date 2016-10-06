extern crate coroutine;
use coroutine::net::{TcpListener, TcpStream};
// use std::time::Duration;
// use std::io::ErrorKind;
use std::io::{Read, Write};


fn handle_client(mut stream: TcpStream) {
    // read 20 bytes at a time from stream echoing back to stream
    // stream.set_read_timeout(Some(Duration::from_secs(10))).expect("can't set read timeout");
    // let mut i = 0;
    let mut read = vec![0; 1024 * 16]; // alloc in heap!
    loop {
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 {
                    // connection was closed
                    break;
                }
                stream.write(&read[0..n]).unwrap();
            }
            Err(err) => {
                println!("err = {:?}", err);
                return;
                // if err.kind() == ErrorKind::TimedOut {
                //     println!("read timeout");
                // } else {
                //     println!("err = {:?}", err);
                // }
                //
                // i += 1;
                // println!("retry = {:?}", i);
                // if i > 5 {
                //     println!("bye!!");
                //     return;
                // }
            }
        }
    }
}


/// simple test: echo hello | nc 127.0.0.1 8080

fn main() {
    coroutine::scheduler_set_workers(1);

    coroutine::spawn(|| {
            let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        coroutine::spawn(move || {
                            handle_client(stream);
                        });
                    }
                    Err(_) => {
                        println!("Error");
                    }
                }
            }
        })
        .join()
        .unwrap();
}
