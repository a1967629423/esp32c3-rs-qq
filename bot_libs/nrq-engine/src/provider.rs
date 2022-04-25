use alloc::boxed::Box;

use crate::{crypto, CoreCryptoRng};
static mut RANDOM_PROVIDER: core::mem::MaybeUninit<Box<dyn CoreCryptoRng>> =
    core::mem::MaybeUninit::uninit();

pub fn init_random_provider(p: Box<dyn CoreCryptoRng>) {
    unsafe {
        RANDOM_PROVIDER.as_mut_ptr().write(p);
    }
}
pub fn init_test_random_provider() {
    init_random_provider(Box::new(crypto::ForceCryptoRng::new(
        rand::rngs::mock::StepRng::new(1, 3),
    )));
}
pub fn get_random_provider() -> &'static mut Box<dyn CoreCryptoRng> {
    unsafe { &mut *RANDOM_PROVIDER.as_mut_ptr() }
}

pub trait TimerProvider: Send + Sync {
    fn now_timestamp(&self) -> i64 {
        self.now_timestamp_nanos() / 1000000000
    }
    fn now_timestamp_millis(&self) -> i64 {
        self.now_timestamp_nanos() / 1000000
    }
    fn now_timestamp_nanos(&self) -> i64;
}

struct TestTimerProvider {}
impl TimerProvider for TestTimerProvider {
    fn now_timestamp_nanos(&self) -> i64 {
        16454983112120000
    }
}

static mut TIMER_PROVIDER: core::mem::MaybeUninit<Box<dyn TimerProvider>> =
    core::mem::MaybeUninit::uninit();
pub fn init_timer_provider(p: Box<dyn TimerProvider>) {
    unsafe {
        TIMER_PROVIDER.as_mut_ptr().write(p);
    }
}
pub fn init_test_timer_provider() {
    init_timer_provider(Box::new(TestTimerProvider {}));
}
pub fn get_timer_provider() -> &'static Box<dyn TimerProvider> {
    unsafe { &*TIMER_PROVIDER.as_mut_ptr() }
}
