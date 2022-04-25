use bytes::{Buf, BufMut, Bytes, BytesMut};
use nrq_engine::{RQError, RQResult};

use crate::{AsyncRead, AsyncWrite};

use super::{decoder::Decoder, encoder::Encoder, framed::Framed};

#[derive(Debug, Clone, Copy)]
pub struct Builder {
    // Maximum frame length
    max_frame_len: usize,

    // Number of bytes representing the field length
    length_field_len: usize,

    // Number of bytes in the header before the length field
    length_field_offset: usize,

    // Adjust the length specified in the header field by this amount
    length_adjustment: isize,

    // Total number of bytes to skip before reading the payload, if not set,
    // `length_field_len + length_field_offset`
    num_skip: Option<usize>,

    // Length field byte order (little or big endian)
    length_field_is_big_endian: bool,
}

/// An error when the number of bytes read is more than max frame length.
pub struct LengthDelimitedCodecError {
    _priv: (),
}

/// A codec for frames delimited by a frame head specifying their lengths.
///
/// This allows the consumer to work with entire frames without having to worry
/// about buffering or other framing logic.
///
/// See [module level] documentation for more detail.
///
/// [module level]: index.html
#[derive(Debug, Clone)]
pub struct LengthDelimitedCodec {
    // Configuration values
    builder: Builder,

    // Read state
    state: DecodeState,
}

#[derive(Debug, Clone, Copy)]
enum DecodeState {
    Head,
    Data(usize),
}

impl LengthDelimitedCodec {
    pub fn new() -> Self {
        Self {
            builder: Builder::new(),
            state: DecodeState::Head,
        }
    }
    pub fn builder() -> Builder {
        Builder::new()
    }
    fn decode_head(&mut self, src: &mut BytesMut) -> RQResult<Option<usize>> {
        // log::info!("decode_head()");
        let head_len = self.builder.num_head_bytes();
        let field_len = self.builder.length_field_len;

        if src.len() < head_len {
            // Not enough data
            return Ok(None);
        }

        let n = {
            //let mut src = Cursor::new(&mut *src);
            let mut src = BytesMut::from(src.chunk());
            // Skip the required bytes
            src.advance(self.builder.length_field_offset);

            // match endianness
            let n = if self.builder.length_field_is_big_endian {
                src.get_uint(field_len)
            } else {
                src.get_uint_le(field_len)
            };

            if n > self.builder.max_frame_len as u64 {
                // return Err(io::Error::new(
                //     io::ErrorKind::InvalidData,
                //     LengthDelimitedCodecError { _priv: () },
                // ));
                return Err(RQError::IO("InvalidData LengthDelimitedCodecError".into()));
            }

            // The check above ensures there is no overflow
            let n = n as usize;

            // Adjust `n` with bounds checking
            let n = if self.builder.length_adjustment < 0 {
                n.checked_sub(-self.builder.length_adjustment as usize)
            } else {
                n.checked_add(self.builder.length_adjustment as usize)
            };

            // Error handling
            match n {
                Some(n) => n,
                None => {
                    // return Err(io::Error::new(
                    //     io::ErrorKind::InvalidInput,
                    //     "provided length would overflow after adjustment",
                    // ));
                    return Err(RQError::IO(
                        "provided length would overflow after adjustment".into(),
                    ));
                }
            }
        };

        let num_skip = self.builder.get_num_skip();

        if num_skip > 0 {
            src.advance(num_skip);
        }

        // Ensure that the buffer has enough space to read the incoming
        // payload
        src.reserve(n);

        // log::info!("decode_head() done");
        Ok(Some(n))
    }
    fn decode_data(&self, n: usize, src: &mut BytesMut) -> Option<BytesMut> {
        // log::info!("decode_data()");
        // At this point, the buffer has already had the required capacity
        // reserved. All there is to do is read.
        if src.len() < n {
            return None;
        }

        // log::info!("decode_data() finish");
        Some(src.split_to(n))
    }
}

impl Decoder for LengthDelimitedCodec {
    type Item = BytesMut;
    type Error = RQError;
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<BytesMut>, Self::Error> {
        let n = match self.state {
            DecodeState::Head => match self.decode_head(src)? {
                Some(n) => {
                    self.state = DecodeState::Data(n);
                    n
                }
                None => return Ok(None),
            },
            DecodeState::Data(n) => n,
        };
        match self.decode_data(n, src) {
            Some(data) => {
                // Update the decode state
                self.state = DecodeState::Head;

                // Make sure the buffer has enough space to read the next head
                src.reserve(self.builder.num_head_bytes());

                Ok(Some(data))
            }
            None => Ok(None),
        }
    }
}

impl Encoder<Bytes> for LengthDelimitedCodec {
    type Error = RQError;
    fn encode(&mut self, data: Bytes, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let n = data.len();

        if n > self.builder.max_frame_len {
            // return Err(io::Error::new(
            //     io::ErrorKind::InvalidInput,
            //     LengthDelimitedCodecError { _priv: () },
            // ));
            return Err(RQError::IO("InvalidInput LengthDelimitedCodecError".into()));
        }

        // Adjust `n` with bounds checking
        let n = if self.builder.length_adjustment < 0 {
            n.checked_add(-self.builder.length_adjustment as usize)
        } else {
            n.checked_sub(self.builder.length_adjustment as usize)
        };

        let n = n.ok_or_else(|| {
            // io::Error::new(
            //     io::ErrorKind::InvalidInput,
            //     "provided length would overflow after adjustment",
            // )
            RQError::IO("provided length would overflow after adjustment".into())
        })?;

        // Reserve capacity in the destination buffer to fit the frame and
        // length field (plus adjustment).
        dst.reserve(self.builder.length_field_len + n);

        if self.builder.length_field_is_big_endian {
            dst.put_uint(n as u64, self.builder.length_field_len);
        } else {
            dst.put_uint_le(n as u64, self.builder.length_field_len);
        }

        // Write the frame to the buffer
        dst.extend_from_slice(&data[..]);

        Ok(())
    }
}

impl Builder {
    pub fn new() -> Self {
        Self {
            max_frame_len: 8 * 1024 * 1024,
            length_field_len: 4,
            length_field_offset: 0,
            length_adjustment: 0,
            num_skip: None,
            length_field_is_big_endian: true,
        }
    }
    pub fn big_endian(&mut self) -> &mut Self {
        self.length_field_is_big_endian = true;
        self
    }
    pub fn little_endian(&mut self) -> &mut Self {
        self.length_field_is_big_endian = false;
        self
    }
    pub fn max_frame_length(&mut self, val: usize) -> &mut Self {
        self.max_frame_len = val;
        self
    }
    pub fn length_field_length(&mut self, val: usize) -> &mut Self {
        assert!(val > 0 && val <= 8, "invalid length field length");
        self.length_field_len = val;
        self
    }
    pub fn length_field_offset(&mut self, val: usize) -> &mut Self {
        self.length_field_offset = val;
        self
    }
    pub fn length_adjustment(&mut self, val: isize) -> &mut Self {
        self.length_adjustment = val;
        self
    }
    pub fn num_skip(&mut self, val: usize) -> &mut Self {
        self.num_skip = Some(val);
        self
    }
    pub fn new_codec(&self) -> LengthDelimitedCodec {
        LengthDelimitedCodec {
            builder: *self,
            state: DecodeState::Head,
        }
    }
    pub fn new_framed<T>(&self, inner: T) -> Framed<T, LengthDelimitedCodec>
    where
        T: AsyncRead + AsyncWrite,
    {
        Framed::new(inner, self.new_codec())
    }
    fn num_head_bytes(&self) -> usize {
        let num = self.length_field_offset + self.length_field_len;
        core::cmp::max(num, self.num_skip.unwrap_or(0))
    }

    fn get_num_skip(&self) -> usize {
        self.num_skip
            .unwrap_or(self.length_field_offset + self.length_field_len)
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Debug for LengthDelimitedCodecError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("LengthDelimitedCodecError").finish()
    }
}

impl core::fmt::Display for LengthDelimitedCodecError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("frame size too big")
    }
}
