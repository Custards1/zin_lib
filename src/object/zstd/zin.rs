
use crate::types::*;
use crate::object::*;
use crate::error::*;

use function::Function;
use super::builtin;
use interpreter::Executor;
pub mod property {
    static_tstring!(execfile);
    static_tstring!(exec);
    static_tstring!(lex);
    static_tstring!(compile);
    static_tstring!(compilefile);
}
#[inline]
pub fn exec_string(e:Executor,args:Vec<Value>)->Result<Value> {
    match args.len() {
        0=>Err(Error::InvalidCall("nothing".to_string())),
        1=> {
            let s = args[0].invoke_to_string(&e);
            e.call_script(s,None)
        }
        _=>{
            match &args[1] {
                Value::Object(obj)=>{
                    let mut obj = Function::from(&obj.clone());
                    obj.consume_script(&e.builtin,args[0].invoke_to_string(&e))?;
                    e.call(obj.object(),Vec::with_capacity(0))
                }
                _=>Err(Error::Custom("TypeError".to_string(),"Expected object to provide globals to exec".to_string()))
            }
        }
    }
}
#[inline]
pub fn exec_file(e:Executor,args:Vec<Value>)->Result<Value> {
    match args.len() {
        0=>Err(Error::InvalidCall("nothing".to_string())),
        1=> {
            let name = args[0].invoke_to_string(&e);
            e.call_script(std::fs::read_to_string(name.as_ref())?,None)
        }
        _=>{
            match &args[1] {
                Value::Object(obj)=>{
                    let mut obj = Function::from(&obj.clone());
                    let name = args[0].invoke_to_string(&e);
                    obj.consume_script(&e.builtin,std::fs::read_to_string(name.as_ref())?)?;
                    
                    e.call(obj.object(),Vec::with_capacity(0))
                }
                _=>Err(Error::Custom("TypeError".to_string(),"Expected object to provide globals to exec".to_string()))
            }
        }
    }
}
#[inline] 
pub fn compile(e:Executor,args:Vec<Value>)->Result<Value> {
    match args.len() {
        0=>Err(Error::InvalidCall("nothing".to_string())),
        1=> {
            let mut obj = Function::root(access::ObjAccess::NONE);
            obj.consume_script(&e.builtin,args[0].invoke_to_string(&e))?;
            Ok(obj.into())
        }
        _=>{
            match &args[1] {
                Value::Object(obj)=>{
                    let mut obj = Function::from(&obj.clone());
                    obj.consume_script(&e.builtin,args[0].invoke_to_string(&e))?;
                    Ok(obj.into())
                }
                _=>Err(Error::Custom("TypeError".to_string(),"Expected object to provide globals to exec".to_string()))
            }
        }
    }
}
#[inline]
pub fn compile_file(e:Executor,args:Vec<Value>)->Result<Value> {
    match args.len() {
        0=>Err(Error::InvalidCall("nothing".to_string())),
        1=> {
            let name = args[0].invoke_to_string(&e);
            let mut obj = Function::root(access::ObjAccess::NONE);
            obj.consume_script(&e.builtin,std::fs::read_to_string(name.as_ref())?)?;
            Ok(obj.into())
        }
        _=>{
            match &args[1] {
                Value::Object(obj)=>{
                    let name = args[0].invoke_to_string(&e);
                    let mut obj = Function::from(&obj.clone());
                    obj.consume_script(&e.builtin,std::fs::read_to_string(name.as_ref())?)?;
                    Ok(obj.into())
                }
                _=>Err(Error::Custom("TypeError".to_string(),"Expected object to provide globals to exec".to_string()))
            }
        }
    }
}
#[inline]
pub fn open_zin()->GcObject {
    GcObject::from(
        Object::new(access::ObjAccess::NONE)
        .with_name("zin")
        .with_doc("interface to the zin engine"))
    .with_member(property::exec(),Function::native_root(access::ObjAccess::NONE,exec_string))
    .with_member(property::execfile(),Function::native_root(access::ObjAccess::NONE, exec_file))
    .with_member(property::compile(),Function::native_root(access::ObjAccess::NONE, compile))
    .with_member(property::compilefile(),Function::native_root(access::ObjAccess::NONE, compile_file))
    
}