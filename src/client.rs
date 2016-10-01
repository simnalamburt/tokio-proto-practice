use futures::{self, Async, Future};
use std::io;
use std::net::SocketAddr;
use tokio_service::Service;
use tokio_proto::{self, pipeline};
use tokio_core::reactor::Handle;
use tokio_core::net::TcpStream;
use futures::stream::Empty;
use new_line_transport;

/// And the client handle.
pub struct Client {
    inner: tokio_proto::Client<String, String, Empty<(), io::Error>, io::Error>,
}

impl Service for Client {
    type Request = String;
    type Response = String;
    type Error = io::Error;
    // Again for simplicity, we are just going to box a future
    type Future = Box<Future<Item = Self::Response, Error = io::Error>>;

    fn call(&self, req: String) -> Self::Future {
        // Make sure that the request does not include any new lines
        if req.chars().find(|&c| c == '\n').is_some() {
            let err = io::Error::new(io::ErrorKind::InvalidInput, "message contained new line");
            return Box::new(futures::done(Err(err)))
        }

        self.inner.call(tokio_proto::Message::WithoutBody(req))
            .boxed()
    }

    fn poll_ready(&self) -> Async<()> {
        Async::Ready(())
    }
}

pub fn connect(handle: Handle, addr: &SocketAddr) -> Client {
    let addr = addr.clone();
    let h = handle.clone();

    let new_transport = move || {
        TcpStream::connect(&addr, &h).map(new_line_transport)
    };

    // Connect the client
    let client = pipeline::connect(new_transport, &handle);
    Client { inner: client }
}
