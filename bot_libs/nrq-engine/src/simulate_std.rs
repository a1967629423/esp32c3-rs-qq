#[macro_export]
macro_rules! println {
    () => (
        // impl
    );
    ($($arg:tt)*) => ({
        // impl
        core::format_args_nl!($($arg)*);

    })
}
pub mod error {
    pub use core_error::Error;
}
pub mod option {
    pub use core::option::*;
}
pub mod fmt {
    pub use core::fmt::*;
}
pub mod convert {
    pub use core::convert::*;
}
pub mod string {
    pub use alloc::string::*;
}
pub mod prelude {
    pub use super::string::*;
    #[cfg(not(feature = "std"))]
    pub use crate::println;
    pub use alloc::borrow::ToOwned;
    pub use alloc::boxed::Box;
    pub use alloc::fmt::Debug;
    pub use alloc::format;
    pub use alloc::vec;
    pub use alloc::vec::Vec;
    pub use core::prelude::rust_2021::*;
}
