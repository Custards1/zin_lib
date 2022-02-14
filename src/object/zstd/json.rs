
use crate::types::*;
use crate::object::*;
use crate::error::*;
use crate::api::lexer;
use crate::api::node::ParserError;
use function::Function;
use super::builtin;
use interpreter::Executor;

use pomelo::*;
pub mod properties {
    static_tstring!(decode);
    static_tstring!(encode);
}
pomelo!{
       
    %include {
        use super::*;
 
    }
    %error ParserError;
    %syntax_error{
        Err(ParserError::err(format!("syntax {:?}",token)))
    }
    %parse_fail {
        ParserError::err("unkown (maybe missing a semicolon)".to_string())
    }
    
    %token #[derive(Debug)] pub enum Token {}; 
    %type value Value;
    %type Num Number;
    %type String ZString;
    %type array Vec<Value>;
    %type elems Vec<Value>;
    %type Bool bool;
    %type props PropertyMap;
    %type object Value;
    %type program Option<Value>;
    %right Comma RBrace RBracket;
    %left LBrace LBracket; 
    program ::= value?(a) {a}

    value ::= Null {Value::NULL}
    value ::= Num(a){Value::Num(a)}
    value ::= String(a){Value::String(a.into())}
    value ::= array(a){Value::Object(builtin::vec_from(a))}
    value ::= object(a)  {a}
    array ::= LBracket elems?(a) RBracket {a.unwrap_or(Vec::new())}
    object ::= LBrace props?(a) RBrace {Value::Object(builtin::map_from(a.unwrap_or(PropertyMap::new())))}

    props ::= String(aa) Colon value(v) {
        let mut a = PropertyMap::new();
        a.insert(Value::String(aa.into()), v);
        a
    }
    props ::=props(mut a) Comma String(aa) Colon value(v) {
        a.insert(Value::String(aa.into()), v);
        a
    }
    elems ::= value(a) {vec![a]}
    elems ::= elems(mut a) Comma value(b) {a.push(b);a}
    
}

pub use parser::*;
use std::convert::TryFrom;
impl TryFrom<lexer::Token> for Token {
    type Error = Error;
    fn try_from(t: lexer::Token) -> Result<Token> {
        Ok(match t {
            lexer::Token::Null =>Token::Null,
            lexer::Token::ZString(a) =>Token::String(a.0),
            lexer::Token::LBrace => Token::LBrace,
            lexer::Token::RBrace=> Token::RBrace,
            lexer::Token::LBracket=> Token::LBracket,
            lexer::Token::RBracket=> Token::RBracket,
            lexer::Token::False=> Token::Bool(false),
            lexer::Token::True=> Token::Bool(true),
            lexer::Token::Num(a)=> Token::Num(a),
            lexer::Token::Comma=> Token::Comma,
            lexer::Token::Colon=> Token::Colon,
            _=>return Err(Error::ParserError("unknown json syntax".to_string()))
        })
    }
}
#[inline]
pub fn fill(val:&mut String,indent:char,amt:usize) {
    for _ in 0..amt {
        val.push(indent);
    }
}
#[inline]
pub fn to_json(val:Vec<Token>,mut indent:usize)->Result<String> {
    let mut json = String::with_capacity(indent);
    fill(&mut json,'\t',indent);
    for i in val {
        match i {
            Token::Null=>{
                json.push_str("null");
            },
            Token::String(a)=>{
                json.push('"');
                json.push_str(&a);
                json.push('"');
            }
            Token::LBracket=>{
                json.push('{');
                json.push('\n');
                indent+=1;
                fill(&mut json,'\t',indent);
            }
            Token::RBracket=>{
                json.push('}');
                if indent >0{
                    indent-=1;
                }
            }
            Token::LBrace=>{
                json.push('[');
                json.push('\n');
                indent+=1;
                fill(&mut json,'\t',indent);
            }
            Token::RBrace=>{
                json.push('\n');
                json.push(']');
                if indent >0{
                    indent-=1;
                }
            }
            Token::Bool(a)=>{
                json.push_str(match a {
                    true=>"true",
                    false=>"false"
                    }
                );
            }
            Token::Num(a)=>{
                json.push_str(&a.to_string());
            }
            Token::Comma=>{
                json.push_str(",\n");
                fill(&mut json,'\t',indent);
            }

            Token::Colon=>{
                json.push_str(": ");
            }
        }
        
    }
    Ok(json)
}

use logos::Logos;
use std::convert::TryInto;
#[inline]
pub fn decode_value(string:ZString)->Result<Value> {
    let mut parser = Parser::new();
    let mut stream = lexer::Token::lexer(&string);
    let mut line = 1;
    while let Some(tok) = stream.next() {
        match tok {
            lexer::Token::Error=>{
                return Err(Error::ParserErrorNear("unknown token".to_string(),stream.span(),stream.slice().into(),line))
            }
            lexer::Token::Line=>{
                line+=1;
            }
            _=>match parser.parse(match tok.try_into(){
                Ok(a)=>a,
                _=>return Ok(Value::NULL)
            }){
                Ok(_) =>{}
                Err(_)=>return Ok(Value::NULL),
            }
        }
        
    }
    match parser.end_of_input(){
        Ok(a) =>Ok(a.unwrap_or(Value::NULL)),
        Err(e)=>Err(Error::ParserErrorNear(e.err,stream.span(),stream.slice().into(),line))
    }
}
#[inline]
pub fn decode(e:Executor,args:Vec<Value>)->Result<Value> {
    match args.len() {
        0=>Ok(Value::NULL),
        _=>{
            decode_value(args[0].invoke_to_string(&e))
        }
    }
}
#[inline]
fn encode_value(exec:&Executor,arg:Value,exclude:&mut Vec<GcObject>,obj_hook:Option<GcObject>) -> Result<Vec<Token>> {
    Ok(match &arg {
        Value::NULL=>vec![Token::Null],
        Value::Bool(a)=>vec![Token::Bool(*a)],
        Value::Bytes(a)=>match std::str::from_utf8(&a) {
            Ok(a)=>vec![Token::String(a.into())],
            _=>vec![Token::Null]
        }
        Value::String(a)=>vec![Token::String(a.clone())],
        Value::Char(a)=>vec![Token::String(a.to_string().into())],
        Value::Num(a)=>vec![Token::Num(*a)],
        Value::Object(a)=>{
            if !super::builtin::implication::gc_contains(exclude.as_ref(),&a) {
                exclude.push(a.clone());
                match a.inner.try_borrow() {
                    Ok(a)=>match &a.vec{
                        Some(InnerContainer::Vector(InternedVector::Vector(vec)))=>{
                            let mut toks = vec![Token::LBrace];
                            let pops = vec.len() >1;
                            for i in vec {
                                toks.extend(encode_value(exec,i.clone(), exclude,obj_hook.clone())?);
                                toks.push(Token::Comma);
                            }
                            if pops {
                                toks.pop();
                            }
                            toks.push(Token::RBrace);
                            
                            toks
                        }
                        Some(InnerContainer::Vector(InternedVector::ByteVector(vec))) => {
                            match std::str::from_utf8(&vec) {
                                Ok(vec) =>vec![Token::String(vec.into())],
                                _=>vec![Token::Null]
                            }
                        }
                        Some(InnerContainer::String(a))=>{
                            vec![Token::String(a.as_str().into())]
                        }
                        Some(InnerContainer::Map(vec))=>{
                            let mut toks = vec![Token::LBracket];
                            let pops = vec.len() >1;
                            for (i,j) in vec.iter() {
                                toks.extend(encode_value(exec,i.clone(), exclude,obj_hook.clone())? );
                                toks.push(Token::Colon);
                                toks.extend(encode_value(exec,j.clone(), exclude,obj_hook.clone())?);
                                toks.push(Token::Comma);
                            }
                            if pops {
                                toks.pop();
                            }
                            toks.push(Token::RBracket);
                            toks
                        }
                        _=>match obj_hook {
                            Some(hook)=>{
                                match exec.call(hook.clone(), vec![a.clone().into()]){
                                    Ok(a)=> encode_value(exec,a.clone(),exclude,None)?,
                                    _=>vec![Token::Null]
                                }
                            }
                            _=>vec![Token::Null]
                        }
                    },
                    _=>return Err(Error::InvalidBorrow(file!(),line!()))
                }
            } else {
                vec![Token::Null]
            }
        }
            
    })
}
pub fn encode(e:Executor,args:Vec<Value>)->Result<Value> {
    match args.len() {
        0=>Ok(Value::NULL),
        1=>{
            let mut exclude =Vec::new();
            Ok(super::builtin::string_from(to_json(encode_value(&e,args[0].clone(), &mut exclude,None)?,0 )?).into())
        }
        2=>{
            let mut exclude:Vec<GcObject> =Vec::new();
            match &args[1] {
                Value::Object(a)=>
                Ok(super::builtin::string_from(to_json(encode_value(&e,args[0].clone(), &mut exclude,Some(a.clone()))?,0 )?).into()),
                _=>Ok(super::builtin::string_from(to_json(encode_value(&e,args[0].clone(), &mut exclude,None)?,0 )?).into())
            }  
        }
        _=>{
            let mut exclude:Vec<GcObject> =Vec::new();
            let idx = match Number::try_from(args[2].clone()){
                Ok(a)=>a,
                _=>0.into()
            };
            match &args[1] {
                Value::Object(a)=>
                Ok(super::builtin::string_from(to_json(encode_value(&e,args[0].clone(), &mut exclude,Some(a.clone()))?,idx.usize() )?).into()),
                _=>Ok(super::builtin::string_from(to_json(encode_value(&e,args[0].clone(), &mut exclude,None)?,idx.usize() )?).into())
            }  
        }
    }
    
}

const JSON_NAME:&'static str = "json";
const JSON_DOC:&'static str = "json provides methods to encode and decode json values from/to zin values";

pub fn json()->GcObject {
    GcObject::from(Object::new(access::ObjAccess::NONE).with_name(JSON_NAME)).with_doc(JSON_DOC)
    .with_member(properties::decode(), Function::native_root(access::ObjAccess::NONE, decode))
    .with_member(properties::encode(), Function::native_root(access::ObjAccess::NONE, encode))
}