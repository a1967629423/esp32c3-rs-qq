mod structs;
pub use structs::*;
pub const APP_CONFIG:AppConfig<'static> = include!("../../out/app.config.out");