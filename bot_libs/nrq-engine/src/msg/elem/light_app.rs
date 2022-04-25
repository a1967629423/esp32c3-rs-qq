#[cfg(not(feature = "std"))]
use crate::simulate_std::prelude::*;
use bytes::BufMut;
use miniz_oxide::{deflate::compress_to_vec_zlib, inflate::decompress_to_vec_zlib};
// use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};

use crate::pb::msg;

#[derive(Default, Debug, Clone)]
pub struct LightApp {
    pub content: String,
}

impl LightApp {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl From<LightApp> for Vec<msg::elem::Elem> {
    fn from(e: LightApp) -> Self {
        vec![msg::elem::Elem::LightApp(msg::LightApp {
            data: Some({
                let mut w: Vec<u8> = Vec::new();
                // let mut encoder = ZlibEncoder::new(vec![1], Compression::default());
                // encoder.write_all(e.content.as_bytes()).ok();
                // encoder.finish().unwrap_or_default()
                let res = compress_to_vec_zlib(e.content.as_bytes(), 6);
                w.put_u8(1);
                w.put_slice(res.as_slice());
                w
            }),
            ..Default::default()
        })]
    }
}

impl From<msg::LightApp> for LightApp {
    fn from(e: msg::LightApp) -> Self {
        let data = e.data.unwrap_or_default();
        if data.len() > 1 {
            let content = if data[0] == 0 {
                data[1..].to_vec()
            } else {
                // let mut uncompressed = Vec::new();
                // ZlibDecoder::new(&data[1..])
                //     .read_to_end(&mut uncompressed)
                //     .unwrap();
                // uncompressed
                let uncompressed = decompress_to_vec_zlib(&data[1..]).unwrap();
                uncompressed
            };
            if !content.is_empty() && content.len() < 1024 ^ 3 {
                return Self {
                    content: String::from_utf8(content).unwrap(),
                };
            }
        }
        Self::default()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    fn init() {}
    #[test]
    fn test_compress() {
        init();
        let app = LightApp {
            content: "Hello world".to_string(),
        };
        let ele = Vec::<msg::elem::Elem>::from(app);
        match &ele[0] {
            msg::elem::Elem::LightApp(app) => {
                assert_eq!(
                    app.data,
                    Some(vec![
                        1, 120, 156, 243, 72, 205, 201, 201, 87, 40, 207, 47, 202, 73, 1, 0, 24,
                        171, 4, 61
                    ])
                );
                assert_eq!(app.msg_resid, None);
            }
            _ => {
                panic!("unreachable");
            }
        }
    }
    #[test]
    fn test_decompress() {
        init();
        let data = vec![
            1u8, 120, 156, 243, 72, 205, 201, 201, 87, 40, 207, 47, 202, 73, 1, 0, 24, 171, 4, 61,
        ];
        let elem = msg::LightApp {
            data: Some(data),
            msg_resid: None,
        };
        let app = LightApp::from(elem);
        assert_eq!(app.content, "Hello world");
    }
}
