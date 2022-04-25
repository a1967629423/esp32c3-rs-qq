use std::sync::atomic::{AtomicU64, Ordering};
#[derive(Debug, Clone, Copy)]
pub struct AtomicUsize(usize);

impl AtomicUsize {
    pub const fn new(v: usize) -> Self {
        AtomicUsize(v)
    }
    pub fn compare_exchange_weak(
        &self,
        current: usize,
        new: usize,
        _success: Ordering,
        _failure: Ordering,
    ) -> Result<usize, usize> {
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
        current: usize,
        new: usize,
        _success: Ordering,
        _failure: Ordering,
    ) -> Result<usize, usize> {
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
    pub fn fetch_or(&self, val: usize, order: Ordering) -> usize {
        let old = self.0;
        unsafe {
            *(&self.0 as *const _ as *mut _) = old | val;
        }
        old
    }
    pub fn fetch_and(&self, val: usize, order: Ordering) -> usize {
        let old = self.0;
        unsafe {
            *(&self.0 as *const _ as *mut _) = old & val;
        }
        old
    }
    pub fn fetch_add(&self, val: usize, order: Ordering) -> usize {
        let old = self.0;
        unsafe {
            *(&self.0 as *const _ as *mut _) = old + val;
        }
        old
    }
    pub fn fetch_sub(&self, val: usize, order: Ordering) -> usize {
        let old = self.0;
        unsafe {
            *(&self.0 as *const _ as *mut _) = old - val;
        }
        old
    }
    pub fn load(&self, order: Ordering) -> usize {
        self.0
    }
    pub fn store(&self, val: usize, order: Ordering) -> usize {
        let old = self.0;
        unsafe {
            *(&self.0 as *const _ as *mut _) = val;
        }
        old
    }
    pub fn swap(&self, val: usize, order: Ordering) -> usize {
        let old = self.0;
        unsafe {
            *(&self.0 as *const _ as *mut _) = val;
        }
        old
    }
    pub fn get_mut(&mut self) -> &mut usize {
        &mut self.0
    }
}
