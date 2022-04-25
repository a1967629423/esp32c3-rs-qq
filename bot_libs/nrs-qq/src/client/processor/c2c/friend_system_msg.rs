use crate::client::event::FriendRequestEvent;
use crate::handler::QEvent;
use crate::provider::*;
use crate::Client;
use alloc::sync::Arc;
use nrq_engine::command::profile_service::FriendSystemMessages;
impl<CP, OSCP, RP, MP, TP, TCP> Client<CP, OSCP, RP, MP, TP, TCP>
where
    CP: ChannelProvider,
    OSCP: OneShotChannelProvider,
    RP: RwLockProvider,
    TP: TaskProvider,
    TCP: TcpStreamProvider,
    MP: MutexProvider,
{
    pub(crate) async fn process_friend_system_messages(
        self: &Arc<Self>,
        msgs: FriendSystemMessages,
    ) {
        for request in msgs.requests {
            self.handler
                .handle(QEvent::FriendRequest(FriendRequestEvent {
                    client: self.clone(),
                    request,
                }))
                .await;
        }
    }
}
