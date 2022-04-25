#[cfg(not(feature = "std"))]
use crate::simulate_std::prelude::*;
use bytes::Bytes;

use crate::command::common::PbToBytes;
use crate::structs::{GroupMemberInfo, GroupMemberPermission};
use crate::{pb, RQError, RQResult};

impl super::super::super::Engine {
    // group_member_card.get_group_member_card_info
    pub fn decode_group_member_info_response(&self, payload: Bytes) -> RQResult<GroupMemberInfo> {
        let resp = pb::GroupMemberRspBody::from_bytes(&payload)
            .map_err(|_| RQError::Decode("GroupMemberRspBody".to_string()))?;
        let group_code = resp.group_code;
        let mem_info = resp
            .mem_info
            .ok_or_else(|| RQError::Decode("mem_info is none".to_string()))?;
        Ok(GroupMemberInfo {
            group_code,
            uin: mem_info.uin,
            gender: mem_info.sex as u8,
            nickname: String::from_utf8_lossy(&mem_info.nick).into(),
            card_name: String::from_utf8_lossy(&mem_info.card).into(),
            level: mem_info.level as u16,
            join_time: mem_info.join,
            last_speak_time: mem_info.last_speak,
            special_title: String::from_utf8_lossy(&mem_info.special_title).into(),
            special_title_expire_time: mem_info.special_title_expire_time as i64,
            permission: match mem_info.role {
                3 => GroupMemberPermission::Owner,
                2 => GroupMemberPermission::Administrator,
                _ => GroupMemberPermission::Member,
            }, // TODO group owner
            ..Default::default()
        })
    }
}
