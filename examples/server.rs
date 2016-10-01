//! An echo server that times out
//!
//! The server can be run by executing:
//!
//! ```
//! cargo run --example server
//! ```
//!
//! Then connect to it using telnet.

extern crate futures;
extern crate tokio_core as tokio;
extern crate tokio_line as line;
extern crate tokio_service as service;
extern crate tokio_timer as timer;
extern crate rand;
extern crate env_logger;

use tokio::reactor::Core;

pub fn main() {
    env_logger::init().unwrap();

    let mut lp = Core::new().unwrap();

    // The address to bind the listener socket to
    let addr = "127.0.0.1:12345".parse().unwrap();

    // The service to run
    let service = service::simple_service(|msg: String| {
        Ok(msg.chars().rev().collect::<String>())
    });

    // Start the server
    line::service::serve(&lp.handle(), addr, service).unwrap();

    println!("Echo server running on {}", addr);

    lp.run(futures::empty::<(), ()>()).unwrap();
}
