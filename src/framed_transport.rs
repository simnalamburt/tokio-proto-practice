use byteorder::LittleEndian;
use bytes::{Buf, MutBuf};
use bytes::buf::BlockBuf;
use std::{io, str, mem};
use tokio_core::io::Io;
use tokio_proto::{pipeline, Parse, Serialize, Framed};

/// This defines the chunks written to our transport, i.e. the representation
/// that the `Service` deals with. In our case, the received and sent frames
/// are the same (Strings with io::Error as failures), however they
/// could also be different (for example HttpRequest for In and HttpResponse
/// for Out).
pub type Frame = pipeline::Frame<String, (), io::Error>;

/// 현재 프로토콜에서 Length Prefix를 나타내는데에 어떤 타입을 사용하고있는지
type PrefixType = u32;
/// 현재 프로토콜에서 Length Prefix의 엔디언이 어떻게 되는지
type PrefixEndian = LittleEndian;

/// 프로토콜 파서의 state를 담고있는 타입.
#[derive(Debug)]
pub enum Parser {
    /// Length Prefix를 읽기 전 상태
    BeforePrefix,
    /// Length Prefix를 읽은 후 상태
    AfterPrefix { length: PrefixType }
}

impl Parser {
    /// Parser 구조체 생성자
    fn new() -> Self { Parser::BeforePrefix }
}

impl Parse for Parser {
    type Out = Frame;

    fn parse(&mut self, buf: &mut BlockBuf) -> Option<Frame> {
        match *self {
            Parser::BeforePrefix => {
                // Length prefix가 아직 파싱되지 못한 상태임.

                // Length Prefix의 길이
                let prefix_len = mem::size_of::<PrefixType>();

                // 버퍼에 length prefix가 다 들어있는지 체크
                if buf.len() < prefix_len { return None }

                // 파서 state 바꿈
                // 버퍼에서 length prefix 읽어들여서, 파서의 state를 바꿈
                *self = Parser::AfterPrefix {
                    // TODO: read_u32 잘 되니..?
                    length: buf.shift(prefix_len).buf().read_u32::<PrefixEndian>()
                };

                None
            }
            Parser::AfterPrefix { length } => {
                // Length prefix가 파싱된 상태임. prefix 대로 길이를 파싱해서 내려주면 됨

                // 버퍼에 원하는 길이만큼 차있는지 체크
                if buf.len() < length as usize { return None }

                // 후에 .bytes() 메소드를 호출하기위해, 버퍼 블록을 하나로 이어준다
                if !buf.is_compact() { buf.compact(); }

                // 버퍼에서 length 만큼 지움
                let line = buf.shift(length as usize);

                // Length prefix가 파싱되었으므로, 다음 파싱을 위해 파서의 state를 바꿈
                *self = Parser::BeforePrefix;

                // Turn this data into a UTF string and return it in a Frame.
                match str::from_utf8(line.buf().bytes()) {
                    Ok(s) => Some(pipeline::Frame::Message(s.to_string())),
                    Err(_) => Some(pipeline::Frame::Error(io::Error::new(io::ErrorKind::Other, "invalid string"))),
                }
            }
        }
    }

    fn done(&mut self, buf: &mut BlockBuf) -> Option<Frame> {
        // 연결이 끊겼음. 버퍼에 남아있는 데이터로 최대한 파싱 시도
        self.

        // TODO: Remove
        println!("\n\n\n{:?}\n\n\n", self);

        assert!(buf.is_empty());
        Some(pipeline::Frame::Done)
    }
}

pub struct Serializer;

impl Serialize for Serializer {
    type In = Frame;

    fn serialize(&mut self, frame: Frame, buf: &mut BlockBuf) {
        use tokio_proto::pipeline::Frame::*;

        let text = match frame {
            Message(text) => text,
            Error(e) => format!("[ERROR] {}", e),
            Done => return,

            MessageWithBody(..) | Body(..) => {
                // Our Line protocol does not support streaming bodies
                unreachable!();
            }
        };

        buf.write_u32::<PrefixEndian>(text.len() as PrefixType);
        buf.write_slice(&text.as_bytes());
    }
}

pub type FramedLineTransport<T> = Framed<T, Parser, Serializer>;

pub fn new_line_transport<T>(inner: T) -> FramedLineTransport<T>
    where T: Io,
{
  Framed::new(inner,
              Parser::new(),
              Serializer,
              BlockBuf::default(),
              BlockBuf::default())
}
