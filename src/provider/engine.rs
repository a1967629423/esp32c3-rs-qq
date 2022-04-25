use nrs_qq::engine::{CoreCryptoRng, TimerProvider};
use smol::net::AsyncToSocketAddrs;
use std::{net::ToSocketAddrs, time::Instant};
// engine provider
pub struct MyTimeProvider {
    pub net_time: i64,
    pub net_begin: Instant,
}
#[inline]
fn read_buf(buf: &[u8]) -> i64 {
    (buf[0] as i64)
        | ((buf[1] as i64) << 8)
        | (buf[2] as i64) << 16
        | (buf[3] as i64) << 24
        | (buf[4] as i64) << 32
        | (buf[5] as i64) << 40
        | (buf[6] as i64) << 48
        | (buf[7] as i64) << 56
}
impl MyTimeProvider {
    pub fn new_form_net_sync<T: ToSocketAddrs>(addr: T) -> Self {
        let buf = &mut [0u8; 8];
        {
            let stream = match std::net::TcpStream::connect(addr) {
                Ok(o) => {o},
                Err(e) => {
                    log::info!("tcp read error {:?}",e);
                    panic!("{:?}",e);
                },
            };
            stream.peek(buf).unwrap();
        }
        let net_begin = Instant::now();
        let net_time = read_buf(buf);
        Self {
            net_begin,
            net_time,
        }
    }
    pub async fn new_from_net_async<T: AsyncToSocketAddrs>(addr: T) -> Self {
        let buf = &mut [0u8; 8];
        {
            let stream = smol::net::TcpStream::connect(addr).await.unwrap();
            stream.peek(buf).await.unwrap();
        }
        let net_begin = Instant::now();
        let net_time = read_buf(buf);
        Self {
            net_begin,
            net_time,
        }
    }
    pub fn new() -> Self {
        Self {
            net_begin: Instant::now(),
            net_time: 0,
        }
    }
}
impl TimerProvider for MyTimeProvider {
    fn now_timestamp_nanos(&self) -> i64 {
        let dur = Instant::now() - self.net_begin;
        return self.net_time + dur.as_nanos() as i64;
    }
}
pub fn fill_bytes_via_next<R: rand::RngCore + ?Sized>(rng: &mut R, dest: &mut [u8]) {
    let mut left = dest;
    while left.len() >= 8 {
        let (l, r) = { left }.split_at_mut(8);
        left = r;
        let chunk: [u8; 8] = rng.next_u64().to_le_bytes();
        l.copy_from_slice(&chunk);
    }
    let n = left.len();
    if n > 4 {
        let chunk: [u8; 8] = rng.next_u64().to_le_bytes();
        left.copy_from_slice(&chunk[..n]);
    } else if n > 0 {
        let chunk: [u8; 4] = rng.next_u32().to_le_bytes();
        left.copy_from_slice(&chunk[..n]);
    }
}
pub struct MyRandomProvider(u64);
impl MyRandomProvider {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }
}
impl rand::RngCore for MyRandomProvider {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        let result = self.0;
        self.0 = self.0.wrapping_mul(181783497276652981);
        result
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        fill_bytes_via_next(self, dest);
        // unsafe {
        //     let mut n:u32 = 0;
        //     for i in 0..4usize {
        //          match dest.get(i) {
        //              Some(val) => {
        //                  n |= ((*val) as u32) << (i * 8);
        //              },
        //              None => {},
        //          }

        //     }
        //     esp_idf_sys::srandom(n);
        // }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        log::info!("try_fill_bytes()");
        self.fill_bytes(dest);
        Ok(())
    }
}
impl rand::CryptoRng for MyRandomProvider {}
impl CoreCryptoRng for MyRandomProvider {}
