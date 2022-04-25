use crate::client::event::{
    DeleteFriendEvent, FriendMessageRecallEvent, FriendPokeEvent, FriendRequestEvent,
    GroupLeaveEvent, GroupMessageEvent, GroupMessageRecallEvent, GroupMuteEvent,
    GroupNameUpdateEvent, GroupRequestEvent, MemberPermissionChangeEvent, NewFriendEvent,
    NewMemberEvent, PrivateMessageEvent, SelfInvitedEvent, TempMessageEvent,
};
use core::future::Future;
#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub enum QEvent<
    CP: crate::ChannelProvider,
    OSCP: crate::OneShotChannelProvider,
    RP: crate::RwLockProvider,
    MP: crate::MutexProvider,
    TP: crate::TaskProvider,
    TCP: crate::TcpStreamProvider,
> {
    TcpConnect,
    TcpDisconnect,
    /// 登录成功事件
    Login(i64),
    /// 群消息
    GroupMessage(GroupMessageEvent<CP, OSCP, RP, MP, TP, TCP>),
    /// 群自身消息
    SelfGroupMessage(GroupMessageEvent<CP, OSCP, RP, MP, TP, TCP>),
    /// 私聊消息
    PrivateMessage(PrivateMessageEvent<CP, OSCP, RP, MP, TP, TCP>),
    /// 私聊消息
    TempMessage(TempMessageEvent<CP, OSCP, RP, MP, TP, TCP>),
    /// 加群申请
    GroupRequest(GroupRequestEvent<CP, OSCP, RP, MP, TP, TCP>),
    /// 加群申请
    SelfInvited(SelfInvitedEvent<CP, OSCP, RP, MP, TP, TCP>),
    /// 加好友申请
    FriendRequest(FriendRequestEvent<CP, OSCP, RP, MP, TP, TCP>),
    /// 新成员入群
    NewMember(NewMemberEvent<CP, OSCP, RP, MP, TP, TCP>),
    /// 成员被禁言
    GroupMute(GroupMuteEvent<CP, OSCP, RP, MP, TP, TCP>),
    /// 好友消息撤回
    FriendMessageRecall(FriendMessageRecallEvent<CP, OSCP, RP, MP, TP, TCP>),
    /// 群消息撤回
    GroupMessageRecall(GroupMessageRecallEvent<CP, OSCP, RP, MP, TP, TCP>),
    /// 新好友
    NewFriend(NewFriendEvent<CP, OSCP, RP, MP, TP, TCP>),
    /// 退群/被踢
    GroupLeave(GroupLeaveEvent<CP, OSCP, RP, MP, TP, TCP>),
    /// 好友戳一戳
    FriendPoke(FriendPokeEvent<CP, OSCP, RP, MP, TP, TCP>),
    /// 群名称修改
    GroupNameUpdate(GroupNameUpdateEvent<CP, OSCP, RP, MP, TP, TCP>),
    /// 好友删除
    DeleteFriend(DeleteFriendEvent<CP, OSCP, RP, MP, TP, TCP>),
    /// 群成员权限变更
    MemberPermissionChange(MemberPermissionChangeEvent<CP, OSCP, RP, MP, TP, TCP>),
    // FriendList(decoder::friendlist::FriendListResponse),
    // GroupMemberInfo(structs::GroupMemberInfo),

    // 群消息发送成功事件 内部处理
    // GroupMessageReceipt(GroupMessageReceiptEvent)
}

pub trait Handler<
    CP: crate::ChannelProvider,
    OSCP: crate::OneShotChannelProvider,
    RP: crate::RwLockProvider,
    MP: crate::MutexProvider,
    TP: crate::TaskProvider,
    TCP: crate::TcpStreamProvider,
>: Sync
{
    type Future: Future<Output = ()> + Send;
    fn handle(&self, _event: QEvent<CP, OSCP, RP, MP, TP, TCP>) -> Self::Future;
}

/// 一个默认 Handler，只是把信息打印出来
pub struct DefaultHandler;

impl<CP, OSCP, RP, MP, TP, TCP> Handler<CP, OSCP, RP, MP, TP, TCP> for DefaultHandler
where
    CP: crate::ChannelProvider,
    OSCP: crate::OneShotChannelProvider,
    RP: crate::RwLockProvider,
    TP: crate::TaskProvider,
    TCP: crate::TcpStreamProvider,
    MP: crate::MutexProvider,
{
    type Future = core::pin::Pin<alloc::boxed::Box<dyn Future<Output = ()> + Send>>;
    fn handle(&self, e: QEvent<CP, OSCP, RP, MP, TP, TCP>) -> Self::Future {
        match e {
            QEvent::GroupMessage(m) => {
                tracing::info!(
                    target = "rs_qq",
                    "MESSAGE (GROUP={}): {}",
                    m.message.group_code,
                    m.message.elements
                )
            }
            QEvent::PrivateMessage(m) => {
                tracing::info!(
                    target = "rs_qq",
                    "MESSAGE (FRIEND={}): {}",
                    m.message.from_uin,
                    m.message.elements
                )
            }
            QEvent::TempMessage(m) => {
                tracing::info!(
                    target = "rs_qq",
                    "MESSAGE (TEMP={}): {}",
                    m.message.from_uin,
                    m.message.elements
                )
            }
            QEvent::GroupRequest(m) => {
                tracing::info!(
                    target = "rs_qq",
                    "REQUEST (GROUP={}, UIN={}): {}",
                    m.request.group_code,
                    m.request.req_uin,
                    m.request.message
                )
            }
            QEvent::FriendRequest(m) => {
                tracing::info!(
                    target = "rs_qq",
                    "REQUEST (UIN={}): {}",
                    m.request.req_uin,
                    m.request.message
                )
            }
            _ => tracing::info!(target = "rs_qq", "unknown"),
        }

        alloc::boxed::Box::pin(futures::future::ready(()))
    }
}
