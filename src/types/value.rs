use crate::error::*;
use crate::object::{GcObject,
                    Object,
                    function::*};
#[macro_use]
use super::number::{Number,Signed,Unsigned};

pub use super::super::zstring::*;



pub type RustFunc = fn (Vec<Value>)->Result<Value>;
//trait to exclude types that should not be hashed
pub trait IntoHashable {
    fn into_hashable(self)-> Value;
}

impl IntoHashable for bool {
    #[inline]
    fn into_hashable(self)-> Value{
        Value::Bool(self)
    }
} 
impl IntoHashable for Number {
    #[inline]
    fn into_hashable(self)-> Value{
        Value::Num(self)
    }
}
impl IntoHashable for ZString {
    #[inline]
    fn into_hashable(self)-> Value{
        Value::String(self)
    }
} 
impl IntoHashable for &str {
    #[inline]
    fn into_hashable(self)-> Value{
        Value::String(self.into())
    }
}   
macro_rules! impl_numbers {
    ($trait:ident,$trait_func:ident,$sself:ident,$from_numbers:ident)=>{
        impl $trait for i8 {
            #[inline]
            fn $trait_func(self)->$sself{
                $sself::$from_numbers(Number::from(Signed::from(self)))
            }
        } 
        impl $trait for i16 {
            #[inline]
            fn $trait_func(self)->$sself{
                $sself::$from_numbers(Number::from(Signed::from(self)))
            }
        } 
        impl $trait for i32 {
            #[inline]
            fn $trait_func(self)->$sself{
                $sself::$from_numbers(Number::from(Signed::from(self)))
            }
        } 
        impl $trait for i64 {
            #[inline]
            fn $trait_func(self)->$sself{
                $sself::$from_numbers(Number::from(Signed::from(self)))
            }
        } 
        impl $trait for u8 {
            #[inline]
            fn $trait_func(self)->$sself{
                $sself::$from_numbers(Number::from(Unsigned::from(self)))
            }
        } 
        impl $trait for u16 {
            #[inline]
            fn $trait_func(self)->$sself{
                $sself::$from_numbers(Number::from(Unsigned::from(self)))
            }
        } 
        impl $trait for u32 {
            #[inline]
            fn $trait_func(self)->$sself{
                $sself::$from_numbers(Number::from(Unsigned::from(self)))
            }
        } 
        impl $trait for u64 {
            #[inline]
            fn $trait_func(self)->$sself{
                $sself::$from_numbers(Number::from(Unsigned::from(self)))
            }
        } 
    }
}

impl_numbers!(IntoHashable,into_hashable,Value,Num);







#[derive(Debug,Clone,Hash,Eq,Ord,gc::Trace,gc::Finalize)]
pub enum Value {
    Bool(bool),
    Char(char),
    Num( #[unsafe_ignore_trace]
        Number),
    Bytes(#[unsafe_ignore_trace]
        super::ZBytes),
    String(super::ZString),
    Object(GcObject),
    NULL,
}


#[inline]
const fn compare_char(zh:char,v:&[u8])->bool{
    let ch = zh as u32;
    let size = zh.len_utf8();
    if v.len()<size {
        return false;
    }
    match size {
        1=>(ch as u8) == v[0],
        2=>(ch&0x000F) as u8 == v[0] &&(ch>>8) as u8 ==v[1],
        3=>(ch&0x000F) as u8 == v[0] &&((ch>>8)&0x000F) as u8 ==v[1] &&((ch>>16)&0x000F) as u8 == v[2],
        _=>(ch&0x000F) as u8 == v[0] &&((ch>>8)&0x000F) as u8 ==v[1] &&((ch>>16)&0x000F) as u8 == v[2] && ((ch >> 24)&0x000F) as u8 == v[3],
    }
}

use std::borrow::Borrow;
impl PartialEq for Value { 
    #[inline]
    fn eq(&self,b:&Self)->bool {
        match self {
            Value::Bool(a)=> match b {
                Value::Bool(c)=>a==c,
                _=>false
            }
            Value::Num(a)=>match b {
                Value::Num(c)=>a==c,
                Value::NULL=>false,
                Value::Char(b)=> *a == (*b as u32).into(),
                Value::Bytes(c)=>a.compare_bytes(&c),
                Value::String(c)=>a.compare_bytes(c.as_bytes()),
                Value::Object(c)=>c.compare_value(self.clone()).0,
                _=>false
            },
            Value::String(a)=>match b {
                Value::String(c)=>a==c,
                Value::Bytes(c)=>a.as_bytes() == c.as_ref(),
                Value::Char(c)=>compare_char(*c,a.as_bytes()),
                Value::Num(c)=>c.compare_bytes(a.as_bytes()),
                Value::Object(c)=>c.compare_value(self.clone()).0,
                _=>false
            }
            Value::Char(a)=>match b {
                Value::Char(c)=>a==c,
                Value::Bytes(c)=>compare_char(*a, &c),
                Value::String(c)=>compare_char(*a, c.as_bytes()),
                Value::Num(b)=> Number::from(*a as u32) == *b,
                _=>false
            }
            Value::Bytes(a)=>match b {
                Value::Bytes(c)=>a==c,
                Value::String(c)=>a.as_ref() == c.as_bytes(),
                Value::Char(c)=>compare_char(*c, &a),
                Value::Num(c)=>c.compare_bytes(a.as_ref()),
                Value::Object(c)=>c.compare_value(self.clone()).0,
                _=>false
            }
            Value::Object(a)=>a.compare_value(b.clone()).0,
            Value::NULL=>match b {
                Value::NULL=>true,
                _=>false
            }
        }
    }
}

impl PartialOrd for Value {
    #[inline]
    fn partial_cmp(&self, b: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Value::Bool(a)=>match b {
                Value::Bool(c)=>Some(a.cmp(c)),
                _=>None
            }
            Value::Object(a)=>match b {
                Value::Object(c)=>Some(a.cmp(c)),
                _=>None
            }
            Value::Num(a)=>match b {
                Value::Num(c)=>Some(a.cmp(c)),
                Value::NULL=>Some(a.cmp(&Number::from(0))),
                _=>None
            }
            Value::Char(a)=>match b {
                Value::Num(c)=>Some(Number::from(*a as u32).cmp(c)),
                Value::NULL=>Some(a.cmp(&(0 as char))),
                Value::Char(c)=>Some(a.cmp(c)),
                _=>None
            }
            Value::String(a)=>match b {
                Value::String(c)=>Some(a.cmp(c)),
                Value::Bytes(d)=> {
                    a.as_bytes().partial_cmp(d.as_ref())
                }
                _=>None
            }
            Value::Bytes(a)=>match b {
                Value::Bytes(c)=>Some(a.cmp(c)),
                Value::String(d)=> {
                    a.as_ref().partial_cmp(d.as_bytes())
                }
                _=>None
            }
            Value::NULL=>match b {
                Value::NULL=>Some(std::cmp::Ordering::Equal),
                Value::Num(a)=>Some(Number::from(0).cmp(a)),
                _=>None
            }
        }
    }
}

impl std::fmt::Display for Value {
    #[inline]
    fn fmt(&self,f: &mut std::fmt::Formatter<'_>)->std::fmt::Result {
        match self {
            Value::Bool(a)=>write!(f,"{}",a),
            Value::Num(a)=>write!(f,"{}",a),
            Value::String(a)=>write!(f,"{}",a),
            Value::NULL=>write!(f,"nullptr"),
            Value::Bytes(a)=>write!(f,"{:?}",a.as_ref()),
            Value::Object(a)=>write!(f,"0x{:p}",a),
            Value::Char(a)=>write!(f,"{}",a),
            /*Value::Array(a)=>{
                write!(f,"[")?;
                let b = a.iter();
                while let Some(c) = b.next() {
                    write!(f,"{}",c)?;
                }
                write!(f,"]")
            }*/
        }
    }
}

impl Value {
    #[inline]
    pub fn ztype(&self) ->ZString{
        match &self {
            Value::Object(a)=>{
                match a.try_borrow() {
                    Ok(a)=>match &a.contents.vec{
                        Some(a)=>{
                            return match &a {
                                crate::object::obj::InnerContainer::String(_)=>"string",
                                crate::object::obj::InnerContainer::Vector(crate::object::obj::InternedVector::ByteVector(_))=>"bytevec",
                                crate::object::obj::InnerContainer::Vector(crate::object::obj::InternedVector::Vector(_))=>"vec",
                                crate::object::obj::InnerContainer::File(_)=>"file",
                                crate::object::obj::InnerContainer::Map(_)=>"map",
                                crate::object::obj::InnerContainer::Function(_)=>"function"
                    
                            }.into();
                        }
                        _=>{
                        
                        }
                    }
                    _=>{

                    }
                }
                match a.name() {
                    Some(a)=>return a.into(),
                    _=>return "object".into()
                }
            }
            Value::Bytes(_) =>return "bytes".into(),
            Value::String(_)=>return "str".into(),
            Value::Num(a)   =>return a.get_type().into(),
            Value::NULL     =>return "null".into(),
            Value::Bool(_)  =>return "bool".into(),
            Value::Char(_)  =>return "char".into()
        }
    }
    #[inline]
    pub fn bin_encode(&self,lib:&mut Vec<Value>,code:Vec<u8>) -> Result<()> {
        let pos = lib.iter().position(|x|x==self);
        match pos {
            Some(a)=>{
            }
            _=>{

            }
        }
        return Ok(())
    }
    #[inline]
    pub fn as_string(self)->String {
        format!("{}",self)
    }
    #[inline]
    pub fn operable(&self)->bool {
        match self {
            Value::String(_)|Value::Num(_)|Value::Object(_)=>true,
            _=>false
        }
    }

    #[inline]
    fn _add(one:&Value, other:&Value)->Value {
        match &one {
            Value::String(a)=>{
                let mut a = String::from(a.clone().as_ref());
                a.push_str(&other.clone().as_string());
                Value::String(ZString::from(a))
            }
           
            Value::Num(a)=> match other {
                Value::Num(c)=>Value::Num(*a+*c),
                _=>{
                    let mut s = one.to_string();
                    s.push_str(&a.to_string());
                    Value::String(ZString::from(s))
                }
            }
            _=>one.clone()
        }
    }
    #[inline]
    pub fn add(self,b:Value)->Value {
        Self::_add(&self,&b)
    }
    #[inline]
    pub fn sub(self,b:Value)->Value {
        match self {
            Value::Num(a)=> match b {
                Value::Num(c)=>Value::Num(a-c),
                _=>Value::NULL
            }
            _=>Value::NULL
        }
    }

    #[inline]
    pub fn shr(self,b:Value)->Value {
        match self {
            Value::Num(a)=> match b {
                Value::Num(c)=>Value::Num(a>>c),
                _=>Value::Num(a)
            }
            _=>Value::NULL
        }
    }
    #[inline]
    pub fn shl(self,b:Value)->Value {
        match self {
            Value::Num(a)=> match b {
                Value::Num(c)=>Value::Num(a<<c),
                _=>Value::Num(a)
            }
            _=>Value::NULL
        }
    }
    #[inline]
    pub fn mul(self,b:Value)->Value {
        match self {
            Value::Num(a)=> match b {
                Value::Num(c)=>Value::Num(a*c),
                _=>Value::Num(a)
            }
            _=>Value::NULL
        }
    }
    #[inline]
    pub fn div(self,b:Value)->Value {
        match self {
            Value::Num(a)=> match b {
                Value::Num(c)=>match a.try_div(c) {Some(a)=>Value::Num(a),_=>Value::NULL},
                _=>Value::NULL
            }
            _=>Value::NULL
        }
    }
    #[inline]
    pub fn exp(self,b:Value)->Value {
        match self {
            Value::Num(a)=> match b {
                Value::Num(c)=>match a.try_div(c) {Some(a)=>Value::Num(a),_=>Value::NULL},
                _=>Value::NULL
            }
            _=>Value::NULL
        }
    }
    #[inline]
    pub fn as_bool(&self)->bool {
        match self {
            Value::NULL=>false,
            Value::String(a)=> a.len()>0,
            Value::Num(a)=>a.as_bool(),
            Value::Bool(a)=>*a,
            _=>false
        }
    }
    #[inline]
    pub fn is_ptr(&self) -> bool {
        match self {
            Value::String(_)|Value::Object(_)=>true,
            _=>false
        }
    }
    #[inline]
    pub fn eqval(&self,other:Self) -> Self {
        (self == &other).into()
    }
    #[inline]
    pub fn neqval(&self,other:Self) -> Self {
        (self != &other).into()
    }
    #[inline]
    pub fn noop(&self,other:Self) -> Self {
        Value::NULL
    }
    #[inline]
    pub fn le(&self,other:Self) -> Self {
        (self < &other).into()
    }
    #[inline]
    pub fn leq(&self,other:Self) -> Self {
        (self <= &other).into()
    }
    #[inline]
    pub fn gre(&self,other:Self) -> Self {
        (self > &other).into()
    }
    #[inline]
    pub fn greq(&self,other:Self) -> Self {
        (self >= &other).into()
    }
    #[inline]
    pub fn is_null(&self) -> bool {
        match self {
            Value::NULL=>true,
            _=>false
        }
    }
    #[inline]
    pub fn invoke_to_string(&self,exec:&crate::object::interpreter::Executor)->ZString {
        match self {
            Value::String(a)=>a.clone(),
            Value::Object(a)=>crate::object::zstd::builtin::zstring::invoke_to_string(exec,&a).unwrap_or(ZString::from("")),
            _=>self.to_string().into()
        }
    }

    #[inline]
    pub fn try_invoke_to_string(&self,exec:&crate::object::interpreter::Executor)->Result<ZString> {
        match self {
            Value::String(a)=>Ok(a.clone()),
            Value::Object(a)=>
            Ok(exec.call_method(a.clone(),
            crate::object::zstd::builtin::property::accessors::__str(),Vec::with_capacity(0))?.as_string().into()),
            _=>Ok(self.to_string().into())
        }
    }
    #[inline]
    pub fn invoke_to_bytes(&self,exec:&crate::object::interpreter::Executor)->ZBytes {
        match self {
            Value::String(a)=>a.as_bytes().into(),
            Value::Bytes(a)=>a.clone(),
            Value::Object(a)=>
            crate::object::zstd::builtin::zstring::invoke_to_bytes(exec,&a),
            _=>self.to_string().as_bytes().into()
        }
    }
   
}
impl From<Number> for Value {
    #[inline]
    fn from(a:Number)->Value {
        Value::Num(a)
    }
}

impl From<&str> for Value {
    #[inline]
    fn from(a:&str)->Value {
        Value::String(ZString::from(a))
    }
}

impl From<Object> for Value {
    #[inline]
    fn from(a:Object)->Value {
        Value::Object(GcObject::from(a))
    }
}
impl From<GcObject> for Value {
    #[inline]
    fn from(a:GcObject)->Value {
        Value::Object(a)
    }
}

impl From<Function> for Value {
    #[inline]
    fn from(a:Function)->Value {
        Value::Object(a.inner)
    }
}

impl From<ZString> for Value {
    #[inline]
    fn from(a:ZString)->Value {
        Value::String(a)
    }
}
impl From<String> for Value {
    #[inline]
    fn from(a:String)->Value {
        Value::String(ZString::from(a))
    }
}

impl Into<Value> for crate::object::NativeFunction {
    fn into(self)->Value {
        Value::Object(Function::native_root(crate::object::access::ObjAccess::NONE, self).inner)
    }
}

use std::convert::TryFrom;
impl TryFrom<Value> for i8 {
    type Error = Error;
    #[inline]
    fn try_from(value: Value) -> Result<Self> {
       match value {
           Value::Num(a)=>Ok(a.i8()),
           _=>Err(Error::Unconvertable(value.ztype(),"i8".into()))
       }
    }
}
impl TryFrom<Value> for i16 {
    type Error = Error;
    #[inline]
    fn try_from(value: Value) -> Result<Self> {
       match value {
           Value::Num(a)=>Ok(a.i16()),
           _=>Err(Error::Unconvertable(value.ztype(),"i16".into()))
       }
    }
}
impl TryFrom<Value> for i32 {
    type Error = Error;
    #[inline]
    fn try_from(value: Value) -> Result<Self> {
       match value {
           Value::Num(a)=>Ok(a.i32()),
           _=>Err(Error::Unconvertable(value.ztype(),"i32".into()))
       }
    }
}
impl TryFrom<Value> for i64 {
    type Error = Error;
    #[inline]
    fn try_from(value: Value) -> Result<Self> {
       match value {
           Value::Num(a)=>Ok(a.i64()),
           _=>Err(Error::Unconvertable(value.ztype(),"i64".into()))
       }
    }
}
impl TryFrom<Value> for u8 {
    type Error = Error;
    #[inline]
    fn try_from(value: Value) -> Result<Self> {
       match value {
           Value::Num(a)=>Ok(a.u8()),
           _=>Err(Error::Unconvertable(value.ztype(),"u8".into()))
       }
    }
}
impl TryFrom<Value> for u16 {
    type Error = Error;
    #[inline]
    fn try_from(value: Value) -> Result<Self> {
       match value {
           Value::Num(a)=>Ok(a.u16()),
           _=>Err(Error::Unconvertable(value.ztype(),"u16".into()))
       }
    }
}
impl TryFrom<Value> for u32 {
    type Error = Error;
    #[inline]
    fn try_from(value: Value) -> Result<Self> {
       match value {
           Value::Num(a)=>Ok(a.u32()),
           _=>Err(Error::Unconvertable(value.ztype(),"u32".into()))
       }
    }
}
impl TryFrom<Value> for Number {
    type Error = Error;
    #[inline]
    fn try_from(value: Value) -> Result<Self> {
       match value {
           Value::Num(a)=>Ok(a),
           Value::NULL=>Ok(0.into()),
           _=>Err(Error::Unconvertable(value.ztype(),"number".into()))
       }
    }
}
impl TryFrom<Value> for u64 {
    type Error = Error;
    #[inline]
    fn try_from(value: Value) -> Result<Self> {
       match value {
           Value::Num(a)=>Ok(a.u64()),
           _=>Err(Error::Unconvertable(value.ztype(),"u64".into()))
       }
    }
}

impl TryFrom<Value> for ZString {
    type Error = Error;
    #[inline]
    fn try_from(value: Value) -> Result<Self> {
       match &value {
           Value::String(a)=>Ok(a.clone()),
           _=>Err(Error::Unconvertable(value.ztype(),"str".into()))
       }
    }
}

impl TryFrom<Value> for GcObject {
    type Error = Error;
    #[inline]
    fn try_from(value: Value) -> Result<Self> {
       match &value {
           Value::Object(a)=>Ok(a.clone()),
           _=>Err(Error::Unconvertable(value.ztype(),"object".into()))
       }
    }
}
impl TryFrom<Value> for Function {
    type Error = Error;
    #[inline]
    fn try_from(value: Value) -> Result<Self> {
       match &value {
           Value::Object(a)=>Function::envelop(a.clone()),
           _=>Err(Error::Unconvertable(value.ztype(),"function".into()))
       }
    }
}
impl From<isize> for Value {
    #[inline]
    fn from(a: isize) -> Value {
        Value::Num(Number::from(a))
    }
}
impl From<ZBytes> for Value {
    #[inline]
    fn from(a: ZBytes) -> Value {
        Value::Bytes(a)
    }
}
impl From<usize> for Value {
    #[inline]
    fn from(a: usize) -> Value {
        Value::Num(Number::from(a))
    }
}
impl From<bool> for Value {
    #[inline]
    fn from(a: bool) -> Value {
        Value::Bool(a)
    }
}
// ($trait:expr,$trait_func:expr,$sself:expr,$from_numbers:expr)
impl_numbers_with_generics!(From,from,Value,Num);
/*
impl From<i8> for Value {
    #[inline]
    fn from(a:i8)->Value {
        Value::Num(Number::SNum(Signed::I8(a)))
    }
}
impl From<i16> for Value {
    #[inline]
    fn from(a:i16)->Value {
        Value::Num(Number::SNum(Signed::I16(a)))
    }
}
impl From<i32> for Value {
    #[inline]
    fn from(a:i32)->Value {
        Value::Num(Number::SNum(Signed::I32(a)))
    }
}
impl From<i64> for Value {
    #[inline]
    fn from(a:i64)->Value {
        Value::Num(Number::SNum(Signed::I64(a)))
    }
}
impl From<u8> for Value {
    #[inline]
    fn from(a:u8)->Value {
        Value::Num(Number::UNum(Unsigned::U8(a)))
    }
}
impl From<u16> for Value {
    #[inline]
    fn from(a:u16)->Value {
        Value::Num(Number::UNum(Unsigned::U16(a)))
    }
}
impl From<u32> for Value {
    #[inline]
    fn from(a:u32)->Value {
        Value::Num(Number::UNum(Unsigned::U32(a)))
    }
}
impl From<u64> for Value {
    #[inline]
    fn from(a:u64)->Value {
        Value::Num(Number::UNum(Unsigned::U64(a)))
    }
}
*/
