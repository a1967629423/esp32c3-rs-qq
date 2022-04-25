use crate::provider::RwLockProvider;
use alloc::vec::Vec;
use core::marker::PhantomData;

pub use nrq_engine::structs::*;

// TODO 大群会占用大量内存，可以考虑提供 trait，用磁盘存储
#[derive(Default, Debug)]
pub struct Group<RP: RwLockProvider> {
    __rw_data: PhantomData<RP>,
    pub info: GroupInfo,
    pub members: RP::RwLock<Vec<GroupMemberInfo>>,
}

impl<RP> Group<RP>
where
    RP: RwLockProvider,
{
    pub fn new(info: GroupInfo, members: RP::RwLock<Vec<GroupMemberInfo>>) -> Self {
        Self {
            __rw_data: PhantomData,
            info,
            members,
        }
    }
}
unsafe impl<RP> Send for Group<RP> where RP: RwLockProvider {}
