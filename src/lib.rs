#[macro_use]
pub mod debug;
#[macro_use]
pub mod zstring;
pub mod types;
#[macro_use]
pub mod error;
pub mod api;
pub mod object;
pub mod stringify;
pub const VERSION:&'static str = env!("CARGO_PKG_VERSION");
