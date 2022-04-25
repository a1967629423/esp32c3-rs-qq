pub mod channel;
pub mod engine;
pub mod mutex;
pub mod oneshot;
pub mod raw;
pub mod rwlock;
pub mod task;
pub mod tcp;
use nrs_qq::provider::ClientProvider;
pub struct MyClientProvider {
    
}
impl ClientProvider for MyClientProvider  {
    type CP = channel::MyChannelProvider;
    type OSCP = oneshot::MyOneShotProvider;
    type RLP = rwlock::MyRwLockProvider;
    type MP = mutex::MyMutexProvider;
    type TP = task::MyTaskProvider;
    type TCP = tcp::MyTcpStreamSyncProvider;
    type Handler = crate::handler::MyHandler;
}