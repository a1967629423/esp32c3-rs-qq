use crate::provider::*;
use alloc::sync::Arc;
use alloc::vec;

use nrq_engine::msg::MessageChain;
use nrq_engine::structs::PrivateMessage;
use nrq_engine::{pb, RQResult};

use crate::client::event::PrivateMessageEvent;
use crate::handler::QEvent;
use crate::Client;
impl<CP, OSCP, RP, MP, TP, TCP> Client<CP, OSCP, RP, MP, TP, TCP>
where
    CP: ChannelProvider,
    OSCP: OneShotChannelProvider,
    RP: RwLockProvider,
    TP: TaskProvider,
    TCP: TcpStreamProvider,
    MP: MutexProvider,
{
    pub(crate) async fn process_private_message(
        self: &Arc<Self>,
        msg: pb::msg::Message,
    ) -> RQResult<()> {
        let private_message = parse_private_message(msg)?;
        if private_message.from_uin == self.uin().await {
            // TODO dispatch self private message event
            // TODO swap friend seq
            return Ok(());
        }
        self.handler
            .handle(QEvent::PrivateMessage(PrivateMessageEvent {
                client: self.clone(),
                message: private_message,
            }))
            .await;
        Ok(())
    }
}

pub fn parse_private_message(msg: pb::msg::Message) -> RQResult<PrivateMessage> {
    let head = msg.head.unwrap();
    Ok(PrivateMessage {
        seqs: vec![head.msg_seq()],
        target: head.to_uin.unwrap(),
        time: head.msg_time.unwrap(),
        from_uin: head.from_uin.unwrap_or_default(),
        from_nick: head.from_nick.unwrap_or_default(),
        rands: vec![
            if let Some(attr) = &msg.body.as_ref().unwrap().rich_text.as_ref().unwrap().attr {
                attr.random()
            } else {
                0
            },
        ],
        elements: MessageChain::from(msg.body.unwrap().rich_text.unwrap().elems), // todo ptt
    })
}
