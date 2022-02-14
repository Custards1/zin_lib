use crate::types::*;
use crate::object::*;
use crate::error::*;

use function::Function;
use super::builtin;
use interpreter::Executor;
pub mod property {
    static_tstring!(parse);
    static_tstring!(traverse);
    static_tstring!(dump);
}
use crate::api::ast::Statement;
use crate::api::ast::Parseable;
#[inline]
pub fn parse(e:Executor,args:Vec<Value>)->Result<Value>{
    match args.len() {
        0=>Err(Error::InvalidArgs(1,0)),
        _=>{
            let script = args[0].invoke_to_string(&e);
            println!("script {}", script);
            Ok(
                match script.parse_zin()?{
                    Some(script) => script.ast_node().into(),
                    _=>Value::NULL
                }
            )
        }
    }
}
#[inline]
pub fn ast()->GcObject {
    GcObject::from(Object::new(access::ObjAccess::NONE)
    .with_name("ast").with_doc("abstract syntax tree for zin"))
    .with_member(ZString::from("parse"), Function::native_root(access::ObjAccess::NONE, parse).object())
}