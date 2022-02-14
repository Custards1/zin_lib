use crate::types::*;
use crate::object::*;
use crate::error::*;
use function::Function;
use interpreter::Executor;
use super::builtin;
use std::path::{PathBuf,Path};
const PATH_NAME: &'static str = "path";
const PATH_DOC: &'static str = "This module providen means to interface with paths for your system";
#[inline]
fn join(e:Executor,args:Vec<Value>)->Result<Value> {
    let mut path = std::path::PathBuf::new();
    for arg in args {
        let t = arg.try_invoke_to_string(&e)?;
        path.push(Path::new(t.as_ref()));
    }
    match path.to_str(){
        Some(p)=>Ok(builtin::string_from(p).into()),
        None=>Err(Error::BadPath(path))
    }
}


pub fn open_path() ->GcObject{
    let mut path = GcObject::from(Object::new(access::ObjAccess::NONE).with_name(PATH_NAME).with_doc(PATH_DOC))
    .with_member("join", Function::native_root(access::ObjAccess::NONE,join));
    return path;
}