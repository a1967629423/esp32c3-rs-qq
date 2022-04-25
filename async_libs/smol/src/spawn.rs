use std::future::Future;
use std::panic::catch_unwind;
use std::thread;

use async_executor::{Executor, Task};
use async_io::block_on;
use futures_lite::future;
// use no_atomic::UnsafeLazy as Lazy;
use once_cell::sync::Lazy;
// struct MyLazy<T,F = fn() -> T>(Lazy<T,F>);

// impl<T,F> MyLazy<T,F> {

/// Spawns a task onto the global executor (single-threaded by default).
///
/// There is a global executor that gets lazily initialized on first use. It is included in this
/// library for convenience when writing unit tests and small programs, but it is otherwise
/// more advisable to create your own [`Executor`].
///
/// By default, the global executor is run by a single background thread, but you can also
/// configure the number of threads by setting the `SMOL_THREADS` environment variable.
///
/// # Examples
///
/// ```
/// let task = smol::spawn(async {
///     1 + 2
/// });
///
/// smol::block_on(async {
///     assert_eq!(task.await, 3);
/// });
/// ```
// pub fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> Task<T> {
//     static mut GLOBAL: MaybeUninit<Executor<'_>> = MaybeUninit::uninit();
//     static mut GLOBAL_INIT: bool = false;
//     unsafe {
//         log::info!("spawn()");
//         if !GLOBAL_INIT {
//             log::info!("init GLOBAL");
//             GLOBAL_INIT = true;
//             let num_threads = {
//                 // Parse SMOL_THREADS or default to 1.
//                 std::env::var("SMOL_THREADS")
//                     .ok()
//                     .and_then(|s| s.parse().ok())
//                     .unwrap_or(1)
//             };
//             log::info!("GLOBAL init success");
//             GLOBAL.write(Executor::new());
//             for n in 1..=num_threads {
//                 thread::Builder::new()
//                     .stack_size(32 * 1024)
//                     .name(format!("smol-{}", n))
//                     .spawn(|| loop {
//                         catch_unwind(|| block_on((&*GLOBAL.as_ptr()).run(future::pending::<()>())))
//                             .ok();
//                     })
//                     .expect("cannot spawn executor thread");
//             }
//         }
//         log::info!("GLOBAL to spawn");
//         (&*GLOBAL.as_ptr()).spawn(future)
//     }
// }

pub fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> Task<T> {
    static GLOBAL: Lazy<Executor<'_>> = Lazy::new(|| {
        let num_threads = {
            // Parse SMOL_THREADS or default to 1.
            std::env::var("SMOL_THREADS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1)
        };

        for n in 1..=num_threads {
            thread::Builder::new()
                .name(format!("smol-{}", n))
                .stack_size(32 * 1024)
                .spawn(|| loop {
                    catch_unwind(|| {
                        log::info!("background thread block on");
                        block_on(GLOBAL.run(future::pending::<()>()))
                    })
                    .ok();
                })
                .expect("cannot spawn executor thread");
        }

        Executor::new()
    });

    GLOBAL.spawn(future)
}
