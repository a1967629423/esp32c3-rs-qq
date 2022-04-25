use crate::{provider::*, SimpleCache};
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use bytes::BufMut;
use bytes::{Buf, Bytes, BytesMut};
use core::marker::PhantomData;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;
use nrq_engine::binary::BinaryReader;
use nrq_engine::binary::BinaryWriter;
use nrq_engine::protocol::device::Device;
use nrq_engine::protocol::version::Version;
use nrq_engine::structs::{AccountInfo, AddressInfo};
use nrq_engine::{protocol::packet::Packet, RQResult};
use nrq_engine::{Engine, RQError};

impl<CP, OSCP, RP, MP, TP, TCP> crate::Client<CP, OSCP, RP, MP, TP, TCP>
where
    CP: ChannelProvider + 'static,
    OSCP: OneShotChannelProvider + 'static,
    RP: RwLockProvider + 'static,
    TP: TaskProvider + 'static,
    TCP: TcpStreamProvider + 'static,
    MP: MutexProvider + 'static,
{
    pub fn new<H>(device: Device, version: &'static Version, handler: H) -> Self
    where
        H: crate::handler::Handler<
                CP,
                OSCP,
                RP,
                MP,
                TP,
                TCP,
                Future = core::pin::Pin<Box<dyn futures::Future<Output = ()> + Send>>,
            >
            + 'static
            + Sync
            + Send,
    {
        // 不能太大，会爆内存
        let (out_pkt_sender, _) = CP::channel(16);
        let (disconnect_signal, _) = CP::channel(8);

        Self {
            __cp_data: PhantomData,
            __mp_data: PhantomData,
            __oscp_data: PhantomData,
            __rp_data: PhantomData,
            __tcp_data: PhantomData,
            __tp_data: PhantomData,

            handler: Box::new(handler),
            engine: RP::RwLock::new(Engine::new(device, version)),
            running: AtomicBool::new(false),
            heartbeat_enabled: AtomicBool::new(false),
            online: AtomicBool::new(false),
            out_pkt_sender,
            disconnect_signal,
            packet_promises: RP::RwLock::new(BTreeMap::new()),
            packet_waiters: RP::RwLock::new(BTreeMap::new()),
            receipt_waiters: MP::Mutex::new(BTreeMap::new()),
            account_info: RP::RwLock::new(AccountInfo::default()),
            address: RP::RwLock::new(AddressInfo::default()),
            friends: RP::RwLock::new(BTreeMap::new()),
            groups: RP::RwLock::new(BTreeMap::new()),
            online_clients: RP::RwLock::new(Vec::new()),
            last_message_time: Default::default(),
            start_time: nrq_engine::get_timer_provider().now_timestamp() as i32,
            group_message_builder: RP::RwLock::new(SimpleCache::new()),
            c2c_cache: RP::RwLock::new(SimpleCache::new()),
            push_req_cache: RP::RwLock::new(SimpleCache::new()),
            push_trans_cache: RP::RwLock::new(SimpleCache::new()),
            group_sys_message_cache: RP::RwLock::new(Default::default()),
            highway_session: RP::RwLock::new(Default::default()),
            highway_addrs: RP::RwLock::new(Default::default()),
        }
    }
}
impl<CP, OSCP, RP, MP, TP, TCP> crate::Client<CP, OSCP, RP, MP, TP, TCP>
where
    CP: ChannelProvider,
    OSCP: OneShotChannelProvider,
    RP: RwLockProvider,
    TP: TaskProvider,
    TCP: TcpStreamProvider,
    MP: MutexProvider,
{
    pub async fn uin(&self) -> i64 {
        self.engine.read().await.uin.load(Ordering::Relaxed)
    }
    pub async fn send(&self, pkt: Packet) -> RQResult<usize> {
        tracing::trace!(target:"rs_qq","sending pkt {}-{}",pkt.command_name,pkt.seq_id);
        let data: bytes::Bytes = self.engine.read().await.transport.encode_packet(pkt);
        self.out_pkt_sender
            .send(data)
            .await
            .map_err(|_| RQError::Other("failed to send out_pkt".into()))
    }
    pub async fn send_and_wait(&self, pkt: Packet) -> RQResult<Packet> {
        //tracing::trace!(target: "rs_qq", "send_and_waitting pkt {}-{},", pkt.command_name, pkt.seq_id);
        log::debug!("send_and_waitting pkt {}-{},", pkt.command_name, pkt.seq_id);
        let seq = pkt.seq_id;
        let expect = pkt.command_name.clone();
        let data = self.engine.read().await.transport.encode_packet(pkt);
        let (sender, receiver) = OSCP::channel::<Packet>();
        {
            let mut packet_promises = self.packet_promises.write().await;
            packet_promises.insert(seq, sender);
        }
        if self.out_pkt_sender.send(data).await.is_err() {
            let mut packet_promises = self.packet_promises.write().await;
            packet_promises.remove(&seq);
            return Err(RQError::Network);
        }
        match TP::timeout(core::time::Duration::from_secs(15), receiver).await {
            Ok(p) => {
                p.unwrap().check_command_name(&expect)
            },
            Err(_) => {
                // tracing::trace!(target: "rs_qq", "waitting pkt {}-{} timeout", expect, seq);
                log::debug!("waitting pkt {}-{} timeout", expect, seq);
                self.packet_promises.write().await.remove(&seq);
                Err(RQError::Timeout)
            }
        }
    }
    pub async fn wait_packet(&self, pkt_name: &str, delay: u64) -> RQResult<Packet> {
        tracing::trace!(target: "rs_qq", "waitting pkt {}", pkt_name);
        let (tx, rx) = OSCP::channel();
        {
            self.packet_waiters
                .write()
                .await
                .insert(pkt_name.to_owned(), tx);
        }
        match TP::timeout(core::time::Duration::from_secs(delay), rx).await {
            Ok(i) => Ok(i.unwrap()),
            Err(_) => {
                tracing::trace!(target: "rs_qq", "waitting pkt {} timeout", pkt_name);
                self.packet_waiters.write().await.remove(pkt_name);
                Err(RQError::Timeout)
            }
        }
    }

    pub async fn gen_token(&self) -> Bytes {
        let mut token = BytesMut::with_capacity(1024); //todo
        let engine = &self.engine.read().await;
        token.put_i64(self.uin().await);
        token.write_bytes_short(&engine.transport.sig.d2);
        token.write_bytes_short(&engine.transport.sig.d2key);
        token.write_bytes_short(&engine.transport.sig.tgt);
        token.write_bytes_short(&engine.transport.sig.srm_token);
        token.write_bytes_short(&engine.transport.sig.t133);
        token.write_bytes_short(&engine.transport.sig.encrypted_a1);
        token.write_bytes_short(&engine.transport.oicq_codec.wt_session_ticket_key);
        token.write_bytes_short(&engine.transport.sig.out_packet_session_id);
        token.write_bytes_short(&engine.transport.sig.tgtgt_key);
        token.freeze()
    }
    pub async fn load_token(&self, token: &mut impl Buf) {
        let mut engine = self.engine.write().await;
        engine.uin.store(token.get_i64(), Ordering::SeqCst);
        engine.transport.sig.d2 = token.read_bytes_short();
        engine.transport.sig.d2key = token.read_bytes_short();
        engine.transport.sig.tgt = token.read_bytes_short();
        engine.transport.sig.srm_token = token.read_bytes_short();
        engine.transport.sig.t133 = token.read_bytes_short();
        engine.transport.sig.encrypted_a1 = token.read_bytes_short();
        engine.transport.oicq_codec.wt_session_ticket_key = token.read_bytes_short();
        engine.transport.sig.out_packet_session_id = token.read_bytes_short();
        engine.transport.sig.tgtgt_key = token.read_bytes_short();
    }
}

impl<CP, OSCP, RP, MP, TP, TCP> Drop for crate::Client<CP, OSCP, RP, MP, TP, TCP>
where
    CP: ChannelProvider,
    OSCP: OneShotChannelProvider,
    RP: RwLockProvider,
    TP: TaskProvider,
    TCP: TcpStreamProvider,
    MP: MutexProvider,
{
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);

        self.disconnect_signal.send(());
    }
}
impl<CP, OSCP, RP, MP, TP, TCP> crate::Client<CP, OSCP, RP, MP, TP, TCP>
where
    CP: ChannelProvider + 'static,
    OSCP: OneShotChannelProvider + 'static,
    RP: RwLockProvider + 'static,
    TP: TaskProvider + 'static,
    TCP: TcpStreamProvider + 'static,
    MP: MutexProvider + 'static,
{
    pub async fn do_heartbeat(&self) {
        self.heartbeat_enabled.store(true, Ordering::SeqCst);
        let mut times = 0;
        while self.online.load(Ordering::SeqCst) {
            TP::sleep(core::time::Duration::from_secs(30)).await;
            match self.heartbeat().await {
                Err(_) => {
                    continue;
                }
                Ok(_) => {
                    times += 1;
                    if times >= 7 {
                        if self.register_client().await.is_err() {
                            break;
                        }
                        times = 0;
                    }
                }
            }
        }
        self.heartbeat_enabled.store(false, Ordering::SeqCst);
    }
}
