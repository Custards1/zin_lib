pub mod hashmap;
pub use hashmap::HashMap;
pub mod ops;
#[macro_use]
pub mod number;
pub mod value;
pub use ops::*;
pub use number::*;
pub use value::*;
pub mod refrence;