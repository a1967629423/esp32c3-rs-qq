#[cfg(not(feature = "std"))]
use crate::simulate_std::prelude::*;
use bytes::Bytes;

use crate::structs::*;

pub mod builder;
pub mod decoder;

#[derive(Debug, Default)]
pub struct FriendListResponse {
    pub list: Vec<FriendInfo>,
    pub total_count: i16,
}

#[derive(Debug)]
pub struct GroupListResponse {
    pub groups: Vec<GroupInfo>,
    pub vec_cookie: Bytes,
}

#[derive(Debug)]
pub struct GroupMemberListResponse {
    pub next_uin: i64,
    pub list: Vec<GroupMemberInfo>,
}
