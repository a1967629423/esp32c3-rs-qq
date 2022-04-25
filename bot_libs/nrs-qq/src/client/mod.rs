use crate::provider::{
    ChannelProvider, MutexProvider, OneShotChannelProvider, RwLockProvider, TaskProvider,
    TcpStreamProvider,
};
use crate::structs::{AccountInfo, AddressInfo, FriendInfo, Group, OtherClientInfo};
use crate::SimpleCache;
use alloc::boxed::Box;
use alloc::collections::BTreeMap as HashMap;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::marker::PhantomData;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::AtomicI64;
use nrq_engine::command::online_push::GroupMessagePart;
use nrq_engine::command::profile_service::GroupSystemMessages;
use nrq_engine::{protocol::packet::Packet, Engine};

mod api;
mod client;
pub mod event;
pub(crate) mod ext;
pub mod handler;
mod net;
mod processor;
pub use ext::read_buf::ReadBuf;

/// ## TODO: 使用泛型确定handler的future
///
/// 目前为了省事直接Box::pin了
pub struct Client<
    CP: ChannelProvider,
    OSCP: OneShotChannelProvider,
    RP: RwLockProvider,
    MP: MutexProvider,
    TP: TaskProvider,
    TCP: TcpStreamProvider,
> {
    // 一堆幽灵
    __cp_data: PhantomData<CP>,
    __oscp_data: PhantomData<OSCP>,
    __rp_data: PhantomData<RP>,
    __mp_data: PhantomData<MP>,
    __tp_data: PhantomData<TP>,
    __tcp_data: PhantomData<TCP>,

    handler: Box<
        dyn crate::Handler<
                CP,
                OSCP,
                RP,
                MP,
                TP,
                TCP,
                Future = core::pin::Pin<Box<dyn futures::Future<Output = ()> + Send>>,
            > + Sync
            + Send
            + 'static,
    >,

    engine: RP::RwLock<Engine>,

    // 是否正在运行（是否需要快速重连）
    pub running: AtomicBool,
    // 是否在线（是否可以快速重连）
    pub online: AtomicBool,
    // 停止网络
    disconnect_signal: CP::Sender<()>,
    pub heartbeat_enabled: AtomicBool,

    out_pkt_sender: CP::Sender<bytes::Bytes>,
    packet_promises: RP::RwLock<HashMap<i32, OSCP::Sender<Packet>>>,
    packet_waiters: RP::RwLock<HashMap<String, OSCP::Sender<Packet>>>,
    receipt_waiters: MP::Mutex<HashMap<i32, OSCP::Sender<i32>>>,

    // account info
    pub account_info: RP::RwLock<AccountInfo>,

    // address
    pub address: RP::RwLock<AddressInfo>,
    pub friends: RP::RwLock<HashMap<i64, Arc<FriendInfo>>>,
    pub groups: RP::RwLock<HashMap<i64, Arc<Group<RP>>>>,
    pub online_clients: RP::RwLock<Vec<OtherClientInfo>>,

    // statics
    pub last_message_time: AtomicI64,
    pub start_time: i32,

    // TODO: cached

    // /// 群消息 builder 寄存 <div_seq, parts> : parts is sorted by pkg_index
    // group_message_builder: RwLock<cached::TimedCache<i32, Vec<GroupMessagePart>>>,
    // /// 每个 28 Byte
    // c2c_cache: RwLock<cached::TimedCache<(i64, i64, i32, i64), ()>>,
    // push_req_cache: RwLock<cached::TimedCache<(i16, i64), ()>>,
    // push_trans_cache: RwLock<cached::TimedCache<(i32, i64), ()>>,
    // group_sys_message_cache: RwLock<GroupSystemMessages>,
    /// 群消息 builder 寄存 <div_seq, parts> : parts is sorted by pkg_index
    // group_message_builder:RP::RwLock<caches::RawLRU<i32,Vec<GroupMessagePart>>>,
    // c2c_cache:RP::RwLock<caches::RawLRU<(i64,i64,i32,i64),()>>,
    // push_trans_cache:RP::RwLock<caches::RawLRU<(i32,i64),()>>,
    // group_sys_message_cache:RP::RwLock<GroupSystemMessages>,

    /// 群消息 builder 寄存 <div_seq, parts> : parts is sorted by pkg_index
    group_message_builder: RP::RwLock<SimpleCache<i32, Vec<GroupMessagePart>>>,
    c2c_cache: RP::RwLock<SimpleCache<(i64, i64, i32, i64), ()>>,
    push_req_cache: RP::RwLock<SimpleCache<(i16, i64), ()>>,
    push_trans_cache: RP::RwLock<SimpleCache<(i32, i64), ()>>,
    group_sys_message_cache: RP::RwLock<GroupSystemMessages>,

    highway_session: RP::RwLock<nrq_engine::highway::Session>,
    highway_addrs: RP::RwLock<Vec<no_std_net::SocketAddr>>,
}
unsafe impl<CP, OSCP, RP, MP, TP, TCP> Send for crate::Client<CP, OSCP, RP, MP, TP, TCP>
where
    CP: ChannelProvider,
    OSCP: OneShotChannelProvider,
    RP: RwLockProvider,
    TP: TaskProvider,
    TCP: TcpStreamProvider,
    MP: MutexProvider,
{
}
// unsafe impl<CP, OSCP, RP, MP, TP, TCP> Sync for crate::Client<CP, OSCP, RP, MP, TP, TCP>
// where
//     CP: ChannelProvider,
//     OSCP: OneShotChannelProvider,
//     RP: RwLockProvider,
//     TP: TaskProvider,
//     TCP: TcpStreamProvider,
//     MP: MutexProvider,
//
// {

// }
// impl<CP, OSCP, RP, MP, TP, TCP> crate::Client<CP, OSCP, RP, MP, TP, TCP>
// where
//     CP: ChannelProvider,
//     OSCP: OneShotChannelProvider,
//     RP: RwLockProvider,
//     TP: TaskProvider,
//     TCP: TcpStreamProvider,
//     MP: MutexProvider,
//
// {

// }
