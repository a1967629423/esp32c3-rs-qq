use std::{net::{TcpListener, ToSocketAddrs, SocketAddrV4, SocketAddr}, io::Write};
fn time_to_bytes(time:i64) -> [u8;8] {
    let mut bytes = [0u8;8];
    bytes[0] = ((time >> 0) & 0xff) as u8;
    bytes[1] = ((time >> 8) & 0xff) as u8;
    bytes[2] = ((time >> 16) & 0xff) as u8;
    bytes[3] = ((time >> 24) & 0xff) as u8;
    bytes[4] = ((time >> 32) & 0xff) as u8;
    bytes[5] = ((time >> 40) & 0xff) as u8;
    bytes[6] = ((time >> 48) & 0xff) as u8;
    bytes[7] = ((time >> 56) & 0xff) as u8;
    return bytes;
}
#[allow(dead_code)]
fn read_buf(buf:&[u8]) -> i64 {
    (buf[0] as i64) | ((buf[1] as i64) << 8) | (buf[2] as i64) << 16 |  (buf[3] as i64) << 24 | (buf[4] as i64) << 32 |(buf[5] as i64) << 40 |(buf[6] as i64) << 48 | (buf[7] as i64) << 56
}
fn main() {
    let addr = std::env::args().skip(1).next().unwrap_or("127.0.0.1:7000".to_string());
    println!("bind: {:?}",addr.parse::<SocketAddr>());
    let tl = TcpListener::bind(addr).unwrap();
    for stream in tl.incoming() {
        match stream {
            Ok(mut stream) => {
                let ts = chrono::Utc::now().timestamp_nanos();
                println!("connect from {:?} ts {}",stream.peer_addr().unwrap(),ts);
                stream.write_all(&time_to_bytes(ts)).unwrap();
            },
            Err(_) => {},
        }

    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_time() {
        let ts = chrono::Utc::now().timestamp_nanos();
        let bytes = time_to_bytes(ts);
        let time = read_buf(&bytes);
        assert_eq!(ts, time);
    }
}