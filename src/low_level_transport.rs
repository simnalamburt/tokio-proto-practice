use proto::pipeline;
use std::io;

/// This defines the chunks written to our transport, i.e. the representation
/// that the `Service` deals with. In our case, the received and sent frames
/// are the same (Strings with io::Error as failures), however they
/// could also be different (for example HttpRequest for In and HttpResponse
/// for Out).
pub type Frame = pipeline::Frame<String, (), io::Error>;
