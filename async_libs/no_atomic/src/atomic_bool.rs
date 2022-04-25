use std::sync::atomic::Ordering;
#[derive(Debug, Clone, Copy)]
pub struct AtomicBool(bool);

impl AtomicBool {
    pub const fn new(v: bool) -> Self {
        AtomicBool(v)
    }
    pub fn compare_exchange_weak(
        &self,
        current: bool,
        new: bool,
        _success: Ordering,
        _failure: Ordering,
    ) -> Result<bool, bool> {
        if self.0 != current {
            Err(self.0)
        } else {
            let old = self.0;
            unsafe {
                *(&self.0 as *const _ as *mut _) = new;
            }
            Ok(old)
        }
    }
    pub fn compare_exchange(
        &self,
        current: bool,
        new: bool,
        _success: Ordering,
        _failure: Ordering,
    ) -> Result<bool, bool> {
        if self.0 != current {
            Err(self.0)
        } else {
            let old = self.0;
            unsafe {
                *(&self.0 as *const _ as *mut _) = new;
            }
            Ok(old)
        }
    }

    pub fn load(&self, order: Ordering) -> bool {
        self.0
    }
    pub fn store(&self, val: bool, order: Ordering) -> bool {
        let old = self.0;
        unsafe {
            *(&self.0 as *const _ as *mut _) = val;
        }
        old
    }
    pub fn swap(&self, val: bool, order: Ordering) -> bool {
        let old = self.0;
        unsafe {
            *(&self.0 as *const _ as *mut _) = val;
        }
        old
    }
}
