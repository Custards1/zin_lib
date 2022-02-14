use crate::types::*;
use crate::object::*;
use crate::error::*;
use function::Function;
use interpreter::Executor;
use std::convert::TryFrom;

pub mod property {
    static_tstring!(read);
    static_tstring!(read_all);
    static_tstring!(read_to_string);
    static_tstring!(write);
    static_tstring!(close);
    static_tstring!(flush);
    static_tstring!(open);
    static_tstring!(create);
    static_tstring!(stdin);
    static_tstring!(stdout);
    static_tstring!(stderr);
        
}

const IO_NAME: &'static str = "io";
const IO_DOC: &'static str = "io...";
use std::io::{Read, Write};
#[inline]
pub fn read(_:Executor,args:Vec<Value>) -> Result<Value> {
    match args.len() {
        0=>Ok(Value::NULL),
        1=>{
            match &args[0].clone() {
                Value::Object(o)=>{
                    match o.inner.try_borrow_mut(){
                        Ok(mut e)=>match &mut e.contents.vec{
                            Some(InnerContainer::File(a))=> {
                                let mut buf = vec![1u8;1];
                                let a = a.read(&mut buf)?;
                                if a == 0 { 
                                    Ok(Value::NULL)
                                } else {
                                    Ok(buf[0].into())
                                }
                            }
                            _=>Ok(Value::NULL),
                        } ,
                        _=>Err(Error::InvalidBorrow(file!(), line!()))
                    }
                }
                _=>Ok(Value::NULL),
            }
        }
        _=>{
            let amt = match Number::try_from(args[1].clone()) {
                Ok(a)=>a,
                _=>1.into()
            }.usize();
            match &args[0].clone() {
                Value::Object(o)=>{
                    match o.inner.try_borrow_mut(){
                        Ok(mut e)=>match &mut e.contents.vec{
                            Some(InnerContainer::File(a))=> {
                                let mut buf = vec![0u8;amt];
                                match a.read(&mut buf) {
                                    Ok(a)=>match a {
                                        0=>Ok(Value::NULL),
                                        _=>Ok(super::builtin::byte_vec_from(&buf).into())
                                    }
                                    _=>Ok(Value::NULL)
                                }
                            }
                            _=>Ok(Value::NULL),
                        } ,
                        _=>Ok(Value::NULL),
                    }
                }
                _=>Ok(Value::NULL),
            }
        }
    }
}

#[inline]
pub fn read_all(_:Executor,args:Vec<Value>) -> Result<Value> {
    match args.len() {
        0=>Ok(Value::NULL),
        _=>{
            match &args[0].clone() {
                Value::Object(o)=>{
                    match o.inner.try_borrow_mut(){
                        Ok(mut e)=>match &mut e.contents.vec{
                            Some(InnerContainer::File(a))=> {
                                let mut buf = Vec::with_capacity(10);
                                match a.read_to_end(&mut buf) {
                                    Ok(_)=>Ok(super::builtin::vec_from(buf).into()),
                                    _=>Ok(Value::NULL)
                                }
                            }
                            _=>Ok(Value::NULL),
                        } ,
                        _=>Ok(Value::NULL),
                    }
                }
                _=>Ok(Value::NULL),
            }
        }
    }
}
#[inline]
pub fn read_to_string(_:Executor,args:Vec<Value>) -> Result<Value> {
    match args.len() {
        0=>Ok(Value::NULL),
        _=>{
            match &args[0].clone() {
                Value::Object(o)=>{
                    match o.inner.try_borrow_mut(){
                        Ok(mut e)=>match &mut e.contents.vec{
                            Some(InnerContainer::File(a))=> {

                                let mut buf = String::with_capacity(10);
                                match a.read_to_string(&mut buf){
                                    Ok(_)=>Ok(super::builtin::string_from(buf).into()),
                                    _=>Ok(Value::NULL)
                                }
                            }
                            _=>Ok(Value::NULL),
                        } ,
                        _=>Ok(Value::NULL),
                    }
                }
                _=>Ok(Value::NULL),
            }
        }
    }
}
#[inline]
pub fn glob_read_to_string(e:Executor,args:Vec<Value>) -> Result<Value> {
    match args.len() {
        0=>Ok(Value::NULL),
        _=>{
            let a = args[0].invoke_to_string(&e);
            match std::fs::read_to_string(a.as_ref()){
                Ok(a)=>Ok(super::builtin::string_from(a).into()),
                _=>Ok(Value::NULL)
            }
        }
    }
}
#[inline]
pub fn glob_read_all(e:Executor,args:Vec<Value>) -> Result<Value> {
    match args.len() {
        0=>Ok(Value::NULL),
        _=>{
            let a = args[0].invoke_to_string(&e);
            match std::fs::read(a.as_ref()) {
                Ok(a)=>Ok(super::builtin::byte_vec_from(&a).into()),
                _=>Ok(Value::NULL)
            }
        }
    }
}

pub fn write(ex:Executor,args:Vec<Value>) -> Result<Value> {
    let len = args.len(); 
    match args.len() {
        0|1=>Ok(Value::NULL),
        _=>{
            
            match &args[0].clone() {
                Value::Object(o)=>{
                    match o.inner.try_borrow_mut(){
                        Ok(mut e)=>match &mut e.contents.vec{
                            Some(InnerContainer::File(a))=> {
                                let mut fin = 0;
                                for i in 1..len {
                                    let amt = args[i].clone().invoke_to_bytes(&ex);
                                    match a.write(&amt) {
                                        Ok(a)=>fin+=a,
                                        _=>return Err(Error::Io(std::io::Error::from(std::io::ErrorKind::WriteZero)))
                                    }
                                }
                                Ok(fin.into())
                                
                            }
                            _=>Ok(Value::NULL),
                        } ,
                        _=>Ok(Value::NULL),
                    }
                }
                _=>Ok(Value::NULL),
            }
        }
    }
}
pub fn flush(_:Executor,args:Vec<Value>) -> Result<Value> {
    match args.len() {
        0=>Ok(Value::NULL),
        _=>{
            match &args[0].clone() {
                Value::Object(o)=>{
                    match o.inner.try_borrow_mut(){
                        Ok(mut e)=>match &mut e.contents.vec{
                            Some(InnerContainer::File(a))=> {
                                a.flush()?;
                                Ok(Value::NULL)
                            }
                            _=>Ok(Value::NULL),
                        } ,
                        _=>Ok(Value::NULL),
                    }
                }
                _=>Ok(Value::NULL),
            }
        }
    }
}
pub fn close(_:Executor,args:Vec<Value>) -> Result<Value> {
    match args.len() {
        0=>Ok(Value::NULL),
        _=>{
            match &args[0].clone() {
                Value::Object(o)=>{
                    match o.inner.try_borrow_mut(){
                        Ok(mut e)=>match &mut e.contents.vec{
                            Some(InnerContainer::File(a))=> {
                                a.close();
                                Ok(Value::NULL)
                            }
                            _=>Ok(Value::NULL),
                        } ,
                        _=>Ok(Value::NULL),
                    }
                }
                _=>Ok(Value::NULL),
            }
        }
    }
}
#[inline]
pub fn obj_file(file:std::fs::File) -> GcObject{
    let mut obj = Object::new(access::ObjAccess::NONE);
    obj.contents.vec = Some(InnerContainer::File(ZFile::File(RFile::new(file))));
    file_methods(GcObject::from(obj),Value::NULL)
}
#[inline]
fn _stdin() -> GcObject{
    let mut obj = Object::new(access::ObjAccess::NONE);
    obj.contents.vec = Some(InnerContainer::File(ZFile::Stdin));
    GcObject::from(obj)
}
#[inline]
fn _stdout() -> GcObject{
    let mut obj = Object::new(access::ObjAccess::NONE);
    obj.contents.vec = Some(InnerContainer::File(ZFile::Stdout));
    GcObject::from(obj)
}
#[inline]
fn _stderr() -> GcObject{
    let mut obj = Object::new(access::ObjAccess::NONE);
    obj.contents.vec = Some(InnerContainer::File(ZFile::Stderr));
    GcObject::from(obj)
}
#[inline]
pub fn stdin() -> GcObject{
    file_methods(_stdin(), Value::NULL)
}
#[inline]
pub fn stdout() -> GcObject {
    file_methods(_stdout(), Value::NULL)
}
#[inline]
pub fn stderr() -> GcObject {
    file_methods(_stderr(), Value::NULL)
}
#[inline]
fn file_methods<T:Into<Value>>(mut obj: GcObject,name:T) -> GcObject{
    obj.set_member("path", name);
    obj.with_method(property::read(),Function::native_root_method(read))
    .with_method(property::read_all(),Function::native_root_method(read_all))
    .with_method(property::read_to_string(),Function::native_root_method(read_to_string))
    .with_method(property::write(),Function::native_root_method(write))
    .with_method(property::flush(),Function::native_root_method(flush))
    .with_method(property::close(),Function::native_root_method(close))
}
#[inline]
pub fn open(e:Executor,args:Vec<Value>) -> Result<Value>{
    match args.len() {
        0=>Ok(Value::NULL),
        1=>{
            let a = args[0].invoke_to_string(&e);
            Ok(obj_file(std::fs::File::open(a.as_ref())?).into())    
        },
        _=>{
            match &args[1] {
                Value::NULL => {
                    let a = args[0].invoke_to_string(&e);
                    Ok(obj_file(std::fs::File::open(a.as_ref())?).into())
                }
                _=>{
                    let a = args[0].invoke_to_string(&e);
                    let opts = args[1].invoke_to_string(&e);
                    Ok(obj_file(open_opts(opts.as_ref()).open(a.as_ref())?).into())
                }
            }
            
        }
    }
}
#[inline]
pub fn create(e:Executor,args:Vec<Value>) -> Result<Value>{
    match args.len() {
        0=>Ok(Value::NULL),
        _=>{
            let a = args[0].invoke_to_string(&e);
            Ok(obj_file(std::fs::File::create(a.as_ref())?).into())
        }
    }
}
#[inline]
fn open_opts(val:&str)->std::fs::OpenOptions {
    let mut opts = std::fs::OpenOptions::new();
    for a in val.bytes() {
        match a {
            b'r'=>{
                opts.read(true);
            }
            b'+'=>{
                opts.truncate(true);
            }
            b'w'=>{
                opts.write(true);
            }
            b'a'=>{
                opts.append(true);
            }
            b' '|b'\n'|b'\t'|b'\r'=>{

            }
            _=>{
                return std::fs::OpenOptions::new();
            }
        }
    }
    opts
}
#[inline]
pub fn io()->GcObject {
    GcObject::from(Object::new(access::ObjAccess::NONE).with_name(IO_NAME)).with_doc(IO_DOC)
    .with_member(property::open(), Function::native_root(access::ObjAccess::NONE, open))
    .with_member(property::create(), Function::native_root(access::ObjAccess::NONE, create))
    .with_member(property::stdin(), stdin())
    .with_member(property::stdout(), stdout())
    .with_member(property::stderr(), stderr())
    .with_member(property::read_all(),Function::native_root(access::ObjAccess::NONE, glob_read_all))
    .with_member(property::read_to_string(),Function::native_root(access::ObjAccess::NONE, glob_read_to_string))
}