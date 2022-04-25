use std::{
    cell::Cell,
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
};

pub struct UnsafeLazy<T, F = fn() -> T> {
    cell: MaybeUninit<T>,
    init: Cell<Option<F>>,
    init_status: bool,
}

impl<T, F> UnsafeLazy<T, F> {
    pub const fn new(init: F) -> Self {
        Self {
            cell: MaybeUninit::uninit(),
            init: Cell::new(Some(init)),
            init_status: false,
        }
    }
}
impl<T, F: FnOnce() -> T> UnsafeLazy<T, F> {
    pub fn force(&self) -> &T {
        if !self.init_status {
            unsafe {
                *(&self.init_status as *const _ as *mut _) = true;
            }
            let func = self.init.take().unwrap();
            let res = func();
            unsafe {
                (self.cell.as_ptr() as *mut T).write(res);
            }
        }
        unsafe { &*self.cell.as_ptr() }
    }
}
impl<T, F> Deref for UnsafeLazy<T, F>
where
    F: FnOnce() -> T,
{
    type Target = T;
    fn deref(&self) -> &T {
        self.force()
    }
}

impl<T, F: FnOnce() -> T> DerefMut for UnsafeLazy<T, F> {
    fn deref_mut(&mut self) -> &mut T {
        self.force();
        unsafe { &mut (*self.cell.as_mut_ptr()) }
    }
}

unsafe impl<T, F> Send for UnsafeLazy<T, F> where T: Send {}
unsafe impl<T, F> Sync for UnsafeLazy<T, F> where T: Send {}
