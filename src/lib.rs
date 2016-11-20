extern crate byteorder;
extern crate bytes;
// The `futures` crate contains the future & stream implementations as well
// as combinators to manipulate the async values.
extern crate futures;
// The `tokio_core` crate contains the async IO runtime.
extern crate tokio_core;
// The `tokio_proto` crate contains the abstractions and building blocks for
// quickly implementing a protocol client or server.
extern crate tokio_proto;
// The `Service` trait
extern crate tokio_service;

#[macro_use]
extern crate log;

// This is the second implementation of the transport. It uses tokio::io::Framed - which works with
// the concept of a Parser and Serializer and works with higher level abstractions from the bytes
// crate. Its implementation is much simpler and less error prone, and would be the correct choice
// in production code.
pub mod framed_transport;
pub use framed_transport::FramedLineTransport as LineTransport;
pub use framed_transport::new_line_transport;

// Contains the definition of the service that is used both by client and server. It also contains
// the function showing how to serve a service up.
pub mod service;

// Contains the client part - connecting and calling a remote service.
pub mod client;
