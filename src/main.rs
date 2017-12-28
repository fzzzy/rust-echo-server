
// example from Programming Rust chapter 18 input and output

use std::net::TcpListener;
use std::io;
use std::thread::spawn;

/// Accept connections forever, spawning a thread for each one.
fn echo_main(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("Listening on {}.", addr);
    loop {
        // Wait for a client to connect.
        let (mut stream, addr) = listener.accept()?;
        println!("Connection received from {}.", addr);

        // Spawn a thread to hand this client.
        let mut write_stream = stream.try_clone()?;
        spawn(move || {
            // Echo everything we receive from `stream` back to it.
            io::copy(&mut stream, &mut write_stream)
                .expect("Error in client thread: ");
            println!("Connection closed.");
        });
    }
}

fn main() {
    echo_main("127.0.0.1:7007").expect("Error in main: ");
}
