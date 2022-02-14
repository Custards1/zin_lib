#[macro_export]
macro_rules! debugln {
    ($($t:tt)*) => {
        if cfg!(feature = "debugable") {
            println!($($t)*)
        }
    };
}
#[macro_export]
macro_rules! debug {
    ($($t:tt)*) => {
        if cfg!(feature = "debugable") {
            print!($($t)*)
        }
    };
}