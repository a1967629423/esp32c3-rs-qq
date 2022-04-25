use alloc::sync::Arc;
use core::sync::atomic::Ordering;

use crate::provider::*;
use crate::Client;

pub async fn after_login<CP, OSCP, RP, MP, TP, TCP>(client: &Arc<Client<CP, OSCP, RP, MP, TP, TCP>>)
where
    CP: ChannelProvider + 'static,
    OSCP: OneShotChannelProvider + 'static,
    RP: RwLockProvider + 'static,
    TP: TaskProvider + 'static,
    TCP: TcpStreamProvider + 'static,
    MP: MutexProvider + 'static,
{
    if let Err(err) = client.register_client().await {
        tracing::error!("failed to register client: {}", err)
    }
    start_heartbeat(client.clone()).await;
    if let Err(err) = client.refresh_status().await {
        tracing::error!("failed to refresh status: {}", err)
    }
}

pub async fn start_heartbeat<CP, OSCP, RP, MP, TP, TCP>(
    client: Arc<Client<CP, OSCP, RP, MP, TP, TCP>>,
) where
    CP: ChannelProvider + 'static,
    OSCP: OneShotChannelProvider + 'static,
    RP: RwLockProvider + 'static,
    TP: TaskProvider + 'static,
    TCP: TcpStreamProvider + 'static,
    MP: MutexProvider + 'static,
{
    if !client.heartbeat_enabled.load(Ordering::Relaxed) {
        TP::spawn(async move {
            client.do_heartbeat().await;
        }).detach();
    }
}
