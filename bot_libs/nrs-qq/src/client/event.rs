use nrq_engine::command::profile_service::{JoinGroupRequest, NewFriendRequest, SelfInvited};
use nrq_engine::structs::{
    DeleteFriend, FriendInfo, FriendMessageRecall, FriendPoke, GroupLeave, GroupMessageRecall,
    GroupMute, GroupNameUpdate, MemberPermissionChange, NewMember, TempMessage,
};

use crate::provider::*;
use crate::structs::{GroupMessage, PrivateMessage};
macro_rules! define_message {
    (
        $(#[$top_meta:meta])*
        $s_name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident:$field_type:ty
            ),*$(,)*
        }
    ) => {
        $(#[$top_meta])*
        #[derive(Clone, derivative::Derivative)]
        #[derivative(Debug)]
        pub struct $s_name<CP:crate::ChannelProvider,OSCP:crate::OneShotChannelProvider,RP:crate::RwLockProvider,MP:crate::MutexProvider,TP:crate::TaskProvider,TCP:crate::TcpStreamProvider>
        {
            #[derivative(Debug = "ignore")]
            pub client:alloc::sync::Arc<$crate::Client<CP,OSCP,RP,MP,TP,TCP>>,

            $(
                $(#[$field_meta])*
                pub $field:$field_type,
            )*
        }
    };
}

// #[derive(Clone, derivative::Derivative)]
// #[derivative(Debug)]
// pub struct GroupMessageEvent {
//     #[derivative(Debug = "ignore")]
//     pub client: Arc<Client>,
//     pub message: GroupMessage,
// }

// impl GroupMessageEvent {
//     pub async fn group(&self) -> Option<Arc<Group>> {
//         self.client.find_group(self.message.group_code, true).await
//     }

//     pub async fn member(&self) -> Option<GroupMemberInfo> {
//         let group = self.group().await?;
//         let members = group.members.read().await;
//         members
//             .iter()
//             .filter(|m| m.uin == self.message.from_uin)
//             .last()
//             .cloned()
//     }

//     pub async fn recall(&self) -> RQResult<()> {
//         // TODO check permission
//         self.client
//             .recall_group_message(
//                 self.message.group_code,
//                 self.message.seqs.clone(),
//                 self.message.rands.clone(),
//             )
//             .await
//     }
// }
define_message!(GroupMessageEvent {
    message: GroupMessage,
});
impl<CP, OSCP, RP, MP, TP, TCP> GroupMessageEvent<CP, OSCP, RP, MP, TP, TCP>
where
    CP: ChannelProvider,
    OSCP: OneShotChannelProvider,
    RP: RwLockProvider,
    TP: TaskProvider,
    TCP: TcpStreamProvider,
    MP: MutexProvider,
{
}
// #[derive(Clone, derivative::Derivative)]
// #[derivative(Debug)]
// pub struct PrivateMessageEvent {
//     #[derivative(Debug = "ignore")]
//     pub client: Arc<Client>,
//     pub message: PrivateMessage,
// }
define_message!(PrivateMessageEvent {
    message: PrivateMessage,
});
// #[derive(Clone, derivative::Derivative)]
// #[derivative(Debug)]
// pub struct TempMessageEvent {
//     #[derivative(Debug = "ignore")]
//     pub client: Arc<Client>,
//     pub message: TempMessage,
// }
define_message!(TempMessageEvent {
    message: TempMessage
});
// #[derive(Clone, derivative::Derivative)]
// #[derivative(Debug)]
// pub struct GroupRequestEvent {
//     #[derivative(Debug = "ignore")]
//     pub client: Arc<Client>,
//     pub request: JoinGroupRequest,
// }

// impl GroupRequestEvent {
//     pub async fn accept(&self) -> RQResult<()> {
//         self.client
//             .solve_group_system_message(
//                 self.request.msg_seq,
//                 self.request.req_uin,
//                 self.request.group_code,
//                 self.request.suspicious,
//                 self.request.invitor_uin.is_some(),
//                 true,
//                 false,
//                 "".into(),
//             )
//             .await
//     }

//     pub async fn reject(&self, reason: String, block: bool) -> RQResult<()> {
//         self.client
//             .solve_group_system_message(
//                 self.request.msg_seq,
//                 self.request.req_uin,
//                 self.request.group_code,
//                 self.request.suspicious,
//                 self.request.invitor_uin.is_some(),
//                 false,
//                 block,
//                 reason,
//             )
//             .await
//     }
// }
define_message!(GroupRequestEvent {
    request: JoinGroupRequest,
});
impl<CP, OSCP, RP, MP, TP, TCP> GroupRequestEvent<CP, OSCP, RP, MP, TP, TCP>
where
    CP: ChannelProvider,
    OSCP: OneShotChannelProvider,
    RP: RwLockProvider,
    TP: TaskProvider,
    TCP: TcpStreamProvider,
    MP: MutexProvider,
{
}
// #[derive(Clone, derivative::Derivative)]
// #[derivative(Debug)]
// pub struct FriendRequestEvent {
//     #[derivative(Debug = "ignore")]
//     pub client: Arc<Client>,
//     pub request: NewFriendRequest,
// }

// impl FriendRequestEvent {
//     pub async fn accept(&self) -> RQResult<()> {
//         self.client
//             .solve_friend_system_message(self.request.msg_seq, self.request.req_uin, true)
//             .await
//     }

//     pub async fn reject(&self) -> RQResult<()> {
//         self.client
//             .solve_friend_system_message(self.request.msg_seq, self.request.req_uin, false)
//             .await
//     }
// }
define_message!(FriendRequestEvent {
    request: NewFriendRequest
});
impl<CP, OSCP, RP, MP, TP, TCP> FriendRequestEvent<CP, OSCP, RP, MP, TP, TCP>
where
    CP: ChannelProvider,
    OSCP: OneShotChannelProvider,
    RP: RwLockProvider,
    TP: TaskProvider,
    TCP: TcpStreamProvider,
    MP: MutexProvider,
{
}
// #[derive(Clone, derivative::Derivative)]
// #[derivative(Debug)]
// pub struct NewMemberEvent {
//     #[derivative(Debug = "ignore")]
//     pub client: Arc<Client>,
//     pub new_member: NewMember,
// }

// impl NewMemberEvent {
//     pub async fn group(&self) -> Option<Arc<Group>> {
//         self.client
//             .find_group(self.new_member.group_code, true)
//             .await
//     }

//     pub async fn member(&self) -> Option<GroupMemberInfo> {
//         let group = self.group().await?;
//         let members = group.members.read().await;
//         members
//             .iter()
//             .filter(|m| m.uin == self.new_member.member_uin)
//             .last()
//             .cloned()
//     }
// }
define_message!(NewMemberEvent {
    new_member: NewMember,
});
impl<CP, OSCP, RP, MP, TP, TCP> NewMemberEvent<CP, OSCP, RP, MP, TP, TCP>
where
    CP: ChannelProvider,
    OSCP: OneShotChannelProvider,
    RP: RwLockProvider,
    TP: TaskProvider,
    TCP: TcpStreamProvider,
    MP: MutexProvider,
{
}
// #[derive(Clone, derivative::Derivative)]
// #[derivative(Debug)]
// pub struct GroupMuteEvent {
//     #[derivative(Debug = "ignore")]
//     pub client: Arc<Client>,
//     pub group_mute: GroupMute,
// }
define_message!(GroupMuteEvent {
    group_mute: GroupMute
});
// #[derive(Clone, derivative::Derivative)]
// #[derivative(Debug)]
// pub struct FriendMessageRecallEvent {
//     #[derivative(Debug = "ignore")]
//     pub client: Arc<Client>,
//     pub recall: FriendMessageRecall,
// }
define_message!(FriendMessageRecallEvent {
    recall: FriendMessageRecall,
});
// #[derive(Clone, derivative::Derivative)]
// #[derivative(Debug)]
// pub struct GroupMessageRecallEvent {
//     #[derivative(Debug = "ignore")]
//     pub client: Arc<Client>,
//     pub recall: GroupMessageRecall,
// }
define_message!(GroupMessageRecallEvent {
    recall: GroupMessageRecall,
});
// #[derive(Clone, derivative::Derivative)]
// #[derivative(Debug)]
// pub struct NewFriendEvent {
//     #[derivative(Debug = "ignore")]
//     pub client: Arc<Client>,
//     pub friend: FriendInfo,
// }
define_message!(NewFriendEvent { friend: FriendInfo });
// #[derive(Clone, derivative::Derivative)]
// #[derivative(Debug)]
// pub struct GroupLeaveEvent {
//     #[derivative(Debug = "ignore")]
//     pub client: Arc<Client>,
//     pub leave: GroupLeave,
// }
define_message!(GroupLeaveEvent { leave: GroupLeave });
// #[derive(Clone, derivative::Derivative)]
// #[derivative(Debug)]
// pub struct FriendPokeEvent {
//     #[derivative(Debug = "ignore")]
//     pub client: Arc<Client>,
//     pub poke: FriendPoke,
// }
define_message!(FriendPokeEvent { poke: FriendPoke });
// #[derive(Clone, derivative::Derivative)]
// #[derivative(Debug)]
// pub struct GroupNameUpdateEvent {
//     #[derivative(Debug = "ignore")]
//     pub client: Arc<Client>,
//     pub update: GroupNameUpdate,
// }
define_message!(GroupNameUpdateEvent {
    update: GroupNameUpdate
});
// #[derive(Clone, derivative::Derivative)]
// #[derivative(Debug)]
// pub struct DeleteFriendEvent {
//     #[derivative(Debug = "ignore")]
//     pub client: Arc<Client>,
//     pub delete: DeleteFriend,
// }
define_message!(DeleteFriendEvent {
    delete: DeleteFriend
});
// #[derive(Clone, derivative::Derivative)]
// #[derivative(Debug)]
// pub struct MemberPermissionChangeEvent {
//     #[derivative(Debug = "ignore")]
//     pub client: Arc<Client>,
//     pub change: MemberPermissionChange,
// }
define_message!(MemberPermissionChangeEvent {
    change: MemberPermissionChange
});
// #[derive(Clone, derivative::Derivative)]
// #[derivative(Debug)]
// pub struct SelfInvitedEvent {
//     #[derivative(Debug = "ignore")]
//     pub client: Arc<Client>,
//     pub request: SelfInvited,
// }
define_message!(SelfInvitedEvent {
    request: SelfInvited
});
