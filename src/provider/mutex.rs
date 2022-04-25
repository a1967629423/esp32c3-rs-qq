use std::ops::{Deref, DerefMut};

use futures::Future;
use nrs_qq::{MutexProvider, TMutex, TMutexGuard};
use smol::lock::{Mutex, MutexGuard};
#[derive(Debug)]
pub struct MyMutexProvider;
#[derive(Debug)]
pub struct MyMutex<T>(Mutex<T>);
#[derive(Debug)]
pub struct MyMutexGuard<'a, T>(MutexGuard<'a, T>);

impl<'a, T> Deref for MyMutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.0.deref()
    }
}

impl<'a, T> DerefMut for MyMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0.deref_mut()
    }
}

impl<'a, T> TMutexGuard<'a, T> for MyMutexGuard<'a, T> {}

impl<T: Send> TMutex<T> for MyMutex<T> {
    type Guard<'a>  = MyMutexGuard<'a, T> where Self: 'a;

    type Future<'a> = impl Future<Output = Self::Guard<'a>> + Send   where Self: 'a,;

    fn new(value: T) -> Self {
        Self(Mutex::new(value))
    }

    fn lock(&self) -> Self::Future<'_> {
        async move {
            let guard = self.0.lock().await;
            MyMutexGuard(guard)
        }
    }
}

impl MutexProvider for MyMutexProvider {
    type Mutex<T: Send + Sync> = MyMutex<T>;
}
