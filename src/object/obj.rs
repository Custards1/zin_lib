use crate::types::*;
use super::access::*;
use std::convert::{TryFrom,TryInto};
use crate::error::*;
use gc::{Gc,GcCell,Trace,Finalize};
pub type ObjectId = u64;

pub type PropertyMap =  HashMap<Value,Value>;
use super::ZString;

use std::convert::AsRef;

pub type NativeFunction = fn (Executor,Vec<Value>) -> Result<Value>;

pub type NativeMethod = NativeFunction;
use super::interpreter::Executor;


#[derive(Debug,Clone,PartialEq,Eq,Trace,Finalize)]
pub struct InnerFunction {
    pub (crate) code:ObjFunction,
}
#[derive(Debug,Clone,PartialEq,Eq,Trace,Finalize)]
pub enum ObjFunction {
    #[unsafe_ignore_trace]
    Native(NativeFunction),
    Zin(InstructionSlice),
}


impl From<NativeFunction> for ObjFunction {
    #[inline]
    fn from(e: NativeFunction) ->ObjFunction { 
        ObjFunction::Native(e)
    }
}
impl From<InstructionSlice> for ObjFunction {
    #[inline]
    fn from(e: InstructionSlice) ->ObjFunction { 
        ObjFunction::Zin(e)
    }
}
impl From<Vec<Instruction>> for ObjFunction {
    #[inline]
    fn from(e: Vec<Instruction>) ->ObjFunction { 
        ObjFunction::Zin(gc::GcCell::new(e))
    }
}
#[derive(Debug,Clone,Eq,Trace,Finalize,Hash)]
pub enum InternedString {
    String(String),
    Str(ZString)
}
impl InternedString {
    #[inline]
    pub fn own(&mut self) {
        match self {
            InternedString::Str(a)=>{
                let mut b = String::new();
                b.push_str(&a);
                *self = InternedString::String(b);

            },
            _=>{}
        }
    }
    pub fn join(&self,exec:&Executor,iter:&[&Value])->Value {
        let a:&str = self.as_str();
        let len = iter.len(); 
        match len {
            0|1=>{
                super::zstd::builtin::string_from(a).into()
            }
            _=>{
                let mut new = String::new();
                for i in 0..len-2 {
                    new.push_str(&iter[i].invoke_to_string(exec));
                    new.push_str(&a);
                }
                new.push_str(&iter[len-1].invoke_to_string(exec));
                super::zstd::builtin::string_from(new).into()
            }
        }
    }
}
impl std::ops::Deref for InternedString {
    type Target = str;
    #[inline]
    fn deref(&self) -> &Self::Target {
        match &self {
            InternedString::String(a)=>{
                &a
            }
            InternedString::Str(a)=> {
                &a
            }
        }
    }
}
impl std::ops::DerefMut for InternedString {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.own();
        match self {
            InternedString::String(ref mut a)=>{
                a
            }
            InternedString::Str(a)=> {
                impossible!();
            }
        }
    }
}
impl PartialEq for InternedString {
    #[inline]
    fn eq(&self, other: &InternedString) ->bool {
        self.as_str() == other.as_str()
    }
}
impl PartialOrd for InternedString {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.as_str().cmp(other.as_str()))
    }
}
impl Ord for InternedString {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}
impl InternedString {
    #[inline]
    pub fn as_str(&self) -> &str {
        match &self {
            InternedString::String(a)=>a.as_str(),
            InternedString::Str(a)=> &a
        }
    }
    #[inline]
    pub fn as_zstr(&self) -> ZString {
        match &self {
            InternedString::String(a)=>a.as_str().into(),
            InternedString::Str(a)=> a.clone()
        }
    }
    #[inline]
    pub fn push<K:Into<InternedString>>(&mut self,value:K) {
        match self {
            InternedString::String(ref mut a)=>{
                a.push_str(&value.into())
            }
            InternedString::Str(a)=> {
                let a:&str = &a;
                let mut b = String::from(a);
                b.push_str(value.into().as_str());
                *self = InternedString::String(b);
            }
        }
    }
    #[inline]
    pub fn pop(&mut self)->Option<char> {
        match self {
            InternedString::String(ref mut a)=>{
                a.pop()
            }
            InternedString::Str(a)=> {
                let a:&str = &a;
                let mut b = String::from(a);
                let a= b.pop();
                *self = InternedString::String(b);
                a
            }
        }
    }
    #[inline]
    pub fn len(&self)->usize {
        match &self {
            InternedString::String(a)=>{
                a.len()
            }
            InternedString::Str(a)=> {
                a.len()
            }
        }
    }
    #[inline]
    pub fn get<Key:Into<SetIndexValue>>(&self,index:Key) -> Option<Value> {
        let key = index.into();
        match &self {
            InternedString::String(a)=>{
                match key {
                    SetIndexValue::Index(v)=>{
                        a.get(v..v+1).and_then(|x|{Some((*x).into())})
                    }
                    SetIndexValue::Range(r,v)=>{
                        let len = a.len();
                        
                        if r < len {
                            Some(if v < len {
                               Value::String(ZString::from(&a[r..v])) 
                            } else {
                               Value::String(ZString::from(&a[r..])) 
                            })
                        } else {
                            None
                        }
                    }
                    SetIndexValue::Val(v)=>{
                        if let Ok(num)= Number::try_from(v) {
                            let num = num.usize();
                            a.get(num..num+1).and_then(|x|{Some((*x).into())})
                        } else {
                            None
                        }
                    }
                }
            }
            InternedString::Str(a)=>{
                 match key {
                    SetIndexValue::Index(v)=>{
                        a.get(v..v+1).and_then(|x|{Some((*x).into())})
                    }
                    SetIndexValue::Range(r,v)=>{
                        let len = a.len();
                        
                        if r < len {
                            Some(if v < len {
                               Value::String(ZString::from(&a[r..v])) 
                            } else {
                               Value::String(ZString::from(&a[r..])) 
                            })
                        } else {
                            None
                        }
                    }
                    SetIndexValue::Val(v)=>{
                        if let Ok(num)= Number::try_from(v) {
                            let num = num.usize();
                            a.get(num..num+1).and_then(|x|{Some((*x).into())})
                        } else {
                            None
                        }
                    }
                }
            }
        }
    }
    #[inline]
    pub fn set<T:Into<InternedString>>(&mut self,index:std::ops::Range<usize>,val:T) {
        self.own();
        match self {
            InternedString::String(ref mut a)=>{
                let len = a.len();
                if index.start < len && a.is_char_boundary(index.start) {
                    if index.end < len && a.is_char_boundary(index.end) {
                        a.replace_range(index, val.into().as_str())  
                    } else {
                        a.replace_range(index.start.., val.into().as_str())  
                    }
                }
            
            }
            InternedString::Str(ref mut a)=>{
                impossible!();
            }
        }
    }
    pub fn remove<Key:Into<SetIndexValue>>(&mut self,key:Key) ->Option<Value> {
        self.own();
        match self {
            InternedString::String(ref mut a)=>{
                let key = key.into();
                match key {
                    SetIndexValue::Index(x)=>{
                         if a.is_char_boundary(x) && x < a.len() {
                            Some((a.remove(x) as u32).into())
                         } else {
                            Some(Value::Bool(false))
                         }
                        
                    }
                    SetIndexValue::Range(r,v)=>{
                        if a.is_char_boundary(r) && r < a.len() {
                            if a.is_char_boundary(v) && v <a.len(){
                                a.replace_range(r..v,"");
                                Some(Value::Bool(true))
                            } else {
                                a.replace_range(r..,"");
                                Some(Value::Bool(true))
                            }
                            
                        } else {
                            Some(Value::Bool(false))
                        }
                    }
                    SetIndexValue::Val(v)=>{
                        if let Ok(x)= Number::try_from(v) {
                            let x = x.usize();
                            if a.is_char_boundary(x) && x < a.len() {
                                Some((a.remove(x) as u32).into())
                             } else {
                                Some(Value::Bool(false))
                             }
                        } else {
                            None
                        }
                    }
                }
            }
            InternedString::Str(ref mut a)=>{
                impossible!();
            }
        }
    }

}
impl std::fmt::Display for InternedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match &self {
           InternedString::String(a)=>write!(f,"{}",a),
           InternedString::Str(a)=>write!(f, "{}",a.as_ref())
       }
    }
}

impl From<ZString> for InternedString {
    #[inline]
    fn from(s: ZString) -> InternedString {
        InternedString::Str(s)
    }
}
impl From<&str> for InternedString {
    #[inline]
    fn from(s: &str) -> InternedString {
        InternedString::Str(ZString::from(s))
    }
}
impl From<char> for InternedString {
    #[inline]
    fn from(s: char) -> InternedString {
        InternedString::Str(ZString::from(s.to_string().as_str()))
    }
}
impl From<String> for InternedString {
    #[inline]
    fn from(s: String) -> InternedString {
        InternedString::String(s)
    }
}

#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord,Trace,Finalize,Hash)]
pub enum InternedVector {
    Vector(Vec<Value>),
    ByteVector(Vec<u8>)
}
impl From<Vec<Value>> for InternedVector {
    #[inline]
    fn from(vec: Vec<Value>)->InternedVector {
        InternedVector::Vector(vec)
    }
}
impl From<Vec<u8>> for InternedVector {
    #[inline]
    fn from(vec: Vec<u8>)->InternedVector {
        InternedVector::ByteVector(vec)
    }
}
macro_rules! push_bytes {
    ($vec:ident, $val:ident) => {
        match &$val {
            Value::Bytes(bytes) => {
                $vec.extend(bytes.as_ref());
            }
            Value::String(bytes) => {
                $vec.extend(bytes.as_bytes());
            }
            Value::Num(bytes)=>{
                let bytes = bytes.u8();
                $vec.push(bytes);
            }
            Value::NULL=>{
                $vec.push(0);
            }
            _=>{}
        }
    };
}


#[inline(always)]
fn push_bytechar(slice:&mut Vec<u8>,zh:char) {
    let ch = zh as u32;
    match zh.len_utf8() {
        1=>slice.push(ch as u8),
        2=>{
            slice.push((ch&0x000F) as u8);
            slice.push( ((ch>>8) & 0x000F) as u8);
        },
        3=>{
            slice.push((ch&0x000F) as u8);
            slice.push( ((ch>>8) & 0x000F) as u8);
            slice.push( ((ch>>16) & 0x000F) as u8);
        }
        _=>{
            slice.push((ch&0x000F) as u8);
            slice.push( ((ch>>8) & 0x000F) as u8);
            slice.push( ((ch>>16) & 0x000F) as u8);
            slice.push( ((ch>>24) & 0x000F) as u8);
        }

    }
}
impl InternedVector {
    #[inline]
    pub fn push(&mut self,exec:&Executor,value:Value) ->Result<()>{
        match self {
            InternedVector::Vector(ref mut a)=>{
                Ok(a.push(value))
            }
            InternedVector::ByteVector(ref mut a)=> {
                match &value {
                    Value::Bytes(bytes) => {
                        a.extend(bytes.as_ref());
                    }
                    Value::String(bytes) => {
                        a.extend(bytes.as_bytes());
                    }
                    Value::Num(bytes)=>{
                        let bytes = bytes.u8();
                        a.push(bytes);
                    }
                    Value::NULL=>{
                        a.push(0);
                    }
                    Value::Char(bytes)=>{
                        push_bytechar(a, *bytes);
                    }

                    Value::Bool(b)=>{
                        a.push(if *b ==true {1} else {0});
                    }
                    Value::Object(bytes)=>{
                        a.extend_from_slice(&zstd::builtin::zstring::invoke_to_bytes(exec, bytes))
                    }
                }
                Ok(())
            }
        }
    }
    #[inline]
    pub fn join(&self,exec:&Executor,iter:&[&Value])->Value {
        match &self {
            InternedVector::ByteVector(a)=>{
                let a:&[u8] = &a;
                let len = iter.len(); 
                match len {
                    0|1=>{
                        super::zstd::builtin::byte_vec_from(a).into()
                    }
                    _=>{
                        let mut new = Vec::new();
                        for i in 0..len-2 {
                           let x =iter[i].invoke_to_bytes(exec);
                           let x :&[u8] = x.as_ref();
                           new.extend_from_slice(x);
                           new.extend_from_slice(&a);
                        }
                        let x =iter[len-1].invoke_to_bytes(exec);
                        let x :&[u8] = x.as_ref();
                        new.extend_from_slice(x);
                        super::zstd::builtin::vec_from(new).into()
                    }
                }
            }
            InternedVector::Vector(a)=> {
                let a:&[Value] = &a;
                let len = iter.len(); 
                match len {
                    0|1=>{
                        super::zstd::builtin::vec_from(Vec::from(a)).into()
                    }
                    _=>{
                        let mut new = Vec::new();
                        for i in 0..len-2 {
                           new.push(iter[i].clone());
                           new.extend_from_slice(a);
                        }
                        new.push(iter[len-1].clone());
                        super::zstd::builtin::vec_from(new).into()
                    }
                }
            }
        }
        
       
    }
    #[inline]
    pub fn pop(&mut self)->Option<Value> {
        match self {
            InternedVector::Vector(ref mut a)=>{
                a.pop()
            }
            InternedVector::ByteVector(ref mut a)=> {
                Some(a.pop()?.into())            
            }
        }
    }
    #[inline]
    pub fn len(&self)->usize {
        match &self {
            InternedVector::Vector(a)=>{
                a.len()
            }
            InternedVector::ByteVector(a)=> {
               a.len()           
            }
        }
    }
    #[inline]
    pub fn get<Key:Into<SetIndexValue>>(&self,key:Key) -> Option<Value> {
        let key = key.into();
        match &self {
            InternedVector::Vector(a)=>{
                match key {
                    SetIndexValue::Index(v) =>{
                        a.get(v).and_then(|x|{Some(x.clone())})
                    }
                    SetIndexValue::Range(r,v)=>{
                        let len = a.len();
                        
                        if r < len {
                            Some(if v < len {
                               super::zstd::builtin::vec_from(Vec::from(&a[r..v])).into() 
                            } else {
                                super::zstd::builtin::vec_from(Vec::from(&a[r..])).into()
                            })
                        } else {
                            None
                        }
                    }
                    SetIndexValue::Val(v)=>{
                        if let Ok(num)= Number::try_from(v) {
                            a.get(num.usize()).and_then(|x|{Some(x.clone())})
                        } else {
                            None
                        }
                        
                    }
                }
                
            }
            InternedVector::ByteVector(a)=> {
                match key {
                    SetIndexValue::Index(v)=>{
                        a.get(v).and_then(|x|{Some((*x).into())})
                    }
                    SetIndexValue::Range(r,v)=>{
                        let len = a.len();
                        
                        if r < len {
                            Some(if v < len {
                               Value::Bytes(ZBytes::from(&a[r..v])) 
                            } else {
                               Value::Bytes(ZBytes::from(&a[r..])) 
                            })
                        } else {
                            None
                        }
                    }
                    SetIndexValue::Val(v)=>{
                        if let Ok(num)= Number::try_from(v) {
                            a.get(num.usize()).and_then(|x|{Some((*x).into())})
                        } else {
                            None
                        }
                    }
                }
                   
            }
        }
    }
    pub fn remove<Key:Into<SetIndexValue>>(&mut self,key:Key) ->Option<Value> {
        let key = key.into();
        match self {
            InternedVector::Vector(ref mut a)=>{
                
                match key {
                    SetIndexValue::Index(x)=>{
                         if x < a.len() {
                            Some(a.remove(x))
                         } else {
                            Some(Value::NULL)
                         }
                        
                    }
                    SetIndexValue::Range(r,v)=>{
                        if r < a.len() {
                            if v <a.len() {
                                if r >v {
                                    a.drain(v..r);
                                } else {
                                    a.drain(r..v);
                                }
                                
                                Some(Value::NULL)
                            } else {
                                a.drain(r..);
                                Some(Value::NULL)
                            }
                            
                        } else {
                            Some(Value::NULL)
                        }
                    }
                    SetIndexValue::Val(v)=>{
                        if let Ok(x)= Number::try_from(v) {
                            let x = x.usize();
                            if  x < a.len() {
                                Some(a.remove(x))
                             } else {
                                Some(Value::NULL)
                             }
                        } else {
                            None
                        }
                    }
                }
            }
            InternedVector::ByteVector(ref mut a)=>{
                match key {
                    SetIndexValue::Index(x)=>{
                         if x < a.len() {
                            Some(a.remove(x).into())
                         } else {
                            Some(Value::NULL)
                         }
                        
                    }
                    SetIndexValue::Range(r,v)=>{
                        if r < a.len() {
                            if v <a.len() {
                                if r >v {
                                    a.drain(v..r);
                                } else {
                                    a.drain(r..v);
                                }
                                
                                Some(Value::NULL)
                            } else {
                                a.drain(r..);
                                Some(Value::NULL)
                            }
                            
                        } else {
                            Some(Value::NULL)
                        }
                    }
                    SetIndexValue::Val(v)=>{
                        if let Ok(x)= Number::try_from(v) {
                            let x = x.usize();
                            if  x < a.len() {
                                Some(a.remove(x).into())
                             } else {
                                Some(Value::NULL)
                             }
                        } else {
                            None
                        }
                    }
                }
            }
        }
    }
    #[inline]
    pub fn set<T:Into<Value>>(&mut self,index:usize,val:T) {
        match self {
            InternedVector::Vector(ref mut a)=>{
                let len = a.len();
                if index < len {
                    a[index] = val.into();
                }
            }
            InternedVector::ByteVector(ref mut a)=> {
                if index < a.len() {
                    let aa = val.into();
                    match aa {
                        Value::Num(m)=>{
                            a[index] = m.u8();
                        }
                        Value::NULL=>{
                            a[index] = 0;
                        }
                        _=>{}
                    }
                }   
            }
        }
    }
}

#[derive(Debug,Default)]
pub struct OptionalFile(pub Option<std::fs::File>);
impl OptionalFile {
    pub fn new(file:std::fs::File) -> Self {
        Self(Some(file))
    }
    pub fn close(&mut self) {
        self.0 =None;
    }
}

impl std::io::Read for OptionalFile {
    fn read(&mut self, buf: &mut [u8])->std::io::Result<usize> {
        match &mut self.0 {
            Some(a)=>a.read(buf),
            _=>{
                Ok(0)
            }
        }
    }
}
impl std::io::Seek for OptionalFile {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        match &mut self.0 {
            Some(a)=>a.seek(pos),
            _=>{
                Ok(0)
            }
        }
    }
}
impl std::io::Write for OptionalFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>{
        match &mut self.0 {
            Some(a)=>a.write(buf),
            _=>{
                Ok(0)
            }
        }
    }
    fn flush(&mut self) -> std::io::Result<()>{
        match &mut self.0 {
            Some(a)=>a.flush(),
            _=>{
                Ok(())
            }
        }
    }
}


#[derive(Debug,Clone,Trace,Finalize)]
pub struct RFile(#[unsafe_ignore_trace]
    pub std::rc::Rc<std::cell::RefCell<OptionalFile>> );

impl From<std::rc::Rc<std::cell::RefCell<OptionalFile>>> for RFile {
    #[inline]
    fn from(f:std::rc::Rc<std::cell::RefCell<OptionalFile>>)->RFile {
        RFile(f)
    }
}
impl RFile {
    pub fn new(f: std::fs::File) -> RFile {
        RFile(std::rc::Rc::new(std::cell::RefCell::new(OptionalFile::new(f))))
    }
    pub fn close(&mut self) {
        match self.0.try_borrow_mut() {
            Ok(mut a)=>a.close(),
            _=>{
            }
        }
    }
}
impl PartialEq for RFile {
    fn eq(&self, other: &Self)-> bool {
        std::ptr::eq(&self,&other)
    }
}
impl Eq for RFile{}

impl std::io::Read for RFile {
    fn read(&mut self, buf: &mut [u8])->std::io::Result<usize> {
        match self.0.try_borrow_mut() {
            Ok(mut a)=>a.read(buf),
            _=>{
                Ok(0)
            }
        }
    }
}
impl std::io::Seek for RFile {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        match self.0.try_borrow_mut() {
            Ok(mut a)=>a.seek(pos),
            _=>{
                Ok(0)
            }
        }
    }
}
impl std::io::Write for RFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>{
        match self.0.try_borrow_mut() {
            Ok(mut a)=>a.write(buf),
            _=>{
                Ok(0)
            }
        }
    }
    fn flush(&mut self) -> std::io::Result<()>{
        match self.0.try_borrow_mut() {
            Ok(mut a)=>a.flush(),
            _=>{
                Ok(())
            }
        }
    }
}

#[derive(Debug,Clone,PartialEq,Eq,Trace,Finalize)]
pub enum ZFile {
    Stdin,
    Stdout,
    Stderr,
    File(RFile)
}
impl ZFile {
    pub fn close(&mut self) {
        match self {
            ZFile::File(a)=>a.close(),
            _=>{}
        }
    }
}
impl std::io::Read for ZFile {
    fn read(&mut self, buf: &mut [u8])->std::io::Result<usize> {
        match self {
            ZFile::Stdin =>std::io::stdin().lock().read(buf),
            ZFile::Stdout =>Ok(0),
            ZFile::Stderr =>Ok(0),
            ZFile::File(ref mut f) =>f.read(buf),
        }
    }
}
impl std::io::Seek for ZFile {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        match self {
            ZFile::Stdin =>Ok(0),
            ZFile::Stdout =>Ok(0),
            ZFile::Stderr =>Ok(0),
            ZFile::File(ref mut f) =>f.seek(pos),
        }
    }
}
impl std::io::Write for ZFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>{
        match self {
            ZFile::Stdin =>Ok(0),
            ZFile::Stdout =>std::io::stdout().write(buf),
            ZFile::Stderr =>std::io::stderr().write(buf),
            ZFile::File(ref mut f) =>f.write(buf),
        }
    }
    fn flush(&mut self) -> std::io::Result<()>{
        match self {
            ZFile::Stdin =>Ok(()),
            ZFile::Stdout =>std::io::stdout().flush(),
            ZFile::Stderr =>std::io::stderr().flush(),
            ZFile::File(ref mut f) =>f.flush(),
        }
    }
}


#[derive(Debug,Clone,PartialEq,Eq, Trace,Finalize)]
pub enum InnerContainer {
    String(InternedString),
    Vector(InternedVector),
    Map(PropertyMap),
    Function(InnerFunction),
    File(ZFile),
}
impl InnerContainer {
    #[inline]
    pub const fn is_string(&self) -> bool {
        match &self {
            InnerContainer::String(_)=>true,
            _=>false
        }
    }
    #[inline]
    pub const fn is_vec(&self) -> bool {
        match &self {
            InnerContainer::Vector(_)=>true,
            _=>false
        }
    }
    #[inline]
    pub const fn is_map(&self) -> bool {
        match &self {
            InnerContainer::Map(_)=>true,
            _=>false
        }
    }
    #[inline]
    pub const fn is_function(&self) -> bool {
        match &self {
            InnerContainer::Function(_)=>true,
            _=>false
        }
    }
    #[inline]
    pub fn push(&mut self,exec:&Executor, value:Value)-> bool {
        match self {
            InnerContainer::String(ref mut a)=>{
                a.push(value.invoke_to_string(exec));
                true
            }
            InnerContainer::Vector(ref mut a)=>{
                let _ = a.push(exec,value);
                true
            }
            _=>false
        }
    }
    #[inline]
    pub fn pop(&mut self)-> Value {
        match self {
            InnerContainer::String(ref mut a)=>{
                match a.pop() {
                    Some(a)=>(a as u32).into(),
                    _=>Value::NULL
                }
            }
            InnerContainer::Vector(ref mut a)=>{
                a.pop().unwrap_or(Value::NULL)
            }
            _=>Value::NULL
        }
    }
    #[inline]
    pub fn len(&self) -> usize {
        match &self {
            InnerContainer::String(a)=>a.len(),
            InnerContainer::Vector(a)=>a.len(),
            InnerContainer::Map(a)=>a.len(),
            _=>0
        }
    }
    pub fn get<Key:Into<SetIndexValue>>(&self,key:Key)-> Option<Value> {
        match &self {
            InnerContainer::String(a)=>{
                a.get(key)
            },
            InnerContainer::Vector(a)=>{
                a.get(key)
            },
            InnerContainer::Map(a)=> {
                match key.into() {
                    SetIndexValue::Val(nau)=>{
                        a.get(&nau).and_then(|x|{Some(x.clone())})
                    }
                    SetIndexValue::Index(nau)=>{
                        let val:Value = nau.into();
                        a.get(&val).and_then(|x|{Some(x.clone())})
                    }
                    _=>None

                }
            }
            _=>None
        }
    }
    pub fn get_index<Key:Into<SetIndexValue>>(&self,key:Key)-> Option<Value> {
        match &self {
            InnerContainer::String(a)=>{
                a.get(key)
            },
            InnerContainer::Vector(a)=>{
                a.get(key)
            },
            InnerContainer::Map(a)=> {
                match key.into() {
                    SetIndexValue::Val(nau)=>{
                        if let Ok(nau)= Number::try_from(nau) {
                            let (w,aa) =  a.get_index(nau.usize())?;
                            Some(super::zstd::builtin::vec_from(vec![w.clone(),aa.clone()]).into())
                        } else {
                            None
                        }
                    }
                    SetIndexValue::Index(nau)=>{
                        let (w,aa) =  a.get_index(nau)?;
                        Some(super::zstd::builtin::vec_from(vec![w.clone(),aa.clone()]).into())
                    }
                    _=>None

                }
            }
            _=>None
        }
    }
    pub fn remove<Key:Into<SetIndexValue>>(&mut self,key:Key)-> Result<Value> {
        match self {
            InnerContainer::String(ref mut a)=>{
                Ok(a.remove(key).unwrap_or(Value::NULL))
            },
            InnerContainer::Vector(ref mut a)=>{
                Ok(a.remove(key).unwrap_or(Value::NULL))
            },
            InnerContainer::Map(ref mut a)=> {
                match key.into() {
                    SetIndexValue::Val(nau)=>{
                        Ok(a.remove(&nau).unwrap_or(Value::NULL))
                    }
                    SetIndexValue::Index(nau)=>{
                        Ok(a.remove(&Value::from(nau)).unwrap_or(Value::NULL))
                    }
                    _=>Ok(Value::NULL)

                }
            }
            _=>Ok(Value::NULL)
        }
    }
    pub fn set<Key:Into<SetIndexValue>>(&mut self,exec:&Executor,key:Key,val:Value) -> Result<()> {
        let key = key.into();
        match self {
            InnerContainer::String(ref mut a)=>{
                match key {
                    SetIndexValue::Index(v)=>{
                        a.set(v..v+1,val.invoke_to_string(exec));
                    }
                    SetIndexValue::Range(r,v)=>{
                        a.set(r..v,val.invoke_to_string(exec));
                    }
                    SetIndexValue::Val(v)=>{
                        let num = Number::try_from(v)?.usize();
                        a.set(num..num+1,val.invoke_to_string(exec));
                    }
                }
               Ok(())
            },
            InnerContainer::Vector(ref mut a)=>{
                match key {
                    SetIndexValue::Index(v)=>{
                        a.set(v,val);
                    }
                    SetIndexValue::Range(r,v)=>{
                        a.set(r,val);
                        //todo
                    }
                    SetIndexValue::Val(v)=>{
                        let num = Number::try_from(v)?.usize();
                        a.set(num,val);
                    }
                }
               Ok(())
            },
            InnerContainer::Map(ref mut a)=> {
                match key {
                    SetIndexValue::Index(v)=>{
                        a.insert(v.into(),val);
                    }
                    SetIndexValue::Range(r,v)=>{
                        a.insert(r.into(),val);
                    }
                    SetIndexValue::Val(v)=>{
                        a.insert(v,val);
                    }
                }
               Ok(())
            }
            _=>{
                Ok(())
            }
        }
    }
    #[inline]
    pub fn join(&self,exec:&Executor,iter:&[&Value])->Value {
        match &self {
            InnerContainer::String(a)=>{
                a.join(exec,iter)
            }
            InnerContainer::Vector(a)=>{
                a.join(exec,iter)
            }
            _=>Value::NULL
        }
    }
    #[inline]
    pub fn zstr_eq(&self,exec:&Executor,iter:&[&Value])->bool {
        match &self {
            InnerContainer::String(a)=>{
                let a:&str = a.as_str();
                for i in iter {
                    let b:&str = &i.invoke_to_string(exec);
                    if a != b {
                        return false
                    }
                }
                true
            }
            InnerContainer::Vector(InternedVector::ByteVector(a))=>{
                let a:&[u8] = &a;
                for i in iter {
                    let b:&[u8] = &i.invoke_to_bytes(exec);
                    if a != b {
                        return false
                    }
                }
                true
            }
            _=>false
        }
    }
    #[inline]
    pub fn __str(&self,exec:&Executor,mself:&GcObject)->ZString {
        match &self {
            InnerContainer::String(a)=>a.as_zstr(),
            InnerContainer::Vector(a)=>{
                let mut seen = vec![mself.clone()];
                let len = a.len();
                let mut out = String::from("[");
                for i in 0..len {
                    let n = match a.get(i){
                        Some(a)=>a,
                        _=>break
                    };
                    match &n {
                        Value::String(a)=>{
                            out.push_str(&a);
                        },
                        Value::Bytes(a)=>{
                            match std::str::from_utf8(&a) {
                                Ok(a)=>{
                                    out.push_str(a);
                                }
                                _=>{}
                            }
                        },
                        Value::Object(a)=>{
                            if !super::zstd::builtin::implication::gc_contains(&seen,a) {
                                seen.push(a.clone());
                                if let Ok(b) =exec.call_method(a.clone(),
                                crate::object::zstd::builtin::property::accessors::__str(),Vec::with_capacity(0)) {
                                    out.push_str(&b.as_string());
                                } else {
                                    out.push_str("0xDEADBEEF");
                                }
                            }
                        }
                        _=>{
                            out.push_str(&n.to_string());
                        }
                    }
                    out.push_str(", ");
                }
                if len > 0 {
                    out.pop();
                    out.pop();
                }
                out.push(']');
                out.into()
            }
            InnerContainer::Map(a)=>{
                let mut seen = vec![mself.clone()];
                let len = a.len();
                let mut out = String::from("[");
                for (i,ia) in &a.0 {
                    match i {
                        Value::String(a)=>{
                            out.push_str(&a);
                        },
                        Value::Bytes(a)=>{
                            match std::str::from_utf8(&a) {
                                Ok(a)=>{
                                    out.push_str(a);
                                }
                                _=>{}
                            }
                        },
                        Value::Object(a)=>{
                            if !super::zstd::builtin::implication::gc_contains(&seen,a) {
                                seen.push(a.clone());
                                if let Ok(b) =exec.call_method(a.clone(),
                                crate::object::zstd::builtin::property::accessors::__str(),Vec::with_capacity(0)) {
                                    out.push_str(&b.as_string());
                                } else {
                                    out.push_str("0xDEADBEEF");
                                }
                            }
                        }
                        _=>{
                            out.push_str(&i.to_string());
                        }
                    }
                    out.push_str(": ");
                    match ia {
                        Value::String(a)=>{
                            out.push_str(&a);
                        },
                        Value::Bytes(a)=>{
                            match std::str::from_utf8(&a) {
                                Ok(a)=>{
                                    out.push_str(a);
                                }
                                _=>{}
                            }
                        },
                        Value::Object(a)=>{
                            if !super::zstd::builtin::implication::gc_contains(&seen,a)  {
                                seen.push(a.clone());
                                if let Ok(b) =exec.call_method(a.clone(),
                                crate::object::zstd::builtin::property::accessors::__str(),Vec::with_capacity(0)) {
                                    out.push_str(&b.as_string());
                                } else {
                                    out.push_str("0xDEADBEEF");
                                }
                            }
                        }
                        _=>{
                            out.push_str(&ia.to_string());
                        }
                    }
                    out.push_str(", ");
                }
                if len > 0 {
                    out.pop();
                    out.pop();
                }
                out.push(']');
                out.into()
            }
            _=>String::from("<file>").into()
        }
    }
}
pub enum SetIndexValue {
    Index(usize),
    Range(usize, usize),
    Val(Value)
}
impl From<ZString> for SetIndexValue {
    fn from(a:ZString)->SetIndexValue {
        SetIndexValue::Val(Value::String(a))
    }
}
impl From<&str> for SetIndexValue {
    fn from(a:&str)->SetIndexValue {
        SetIndexValue::Val(Value::String(a.into()))
    }
}
impl From<Value> for SetIndexValue {
    fn from(a:Value)->SetIndexValue {
        SetIndexValue::Val(a)
    }
}
impl From<(usize, usize)> for SetIndexValue {
    fn from(a:(usize, usize))->SetIndexValue {
        SetIndexValue::Range(a.0,a.1)
    }
}
impl From<std::ops::Range<usize>> for SetIndexValue {
    fn from(a:std::ops::Range<usize>)->SetIndexValue {
        SetIndexValue::Range(a.start,a.end)
    }
}
impl From<usize> for SetIndexValue {
    fn from(a:usize)->SetIndexValue {
        SetIndexValue::Index(a)
    }
}


#[derive(Clone,Trace,Finalize,PartialEq,Eq)]
pub struct Container {
    pub(crate) property:Option<PropertyMap>,
    pub(crate) vec:Option<InnerContainer>,
    pub(crate) is_method:bool,
    #[unsafe_ignore_trace]
    pub(crate) access:ObjAccess,
}
impl std::fmt::Debug for Container {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:p}",self)
    }
}
impl Container {

    #[inline]
    pub fn with_map(map:PropertyMap,access:ObjAccess) -> Container {
        Self {
            property:None,
            vec:Some(InnerContainer::Map(map)),
            is_method:false,
            access:access,
        }
    }
    #[inline]
    pub fn function(&self) ->Option<&InnerFunction> {
        match &self.vec {
            Some(InnerContainer::Function(a))=>Some(&a),
            _=>None,
        }
    }
    #[inline]
    pub fn file(&self) ->Option<&ZFile> {
        match &self.vec {
            Some(InnerContainer::File(a))=>Some(&a),
            _=>None,
        }
    }
    #[inline]
    pub fn with_vec<T:Into<InternedVector>>(map:T,access:ObjAccess) -> Container {
        Self {
            property:None,
            vec:Some(InnerContainer::Vector(map.into())),
            is_method:false,
            access:access,
        }
    }
    #[inline]
    pub fn with_string<T:Into<InternedString>>(map:T,access:ObjAccess) -> Container {
        Self {
            property:None,
            vec:Some(InnerContainer::String(map.into())),
            is_method:false,
            access:access,
        }
    }
    #[inline]
    pub fn access(&self)->ObjAccess {
        self.access
    }
    #[inline]
    pub fn new(access:ObjAccess) -> Container {
        Self {
            property:None,
            vec:None,
            is_method:false,
            access:access,
        }
    }
    #[inline]
    pub fn map(access:ObjAccess)->Self {
        Self{
            property:Some(PropertyMap::new()),
            vec:Some(InnerContainer::Map(PropertyMap::new())),
            is_method:false,
            access:access,
        }
    }
    #[inline]
    pub fn vec(access:ObjAccess)->Self{
        Self {
            property:None,
            vec:Some(InnerContainer::Vector(InternedVector::Vector(Vec::new()))),
            is_method:false,
            access:access,
        }
    }

    #[inline]
    pub fn bytes(access:ObjAccess)->Self{
        Self {
            property:None,
            vec:Some(InnerContainer::Vector(InternedVector::ByteVector(Vec::new()))),
            is_method:false,
            access:access,
        }
    }
    #[inline]
    pub fn force_method(&mut self,is_method:bool){
        self.is_method = is_method;
    }
    #[inline]
    pub(crate) fn member_unchecked(&self,name:Value)->Option<Value> {
        match &self.property {
            Some(map)=>Some(map.get(&name)?.clone()),
            None=>None
        }
    }
    #[inline]
    pub fn __str(&self,exec:&Executor,o:&GcObject)->Option<ZString> {
        match &self.vec {
            Some(a)=>{
               Some(a.__str(exec,o))
            }
            _=>None
        }
    }
    #[inline]
    pub(crate) fn remove_member_unchecked(&mut self,name:Value)->Option<Value> {
        match &mut self.property {
            Some(ref mut map)=>Some(map.remove(&name)?),
            None=>None
        }
    }
    #[inline]
    pub fn member<T:IntoHashable>(&self,name:T)->Option<Value> {
        self.member_unchecked(name.into_hashable())
    }
    #[inline]
    pub fn remove_member<T:IntoHashable>(&mut self,name:T)->Option<Value> {
        self.remove_member_unchecked(name.into_hashable())
    }
    #[inline]
    pub fn ensure_value<T:Into<Value>+std::convert::TryFrom<Value>,K:IntoHashable>(&mut self,key:K,val_if_nonvalid:Option<T>,allow_null:bool)->Result<()> {
        self.ensure_value_unchecked(key.into_hashable(), val_if_nonvalid,allow_null)
    }
    #[inline]
    pub fn ensure_value_exists<T:IntoHashable>(&mut self,key:T) {
        self.ensure_value_exists_unchecked(key.into_hashable())
    }
    #[inline]
    pub fn ensure_value_exists_unchecked(&mut self,key:Value) {
        match self.member_unchecked(key.clone()) {
            Some(_)=>{},
            _=>{
                self.set_member_value(key,Value::NULL)
            }
        }
    }
    
   
    #[inline]
    pub fn ensure_value_unchecked<T:Into<Value>+std::convert::TryFrom<Value>>(&mut self,key:Value,val_if_nonvalid:Option<T>,allow_null:bool)->Result<()> {
        match self.member_unchecked(key.clone()) {
            Some(a)=>{
                if a.is_null() {
                    if allow_null {
                        Ok(())
                    } else {
                        match val_if_nonvalid {
                            Some(v)=>{
                                self.set_member_value(key, match v.try_into(){
                                    Ok(a)=>a,
                                    _=>return Err(Error::InvalidBorrow(file!(),line!()))
                                });
                                Ok(())
                            }
                            _=>Err(Error::InvalidBorrow(file!(),line!()))
                        }
                    }
                } else {
                    let member = T::try_from(a);
                    match member{
                        Ok(_)=>{},
                        Err(e)=>match val_if_nonvalid {
                            Some(v)=>{
                                self.set_member_value(key, match v.try_into(){
                                    Ok(a)=>a,
                                    _=>return Err(Error::InvalidBorrow(file!(),line!()))
                                });
                                {}
                            }
                            _=>return Err(Error::InvalidBorrow(file!(),line!()))
                        }
                    };
                    Ok(())
                }
            },
            _=>match val_if_nonvalid {
                Some(v)=>{
                    self.set_member_value(key, match v.try_into(){
                        Ok(a)=>a,
                        _=>return Err(Error::InvalidBorrow(file!(),line!()))
                    });
                    Ok(())
                }
                _=>Err(Error::InvalidBorrow(file!(),line!()))
            }
            
        }
    }

    #[inline]
    pub fn set_member<T:Into<Value>,H:Into<Value>>(&mut self,name:T,contents:H) {
        self.set_member_value(name.into(), contents.into())
    }
    #[inline]
    pub fn set_doc<H:Into<Value>>(&mut self,contents:H) {
        self.set_member(super::zstd::builtin::property::accessors::__doc(), contents)
    }
    #[inline]
    pub fn set_member_value(&mut self,name:Value,contents:Value) {
        if let Some(map) = &mut self.property {
            map.insert(name,contents);
        } else {
            let mut map = PropertyMap::new();
            map.insert(name,contents);
            self.property = Some(map);
        }
    }
   
    #[inline]
    pub fn native(&self)->Option<NativeFunction> {
        match self.function() {
            Some(function)=>{
                match &function.code {
                    ObjFunction::Native(a)=>Some(*a),
                    _=>None,
                }
            }
            _=>None
        }
    }
    #[inline]
    pub fn push(&mut self,exec:&Executor,val:Value) {
        if let Some(vec) = &mut self.vec {
            vec.push(exec,val);
        }
    }
    #[inline]
    pub fn vec_len(&self) -> usize {
        match &self.vec{
            Some(a)=>a.len(),
            None=>0
        }
    }
    #[inline]
    pub fn vec_join(&self,exec:&Executor,v:&[&Value]) -> Value {
        match &self.vec{
            Some(a)=>a.join(exec, v),
            None=>Value::NULL
        }
    }
    #[inline]
    pub fn vec_remove<Key:Into<SetIndexValue>>(&mut self,id:Key)->Value{
        match &mut self.vec{
            Some(a)=>if let Ok(a) = a.remove(id){
                a
            }else {
                Value::NULL
            },
            None=>Value::NULL
        }
    }
    #[inline]
    pub fn map_len(&self) -> usize {
        match &self.property{
            Some(a)=>a.len(),
            None=>0
        }
    }
    #[inline]
    pub fn pop(&mut self)->Value {
        if let Some(vec) = &mut self.vec {
            vec.pop()
        } else {
           Value::NULL
        }
    }
    #[inline]
    pub fn map_index<Key:Into<SetIndexValue>>(&self,idx:Key)->Option<Value> {
        match &self.vec {
            Some(map)=>map.get_index(idx),
            None=>None
        }
    }
    #[inline]
    pub fn set_map_index<Key:Into<SetIndexValue>>(&self,idx:Key,key:Key,val:Value) ->Option<Value> {
        match &self.vec {
            Some(map)=>map.get_index(idx),
            None=>None
        }
    }
    #[inline]
    pub fn index<Key:Into<SetIndexValue>>(&self,idx:Key)->Option<Value> {
        match &self.vec {
            Some(map)=>map.get(idx),
            None=>None
        }
    }
    #[inline]
    pub fn set_index<Key:Into<SetIndexValue>>(&mut self,exec:&Executor,idx:Key,val:Value)->Result<()> {
        match self.vec {
            Some(ref mut map)=>{
                map.set(exec,idx, val)
            },
            None=>Err(Error::Unconvertable("object".into(),"map".into()))
        }
    }
    #[inline]
    pub fn is_callable(&self) ->bool {
        if let Some(fun) = self.function() {
            return true;
        }
        false
    }
   
}

#[derive(Eq,PartialOrd,Ord,Trace,Finalize)]
#[repr(transparent)]
pub struct GcObject {
    pub (crate) inner:Gc<GcCell<Object>>
}
impl PartialEq for GcObject {
    fn eq(&self, other: &Self) -> bool {       
        gc::Gc::ptr_eq(&self.inner, &other.inner)
    }
}
impl std::fmt::Debug for GcObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:p}",self)
    }
}
impl std::hash::Hash for GcObject {
    fn hash<H: std::hash::Hasher>(&self,state:&mut H) {
        let id = 0;
        id.hash(state)
    }
}
impl  Clone for GcObject {
    fn clone(&self) -> GcObject {
        GcObject::from(Gc::clone(&self.inner))
    }
}
impl std::convert::AsRef<Gc<GcCell<Object>>> for GcObject {
    fn as_ref(&self) ->&Gc<GcCell<Object>> {
        &self.inner
    }
}
impl std::ops::Deref for GcObject {
    type Target = GcCell<Object>;
    fn deref(&self) ->&Self::Target {
        self.inner.deref()
    }
}
impl From<Object> for GcObject {
    fn from(a:Object)->GcObject {
        Self{inner:Gc::from(GcCell::new(a))}
    }
}
impl From<Gc<GcCell<Object>>> for GcObject {
    fn from(a:Gc<GcCell<Object>>)->GcObject {
        Self{inner:a}
    }
}

impl GcObject {
    #[inline]
    pub fn as_ref_rc(&self)->&Gc<GcCell<Object>> {
        &self.inner
    }
    #[inline]
    pub fn force_method(&mut self,is_method: bool) {
        match self.inner.try_borrow_mut() {
            Ok(mut a)=>{
                a.contents.is_method = is_method;
            }
            _=>{panic!("unexpected borrow");}
        }
    }
  
    #[inline]
    pub(crate) fn member_unchecked(&self,name:Value)->Option<Value> {
        match self.inner.try_borrow() {
            Ok(a)=>{
                a.member_unchecked(name)
            }
            _=>None
        }
       
    }
    #[inline]
    pub(crate) fn __str(&self,exec:&Executor)->Option<Value> {
        match self.inner.try_borrow() {
            Ok(a)=>{
                Some(a.contents.__str(exec,&self.clone())?.into())
            }
            _=>None
        }
       
    }
    #[inline]
    pub fn ensure_value_exists<T:IntoHashable>(&mut self,key:T) {
        match self.inner.try_borrow_mut() {
            Ok(mut a)=>{
                a.ensure_value_exists(key)
            }
            _=>{}
        }
    }
    #[inline]
    pub(crate) fn code(&self)->Option<ObjFunction> {
        match self.inner.try_borrow() {
            Ok(a)=>{
                match &a.function() {
                    Some(a)=> Some(a.code.clone()),
                   
                    _=>None
                }
            }
            _=>None
        }
    }

    #[inline]
    pub fn ensure_value_exists_unchecked(&mut self,key:Value) {
        match self.inner.try_borrow_mut() {
            Ok(mut a)=>{
                a.ensure_value_exists_unchecked(key)
            }
            _=>{}
        }
    }
    #[inline(always)]
    fn compare_slice(lhs:&[u8],rhs:&[u8]) -> bool {
        lhs==rhs
    }
    #[inline]
    pub fn create_method(method:&mut GcObject,parent:GcObject)->bool {
        match method.is_callable() {
            true=>{
                method.set_parent(parent);
                method.force_method(true);
                true
            }
            _=>false
        }
    }
    #[inline]
    pub fn bind_method(method:&mut GcObject,parent:GcObject)->bool {
        match method.is_callable() {
            true=>{
                method.set_parent(parent);
                method.force_method(true);
                true
            }
            _=>false
        }
    }
    #[inline]
    pub fn compare_value(&self,value:Value) ->(bool,bool) {
        match self.inner.try_borrow() {
            Ok(vec)=>{
                match &vec.vec {
                    Some(a)=> match &value {
                        Value::String(b)=>{
                            match a {
                                InnerContainer::String(a)=>(a.as_str()==b.as_ref(),true),
                                InnerContainer::Vector(InternedVector::ByteVector(a))=>(Self::compare_slice(&a, b.as_bytes()),true),
                                _=>(false,true),
                            }
                        }
                        Value::Bytes(b)=>{
                            match a {
                                InnerContainer::String(a)=>(a.as_bytes()==b.as_ref(),true),
                                InnerContainer::Vector(InternedVector::ByteVector(a))=>(Self::compare_slice(&a, b.as_ref()),true),
                                _=>(false,true),
                            }
                        }
                        Value::Num(b)=>{
                            match a {
                                InnerContainer::String(a)=>(b.compare_bytes(a.as_bytes()),true),
                                InnerContainer::Vector(InternedVector::ByteVector(a))=>(b.compare_bytes(&a),true),
                                _=>(false,true),
                            }
                        }
                        Value::Object(b)=>{
                            if Gc::ptr_eq(&self.inner,&b.inner) {
                                return (true,true)
                            }
                            match a {
                                InnerContainer::String(a)=>{
                                    
                                    match b.try_borrow() {
                                        Ok(b)=>{
                                            match &b.contents.vec {
                                                Some(b)=>match b{
                                                    InnerContainer::String(b)=> (a.as_str()==b.as_str(),true),
                                                    InnerContainer::Vector(InternedVector::ByteVector(b))=>(Self::compare_slice(a.as_bytes(), &b),true) ,
                                                    _=>(false,true),
                                                }
                                                _=> (false,true)
                                            }
                                        }
                                        _=>(false,false),
                                    }
                                   
                                },
                                InnerContainer::Vector(InternedVector::ByteVector(a))=>{

                                    match b.try_borrow() {
                                        Ok(b)=>{
                                            match &b.contents.vec {
                                                Some(b)=>match b{
                                                    InnerContainer::String(b)=> (Self::compare_slice(&a,b.as_bytes()),true),
                                                    InnerContainer::Vector(InternedVector::ByteVector(b))=>(Self::compare_slice(&a, &b),true) ,
                                                    _=>(false,true),
                                                }
                                                _=> (false,true)
                                            }
                                        }
                                        _=>(false,false),
                                    }
                                },
                                _=>(false,false),
                            }
                        }
                        _=>(false,true),
                    },
                    _=>(false,false)
                }
            }
            _=>(false,false)
        }
        
        
    }

    #[inline]
    pub fn with_member<T:Into<Value>,H:Into<Value>>(mut self, name:T,member:H) -> Self {
        self.set_member(name,member);
        self
    }
    #[inline]
    pub fn with_doc<H:Into<Value>>(mut self,member:H) -> Self {
        self.set_doc(member);
        self
    }
    #[inline]
    pub fn with_method(mut self,name:ZString, mut val:super::function::Function)->Self {
        Self::bind_method(&mut val,self.clone());
        self.set_member(name,val);
        self
    }
    #[inline]
    pub fn native(&self)->Option<NativeFunction> {
        match self.try_borrow() {
            Ok(a)=>a.native(),
            _=>None
        }
    }
    
    #[inline]
    pub fn member<T:IntoHashable>(&self,name:T)->Option<Value> {
        self.member_unchecked(name.into_hashable())
    }
    #[inline]
    pub fn parent(&self)->Option<GcObject> {
        let t = self.inner.try_borrow();
        match t {
            Ok(a)=>{
                a.parent()
            }
            _=>None
        }

    }
    #[inline]
    pub fn name(&self)->Option<ZString> {
        let t = self.inner.try_borrow();
        match t {
            Ok(a)=>{
                a.name()
            }
            _=>None
        }
    }
    #[inline]
    pub fn set_parent<T:Objectable>(&mut self,parent:T)->Option<()> {
        let t = self.inner.try_borrow_mut();
        match t {
            Ok(mut a)=>{
                a.set_parent(parent);
                Some(())
            }
            _=>None
        }

    }
    #[inline]
    pub fn set_name<T:Into<ZString>>(&mut self,name:T)->Option<()> {
        let t = self.inner.try_borrow_mut();
        match t {
            Ok(mut a)=>{
                a.set_name(name);
                Some(())
            }
            _=>None
        }

    }
    #[inline]
    pub fn map_index<Key:Into<SetIndexValue>>(&self,id:Key) ->Result<Value>{
        match self.inner.try_borrow() {
            Ok(mut a)=>Ok(a.map_index(id).unwrap_or(Value::NULL)),
            _=>Err(Error::InvalidBorrow(file!(),line!()))
        }
    }
    #[inline]
    pub fn index<Key:Into<SetIndexValue>>(&self,id:Key) ->Result<Value>{
        match self.inner.try_borrow() {
            Ok(mut a)=>Ok(a.index(id).unwrap_or(Value::NULL)),
            _=>Err(Error::InvalidBorrow(file!(),line!()))
        }
    }
    #[inline]
    pub fn index_access<Key:Into<SetIndexValue>>(&self,id:Key) ->Option<Value>{
        match self.inner.try_borrow() {
            Ok(mut a)=>a.index(id),
            _=>None
        }
    }
    #[inline]
    pub fn set_index<Key:Into<SetIndexValue>>(&self,exec:&Executor,id:Key,val:Value) ->Result<()>{
        match self.inner.try_borrow_mut() {
            Ok(mut a)=>a.set_index(exec,id,val),
            _=>Err(Error::InvalidBorrow(file!(),line!()))
        }
    }
    #[inline]
    pub fn set_member<T:Into<Value>,H:Into<Value>>(&mut self,name:T,contents:H)->Option<()> {
        let t = self.inner.try_borrow_mut();
        match t {
            Ok(mut a)=>{
                a.set_member_value(name.into(),contents.into());
                Some(())
            }
            _=>None
        }
    }
    #[inline]
    pub fn set_doc<H:Into<Value>>(&mut self,contents:H)->Option<()> {
        let t = self.inner.try_borrow_mut();
        match t {
            Ok(mut a)=>{
                a.set_doc(contents);
                Some(())
            }
            _=>None
        }
    }
    #[inline]
    pub fn set_member_value(&mut self,name:Value,contents:Value)->Option<()> {
        let t = self.inner.try_borrow_mut();
        match t {
            Ok(mut a)=>{
                a.set_member_value(name,contents);
                Some(())
            }
            _=>None
        }

    }
    #[inline]
    pub fn hard_clone(&self)->GcObject {
        GcObject::from(self.inner.borrow().hard_clone())
    }
    #[inline]
    pub(crate) fn is_method(&self)->bool {
        match self.inner.try_borrow() {
            Ok(a)=>a.is_method,
            _=>false
        }
    }
    #[inline]
    pub fn get_root(&self) -> GcObject {
        let mut temp = self.clone();
        while let Some(parent) = temp.parent() {
            temp= parent;
        }
        temp
    }
    #[inline]
    pub fn ensure_value<T:Into<Value>+std::convert::TryFrom<Value>,K:IntoHashable>(&mut self,key:K,val_if_nonvalid:Option<T>,allow_null:bool)->Result<()> {
        match self.inner.try_borrow_mut() {
            Ok(mut a)=>a.ensure_value(key,val_if_nonvalid,allow_null),
            _=>Err(Error::InvalidBorrow(file!(),line!()))
        }
    }
    #[inline]
    pub fn ensure_value_unchecked<T:Into<Value>+std::convert::TryFrom<Value>>(&mut self,key:Value,val_if_nonvalid:Option<T>,allow_null:bool)->Result<()> {
        match self.inner.try_borrow_mut() {
            Ok(mut a)=>a.ensure_value_unchecked(key,val_if_nonvalid,allow_null),
            _=>Err(Error::InvalidBorrow(file!(),line!()))
        }
    }
    #[inline]
    pub fn push(&mut self,exec:&Executor,v:Value) -> Result<()> {
        match self.inner.try_borrow_mut() {
            Ok(mut a)=>{a.push(exec,v);Ok(())},
            _=>Err(Error::InvalidBorrow(file!(),line!()))
        }
    }
    #[inline]
    pub fn pop(&mut self) -> Result<Value> {
        match self.inner.try_borrow_mut() {
            Ok(mut a)=>Ok(a.pop()),
            _=>Err(Error::InvalidBorrow(file!(),line!()))
        }
    }
    #[inline]
    pub fn zstr_eq(&self,exec:&Executor,args:&[&Value])->bool {
        match self.inner.try_borrow() {
            Ok(a)=>a.zstr_eq(exec,args),
            _=>false
        }
    }
    #[inline]
    pub fn vec_len(&self) -> Result<usize> {
        match self.inner.try_borrow() {
            Ok(mut a)=>Ok(a.vec_len()),
            _=>Err(Error::InvalidBorrow(file!(),line!()))
        }
    }
    #[inline]
    pub fn vec_remove<Key:Into<SetIndexValue>>(&mut self,id:Key) -> Result<Value> {
        match self.inner.try_borrow_mut() {
            Ok(mut a)=>Ok(a.vec_remove(id)),
            _=>Err(Error::InvalidBorrow(file!(),line!()))
        }
    }
    #[inline]
    pub fn is_callable(&self) -> bool {
        match self.inner.try_borrow() {
            Ok(mut a)=>a.is_callable(),
            _=>false
        }
    }
    #[inline]
    pub fn vec_join(&self,exec:&Executor,v:&[&Value]) -> Result<Value> {
        match self.inner.try_borrow() {
            Ok(a)=>Ok(a.vec_join(exec,v)),
            _=>Err(Error::InvalidBorrow(file!(),line!()))
        }
    }
    #[inline]
    pub fn get_fields(&self) -> Option<Vec<Value>> {
        match self.inner.try_borrow() {
            Ok(a)=>match &a.property {
                Some(a)=> {
                    let mut fields = Vec::new();
                    for (i,_) in &a.0 {
                        fields.push(i.clone())
                    }
                    Some(fields)
                }
                _=>None
            },
            _=>None
        }
    }
    #[inline]
    pub fn traverse<F:FnMut(&mut GcObject) ->bool>(&self,mut func:F ) -> GcObject {
        let mut temp = self.clone();
        loop {
            if func(&mut temp) {
                break;
            }
            if let Some(parent) = temp.parent() {
                temp = parent;
            } else {
                break;
            }
        }
        temp
    }
}
#[derive(Debug, Clone,Trace,Finalize,Eq)]
pub struct Object {
    id:ObjectId,
    pub(crate) contents:Container,
}

impl std::hash::Hash for Object {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl std::ops::Deref for Object {
    type Target = Container;
    fn deref(&self) -> &Self::Target {
        &self.contents
    }
}
impl std::ops::DerefMut for Object {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.contents
    }
}

impl Object {
    #[inline]
    pub fn new(access:ObjAccess)->Object {
        Self{
            id:0,
            contents:Container::new(access)
        }
    }
    #[inline]
    pub fn is_root(&self)-> bool {
        match self.member(super::function::options::__parent()) {
            Some(a)=>match a {
                Value::Object(_)=>false,
                _=>true
            }
            _=>false
        }
    }
    
    #[inline]
    pub fn with_parent<T:Objectable>(mut self,code:T)->Self{
        self.set_parent(code);
        self
    }
    #[inline]
    pub fn with_name<T:Into<ZString>>(mut self,code:T)->Self{
        self.set_name(code);
        self
    }
    #[inline]
    pub fn with_doc<H:Into<Value>>(mut self,member:H) -> Self {
        self.set_doc(member);
        self
    }
    #[inline]
    pub fn set_parent<T:Objectable>(&mut self,code:T){
        self.set_member(super::function::options::__parent(), code.object());
    }
    #[inline]
    pub (crate)fn with_is_method(mut self,is_method: bool)->Self{
        self.force_method(is_method);
        self
    }
    #[inline]
    pub fn set_name<T:Into<ZString>>(&mut self,code:T){
        self.set_member(super::function::options::__name(), code.into());
    }
    #[inline]
    pub fn with_member<T:Into<Value>,V:Into<Value>>(mut self,key:T,val:V)->Self {
        self.set_member(key, val);
        self
    }
    #[inline]
    pub fn zstr_eq(&self,exec:&Executor,args:&[&Value])->bool {
        match &self.vec {
            Some(a)=>a.zstr_eq(exec,args),
            _=>false
        }
    }
    #[inline]
    pub fn hard_clone(&self) ->Object {
        self.clone()
    }
    #[inline]
    pub fn exists_index<T:IntoHashable>(&self,name:T) -> bool {
        match self.member_unchecked(name.into_hashable()) {
            Some(_)=>true,
            None => false,
        }
    }
    #[inline]
    pub fn exists_index_unchecked(&self,name:Value) -> bool {
        match self.member_unchecked(name) {
            Some(_)=>true,
            None => false,
        }
    }
    #[inline]
    pub fn parent(&self)->Option<GcObject> {
        match &self.member(super::function::options::__parent())? {
            Value::Object(a)=>Some(a.clone()),
            Value::NULL=>{
                None
            }
            _=>None
        }
    }
    #[inline]
    pub fn name(&self)->Option<ZString> {
        match &self.member(super::function::options::__name())? {
            Value::String(a)=>Some(a.clone()),
            Value::NULL=>{
                None
            }
            _=>None
        }
    }
    #[inline]
    pub fn id(&self) ->ObjectId {
        self.id
    }
    #[inline]
    pub fn access(&self)->ObjAccess {
        self.contents.access()
    }
    #[inline]
    pub fn with_map(map:PropertyMap,access:ObjAccess) -> Object {
        Self {
            id:0,
            contents:Container::with_map(map,access),
        }
    }
    #[inline]
    pub fn with_vec<T:Into<InternedVector>>(map:T,access:ObjAccess) -> Object {
        Self {
            id:0,
            contents:Container::with_vec(map,access),
        }
    }
    #[inline]
    pub fn with_string<T:Into<InternedString>>(string:T,access:ObjAccess) -> Object {
        Self {
            id:0,
            contents:Container::with_string(string,access),
        }
    }
    #[inline]
    pub fn is_callable(&self) -> bool {
        self.contents.is_callable()
    }
  
    #[inline]
    pub (crate) fn is_method(&self)->bool {
        self.is_method
    }
    #[inline]
    pub(crate) fn map(access:ObjAccess) ->Self {
        Self {
            id:0,
            contents:Container::map(access),
        }
    }
    #[inline]
    pub(crate) fn vec(access:ObjAccess) ->Self {
        Self {
            id:0,
            contents:Container::vec(access),
        }
    }
   
  
    #[inline]
    pub fn container(&self) ->&Container {
        &self.contents
    }
    #[inline]
    pub fn container_mut(&mut self)->&mut Container {
        &mut self.contents
    }
}
impl PartialEq for Object {
    #[inline]
    fn eq(&self, other:&Self)->bool {
        self.id.eq(&other.id)
    }
}
impl  PartialOrd for Object {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.id.cmp(&other.id))
    }
    
}
impl  Ord for Object {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

use crate::object::*;
pub trait Objectable{
    fn object(self) ->GcObject;
}

impl Objectable for GcObject {
    #[inline]
    fn object(self)->GcObject{
        self
    }
}
impl Objectable for Object {
    #[inline]
    fn object(self)->GcObject{
        GcObject::from(self)    
    }
}

impl Objectable for NativeFunction {
    fn object(self)->GcObject {
        let mut obj = Object::new(access::ObjAccess::NONE);
        obj.vec = Some(InnerContainer::Function(InnerFunction{code:ObjFunction::Native(self)}));
        GcObject::from(obj)
    }
}

impl From<GcObject> for Error {
    #[inline]
    fn from(g: GcObject) -> Error {
        let name = match g.name() {
            Some(a)=>a.to_string(),
            None=>"unknown".to_string()          
        };
        let why = g.member("__why");
        match why {
            Some(why)=>{
                Error::Custom(name,why.to_string())
            }
            _=>{
                Error::CustomSingle(name)
            }
        }
        
    }
}

impl From<Error> for GcObject {
    fn from(e: Error) -> GcObject {
        match e {
            Error::Custom(a,b)=>{
                GcObject::from(Object::new(access::ObjAccess::NONE)).with_member("__name",a).with_member("__why",b)
            }
            Error::CustomSingle(a)=>{
                GcObject::from(Object::new(access::ObjAccess::NONE)).with_member("__name",a)
            }
            _=> GcObject::from(Object::new(access::ObjAccess::NONE)).with_member("__name",e.kind()).with_member("__why",e.arg())
        }
       
    }
}