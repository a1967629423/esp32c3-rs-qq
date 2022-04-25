use crate::provider::*;
use crate::Client;
use alloc::sync::Arc;
use core::sync::atomic::Ordering;
use futures::SinkExt;
use futures::StreamExt;
use no_std_net::{Ipv4Addr, SocketAddr};

use super::ext::fuse::FusedSplitStream;
use super::ext::length_delimited::LengthDelimitedCodec;
impl<CP, OSCP, RP, MP, TP, TCP> Client<CP, OSCP, RP, MP, TP, TCP>
where
    CP: ChannelProvider,
    OSCP: OneShotChannelProvider,
    RP: RwLockProvider,
    TP: TaskProvider,
    TCP: TcpStreamProvider,
    MP: MutexProvider,
{
    pub fn get_address(&self) -> SocketAddr {
        // TODO 选择最快地址
        SocketAddr::new(Ipv4Addr::new(42, 81, 176, 211).into(), 443)
    }
}
impl<CP, OSCP, RP, MP, TP, TCP> Client<CP, OSCP, RP, MP, TP, TCP>
where
    CP: ChannelProvider + 'static,
    OSCP: OneShotChannelProvider + 'static,
    RP: RwLockProvider + 'static,
    TP: TaskProvider + 'static,
    TCP: TcpStreamProvider + 'static,
    MP: MutexProvider + 'static,
{
    pub fn stop(self: &Arc<Self>) {
        self.running.store(false, Ordering::Relaxed);
        self.disconnect();
    }
    fn disconnect(self: &Arc<Self>) {
        // TODO dispatch disconnect event
        // don't unwrap (Err means there is no receiver.)
        let my = self.clone();
        TP::spawn(async move { my.disconnect_signal.send(()).await.ok() }).detach();
    }

    // 开始处理流数据
    pub async fn start<S: AsyncRead + AsyncWrite>(self: &Arc<Self>, stream: S) {
        self.running.store(true, Ordering::Relaxed);
        self.net_loop(stream).await; // 阻塞到断开
        self.disconnect();
    }

    async fn net_loop<S: AsyncRead + AsyncWrite>(self: &Arc<Self>, stream: S) {
        let (mut write_half, read_half) = LengthDelimitedCodec::builder()
            .length_field_length(4)
            .length_adjustment(-4)
            .new_framed(stream)
            .split();
        let cli = self.clone();
        let mut rx = self.out_pkt_sender.subscribe();
        let mut disconnect_signal = self.disconnect_signal.subscribe();
        let mut read_half = FusedSplitStream(read_half);
        loop {
            futures::select_biased! {
                input = read_half.next() => {
                    if let Some(Ok(mut input)) = input {
                        if let Ok(pkt)=cli.engine.read().await.transport.decode_packet(&mut input){
                            cli.process_income_packet(pkt).await;
                        }else {
                            break;
                        }
                    }else {
                        break;
                    }
                }
                output = rx.recv() => {
                    if let Ok(output) = output {
                        if write_half.send(output).await.is_err(){
                            break;
                        }
                    }
                }
                _ = disconnect_signal.recv() => {
                    break;
                }
            }
        }
    }
}
