use crate::types::*;
use refrence::*;
use crate::error::*;
use super::node;
use logos::Logos;
use crate::object::*;
use crate::object::zstd::builtin;
pub trait Parseable {
    fn parse_zin(&self)->Result<Option<Box<Statement>>>;
}
impl<L: ?Sized + std::convert::AsRef<str>> Parseable for L {
    #[inline]
    fn parse_zin(&self)->Result<Option<Box<Statement>>>{
        //println!("gots me a steams of {:?}",tokens);
        let source = self.as_ref();
        if source.is_empty() {
            return Ok(None);
        }
        let mut parser = node::Parser::new();
        let mut stream = super::lexer::Token::lexer(source);
        let mut line = 1;
        while let Some(tok) = stream.next() {
            match tok {
                super::lexer::Token::Error=>{
                    return Err(Error::ParserErrorNear("unknown token".to_string(),stream.span(),stream.slice().into(),line))
                }
                super::lexer::Token::Line=>{
                    line+=1;
                }
                super::lexer::Token::ZString((string,lines))=>{
                    
                    match parser.parse(node::Token::String(string)){
                        Ok(_) =>{}
                        Err(e)=>return Err(Error::ParserErrorNear(e.err,stream.span(),stream.slice().into(),line))
                    }
                    line+=lines;
                }
                super::lexer::Token::Bytes((bytes,lines))=>{
                    
                    match parser.parse(node::Token::ByteString(bytes)){
                        Ok(_) =>{}
                        Err(e)=>return Err(Error::ParserErrorNear(e.err,stream.span(),stream.slice().into(),line))
                    }
                    line+=lines;
                }
                _=>match parser.parse(tok.into()){
                    Ok(_) =>{}
                    Err(e)=>return Err(Error::ParserErrorNear(e.err,stream.span(),stream.slice().into(),line))
                }
            }
            
        }
        match parser.end_of_input(){
            Ok(a) =>Ok(a),  
            Err(e)=>Err(Error::ParserErrorNear(e.err,stream.span(),stream.slice().into(),line))
        }
    }
}

#[derive(Clone,Debug)]
pub enum Statement {
    ArraySet(Box<Expression>,Box<Expression>,Box<Expression>),
    Expr(Box<Expression>),
    AccessAssignment(Box<Expression>,Box<Expression>),
    Assignment(Vec<ZString> /*Name*/,Box<Expression>),
    NewAssignment(Vec<ZString> /*Name*/,Box<Expression>),
    ExternAssignment(Vec<ZString> /*Name*/,Box<Expression>),
    Block(Vec<Box<Statement>>),
    If(Box<Expression>,Box<(Statement,Option<Box<Statement>>)>),
    While(Box<Expression>,Box<Statement>),
    Function(ZString,Vec<ZString>,Option<ZString>,Box<Statement>),
    ExternFunction(ZString,Vec<ZString>,Option<ZString>,Box<Statement>),
    Return(Option<Box<Expression>>),
    For(Vec<ZString>,Box<Expression>,Box<Statement>),
    Try(Box<Statement>,Option<Vec<ZString>>,Option<ZString>,Box<Statement>),
    TryFinally(Box<Statement>,Option<Vec<ZString>>,Option<ZString>,Box<Statement>,Box<Statement>),
    RaiseCurrent,
    RaiseNorm(ZString),
    RaiseNormWithWhy(ZString,Box<Expression>),
    RaiseObj(Box<Expression>),
    Break,
    Continue,
    None
}

use function::Compileable;
struct Lambda(Vec<ZString>,Box<Statement>,Option<ZString>,Option<ZString>);
#[inline(always)]
pub fn make_ast_node(name: &'static str ,value:Value) ->GcObject {
    GcObject::from(Object::new(access::ObjAccess::NONE).with_name(name).
    with_member("value", value).with_member("__group", "expr"))
}
#[inline(always)]
pub fn make_stmt_ast_node(name: &'static str ,value:Value) ->GcObject {
    GcObject::from(Object::new(access::ObjAccess::NONE).with_name(name).
    with_member("value", value).with_member("__group", "statement"))
}

enum LambdaVal {
    Upval(bool,Vec<Instruction>),
    Static(function::Function)
}
impl Lambda {
    fn compile(self,builtin:&GcObject,lib:&mut function::Function,blocks:&mut BlockChain,constants:&mut function::ConstantLib)->Result<LambdaVal> {
        let mut block = WizardBlock::new();
        let nargs = self.0.len();
        for arg in &self.0 {
            match block.try_create_index(arg.clone()){
                Some(_)=>{},
                None=>return Err(Error::ParserError(format!("Duplicate argument: {}",arg)))
            }
        }
        blocks.push(block);
       let mut is_method = false;
        let mut function = 
        match nargs {
            0=>function::Function::child_callable(access::ObjAccess::NONE,0,false),
            _=>match self.0[0].as_ref() {
                "self"=>{
                    
                    is_method=true;
                    function::Function::child_callable(access::ObjAccess::NONE,nargs-1,is_method)
                },
                _=>{
                    function::Function::child_callable(access::ObjAccess::NONE,nargs,is_method)
                }
            }
        };
      
        let top_level = blocks.len()<=1;
        let stmt:Statement = *self.1;
        let mut code = stmt.compile(builtin,&mut function, blocks,constants)?;
        match code.last() {
            Some(a)=>match a {
                Instruction::Return(_)=>{}
                _=>{
                    
                    code.push(Instruction::Return(0));
                }
                },
                None=>code.push(Instruction::Return(0))
            }
        let block = blocks.pop().unwrap();
        let nlocals = block.local.len();
        match self.3 {
            Some(doc)=>{
                function.inner.set_doc(doc);
            }
            _=>{}
        }
        function.set_nlocals(nlocals as u32);
        function.set_code(code,is_method);
        match self.2 {
            Some(name)=>{
                function.inner.set_name(name);
            }
            _=>{

            }
        }
        let len = block.upvals.len();
        match len {
            0=>{
                Ok(LambdaVal::Static(function))
            },
            _=>{
                let mut ret = vec![Instruction::Cld(function.into())];
                for i in 0..len{
                    match block.upvals.all_at_index(i) {
                        Some((id,(_,(a,b,_)))) =>{
                            if let Some(aa) = blocks.last_mut() { 
                                
                                match aa.local.index_of(id.clone()) {
                                    Some(a) => {
                                        ret.push(Instruction::Ldl(0,a))
                                    }
                                    _=>match aa.upvals.index_of(id.clone()){
                                        Some(xx)=>{
                                            ret.push(Instruction::Ldup(xx));
                                            
                                        }
                                        _=>{
                                            if *a >0 {
                                                
                                                ret.push(Instruction::Ldup(aa.upvals.get_or_create_index(id.clone(),(*a,*b))));
                                                   
                                            } else {
                                                ret.push(Instruction::Ldl(*a,*b));
                                            }
                                            
                                        }
                                    }
                                }
                                
                                
                            }else {
                                ret.push(Instruction::Ldl(*a,*b))
                            }
                            
                           
                            
                        }
                        None=>panic!("huh?")
                    }
                }
                ret.push(Instruction::MkFunc(len));
                Ok(LambdaVal::Upval(block.local.len() == 0,ret))
            }
        }
    }
}

#[derive(Debug,Clone)]
pub enum Expression {
    Import(ZString),
    Dotted(Vec<ZString>),
    Tuple(Box<Expression>),
    InlineDotted(Box<Expression>,Vec<ZString>),
    Len(Box<Expression>),
    Bool(bool),
    Number(Number),
    LString(ZString),
    BString(ZBytes),
    Variable(ZString),
    MakeString(Box<Expression>),
    MakeBytes(Box<Expression>),
    None,
    This,
    List(Vec<Box<Expression>>),
    Add(Box<Expression>,Box<Expression>),
    Sub(Box<Expression>,Box<Expression>),
    Mul(Box<Expression>,Box<Expression>),
    Lambda(Vec<ZString>,Box<Statement>,Option<ZString>),
    Div(Box<Expression>,Box<Expression>),
    Exp(Box<Expression>,Box<Expression>),
    Neq(Box<Expression>,Box<Expression>),
    Equals(Box<Expression>,Box<Expression>),
    Shl(Box<Expression>,Box<Expression>),
    Shr(Box<Expression>,Box<Expression>),
    Call(Box<Expression>,Box<Expression>),
    Negate(Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>,  Box<Expression>),
    OrDefault(Box<Expression>,  Box<Expression>),
    Not(Box<Expression>),
    Xor(Box<Expression>, Box<Expression>),
    BAnd(Box<Expression>,Box<Expression>),
    Less(Box<Expression>,Box<Expression>),
    Great(Box<Expression>,Box<Expression>),
    LessEq(Box<Expression>,Box<Expression>),
    GreatEq(Box<Expression>,Box<Expression>),
    Object(Vec<(ZString,Box<Expression>)>),
    NamedObject(ZString,Vec<(ZString,Box<Expression>)>),
    BOr(Box<Expression>, Box<Expression>),
    BXor(Box<Expression>,Box<Expression>),
    Map(Vec<(Box<Expression>,Box<Expression>)>),
    Array(Vec<Box<Expression>>),
    
    ArrayAccess(Box<Expression>,Box<Expression>),
    Slice(Box<Expression>,Box<Expression>,Box<Expression>),
    SliceMBegin(Box<Expression>,Box<Expression>),
    SliceMEnd(Box<Expression>,Box<Expression>),
    SliceMBE(Box<Expression>),
}

macro_rules! binary_compile {
    ($inst:expr,$a:expr,$b:expr,$lib:expr,$blocks:expr,$builtin:expr,$constants:expr) => {
        {
            let mut aazzxca_ie = $a.compile($builtin,$lib, $blocks,$constants)?;
            aazzxca_ie.extend($b.compile($builtin,$lib, $blocks,$constants)?);
            aazzxca_ie.push($inst);
            aazzxca_ie
        }
        
    };
}
macro_rules! binary_compile_into_zin_ast {
    ($name:literal,$a:ident,$b:ident) => {
        {
            make_ast_node($name,builtin::vec_from(
                vec![Value::from($a.ast_node()),
             Value::from($b.ast_node())]).into())
        }
        
    };
}
macro_rules! compile_into_zin_stmt_ast {
    ($name:literal,$a:ident,$b:ident) => {
        {
            make_stmt_ast_node($name,builtin::vec_from(
                vec![Value::from($a.ast_node()),
             Value::from($b.ast_node())]).into())
        }
        
    };
}
macro_rules! tri_compile_into_zin_ast {
    ($name:literal,$a:ident,$b:ident,$c:ident) => {
        {
            make_ast_node($name,builtin::vec_from(
                vec![Value::from($a.ast_node()),
                     Value::from($b.ast_node()),
                     Value::from($c.ast_node())]).into())
        }
        
    };
}
macro_rules! tri_compile_into_zin_stmt_ast {
    ($name:literal,$a:ident,$b:ident,$c:ident) => {
        {
            make_stmt_ast_node($name,builtin::vec_from(
                vec![Value::from($a.ast_node()),
                     Value::from($b.ast_node()),
                     Value::from($c.ast_node())]).into())
        }
        
    };
}

impl function::Compileable for Expression {
    #[inline]
    fn compile(self,builtin:&GcObject,lib:&mut function::Function,blocks:&mut BlockChain,constants:&mut function::ConstantLib)->Result<Vec<Instruction>> {
        Ok(match self {
            Expression::Import(a) => {
                let (_,string) = constants.get_or_create_constant(a);
                
                vec![Instruction::Import(string)]
            }
            Expression::Dotted(mut a)=>{
                let mut iter = a.iter_mut();
                let var = iter.next().unwrap();
                match var.as_ref() {
                    "this"=>{
                        let mut code = vec![Instruction::This];
                        for d in iter {
                            let (_,string) = constants.get_or_create_constant(d.clone());
                            code.push(Instruction::Ldm(string.into()));    
                        }
                        code
                    }
                    _=>match blocks.compiled_var(var.clone(),builtin,lib,constants) {
                        Some(mut code)=>{
                            for d in iter {
                                let (_,string) = constants.get_or_create_constant(d.clone());
                                code.push(Instruction::Ldm(string.into()));    
                            }
                            code
                        },
                        _=>return Err(Error::UnknownVariable(var.to_string()))
                    }
                }
                
            }
            Expression::Len(a)=>{
                let mut a = a.compile(builtin,lib, blocks,constants)?;
                a.push(Instruction::Len);
                a
            }
            Expression::InlineDotted(mut b,mut a)=>{
                let mut iter = a.iter_mut();
                let mut a = b.compile(builtin,lib, blocks,constants)?;
                for d in iter {
                    let (_,string) = constants.get_or_create_constant(d.clone());
                    a.push(Instruction::Ldm(string.into()));    
                }
                a
            }
            Expression::This=>{
                vec![Instruction::This]
            }
            Expression::MakeString(a)=>{
                let mut a = a.compile(builtin,lib, blocks,constants)?;
                a.push(Instruction::Str);
                a
            }
            Expression::MakeBytes(a)=>{
                let mut a = a.compile(builtin,lib, blocks,constants)?;
                a.push(Instruction::Bytes);
                a
            }
            Expression::None=>{
                vec![Instruction::Cld(Value::NULL)]
            }
            Expression::Bool(a)=>vec![match a {
             true=>Instruction::Cl1,
             false=>Instruction::Cl0,
            }],
            Expression::Number(a)=>{
                vec![Instruction::Cld(a.into())]    
            }
            Expression::LString(a)=>{
                let (_,string) = constants.get_or_create_constant(a);
                vec![Instruction::Cld(string.into())]
            }
            Expression::BString(a)=>{
                let (_,string) = constants.get_or_create_bconstant(a);
                vec![Instruction::Cld(string.into())]
            }
            Expression::Variable(b)=>{
                match blocks.compiled_var(b.clone(),builtin,lib,constants) {
                    Some(a)=>a,
                    _=>return Err(Error::UnknownVariable(b.to_string()))
                }
            }
            Expression::Lambda(a,b,c)=>{
                match Lambda(a,b,None,c).compile(builtin,lib, blocks, constants)? {
                    LambdaVal::Static(function)=>{
                        vec![Instruction::Cld(function.into())]
                    }
                    LambdaVal::Upval(_,b)=>{
                        b
                    }
                }
            }


            Expression::Call(a,b)=> {
                let mut a = a.compile(builtin,lib, blocks,constants)?;
                match *b {
                    Expression::List(b) => {
                        let call = Instruction::Call(b.len());
                        for i in b {
                            a.extend(i.compile(builtin,lib, blocks,constants)?);
                        }
                        a.push(call);
                        
                    }
                    Expression::None=>{
                        a.push(Instruction::Call(0));
                    }
                    _=>{
                        a.extend(b.compile(builtin,lib, blocks,constants)?);
                        a.push(Instruction::Call(1));
                        
                    }
                };
                a
               
            },
            Expression::List(a)=> {
                let mut out = Vec::new();
                for i in a {
                    out.extend(i.compile(builtin,lib, blocks,constants)?);
                }
                out
            }
            Expression::Add(a,b)=> binary_compile!(Instruction::Add,a,b,lib,blocks,builtin,constants),
            Expression::Sub(a,b)=> binary_compile!(Instruction::Sub,a,b,lib,blocks,builtin,constants),
            Expression::Mul(a,b)=> binary_compile!(Instruction::Mul,a,b,lib,blocks,builtin,constants),
            Expression::Div(a,b)  => binary_compile!(Instruction::Div,a,b,lib,blocks,builtin,constants),
            Expression::Equals(a,b)=>binary_compile!(Instruction::Cmp,a,b,lib,blocks,builtin,constants),
            Expression::Neq(a,b)=>{
            let mut code = binary_compile!(Instruction::Cmp,a,b,lib,blocks,builtin,constants);
            code.push(Instruction::Negate);
            code
            },
            Expression::Xor(a,b)=>binary_compile!(Instruction::AXor,a,b,lib,blocks,builtin,constants),
            Expression::BXor(a,b)=>binary_compile!(Instruction::Xor,a,b,lib,blocks,builtin,constants),
            Expression::Negate(a)=>{
                let mut code = a.compile(builtin,lib, blocks,constants)?;
                code.push(Instruction::Negate);
                code
            }
            Expression::And(a,b)=>binary_compile!(Instruction::AAnd,a,b,lib,blocks,builtin,constants),
            Expression::BAnd(a,b)=>binary_compile!(Instruction::And,a,b,lib,blocks,builtin,constants),
            
            Expression::Or(a,b)=>binary_compile!(Instruction::AOr,a,b,lib,blocks,builtin,constants),
            Expression::OrDefault(a,b)=>{
                let mut code = a.compile(builtin,lib, blocks,constants)?;
                code.push(Instruction::Rtest(0));
                let idx = code.len()-1;
                let c =b.compile(builtin,lib, blocks,constants)?;
                code[idx] = Instruction::Rtest(c.len()+1);
                code.extend(c);
                code
            }
            Expression::BOr(a,b)=>binary_compile!(Instruction::Or,a,b,lib,blocks,builtin,constants),
            
            Expression::Less(a,b)=>binary_compile!(Instruction::Less,a,b,lib,blocks,builtin,constants),
            Expression::Great(a,b)=>binary_compile!(Instruction::Great,a,b,lib,blocks,builtin,constants),
            Expression::LessEq(a,b)=>binary_compile!(Instruction::LessEq,a,b,lib,blocks,builtin,constants),
            Expression::GreatEq(a,b)=>binary_compile!(Instruction::GreatEq,a,b,lib,blocks,builtin,constants),
            Expression::Array(a)=>{
                let mut out = Vec::new();
                let len = a.len();
                for i in a {
                    out.extend(i.compile(builtin,lib, blocks,constants)?);
                }
                out.push(Instruction::MkArr(len));
                out
            }
            Expression::ArrayAccess(a,b)=>{
                let mut out = a.compile(builtin,lib, blocks,constants)?;
                out.extend(b.compile(builtin,lib, blocks,constants)?);
                out.push(Instruction::ArrayAccess);
                out
            }
            Expression::Slice(a,b,c)=>{
                let mut out = a.compile(builtin,lib, blocks,constants)?;
                out.extend(b.compile(builtin,lib, blocks,constants)?);
                out.extend(c.compile(builtin,lib, blocks,constants)?);
                out.push(Instruction::Slice(false));
                out
            }
            Expression::SliceMBegin(a,b)=>{
                let mut out = a.compile(builtin,lib, blocks,constants)?;
                out.push(Instruction::Cld(0usize.into()));
                out.extend(b.compile(builtin,lib, blocks,constants)?);
                out.push(Instruction::Slice(false));
                out
            }
            Expression::SliceMEnd(a,b)=>{
                let mut out = a.compile(builtin,lib, blocks,constants)?;
                out.extend(b.compile(builtin,lib, blocks,constants)?);
                out.push(Instruction::Slice(true));
                out
            }
            Expression::SliceMBE(a)=>{
                let mut out = a.compile(builtin,lib, blocks,constants)?;
                out.push(Instruction::Cld(0usize.into()));
                out.push(Instruction::Slice(true));
                out
            }
            Expression::Map(a)=>{
                let mut out = Vec::new();
                let len = a.len();
                for (i,j) in a {
                    out.extend(i.compile(builtin,lib, blocks,constants)?);
                    out.extend(j.compile(builtin,lib, blocks,constants)?);
                }
                out.push(Instruction::MkMap(len));
                out
            }
            Expression::Object(a)=>{
                let mut out = Vec::new();
                let len = a.len();
                for (i,j) in a {
                    let (_,string) = constants.get_or_create_constant(i);
                    out.push(Instruction::Cld(string.into()));
                    out.extend(j.compile(builtin,lib, blocks,constants)?);
                }
                out.push(Instruction::Formula(len));
                out
            }
            Expression::NamedObject(name,a)=>{
                let (_,name) = constants.get_or_create_constant(name);
                let mut out = Vec::new();
                let len = a.len();
                for (i,j) in a {
                    let (_,string) = constants.get_or_create_constant(i);
                    out.push(Instruction::Cld(string.into()));
                    out.extend(j.compile(builtin,lib, blocks,constants)?);
                }
                out.push(Instruction::Cld(name.into()));
                out.push(Instruction::NamedFormula(len));
                out
            }
            _=>unimplemented!()
        })
    }
}
impl Expression {
    #[inline]
    pub fn ast_node(self) ->GcObject {
        match self {
            Expression::Bool(a)=>{
                make_ast_node("Bool",a.into())
            }
            Expression::This=>{
                make_ast_node("This",Value::NULL)
            }
            Expression::Variable(a)=>{
                make_ast_node("Variable",a.into())
            }
            Expression::Tuple(a)=>{
                make_ast_node("Tuple",a.ast_node().into())
            }
            
            Expression::LString(a)=>{
                make_ast_node("Str", a.into())
            }
            Expression::BString(a)=>{
                make_ast_node("Bytes", a.into())
            }
            Expression::Number(n) =>{
                make_ast_node("Number",n.into())
            }
            Expression::Import(a)=>{
                make_ast_node("Import",a.into())
            }
            Expression::Dotted(a)=>{
                let mut values:Vec<Value> = Vec::with_capacity(a.len());
                for value in a {
                    values.push(make_ast_node("String",value.into()).into());
                }
                make_ast_node("Dotted",builtin::vec_from(values).into())
            }
            Expression::Len(a)=>{
                make_ast_node("Len", a.ast_node().into())
            }
            Expression::InlineDotted(mut b,mut a)=>{
                let mut values:Vec<Value> = vec![b.ast_node().into()];
                for value in a {
                    values.push(make_ast_node("String",value.into()).into());
                }
                make_ast_node("Dotted",builtin::vec_from(values).into())
            }
            Expression::MakeString(a)=>{
                make_ast_node("MakeString",a.ast_node().into())
            }
            Expression::MakeBytes(a)=>{
                make_ast_node("MakeByteString",a.ast_node().into())
            }
            Expression::None=>{
                make_ast_node("None",Value::NULL)
            }
            Expression::Lambda(a,b,c)=>{

                let args: Vec<Value>= a.iter().map(|x|x.clone().into()).collect();

                make_ast_node("Lambda",builtin::vec_from(
                                       vec![Value::from(make_ast_node("Args",builtin::vec_from(args).into())),
                                       match c{
                                        Some(doc)=>make_ast_node("String", doc.into()).into(),
                                        _=>Value::NULL
                                       },
                                       b.ast_node().into()]).into())
            }


            Expression::Call(a,b)=> {
                make_ast_node("Call",builtin::vec_from(vec![Value::from(a.ast_node()),Value::from(b.ast_node())]).into() )
            },
            Expression::List(a)=> {
                let mut out:Vec<Value> = Vec::new();
                for i in a {
                    out.push(i.ast_node().into());
                }
                make_ast_node("ExprList",builtin::vec_from(out).into())
            }
            Expression::Add(a,b)=> binary_compile_into_zin_ast!("Add",a,b),
            Expression::Sub(a,b)=> binary_compile_into_zin_ast!("Sub",a,b),
            Expression::Mul(a,b)=> binary_compile_into_zin_ast!("Mul",a,b),
            Expression::Div(a,b)  => binary_compile_into_zin_ast!("Div",a,b),
            Expression::Equals(a,b)=>binary_compile_into_zin_ast!("Equals",a,b),
            Expression::Xor(a,b)=>binary_compile_into_zin_ast!("Xor",a,b),
            Expression::BXor(a,b)=>binary_compile_into_zin_ast!("BinXor",a,b),
            Expression::And(a,b)=>binary_compile_into_zin_ast!("And",a,b),
            Expression::BAnd(a,b)=>binary_compile_into_zin_ast!("BinAnd",a,b),
            Expression::Or(a,b)=>binary_compile_into_zin_ast!("Or",a,b),
            Expression::OrDefault(a,b)=>binary_compile_into_zin_ast!("Defs",a,b),
            Expression::BOr(a,b)=>binary_compile_into_zin_ast!("BinOr",a,b),
            Expression::Neq(a,b)=>binary_compile_into_zin_ast!("Neq",a,b),
            Expression::Shl(a,b)=>binary_compile_into_zin_ast!("Shl",a,b),
            Expression::Shr(a,b)=> binary_compile_into_zin_ast!("Shr",a,b),
            Expression::Exp(a,b)=> binary_compile_into_zin_ast!("Exp",a,b),
            Expression::Negate(a)=>{
                make_ast_node("Negate",a.ast_node().into())
            }
            Expression::Not(a)=>{
                make_ast_node("Not",a.ast_node().into())
            }
            Expression::Less(a,b)=>binary_compile_into_zin_ast!("Less",a,b),
            Expression::Great(a,b)=>binary_compile_into_zin_ast!("Greater",a,b),
            Expression::LessEq(a,b)=>binary_compile_into_zin_ast!("LessEq",a,b),
            Expression::GreatEq(a,b)=>binary_compile_into_zin_ast!("GreaterEq",a,b),
            Expression::Array(a)=>{
                let mut out:Vec<Value> = Vec::new();
                let len = a.len();
                for i in a {
                    out.push(i.ast_node().into());
                }
                make_ast_node("Array", builtin::vec_from(out).into())
            }
            Expression::ArrayAccess(a,b)=>binary_compile_into_zin_ast!("ArrayAccess",a,b),
            Expression::Slice(a,b,c)=>{
                tri_compile_into_zin_ast!("Slice",a,b,c)
            }
            Expression::SliceMBegin(a,b)=>{
                let c = Expression::Number(0usize.into());
                tri_compile_into_zin_ast!("Slice",a,c,b)
            }
            Expression::SliceMEnd(a,b)=>{
                let c = Expression::Number(std::usize::MAX.into());
                tri_compile_into_zin_ast!("Slice",a,b,c)
            }
            Expression::SliceMBE(a)=>{
                let b = Expression::Number(0usize.into());
                let c = Expression::Number(std::usize::MAX.into());
                tri_compile_into_zin_ast!("Slice",a,b,c)
            }
            Expression::Map(a)=>{
                let mut out:Vec<Value> = Vec::new();
                let len = a.len();
                for (i,j) in a {
                    out.push(i.ast_node().into());
                    out.push(j.ast_node().into());
                }
                make_ast_node("Map", builtin::vec_from(out).into())
            }
            Expression::Object(a)=>{
                let mut out:Vec<Value> = Vec::new();
                let len = a.len();
                for (i,j) in a {
                    out.push(make_ast_node("String",i.into()).into() );
                    out.push(j.ast_node().into());
                }
                make_ast_node("Object", builtin::vec_from(out).into())
            }
            Expression::NamedObject(name,a)=>{
                let mut out:Vec<Value> = Vec::new();
                let len = a.len();
                for (i,j) in a {
                    out.push(make_ast_node("String",i.into()).into() );
                    out.push(j.ast_node().into());
                }
                let obj:Vec<Value> = vec![make_ast_node("String",name.into()).into(),builtin::vec_from(out).into()];

                make_ast_node("NamedObject", builtin::vec_from(obj).into())
            }
        }
    }
}
impl Statement {
    #[inline]
    pub fn ast_node(self) -> GcObject {
        match self {
            Statement::While(cond,body)=>{
                make_stmt_ast_node("While", 
                builtin::vec_from(vec![Value::from(cond.ast_node()),body.ast_node().into()]).into())
            }
            Statement::TryFinally(trys,err,name,catches,finally)=>{
                make_stmt_ast_node("TryFinally", 
                builtin::vec_from(vec![trys.ast_node().into(),
                if let Some(err) = err {
                    let mut out:Vec<Value> = Vec::new();
                    for i in err {
                        out.push(make_ast_node("String",i.into()).into())
                    }
                    make_ast_node("String",builtin::vec_from(out).into()).into()
                } else{Value::NULL},
                if let Some(name) = name {
                    make_ast_node("String",name.clone().into()).into()
                } else{Value::NULL} ,
                catches.ast_node().into(),
                finally.ast_node().into()]).into())
            }
            Statement::Try(trys,err,name,catches) =>{
                make_stmt_ast_node("Try", 
                builtin::vec_from(vec![trys.ast_node().into(),
                if let Some(err) = err {
                    let mut out:Vec<Value> = Vec::new();
                    for i in err {
                        out.push(make_ast_node("String",i.into()).into())
                    }
                    make_ast_node("String",builtin::vec_from(out).into()).into()
                } else{Value::NULL},
                if let Some(name) = name {
                    make_ast_node("String",name.clone().into()).into()
                } else{Value::NULL} ,
                catches.ast_node().into()]).into())
            }
            Statement::Return(a)=>{
                match a {
                    Some(a)=>{
                        make_stmt_ast_node("Return",a.ast_node().into())
                    }
                    _=>make_stmt_ast_node("Return", Value::NULL)
                }
            }
            Statement::Block(a)=>{
                let mut out:Vec<Value> = Vec::new();
                for i in a {
                    out.push(i.ast_node().into())
                }
                make_stmt_ast_node("Block", builtin::vec_from(out).into())
            }
            Statement::RaiseObj(a)=>{
                make_stmt_ast_node("Lift", a.ast_node().into())
            }
            Statement::RaiseNormWithWhy(a,b)=> {
                make_stmt_ast_node("Raise",builtin::vec_from(
                    vec![Value::from(make_ast_node("String",a.into())),
                    b.ast_node().into()]
                ).into())
            }
            Statement::RaiseNorm(a)=>{
                make_stmt_ast_node("Raise",a.into())
            }
            Statement::RaiseCurrent=>{
                make_stmt_ast_node("Raise", Value::NULL)
            }
            Statement::None=>{
                make_stmt_ast_node("None",Value::NULL)
            }
            Statement::NewAssignment(names,value)=>{
                let args: Vec<Value>= names.iter().map(|x|x.clone().into()).collect();
                make_stmt_ast_node("NewAssignment",builtin::vec_from(vec![
                    Value::from(make_ast_node("Names",builtin::vec_from(args).into())),
                    value.ast_node().into(),
                    ]).into())
            }
            Statement::ExternAssignment(names,value)=>{
                let args: Vec<Value>= names.iter().map(|x|x.clone().into()).collect();
                make_stmt_ast_node("ExternAssignment",builtin::vec_from(vec![
                    Value::from(make_ast_node("Names",builtin::vec_from(args).into())),
                    value.ast_node().into(),
                    ]).into())
            }
            Statement::Assignment(names,value)=>{
                let args: Vec<Value>= names.iter().map(|x|x.clone().into()).collect();
                make_stmt_ast_node("Assignment",builtin::vec_from(vec![
                    Value::from(make_ast_node("Names",builtin::vec_from(args).into())),
                    value.ast_node().into(),
                    ]).into())
            }
            Statement::If(a,b)=>{
                match b.1 {
                    Some(c)=>{
                        make_stmt_ast_node("IfElse",
                        builtin::vec_from(
                            vec![Value::from(a.ast_node()),
                                 Value::from(c.ast_node()),
                            ]).into() )
                    }
                    _=>{
                        make_stmt_ast_node("If",builtin::vec_from(vec![Value::from(a.ast_node())]).into() )
                    }
                }
                
            }
            Statement::Function(name,args,docs,code)=>{
                let args: Vec<Value>= args.iter().map(|x|x.clone().into()).collect();
                make_stmt_ast_node("Function", builtin::vec_from(vec![
                    Value::from(make_ast_node("String",name.into())),
                    make_ast_node("Args",builtin::vec_from(args).into()).into(),
                    match docs{
                        Some(doc)=>make_ast_node("String", doc.into()).into(),
                        _=>Value::NULL
                    },
                    code.ast_node().into()
                ]).into())
            }
            Statement::For(names,iter,body)=>{
                let names: Vec<Value>= names.iter().map(|x|x.clone().into()).collect();
                make_stmt_ast_node("For", builtin::vec_from(vec![
                    Value::from(make_ast_node("Names",builtin::vec_from(names).into())),
                    iter.ast_node().into(),
                    body.ast_node().into()
                ]).into())
            }
            Statement::ExternFunction(name,args,docs,code)=>{
                let args: Vec<Value>= args.iter().map(|x|x.clone().into()).collect();
                make_stmt_ast_node("ExternFunction", builtin::vec_from(vec![
                    Value::from(make_ast_node("String",name.into())),
                    make_ast_node("Args",builtin::vec_from(args).into()).into(),
                    match docs{
                        Some(doc)=>make_ast_node("String", doc.into()).into(),
                        _=>Value::NULL
                    },
                    code.ast_node().into()
                ]).into())
            }
            Statement::Expr(e)=>{
                make_stmt_ast_node("SExpr", e.ast_node().into())
            }
            Statement::Continue=>make_stmt_ast_node("Continue",Value::NULL),
            Statement::Break=>make_stmt_ast_node("Break",Value::NULL),
            Statement::ArraySet(a,b,c)=>tri_compile_into_zin_stmt_ast!("ArraySet",a,b,c),
            Statement::AccessAssignment(a,b)=>compile_into_zin_stmt_ast!("AccessAssignment",a,b),
        }
    }
}
use std::convert::TryInto;
impl function::Compileable for Statement {
    #[inline]
    fn compile(self,builtin:&GcObject,lib:&mut function::Function,blocks:&mut BlockChain,constants:&mut function::ConstantLib)->Result<Vec<Instruction>> {
        Ok(match self {
            Statement::ArraySet(a,b,c)=>{
                let mut out = c.compile(builtin,lib, blocks,constants)?;
                out.extend(a.compile(builtin,lib, blocks,constants)?);
                out.extend(b.compile(builtin,lib, blocks,constants)?);
                out.push(Instruction::ArraySet);
                out
            }
            Statement::Expr(e)=>{
               let mut c =  e.compile(builtin,lib, blocks,constants)?;
               if c.len() >0 {
                c.push(Instruction::Pop);
               }
               c
            },
    
            Statement::AccessAssignment(a,b) => {
                let mut code = b.compile(builtin,lib, blocks,constants)?;
                code.extend(a.compile(builtin,lib, blocks,constants)?);
                let tt = code.pop().unwrap().into_loaded();
                code.push(tt);
                code
            }
            Statement::None=>vec![],

            Statement::Block(e)=>{
                let mut out = Vec::new();
                for i in e {
                    out.extend(i.compile(builtin,lib, blocks,constants)?)
                }
                out
            }
            Statement::Function(name,args,docs,code)=>{
                let (_,name) = constants.get_or_create_constant(ZString::from(name));
                let val = Lambda(args,code,Some(name.clone()),docs);
                match val.compile(builtin,lib, blocks, constants)? {
                    LambdaVal::Static(function)=>{
                        let j = blocks.len();
                        let b = match j {
                            0=>return Err(Error::MalformedStack),
                            _=>j
                        }-1;
                        let a = blocks[b].get_or_create_index(name);
                        vec![Instruction::Cld(function.into()),if a.0 {
                            Instruction::Stup(a.1)
                        } else {
                            Instruction::Stl(0,a.1)
                        }]
                    }
                    LambdaVal::Upval(glob,mut code)=>{
                        let j = blocks.len();
                        let b = match j {
                            0=>return Err(Error::MalformedStack),
                            _=>j
                        }-1;
                        let a = blocks[b].get_or_create_index(name);
                        code.push( if a.0 {
                            Instruction::Stup(a.1)
                        } else {
                            Instruction::Stl(0,a.1)
                        }
                        );
                        code
                    }
                }
            },
            Statement::ExternFunction(name,args,docs,code)=>{
                let (_,name) = constants.get_or_create_constant(ZString::from(name));
                lib.ensure_value_exists(name.clone());
                let val = Lambda(args,code,Some(name.clone()),docs);
                match val.compile(builtin,lib, blocks, constants)? {
                    LambdaVal::Static(function)=>{
                        lib.set_member(name, function);
                        //vec![Instruction::Cld(function.into()),Instruction::Cld(lib.clone().into()),Instruction::Stm(name.into())]
                        vec![]
                    }
                    LambdaVal::Upval(glob,mut code)=>{
                        code.push(Instruction::This);
                        code.push(Instruction::Stm(name.into()));
                        code
                    }
                }
            }
            Statement::If(cond,then)=>{
                let mut out = cond.compile(builtin,lib,blocks,constants)?;
                let (a,b) = *then;
                out.push(Instruction::Test(0));
                let index = out.len()-1;
                out.extend(a.compile(builtin,lib,blocks,constants)?);
                match b {
                    Some(g)=>{
                        
                        out.push(Instruction::Jmp(0));
                        let out_index = out.len()-1;
                        out[index] = Instruction::Test(out.len()-index);

                        out.push(Instruction::Test(0));
                        let index = out.len()-1;
                        out.extend(g.compile(builtin,lib,blocks,constants)?);
                        out[index] = Instruction::Ktest((out.len()-index));
                        out[out_index]= Instruction::Jmp((out.len()-index) as i64);
                    }
                    _=>{
                        out[index] = Instruction::Ktest((out.len()-index));
                    }
                };
               
                out
            },
            Statement::While(expr,blok)=>{
                let mut out = expr.compile(builtin,lib,blocks,constants)?;
                out.push(Instruction::Ktest(0));
                let index = out.len()-1;
                out.extend(blok.compile(builtin,lib,blocks,constants)?);
                let loop_end = out.len()+1;
                out.push(Instruction::Jmp(-((loop_end) as i64)));
                let loop_begin = out.len()-index;
                out[index]= Instruction::Ktest(loop_begin);
              

                let mut pos = 0;
                for i in &mut out {
                    match i {
                        Instruction::Pg45Break=>{
                            if pos > 0 && loop_end > 0 {
                                *i=Instruction::Jmp((loop_end-pos-1) as i64);
                            }
                        },
                        Instruction::Pg45Continue=>{
                            if pos > 0 && loop_end > 0 {
                                //println!("pusoentu {} {} {}",loop_begin,pos,-((loop_begin-pos-1) as i64));
                                *i=Instruction::Jmp(- ((pos+1) as i64) );
                            }
                            //*i=Instruction::Jmp(-(pos as i64));
                        },
                        _=>{}
                    }
                    pos+=1;
                }
                out
            },
            Statement::For(iname,expr,bl)=>{
                let mut code = expr.compile(builtin,lib, blocks,constants)?;
                let begin = code.len();
                code.extend_from_slice(&[Instruction::Dup,
                                        Instruction::Call(0),
                                        Instruction::Unpack(2),
                                        Instruction::Rot,
                                        Instruction::Ktest(0)]);
                                        
                let index = code.len()-1;
                let j = blocks.len();
                let b = match j {
                    0=>return Err(Error::MalformedStack),
                    _=>j
                }-1;
                let len = iname.len();
                if len > 1 {
                    code.push(Instruction::Unpack(len))
                }
                for name in iname {
                    let a = blocks[b].local.get_or_create_index(name);
                    code.push(Instruction::Stl(0,a));
                }
                code.extend(bl.compile(builtin,lib, blocks,constants)?);
                let diff =  (code.len()+1)-begin;
                let mut pos = 0;
                for i in &mut code.iter_mut().skip(begin) {
                    match i {
                        Instruction::Pg45Break=>{
                            if pos > 0 && diff > 0 {
                                *i=Instruction::Jmp((diff-pos-1) as i64);
                            }
                        },
                        Instruction::Pg45Continue=>{
                            if pos > 0 && diff > 0 {
                                //println!("pusoentu {} {} {}",loop_begin,pos,-((loop_begin-pos-1) as i64));
                                *i=Instruction::Jmp(- ((pos+1) as i64) );
                            }
                            //*i=Instruction::Jmp(-(pos as i64));
                        },
                        _=>{}
                    }
                    pos+=1;
                }
                
                code.push(Instruction::Jmp(-(diff as i64)));
                let loop_begin = code.len()-index;
                code.push(Instruction::Pop);
                code.push(Instruction::Pop);
                
                code[index]= Instruction::Ktest(loop_begin);
               
                code
            
            }
            Statement::Return(a)=>{
                match a {
                    Some(b)=>{
                        let mut code = Vec::new();
                        code.extend(b.compile(builtin,lib,blocks,constants)?);
                        code.push(Instruction::Return(1));
                        code
                    },
                    None=>vec![Instruction::Return(0)]
                }
            },
            Statement::Assignment(iname,value)=> {
                
                let mut code = value.compile(builtin,lib, blocks,constants)?;
                let len = iname.len();
                if len > 1 {
                    code.push(Instruction::Unpack(len))
                }
                for name in iname {
                    match blocks.load_compiled_var(name.clone(),builtin,lib,constants) {
                        Some(a)=>code.extend(a),
                        None=>return Err(Error::UnknownVariable(name.to_string()))
                    };
                }
                code
            }
            Statement::NewAssignment(iname,value)=>{
                let mut code = value.compile(builtin,lib, blocks,constants)?;
                let j = blocks.len();
                let b = match j {
                    0=>return Err(Error::MalformedStack),
                    _=>j
                }-1;
                let len = iname.len();
                if len > 1 {
                    code.push(Instruction::Unpack(len))
                }
                for name in iname {
                    let a = blocks[b].local.get_or_create_index(name);
                    code.push(Instruction::Stl(0,a));
                }
                code
            }
            Statement::ExternAssignment(iname,value)=>{
                let mut code = value.compile(builtin,lib, blocks,constants)?;
                let len = iname.len();
                if len > 1 {
                    code.push(Instruction::Unpack(len))
                }
                for name in iname {
                    let (_,name) = constants.get_or_create_constant(name);
                    lib.set_member(name.clone(), Value::NULL);
                    code.extend([Instruction::This,Instruction::Stm(name.clone().into())]); 
                }
               
                code
            },

            Statement::Try(trys,err,maybe_name,catches_exp) => {
                let mut code = vec![Instruction::Ltb(0)];
                let mut trys = trys.compile(builtin,lib, blocks,constants)?;
                code[0] = Instruction::Ltb(trys.len()+2);
                code.extend(trys);
                code.push(Instruction::Ftb(0));
                let index = code.len()-1;
                
                let mut catches =Vec::new();
                match err {
                    Some(iname)=>{
                        let len = iname.len();
                        for name in iname {
                            code.push(Instruction::Cld(name.into()));
                        }
                        code.push(Instruction::ESwitch(len));
                    }
                    _=>{
                        code.push(Instruction::Err);
                    }
                }
                
                
                match maybe_name {
                    Some(name) => {
                        let j = blocks.len();
                        let b = match j {
                            0=>return Err(Error::MalformedStack),
                            _=>j
                        }-1; 
                        let a = blocks[b].local.get_or_create_index(name);
                        catches.push(Instruction::Stl(0,a));
                    }
                    None => {
                        catches.push(Instruction::Pop);
                    }
                }
                let j = blocks.len();
                let b = match j {
                    0=>return Err(Error::MalformedStack),
                    _=>j
                }-1; 
    
                catches.extend(catches_exp.compile(builtin,lib, blocks,constants)?); 
                catches.push(Instruction::Fcb);
                let clen = catches.len();
                code[index]= Instruction::Ftb(clen+2);
                code.extend(catches);
                code
            }
            Statement::TryFinally(trys,err,maybe_name,catches_exp,finally_exp) => {
                let mut code = vec![Instruction::Ltb(0)];
                let mut trys = trys.compile(builtin,lib, blocks,constants)?;
                code[0] = Instruction::Ltb(trys.len()+2);
                code.extend(trys);
                code.push(Instruction::Ftb(0));
                let index = code.len()-1;
                
                let mut catches =Vec::new();
                match err {
                    Some(iname)=>{
                        let len = iname.len();
                        for name in iname {
                            code.push(Instruction::Cld(name.into()));
                        }
                        code.push(Instruction::ESwitch(len));
                    }
                    _=>{
                        code.push(Instruction::Err);
                    }
                }
                
                
                match maybe_name {
                    Some(name) => {
                        let j = blocks.len();
                        let b = match j {
                            0=>return Err(Error::MalformedStack),
                            _=>j
                        }-1; 
                        let a = blocks[b].local.get_or_create_index(name);
                        catches.push(Instruction::Stl(0,a));
                    }
                    None => {
                        catches.push(Instruction::Pop);
                    }
                }
                let j = blocks.len();
                let b = match j {
                    0=>return Err(Error::MalformedStack),
                    _=>j
                }-1; 
    
                catches.extend(catches_exp.compile(builtin,lib, blocks,constants)?); 
                catches.push(Instruction::Fcb);
                let clen = catches.len();
                
                let mut fin = finally_exp.compile(builtin,lib, blocks,constants)?;
                let flen = match fin.len().try_into() {
                    Ok(a)=>a,
                    _=>return Err(Error::AssertionFailed("too big a block for a jump".to_string()))
                };
                
                catches.push(Instruction::Jmp(flen));
                code[index]= Instruction::Ftb(clen+2);
                code.extend(catches);
                code.extend(fin);
                code
            }
            Statement::RaiseNorm(err)=>{
                let (_,err) = constants.get_or_create_constant(err);
                vec![Instruction::Cld(err.into()),Instruction::Raise(RaiseErrKind::FromStack)]
            }
            Statement::RaiseNormWithWhy(err,why)=>{
                let (_,err) = constants.get_or_create_constant(err);
                let mut code = vec![Instruction::Cld(err.into())];
                code.extend(why.compile(builtin,lib, blocks,constants)?);
                code.push(Instruction::Raise(RaiseErrKind::FromStackWithWhy));
                code
            }
            Statement::RaiseObj(obj)=>{
                let mut code = obj.compile(builtin,lib, blocks,constants)?;
                code.push(Instruction::Raise(RaiseErrKind::FromStackObj));
                code
            }
            Statement::RaiseCurrent=>vec![Instruction::Raise(RaiseErrKind::CurrentError)],

            Statement::Continue=>vec![Instruction::Pg45Continue],
            
            Statement::Break=>vec![Instruction::Pg45Break]
        })
    }
}

pub struct Assignment {
    pub name:String,
    pub expr:Expression
}


impl function::Compileable for Vec<Statement> {
    #[inline]
    fn compile(self,builtin:&GcObject,lib:&mut function::Function,blocks:&mut BlockChain,constants:&mut function::ConstantLib)->Result<Vec<Instruction>> {
        let mut out = Vec::new();
        for i in self {
            out.extend(i.compile(builtin,lib, blocks,constants)?);
        }
        Ok(out)
    }
}
impl function::Compileable for Vec<Box<Statement>> {
    #[inline]
    fn compile(self,builtin:&GcObject,lib:&mut function::Function,blocks:&mut BlockChain,constants:&mut function::ConstantLib)->Result<Vec<Instruction>> {
        let mut out = Vec::new();
        for i in self {
            out.extend(i.compile(builtin,lib, blocks,constants)?);
        }
        Ok(out)
    }
}
