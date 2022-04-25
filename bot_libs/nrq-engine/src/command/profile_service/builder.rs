#[cfg(not(feature = "std"))]
use crate::simulate_std::prelude::*;
use jcers::JcePut;

use crate::command::common::PbToBytes;
use crate::pb;
use crate::protocol::packet::Packet;

impl super::super::super::Engine {
    // ProfileService.Pb.ReqSystemMsgNew.Group
    pub fn build_system_msg_new_group_packet(&self, suspicious: bool) -> Packet {
        let req = pb::structmsg::ReqSystemMsgNew {
            msg_num: 100,
            version: 1000,
            checktype: 3,
            flag: Some(pb::structmsg::FlagInfo {
                grp_msg_kick_admin: 1,
                grp_msg_hidden_grp: 1,
                grp_msg_wording_down: 1,
                grp_msg_get_official_account: 1,
                grp_msg_get_pay_in_group: 1,
                frd_msg_discuss2_many_chat: 1,
                grp_msg_not_allow_join_grp_invite_not_frd: 1,
                frd_msg_need_waiting_msg: 1,
                frd_msg_uint32_need_all_unread_msg: 1,
                grp_msg_need_auto_admin_wording: 1,
                grp_msg_get_transfer_group_msg_flag: 1,
                grp_msg_get_quit_pay_group_msg_flag: 1,
                grp_msg_support_invite_auto_join: 1,
                grp_msg_mask_invite_auto_join: 1,
                grp_msg_get_disbanded_by_admin: 1,
                grp_msg_get_c2c_invite_join_group: 1,
                ..Default::default()
            }),
            friend_msg_type_flag: 1,
            req_msg_type: if suspicious { 2 } else { 1 },
            ..Default::default()
        };
        let payload = req.to_bytes();
        self.uni_packet("ProfileService.Pb.ReqSystemMsgNew.Group", payload)
    }

    // ProfileService.Pb.ReqSystemMsgNew.Friend
    pub fn build_system_msg_new_friend_packet(&self) -> Packet {
        let req = pb::structmsg::ReqSystemMsgNew {
            msg_num: 20,
            version: 1000,
            checktype: 2,
            flag: Some(pb::structmsg::FlagInfo {
                frd_msg_discuss2_many_chat: 1,
                frd_msg_get_busi_card: 1,
                frd_msg_need_waiting_msg: 1,
                frd_msg_uint32_need_all_unread_msg: 1,
                grp_msg_mask_invite_auto_join: 1,
                ..Default::default()
            }),
            friend_msg_type_flag: 1,
            ..Default::default()
        };
        let payload = req.to_bytes();
        self.uni_packet("ProfileService.Pb.ReqSystemMsgNew.Friend", payload)
    }

    // ProfileService.Pb.ReqSystemMsgAction.Group
    pub fn build_system_msg_group_action_packet(
        &self,
        msg_seq: i64,
        req_uin: i64,
        group_code: i64,
        msg_type: i32,
        is_invite: bool,
        accept: bool,
        block: bool,
        reason: String,
    ) -> Packet {
        let req = pb::structmsg::ReqSystemMsgAction {
            msg_type,
            msg_seq,
            req_uin,
            sub_type: 1,
            src_id: 3,
            sub_src_id: if is_invite { 10016 } else { 31 },
            group_msg_type: if is_invite { 2 } else { 1 },
            action_info: Some(pb::structmsg::SystemMsgActionInfo {
                r#type: if accept { 11 } else { 12 },
                group_code,
                blacklist: block,
                msg: reason,
                sig: vec![],
                ..Default::default()
            }),
            language: 1000,
        };
        let payload = req.to_bytes();
        self.uni_packet("ProfileService.Pb.ReqSystemMsgAction.Group", payload)
    }

    // ProfileService.Pb.ReqSystemMsgAction.Friend
    pub fn build_system_msg_friend_action_packet(
        &self,
        req_id: i64,
        req_uin: i64,
        accept: bool,
    ) -> Packet {
        let req = pb::structmsg::ReqSystemMsgAction {
            msg_type: 1,
            msg_seq: req_id,
            req_uin,
            sub_type: 1,
            src_id: 6,
            sub_src_id: 7,
            action_info: Some(pb::structmsg::SystemMsgActionInfo {
                r#type: if accept { 2 } else { 3 },
                blacklist: false,
                add_frd_sn_info: Some(pb::structmsg::AddFrdSnInfo::default()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let payload = req.to_bytes();
        self.uni_packet("ProfileService.Pb.ReqSystemMsgAction.Friend", payload)
    }

    // ProfileService.GroupMngReq
    pub fn build_quit_group_packet(&self, group_code: i64) -> Packet {
        let mut jce_mut = jcers::JceMut::new();
        jce_mut.put_i32(2, 0);
        jce_mut.put_i64(self.uin(), 1);
        jce_mut.put_bytes(
            bytes::Bytes::from({
                let mut v = Vec::with_capacity(8);
                v.extend((self.uin() as u32).to_be_bytes());
                v.extend(group_code.to_be_bytes());
                v
            }),
            2,
        );
        let buf = crate::jce::RequestDataVersion3 {
            map: [(
                "GroupMngReq".to_owned(),
                crate::command::common::pack_uni_request_data(&jce_mut.freeze()),
            )]
            .into(),
        };
        let pkt = crate::jce::RequestPacket {
            i_version: 3,
            i_request_id: self.next_packet_seq(),
            s_servant_name: "KQQ.ProfileService.ProfileServantObj".to_owned(),
            s_func_name: "GroupMngReq".to_owned(),
            s_buffer: buf.freeze(),
            ..Default::default()
        };
        self.uni_packet("ProfileService.GroupMngReq", pkt.freeze())
    }
}
