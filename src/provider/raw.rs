use std::{
    mem,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    task::{RawWaker, RawWakerVTable, Waker},
};
#[derive(Debug, Clone)]
pub struct RawTask(Arc<AtomicBool>);

impl RawTask {
    const RAW_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
        Self::clone_waker,
        Self::wake,
        Self::wake_by_ref,
        Self::drop_waker,
    );
    pub fn new() -> Self {
        Self(Arc::new(AtomicBool::new(true)))
    }
    pub fn already(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
    pub fn reset(&self) {
        self.0.store(false, Ordering::Relaxed);
    }
    pub fn to_waker(&self) -> Waker {
        unsafe { Waker::from_raw(self.to_raw_waker()) }
    }
    pub fn to_raw_waker(&self) -> RawWaker {
        // log::info!("to_raw_waker()");
        // let arc = self.0.clone();
        // let n = Self(arc);
        // log::info!("clone()");
        // let ptr = (&n as *const Self as *const ());
        // mem::forget(n);
        // log::info!("new()");
        let ptr = self as *const Self as *const ();
        RawWaker::new(ptr, &Self::RAW_WAKER_VTABLE)
    }
    pub fn from_ptr(ptr: *const ()) -> &'static Self {
        unsafe { &*(ptr as *const Self) }
    }
    pub unsafe fn clone_waker(ptr: *const ()) -> RawWaker {
        let raw = Self::from_ptr(ptr);
        raw.to_raw_waker()
    }
    pub unsafe fn wake(ptr: *const ()) {
        // Self::wake_by_ref(ptr);
        // Self::drop_waker(ptr);
    }
    pub unsafe fn wake_by_ref(ptr: *const ()) {
        // let raw = Self::from_ptr(ptr);
        // raw.0.store(true,Ordering::Release);
    }
    pub unsafe fn drop_waker(ptr: *const ()) {
        // let raw = Self::from_ptr(ptr);
        // drop(raw);
        // (ptr as *const Self as *mut Self).drop_in_place();
    }
}
