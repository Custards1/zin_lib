
pub type Zstr = [u8];
pub type ZString = std::rc::Rc<str>;
pub type ZBytes = std::rc::Rc<Zstr>;

#[macro_export]
macro_rules! static_tstring  {
    ($($t:tt)*) => {
        pub fn $($t)*()->crate::zstring::ZString{
            static VAL:&'static str = stringify!( $($t)*);
            crate::zstring::ZString::from(VAL)
        }
        
    };
}
#[macro_export]
macro_rules! static_named_tstring  {
    ($b:literal,$($t:tt)*) => {
        pub fn $($t)*()->crate::zstring::ZString{
            static VAL:&'static str = $b;
            crate::zstring::ZString::from(VAL)
        }
        
    };
}