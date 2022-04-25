use crate::pb;
#[cfg(not(feature = "std"))]
use crate::simulate_std::prelude::*;

pub mod builder;
pub mod decoder;

pub struct MessageSyncResponse {
    pub msg_rsp_type: i32,
    pub sync_flag: i32,
    pub sync_cookie: Option<Vec<u8>>,
    pub pub_account_cookie: Option<Vec<u8>>,
    pub msgs: Vec<pb::msg::Message>,
}
