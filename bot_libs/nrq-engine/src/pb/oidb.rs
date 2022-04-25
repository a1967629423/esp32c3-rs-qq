#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Db77ReqBody {
    #[prost(uint64, tag="1")]
    pub app_id: u64,
    #[prost(uint32, tag="2")]
    pub app_type: u32,
    #[prost(uint32, tag="3")]
    pub msg_style: u32,
    #[prost(uint64, tag="4")]
    pub sender_uin: u64,
    #[prost(message, optional, tag="5")]
    pub client_info: ::core::option::Option<Db77ClientInfo>,
    #[prost(string, tag="6")]
    pub text_msg: ::prost::alloc::string::String,
    #[prost(message, optional, tag="7")]
    pub ext_info: ::core::option::Option<Db77ExtInfo>,
    #[prost(uint32, tag="10")]
    pub send_type: u32,
    #[prost(uint64, tag="11")]
    pub recv_uin: u64,
    #[prost(message, optional, tag="12")]
    pub rich_msg_body: ::core::option::Option<Db77RichMsgBody>,
    #[prost(uint64, tag="19")]
    pub recv_guild_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Db77ClientInfo {
    #[prost(uint32, tag="1")]
    pub platform: u32,
    #[prost(string, tag="2")]
    pub sdk_version: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub android_package_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub android_signature: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub ios_bundle_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub pc_sign: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Db77ExtInfo {
    #[prost(uint32, repeated, tag="11")]
    pub custom_feature_id: ::prost::alloc::vec::Vec<u32>,
    #[prost(string, tag="12")]
    pub apns_wording: ::prost::alloc::string::String,
    #[prost(uint32, tag="13")]
    pub group_save_db_flag: u32,
    #[prost(uint32, tag="14")]
    pub receiver_app_id: u32,
    #[prost(uint64, tag="15")]
    pub msg_seq: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Db77RichMsgBody {
    #[prost(string, tag="10")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub summary: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub brief: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub url: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub picture_url: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub action: ::prost::alloc::string::String,
    ///ImageInfo imageInfo = 17;
    #[prost(string, tag="16")]
    pub music_url: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cpu {
    #[prost(string, optional, tag="1")]
    pub model: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="2")]
    pub cores: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub frequency: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Camera {
    #[prost(uint64, optional, tag="1")]
    pub primary: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub secondary: ::core::option::Option<u64>,
    #[prost(bool, optional, tag="3")]
    pub flash: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D769ConfigSeq {
    #[prost(uint32, optional, tag="1")]
    pub r#type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub version: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Content {
    #[prost(uint32, optional, tag="1")]
    pub task_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub compress: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="10")]
    pub content: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D769DeviceInfo {
    #[prost(string, optional, tag="1")]
    pub brand: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub model: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub os: ::core::option::Option<C41219os>,
    #[prost(message, optional, tag="4")]
    pub cpu: ::core::option::Option<Cpu>,
    #[prost(message, optional, tag="5")]
    pub memory: ::core::option::Option<Memory>,
    #[prost(message, optional, tag="6")]
    pub storage: ::core::option::Option<Storage>,
    #[prost(message, optional, tag="7")]
    pub screen: ::core::option::Option<Screen>,
    #[prost(message, optional, tag="8")]
    pub camera: ::core::option::Option<Camera>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Memory {
    #[prost(uint64, optional, tag="1")]
    pub total: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub process: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C41219os {
    #[prost(uint32, optional, tag="1")]
    pub r#type: ::core::option::Option<u32>,
    #[prost(string, optional, tag="2")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub sdk: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub kernel: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub rom: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUinPackageUsageReq {
    #[prost(uint32, optional, tag="1")]
    pub r#type: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="2")]
    pub uin_file_size: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUinPackageUsageRsp {
    #[prost(uint32, optional, tag="1")]
    pub status: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="2")]
    pub left_uin_num: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub max_uin_num: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="4")]
    pub proportion: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="10")]
    pub uin_package_used_list: ::prost::alloc::vec::Vec<UinPackageUsedInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D769ReqBody {
    #[prost(message, repeated, tag="1")]
    pub config_list: ::prost::alloc::vec::Vec<D769ConfigSeq>,
    #[prost(message, optional, tag="2")]
    pub device_info: ::core::option::Option<D769DeviceInfo>,
    #[prost(string, optional, tag="3")]
    pub info: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub province: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub city: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="6")]
    pub req_debug_msg: ::core::option::Option<i32>,
    #[prost(message, optional, tag="101")]
    pub query_uin_package_usage_req: ::core::option::Option<QueryUinPackageUsageReq>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D769RspBody {
    #[prost(uint32, optional, tag="1")]
    pub result: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="2")]
    pub config_list: ::prost::alloc::vec::Vec<D769ConfigSeq>,
    #[prost(message, optional, tag="101")]
    pub query_uin_package_usage_rsp: ::core::option::Option<QueryUinPackageUsageRsp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Screen {
    #[prost(string, optional, tag="1")]
    pub model: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="2")]
    pub width: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub height: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub dpi: ::core::option::Option<u32>,
    #[prost(bool, optional, tag="5")]
    pub multi_touch: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Storage {
    #[prost(uint64, optional, tag="1")]
    pub builtin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub external: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UinPackageUsedInfo {
    #[prost(uint32, optional, tag="1")]
    pub rule_id: ::core::option::Option<u32>,
    #[prost(string, optional, tag="2")]
    pub author: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="4")]
    pub uin_num: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D88dGroupHeadPortraitInfo {
    #[prost(uint32, optional, tag="1")]
    pub pic_id: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D88dGroupHeadPortrait {
    #[prost(uint32, optional, tag="1")]
    pub pic_count: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="2")]
    pub msg_info: ::prost::alloc::vec::Vec<D88dGroupHeadPortraitInfo>,
    #[prost(uint32, optional, tag="3")]
    pub default_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub verifying_pic_cnt: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="5")]
    pub msg_verifying_pic_info: ::prost::alloc::vec::Vec<D88dGroupHeadPortraitInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D88dGroupExInfoOnly {
    #[prost(uint32, optional, tag="1")]
    pub tribe_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub money_for_add_group: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D88dGroupInfo {
    #[prost(uint64, optional, tag="1")]
    pub group_owner: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="2")]
    pub group_create_time: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub group_flag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub group_flag_ext: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub group_member_max_num: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="6")]
    pub group_member_num: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="7")]
    pub group_option: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="8")]
    pub group_class_ext: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="9")]
    pub group_special_class: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="10")]
    pub group_level: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="11")]
    pub group_face: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="12")]
    pub group_default_page: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="13")]
    pub group_info_seq: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="14")]
    pub group_roaming_time: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="15")]
    pub group_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="16")]
    pub group_memo: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="17")]
    pub group_finger_memo: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="18")]
    pub group_class_text: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, repeated, packed="false", tag="19")]
    pub group_alliance_code: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, optional, tag="20")]
    pub group_extra_aadm_num: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="21")]
    pub group_uin: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="22")]
    pub group_cur_msg_seq: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="23")]
    pub group_last_msg_time: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="24")]
    pub group_question: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="25")]
    pub group_answer: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="26")]
    pub group_visitor_max_num: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="27")]
    pub group_visitor_cur_num: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="28")]
    pub level_name_seq: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="29")]
    pub group_admin_max_num: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="30")]
    pub group_aio_skin_timestamp: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="31")]
    pub group_board_skin_timestamp: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="32")]
    pub group_aio_skin_url: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="33")]
    pub group_board_skin_url: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="34")]
    pub group_cover_skin_timestamp: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="35")]
    pub group_cover_skin_url: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="36")]
    pub group_grade: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="37")]
    pub active_member_num: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="38")]
    pub certification_type: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="39")]
    pub certification_text: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="40")]
    pub group_rich_finger_memo: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag="41")]
    pub tag_record: ::prost::alloc::vec::Vec<D88dTagRecord>,
    #[prost(message, optional, tag="42")]
    pub group_geo_info: ::core::option::Option<D88dGroupGeoInfo>,
    #[prost(uint32, optional, tag="43")]
    pub head_portrait_seq: ::core::option::Option<u32>,
    #[prost(message, optional, tag="44")]
    pub msg_head_portrait: ::core::option::Option<D88dGroupHeadPortrait>,
    #[prost(uint32, optional, tag="45")]
    pub shutup_timestamp: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="46")]
    pub shutup_timestamp_me: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="47")]
    pub create_source_flag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="48")]
    pub cmduin_msg_seq: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="49")]
    pub cmduin_join_time: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="50")]
    pub cmduin_uin_flag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="51")]
    pub cmduin_flag_ex: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="52")]
    pub cmduin_new_mobile_flag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="53")]
    pub cmduin_read_msg_seq: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="54")]
    pub cmduin_last_msg_time: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="55")]
    pub group_type_flag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="56")]
    pub app_privilege_flag: ::core::option::Option<u32>,
    #[prost(message, optional, tag="57")]
    pub st_group_ex_info: ::core::option::Option<D88dGroupExInfoOnly>,
    #[prost(uint32, optional, tag="58")]
    pub group_sec_level: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="59")]
    pub group_sec_level_info: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="60")]
    pub cmduin_privilege: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="61")]
    pub poid_info: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="62")]
    pub cmduin_flag_ex2: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="63")]
    pub conf_uin: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="64")]
    pub conf_max_msg_seq: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="65")]
    pub conf_to_group_time: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="66")]
    pub password_redbag_time: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="67")]
    pub subscription_uin: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="68")]
    pub member_list_change_seq: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="69")]
    pub membercard_seq: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="70")]
    pub root_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="71")]
    pub parent_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="72")]
    pub team_seq: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="73")]
    pub history_msg_begin_time: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="74")]
    pub invite_no_auth_num_limit: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="75")]
    pub cmduin_history_msg_seq: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="76")]
    pub cmduin_join_msg_seq: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="77")]
    pub group_flagext3: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="78")]
    pub group_open_appid: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="79")]
    pub is_conf_group: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="80")]
    pub is_modify_conf_group_face: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="81")]
    pub is_modify_conf_group_name: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="82")]
    pub no_finger_open_flag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="83")]
    pub no_code_finger_open_flag: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqGroupInfo {
    #[prost(uint64, optional, tag="1")]
    pub group_code: ::core::option::Option<u64>,
    #[prost(message, optional, tag="2")]
    pub stgroupinfo: ::core::option::Option<D88dGroupInfo>,
    #[prost(uint32, optional, tag="3")]
    pub last_get_group_name_time: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D88dReqBody {
    #[prost(uint32, optional, tag="1")]
    pub app_id: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="2")]
    pub req_group_info: ::prost::alloc::vec::Vec<ReqGroupInfo>,
    #[prost(uint32, optional, tag="3")]
    pub pc_client_version: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspGroupInfo {
    #[prost(uint64, optional, tag="1")]
    pub group_code: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="2")]
    pub result: ::core::option::Option<u32>,
    #[prost(message, optional, tag="3")]
    pub group_info: ::core::option::Option<D88dGroupInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D88dRspBody {
    #[prost(message, repeated, tag="1")]
    pub rsp_group_info: ::prost::alloc::vec::Vec<RspGroupInfo>,
    #[prost(bytes="vec", optional, tag="2")]
    pub str_error_info: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D88dTagRecord {
    #[prost(uint64, optional, tag="1")]
    pub from_uin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub group_code: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="3")]
    pub tag_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="4")]
    pub set_time: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="5")]
    pub good_num: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="6")]
    pub bad_num: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="7")]
    pub tag_len: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="8")]
    pub tag_value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D88dGroupGeoInfo {
    #[prost(uint64, optional, tag="1")]
    pub owneruin: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="2")]
    pub settime: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub cityid: ::core::option::Option<u32>,
    #[prost(int64, optional, tag="4")]
    pub longitude: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="5")]
    pub latitude: ::core::option::Option<i64>,
    #[prost(bytes="vec", optional, tag="6")]
    pub geocontent: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="7")]
    pub poi_id: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TranslateReqBody {
    /// TranslateReq translate_req = 1;
    #[prost(message, optional, tag="2")]
    pub batch_translate_req: ::core::option::Option<BatchTranslateReq>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TranslateRspBody {
    /// TranslateRsp translate_rsp = 1;
    #[prost(message, optional, tag="2")]
    pub batch_translate_rsp: ::core::option::Option<BatchTranslateRsp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchTranslateReq {
    #[prost(string, tag="1")]
    pub src_language: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub dst_language: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub src_text_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchTranslateRsp {
    #[prost(int32, tag="1")]
    pub error_code: i32,
    #[prost(bytes="vec", tag="2")]
    pub error_msg: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub src_language: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub dst_language: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="5")]
    pub src_text_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="6")]
    pub dst_text_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InviteUinInfo {
    #[prost(uint64, optional, tag="1")]
    pub uin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub judge_group_code: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub judge_conf_code: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D758ReqBody {
    #[prost(uint64, optional, tag="1")]
    pub join_group_code: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="2")]
    pub be_invited_uin_info: ::prost::alloc::vec::Vec<InviteUinInfo>,
    #[prost(string, optional, tag="3")]
    pub msg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="4")]
    pub main_source_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub sub_source_id: ::core::option::Option<u32>,
    #[prost(string, optional, tag="6")]
    pub verify_token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="7")]
    pub verify_type: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D758RspBody {
    #[prost(uint64, optional, tag="1")]
    pub group_code: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub current_max_msgseq: ::core::option::Option<u64>,
    #[prost(string, optional, tag="3")]
    pub verify_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="4")]
    pub verify_type: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct De07ReqBody {
    #[prost(int32, tag="1")]
    pub version: i32,
    #[prost(int32, tag="2")]
    pub client: i32,
    #[prost(int32, tag="3")]
    pub entrance: i32,
    #[prost(message, optional, tag="10")]
    pub ocr_req_body: ::core::option::Option<OcrReqBody>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OcrReqBody {
    #[prost(string, tag="1")]
    pub image_url: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub language_type: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub scene: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub origin_md5: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub after_compress_md5: ::prost::alloc::string::String,
    #[prost(int32, tag="12")]
    pub after_compress_file_size: i32,
    #[prost(int32, tag="13")]
    pub after_compress_weight: i32,
    #[prost(int32, tag="14")]
    pub after_compress_height: i32,
    #[prost(bool, tag="15")]
    pub is_cut: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct De07RspBody {
    #[prost(int32, tag="1")]
    pub ret_code: i32,
    #[prost(string, tag="2")]
    pub err_msg: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub wording: ::prost::alloc::string::String,
    #[prost(message, optional, tag="10")]
    pub ocr_rsp_body: ::core::option::Option<OcrRspBody>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextDetection {
    #[prost(string, tag="1")]
    pub detected_text: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub confidence: i32,
    #[prost(message, optional, tag="3")]
    pub polygon: ::core::option::Option<Polygon>,
    #[prost(string, tag="4")]
    pub advanced_info: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Polygon {
    #[prost(message, repeated, tag="1")]
    pub coordinates: ::prost::alloc::vec::Vec<Coordinate>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Coordinate {
    #[prost(int32, tag="1")]
    pub x: i32,
    #[prost(int32, tag="2")]
    pub y: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Language {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub language_desc: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OcrRspBody {
    #[prost(message, repeated, tag="1")]
    pub text_detections: ::prost::alloc::vec::Vec<TextDetection>,
    #[prost(string, tag="2")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="101")]
    pub ocr_language_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="102")]
    pub dst_translate_language_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="103")]
    pub language_list: ::prost::alloc::vec::Vec<Language>,
    #[prost(int32, tag="111")]
    pub after_compress_weight: i32,
    #[prost(int32, tag="112")]
    pub after_compress_height: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D8fcReqBody {
    #[prost(int64, optional, tag="1")]
    pub group_code: ::core::option::Option<i64>,
    #[prost(int32, optional, tag="2")]
    pub show_flag: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="3")]
    pub mem_level_info: ::prost::alloc::vec::Vec<D8fcMemberInfo>,
    #[prost(message, repeated, tag="4")]
    pub level_name: ::prost::alloc::vec::Vec<D8fcLevelName>,
    #[prost(int32, optional, tag="5")]
    pub update_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="6")]
    pub office_mode: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="7")]
    pub group_open_appid: ::core::option::Option<i32>,
    #[prost(message, optional, tag="8")]
    pub msg_client_info: ::core::option::Option<D8fcClientInfo>,
    #[prost(bytes="vec", optional, tag="9")]
    pub auth_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D8fcMemberInfo {
    #[prost(int64, optional, tag="1")]
    pub uin: ::core::option::Option<i64>,
    #[prost(int32, optional, tag="2")]
    pub point: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub active_day: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="4")]
    pub level: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="5")]
    pub special_title: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="6")]
    pub special_title_expire_time: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="7")]
    pub uin_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="8")]
    pub member_card_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="9")]
    pub phone: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="10")]
    pub email: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="11")]
    pub remark: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="12")]
    pub gender: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="13")]
    pub job: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="14")]
    pub tribe_level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="15")]
    pub tribe_point: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="16")]
    pub rich_card_name: ::prost::alloc::vec::Vec<D8fcCardNameElem>,
    #[prost(bytes="vec", optional, tag="17")]
    pub comm_rich_card_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D8fcCardNameElem {
    #[prost(int32, optional, tag="1")]
    pub enum_card_type: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D8fcLevelName {
    #[prost(int32, optional, tag="1")]
    pub level: ::core::option::Option<i32>,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D8fcClientInfo {
    #[prost(int32, optional, tag="1")]
    pub implat: ::core::option::Option<i32>,
    #[prost(string, optional, tag="2")]
    pub ing_clientver: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D8fcCommCardNameBuf {
    #[prost(message, repeated, tag="1")]
    pub rich_card_name: ::prost::alloc::vec::Vec<D8fcRichCardNameElem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D8fcRichCardNameElem {
    #[prost(bytes="vec", optional, tag="1")]
    pub ctrl: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="2")]
    pub text: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
//
//message ArkMsg {
//optional string appName = 1;
//optional string json = 2;
//}
//
//message BatchReqBody {
//optional uint64 groupCode = 1;
//repeated MsgInfo msgs = 2;
//}
//
//message BatchRspBody {
//optional string wording = 1;
//optional uint32 errorCode = 2;
//optional int32 succCnt = 3;
//repeated MsgProcessInfo procInfos = 4;
//optional uint32 digestTime = 5;
//}
//
//message DigestMsg {
//optional uint64 groupCode = 1;
//optional uint32 seq = 2;
//optional uint32 random = 3;
//repeated MsgElem content = 4;
//optional uint64 textSize = 5;
//optional uint64 picSize = 6;
//optional uint64 videoSize = 7;
//optional uint64 senderUin = 8;
//optional uint32 senderTime = 9;
//optional uint64 addDigestUin = 10;
//optional uint32 addDigestTime = 11;
//optional uint32 startTime = 12;
//optional uint32 latestMsgSeq = 13;
//optional uint32 opType = 14;
//}
//
//message FaceMsg {
//optional uint32 index = 1;
//optional string text = 2;
//}
//
//message GroupFileMsg {
//optional bytes fileName = 1;
//optional uint32 busId = 2;
//optional string fileId = 3;
//optional uint64 fileSize = 4;
//optional uint64 deadTime = 5;
//optional bytes fileSha1 = 6;
//optional bytes ext = 7;
//optional bytes fileMd5 = 8;
//}
//
//message ImageMsg {
//optional string md5 = 1;
//optional string uuid = 2;
//optional uint32 imgType = 3;
//optional uint32 fileSize = 4;
//optional uint32 width = 5;
//optional uint32 height = 6;
//optional uint32 fileId = 101;
//optional uint32 serverIp = 102;
//optional uint32 serverPort = 103;
//optional string filePath = 104;
//optional string thumbUrl = 201;
//optional string originalUrl = 202;
//optional string resaveUrl = 203;
//}
//
//message MsgElem {
//optional uint32 type = 1;
//optional TextMsg textMsg = 11;
//optional FaceMsg faceMsg = 12;
//optional ImageMsg imageMsg = 13;
//optional GroupFileMsg groupFileMsg = 14;
//optional ShareMsg shareMsg = 15;
//optional RichMsg richMsg = 16;
//optional ArkMsg arkMsg = 17;
//}
//
//message MsgInfo {
//optional uint32 seq = 1;
//optional uint32 random = 2;
//}
//
//message MsgProcessInfo {
//optional MsgInfo msg = 1;
//optional uint32 errorCode = 2;
//optional uint64 digestUin = 3;
//optional uint32 digestTime = 4;
//}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EacReqBody {
    #[prost(uint64, optional, tag="1")]
    pub group_code: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="2")]
    pub seq: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub random: ::core::option::Option<u32>,
}
//
//message RichMsg {
//optional uint32 serviceId = 1;
//optional string xml = 2;
//optional string longMsgResid = 3;
//}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EacRspBody {
    #[prost(string, optional, tag="1")]
    pub wording: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="2")]
    pub digest_uin: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="3")]
    pub digest_time: ::core::option::Option<u32>,
    ///optional DigestMsg msg = 4;
    #[prost(uint32, optional, tag="10")]
    pub error_code: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D8a7ReqBody {
    #[prost(uint32, optional, tag="1")]
    pub sub_cmd: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub limit_interval_type_for_uin: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub limit_interval_type_for_group: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="4")]
    pub uin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub group_code: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D8a7RspBody {
    #[prost(bool, optional, tag="1")]
    pub can_at_all: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag="2")]
    pub remain_at_all_count_for_uin: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub remain_at_all_count_for_group: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="4")]
    pub prompt_msg1: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="5")]
    pub prompt_msg2: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OidbssoPkg {
    #[prost(int32, tag="1")]
    pub command: i32,
    #[prost(int32, tag="2")]
    pub service_type: i32,
    #[prost(int32, tag="3")]
    pub result: i32,
    #[prost(bytes="vec", tag="4")]
    pub bodybuffer: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="5")]
    pub error_msg: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub client_version: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D8a0RspBody {
    #[prost(int64, tag="1")]
    pub opt_uint64_group_code: i64,
    #[prost(message, repeated, tag="2")]
    pub msg_kick_result: ::prost::alloc::vec::Vec<D8a0KickResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D8a0KickResult {
    #[prost(int32, tag="1")]
    pub opt_uint32_result: i32,
    #[prost(int64, tag="2")]
    pub opt_uint64_member_uin: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D8a0KickMemberInfo {
    #[prost(int32, tag="1")]
    pub opt_uint32_operate: i32,
    #[prost(int64, tag="2")]
    pub opt_uint64_member_uin: i64,
    #[prost(int32, tag="3")]
    pub opt_uint32_flag: i32,
    #[prost(bytes="vec", tag="4")]
    pub opt_bytes_msg: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D8a0ReqBody {
    #[prost(int64, tag="1")]
    pub opt_uint64_group_code: i64,
    #[prost(message, repeated, tag="2")]
    pub msg_kick_list: ::prost::alloc::vec::Vec<D8a0KickMemberInfo>,
    #[prost(int64, repeated, tag="3")]
    pub kick_list: ::prost::alloc::vec::Vec<i64>,
    #[prost(int32, tag="4")]
    pub kick_flag: i32,
    #[prost(bytes="vec", tag="5")]
    pub kick_msg: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D89aReqBody {
    #[prost(int64, tag="1")]
    pub group_code: i64,
    #[prost(message, optional, tag="2")]
    pub st_group_info: ::core::option::Option<D89aGroupinfo>,
    #[prost(int64, tag="3")]
    pub original_operator_uin: i64,
    #[prost(int32, tag="4")]
    pub req_group_open_appid: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D89aGroupinfo {
    #[prost(int32, tag="1")]
    pub group_ext_adm_num: i32,
    #[prost(int32, tag="2")]
    pub flag: i32,
    #[prost(bytes="vec", tag="3")]
    pub ing_group_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub ing_group_memo: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub ing_group_finger_memo: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub ing_group_aio_skin_url: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub ing_group_board_skin_url: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub ing_group_cover_skin_url: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="9")]
    pub group_grade: i32,
    #[prost(int32, tag="10")]
    pub active_member_num: i32,
    #[prost(int32, tag="11")]
    pub certification_type: i32,
    #[prost(bytes="vec", tag="12")]
    pub ing_certification_text: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="13")]
    pub ing_group_rich_finger_memo: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="14")]
    pub st_group_newguidelines: ::core::option::Option<D89aGroupNewGuidelinesInfo>,
    #[prost(int32, tag="15")]
    pub group_face: i32,
    #[prost(int32, tag="16")]
    pub add_option: i32,
    #[prost(int32, tag="18")]
    pub group_type_flag: i32,
    #[prost(bytes="vec", tag="19")]
    pub string_group_tag: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="20")]
    pub msg_group_geo_info: ::core::option::Option<D89aGroupGeoInfo>,
    #[prost(int32, tag="21")]
    pub group_class_ext: i32,
    #[prost(bytes="vec", tag="22")]
    pub ing_group_class_text: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="23")]
    pub app_privilege_flag: i32,
    #[prost(int32, tag="24")]
    pub app_privilege_mask: i32,
    #[prost(message, optional, tag="25")]
    pub st_group_ex_info: ::core::option::Option<D89aGroupExInfoOnly>,
    #[prost(int32, tag="26")]
    pub group_sec_level: i32,
    #[prost(int32, tag="27")]
    pub group_sec_level_info: i32,
    #[prost(int64, tag="28")]
    pub subscription_uin: i64,
    #[prost(int32, tag="29")]
    pub allow_member_invite: i32,
    #[prost(bytes="vec", tag="30")]
    pub ing_group_question: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="31")]
    pub ing_group_answer: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="32")]
    pub group_flagext3: i32,
    #[prost(int32, tag="33")]
    pub group_flagext3_mask: i32,
    #[prost(int32, tag="34")]
    pub group_open_appid: i32,
    #[prost(int32, tag="35")]
    pub no_finger_open_flag: i32,
    #[prost(int32, tag="36")]
    pub no_code_finger_open_flag: i32,
    #[prost(int64, tag="37")]
    pub root_id: i64,
    #[prost(int32, tag="38")]
    pub msg_limit_frequency: i32,
    #[prost(oneof="d89a_groupinfo::ShutupTime", tags="17")]
    pub shutup_time: ::core::option::Option<d89a_groupinfo::ShutupTime>,
}
/// Nested message and enum types in `D89AGroupinfo`.
pub mod d89a_groupinfo {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ShutupTime {
        #[prost(int32, tag="17")]
        Val(i32),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D89aGroupNewGuidelinesInfo {
    #[prost(bool, tag="1")]
    pub bool_enabled: bool,
    #[prost(bytes="vec", tag="2")]
    pub ing_content: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D89aGroupExInfoOnly {
    #[prost(int32, tag="1")]
    pub tribe_id: i32,
    #[prost(int32, tag="2")]
    pub money_for_add_group: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D89aGroupGeoInfo {
    #[prost(int32, tag="1")]
    pub city_id: i32,
    #[prost(int64, tag="2")]
    pub longtitude: i64,
    #[prost(int64, tag="3")]
    pub latitude: i64,
    #[prost(bytes="vec", tag="4")]
    pub ing_geo_content: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="5")]
    pub poi_id: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ded3ReqBody {
    #[prost(int64, tag="1")]
    pub to_uin: i64,
    #[prost(int64, tag="2")]
    pub group_code: i64,
    #[prost(int32, tag="3")]
    pub msg_seq: i32,
    #[prost(int32, tag="4")]
    pub msg_rand: i32,
    #[prost(int64, tag="5")]
    pub aio_uin: i64,
}
