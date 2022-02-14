use pomelo::pomelo;
use super::ast::*;
use crate::error::*;
use crate::types::Number;
use super::ast::Assignment;
use crate::zstring::ZBytes;
use crate::types::ZString;
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq,Eq,Default)]
pub struct ParserError {
    pub err:String,
}

impl ParserError {
    #[inline]
    pub fn err<T:std::string::ToString>(err:T)->ParserError {
        ParserError{err:err.to_string()}
    }
}
pomelo!{
     //%verbose;
      //%verbose;
     
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
    %type Ident ZString;
    %type Type ZString;
    %type Num Number;
    %type String ZString;
    %type Access Vec<ZString>;
    %type expr Expression;
    %type mme Expression;
    %type expr_list Vec<Box<Expression>>;
    %type hard_list Vec<Box<Expression>>;
    %type stmt Statement;
    %type input Option<Box<Statement>>;
    %type block Vec<Box<Statement>>;
    %type function Statement;
    %type decl_list Vec<Box<Statement>>;
    %type stmt_list Vec<Box<Statement>>;
    %type func_list Vec<Box<Statement>>;
    %type arg_list Vec<ZString>;
    %type map_list Vec<(Box<Expression>,Box<Expression>)>;
    %type obj_list Vec<(ZString,Box<Expression>)>;
    %type type_list Vec<ZString>;
    %type package ZString;
    %type f_decl Statement;
    %type Bool bool;
    %type type_t Vec<Box<Statement>>;
    %type type_s Vec<Box<Statement>>;
    %type decl Vec<Box<Statement>>;
    %type assignment (Box<Expression>,Box<Expression>);
    %type arg_assignment Assignment;
    %type ByteString ZBytes;
    %right RParen Else Comma RBracket;
    %right Equal Neq Not;
    %left Exp LParen Or Xor BOr BXor LBracket;
    %left And BAnd Less LessEq Greater GreaterEq Dot At OrDefault Range;
    %nonassoc Eq QuickAssign;
    %left Add Sub Shl Shr Str Bytes Len;
    %left Mul Div;

    input ::= type_s?(v){match v { 
        Some(v) =>Some(Box::new(Statement::Block(v))),
        _=>None
        }
    }
    
    type_t ::= stmt(a) {vec![Box::new(a)]}
    type_s ::= type_t(t) {t}
    type_s ::= type_s(mut a) type_t(b) {a.extend(b);a}

    function ::= Ident(name) Arrow LParen arg_list?(args)  RParen String?(doc) block(code) {
        Statement::Function(name, 
            args.unwrap_or_else(Vec::new),
            doc,
            Box::new(Statement::Block(code)))
    }
    
    stmt ::= function(a){a}
    stmt ::= Extern Ident(name) Arrow LParen arg_list?(args) RParen String?(doc) block(code) {
        Statement::ExternFunction(name, 
            args.unwrap_or_else(Vec::new),
            doc,
            Box::new(Statement::Block(code)))}



    arg_list ::= Ident(n)  [Semicolon] { vec![n] }  
    arg_list ::= arg_list(mut args) Comma Ident(n) { args.push(n); args }
    

    block ::= LBrace stmt_list?(ss) RBrace { ss.unwrap_or(Vec::new()) }
    
    stmt_list ::= stmt(s) { vec![Box::new(s)] }
    stmt_list ::= stmt_list(mut ss) stmt(s) { ss.push(Box::new(s)); ss }
    stmt ::= block(ss) { Statement::Block(ss) }

    stmt ::= expr(e) Semicolon {
        match e {
            Expression::Import(name)=>{
                let nameo = name.split('.').map(|x| x.into()).collect::<Vec<ZString>>();
                Statement::NewAssignment(vec![nameo.last().unwrap().clone()],Box::new(Expression::Import(name)))
            }
            _=>Statement::Expr(Box::new(e))
        } 
    }
    
    stmt ::= arg_list(a) Eq expr(b) Semicolon { Statement::Assignment(a,Box::new(b)) }
    
    stmt ::= Var arg_list(a) Eq expr(b) Semicolon { Statement::NewAssignment(a,Box::new(b)) }
    //stmt ::= Ident(a) QuickAssign expr(b) Semicolon { Statement::NewAssignment(a,b) }
    stmt ::= Extern arg_list(a) QuickAssign expr(b) Semicolon { Statement::ExternAssignment(a,Box::new(b)) }
    stmt ::= Extern arg_list(a) Eq expr(b) Semicolon { Statement::ExternAssignment(a,Box::new(b)) }
    stmt ::= arg_list(a) QuickAssign expr(b) Semicolon { Statement::NewAssignment(a,Box::new(b)) }
    


    stmt ::= Access(a) Eq expr(b) Semicolon { Statement::AccessAssignment(Box::new(Expression::Dotted(a)),Box::new(b)) }
    stmt ::= Access(a) QuickAssign expr(b) Semicolon {Statement::AccessAssignment(Box::new(Expression::Dotted(a)),Box::new(b)) }
    
    
    stmt ::= expr(n) LBracket expr(a) RBracket Eq|QuickAssign expr(b) Semicolon {Statement::ArraySet(Box::new(n),Box::new(a),Box::new(b)) }
    
    
    stmt ::= If expr(e) block(s1) Else stmt(s2) {Statement::If(Box::new(e), Box::new((Statement::Block(s1), Some(Box::new(s2)))))  }
    stmt ::= If expr(e) block(s1) [Else] { Statement::If(Box::new(e), Box::new((Statement::Block(s1), None))) }
    
    stmt ::= While expr(e) block(s) { Statement::While(Box::new(e), Box::new(Statement::Block(s))) }
    stmt ::= Return expr(e) Semicolon { Statement::Return(Some(Box::new(e))) }
    stmt ::= Return Semicolon { Statement::Return(None) }
    stmt ::= Break Semicolon { Statement::Break }
    stmt ::= Continue Semicolon {Statement::Continue }
    stmt ::= For arg_list(a) In expr(e) block(s) { Statement::For(a,Box::new(e),Box::new(Statement::Block(s)))}

    stmt ::= Try block(a) Catch arg_list?(c) block(b) [Finally] { Statement::Try(Box::new(Statement::Block(a)),c,None,Box::new(Statement::Block(b))) }
    stmt ::= Try block(a) Catch arg_list?(f) block(b) Finally block(c) { Statement::TryFinally(Box::new(Statement::Block(a)),f,None,Box::new(Statement::Block(b)),Box::new(Statement::Block(c))) }
    stmt ::= Try block(a) Catch arg_list?(c) As Ident(as_) block(b) [Finally] { Statement::Try(Box::new(Statement::Block(a)),c,Some(as_),Box::new(Statement::Block(b))) }
    stmt ::= Try block(a) Catch arg_list?(f) As Ident(as_) block(b) Finally block(c) { Statement::TryFinally(Box::new(Statement::Block(a)),f,Some(as_),Box::new(Statement::Block(b)),Box::new(Statement::Block(c))) }

    stmt ::= Raise Semicolon { Statement::RaiseCurrent}
    stmt ::= Raise Ident(e) Semicolon { Statement::RaiseNorm(e)}
    stmt ::= Raise Ident(e) Because expr(w) Semicolon { Statement::RaiseNormWithWhy(e,Box::new(w))}
    stmt ::= Lift expr(e) Semicolon { Statement::RaiseObj(Box::new(e))};
    mme ::= Access(a) {Expression::Dotted(a)}
    mme ::= Ident(a) {Expression::Variable(a)}
    mme ::= expr(a) Dot Ident(b) { Expression::InlineDotted(Box::new(a),vec![b])}
    mme ::= expr(a) Dot Access(b) { Expression::InlineDotted(Box::new(a),b)}

    expr ::= mme(a) {a}
    expr ::= Dollar LParen arg_list?(args) RParen String?(doc) block(code) {
        Expression::Lambda(args.unwrap_or(Vec::new()),Box::new(Statement::Block(code)),doc )
    }
    expr ::= Null {Expression::None}
    expr ::= Dollar String(a) {Expression::MakeString(Box::new(Expression::LString(a)))}
    expr ::= Dollar ByteString(a) {Expression::MakeBytes(Box::new(Expression::BString(a)))}
    expr ::= Dollar Num(a) {Expression::MakeBytes(Box::new(Expression::Number(a)))}
    
    expr ::= Str expr(a) {Expression::MakeString(Box::new(a))}
    expr ::= Bytes expr(a) {Expression::MakeString(Box::new(a))}
    expr ::= Len expr(a) {Expression::Len(Box::new(a))}
    expr ::= Colon Ident(a){Expression::LString(a)};
    expr ::= Bool(a) {Expression::Bool(a)}
    expr ::= Num(n) { Expression::Number(n) }
    expr ::= String(n) {Expression::LString(n)}
    expr ::= ByteString(n) {Expression::BString(n)}
    
  
    expr ::= expr(a) At Ident(b) {Expression::Call(Box::new(Expression::Variable(b)),Box::new(a)) }
    expr ::= expr(a) At Access(b) {Expression::Call(Box::new(Expression::Dotted(b)),Box::new(a)) }
    //expr ::= expr(a) At expr(a) Dot Access(b) {Expression::Call(Box::new(Expression::Dotted(b)),Box::new(a)) }
    expr ::= LParen hard_list(a) RParen At Ident(b) {Expression::Call(Box::new(Expression::Variable(b)),Box::new(Expression::List(a)))}
    expr ::= LParen hard_list(a) RParen At Access(b) {Expression::Call(Box::new(Expression::Dotted(b)),Box::new(Expression::List(a)))}
    
    expr ::= LParen expr(a) RParen At Ident(b) {Expression::Call(Box::new(Expression::Variable(b)),Box::new(Expression::List(vec![Box::new(a)])))}
    expr ::= LParen expr(a) RParen At Access(b) {Expression::Call(Box::new(Expression::Dotted(b)),Box::new(Expression::List(vec![Box::new(a)])))}
   
    expr ::= LParen RParen At Ident(b) {Expression::Call(Box::new(Expression::Variable(b)),Box::new(Expression::List(Vec::new())))}
    expr ::= LParen RParen At Access(b) {Expression::Call(Box::new(Expression::Dotted(b)),Box::new(Expression::List(Vec::new())))}


    expr ::= Import Ident(n) { Expression::Import(n)}
    expr ::= Import Access(na) {
        let mut n = String::new();
        for i in na {
            n.push_str(&i);
            n.push('.');
        } 
        n.pop();
        Expression::Import( n.into())
    }
    expr ::=  expr(n) LParen expr_list?(es) RParen { Expression::Call(Box::new(n), Box::new(Expression::List(es.unwrap_or(Vec::new()))))}
    
    expr ::= LParen expr(e) RParen { e }
    expr ::= LParen expr(e) RParen LParen expr_list?(es) RParen {
        Expression::Call(Box::new(e),Box::new(Expression::List(es.unwrap_or(Vec::new()))))
     }
    expr ::= LBracket expr_list?(es) RBracket {Expression::Array(es.unwrap_or(Vec::new()))}

    expr ::= This { Expression::This}
    expr ::= expr(n) LBracket expr(a) RBracket { Expression::ArrayAccess(Box::new(n),Box::new(a))}
    
    expr ::= expr(n) LBracket expr(a) Range RBracket { Expression::SliceMEnd(Box::new(n),Box::new(a))}
    
    expr ::= expr(n) LBracket Range RBracket { Expression::SliceMBE(Box::new(n)) }
    
    expr ::= expr(n) LBracket expr(a) Range expr(b) RBracket { Expression::Slice(Box::new(n),Box::new(a),Box::new(b))}

    expr ::= expr(n) LBracket Range expr(a) RBracket { Expression::SliceMBegin(Box::new(n),Box::new(a))}



    expr ::= LParent expr(n) RParen LBracket expr(a) RBracket { Expression::ArrayAccess(Box::new(n),Box::new(a))}
    expr ::= LParent expr(n) RParen LBracket expr(a) Range RBracket { Expression::SliceMEnd(Box::new(n),Box::new(a))}
    expr ::= LParent expr(n) RParen LBracket Range expr(a) RBracket { Expression::SliceMBegin(Box::new(n),Box::new(a))}
    expr ::= LParent expr(n) RParen LBracket Range RBracket { Expression::SliceMBE(Box::new(n))}
    expr ::= LParent expr(n) RParen LBracket expr(a) Range expr(b) RBracket { Expression::Slice(Box::new(n),Box::new(a),Box::new(b))}


    expr ::= Dollar LBracket map_list?(a) RBracket { Expression::Map(a.unwrap_or(Vec::new()))}
    
    expr ::= Dollar LBrace obj_list?(a) RBrace { Expression::Object(a.unwrap_or(Vec::new()))}
    expr ::= Dollar Ident(name) LBrace obj_list?(a) RBrace { Expression::NamedObject(name,a.unwrap_or(Vec::new()))}
  
    expr ::= expr(a) Add expr(b) { if let Expression::Number(aa) = a{
        if let Expression::Number(bb) = b {
            Expression::Number(aa+bb)
        } else {
            Expression::Add(Box::new(a),Box::new(b) )
        }
    } else{
        Expression::Add(Box::new(a),Box::new(b) ) 
    }} 
    expr ::= expr(a) Sub expr(b) { if let Expression::Number(aa) = a{
        if let Expression::Number(bb) = b {
            Expression::Number(aa-bb)
        } else {
            Expression::Sub(Box::new(a), Box::new(b))
        }
    } else{
        Expression::Sub(Box::new(a), Box::new(b)) 
    }} 
    expr ::= expr(a) Mul expr(b) { if let Expression::Number(aa) = a{
        if let Expression::Number(bb) = b {
            Expression::Number(aa*bb)
        } else {
            Expression::Mul(Box::new(a),Box::new(b) )
        }
    } else{
        Expression::Mul(Box::new(a),Box::new(b) ) 
    }} 
    expr ::= expr(a) Div expr(b) { if let Expression::Number(aa) = a{
        if let Expression::Number(bb) = b {
            Expression::Number(aa/bb)
        } else {
            Expression::Div(Box::new(a),Box::new(b) )
        }
    } else{
        Expression::Div(Box::new(a),Box::new(b) ) 
    }}
    expr ::= expr(a) Exp expr(b) { if let Expression::Number(aa) = a{
        if let Expression::Number(bb) = b {
            Expression::Number(aa.exp(bb))
        } else {
            Expression::Exp(Box::new(a),Box::new(b) )
        }
    } else{
        Expression::Exp(Box::new(a),Box::new(b) ) 
    }}
    expr ::= expr(a) Shl expr(b) { if let Expression::Number(aa) = a{
        if let Expression::Number(bb) = b {
            Expression::Number(aa<<bb)
        } else {
            Expression::Exp(Box::new(a),Box::new(b) )
        }
    } else{
        Expression::Exp(Box::new(a),Box::new(b) ) 
    }}
    expr ::= expr(a) Shr expr(b) { if let Expression::Number(aa) = a{
        if let Expression::Number(bb) = b {
            Expression::Number(aa>>bb)
        } else {
            Expression::Exp(Box::new(a),Box::new(b) )
        }
    } else{
        Expression::Exp(Box::new(a),Box::new(b) ) 
    }}

    expr ::= Sub expr(a) [Not] { Expression::Negate(Box::new(a)) }

    expr ::= expr(a) Equal expr(b) { Expression::Equals(Box::new(a),Box::new(b)) }
    expr ::= expr(a) Neq expr(b) { Expression::Neq(Box::new(a),Box::new(b)) }

    expr ::= expr(a) And expr(b) { Expression::And(Box::new(a),Box::new(b)) }
    expr ::= expr(a) Or expr(b) { Expression::Or(Box::new(a),Box::new(b)) }

    expr ::= expr(a) OrDefault expr(b) { Expression::OrDefault(Box::new(a),Box::new(b)) }

    
    expr ::= expr(a) Xor expr(b) { Expression::Xor(Box::new(a),Box::new(b)) }
    expr ::= expr(a) BAnd expr(b) { Expression::BAnd(Box::new(a),Box::new(b)) }
    expr ::= expr(a) BOr expr(b) { Expression::BOr(Box::new(a),Box::new(b)) }
    expr ::= expr(a) BXor expr(b) { Expression::BXor(Box::new(a),Box::new(b)) }
    
    expr ::= Not expr(a) { Expression::Not(Box::new(a)) }

    expr ::= expr(a) Less expr(b) { Expression::Less(Box::new(a),Box::new(b)) }
    expr ::= expr(a) Greater expr(b) { Expression::Great(Box::new(a),Box::new(b)) }
    expr ::= expr(a) LessEq expr(b) { Expression::LessEq(Box::new(a),Box::new(b)) }
    expr ::= expr(a) GreaterEq expr(b) { Expression::GreatEq(Box::new(a),Box::new(b)) }

    map_list ::= expr(a) Colon expr(b) {
        vec![(Box::new(a),Box::new(b))]
    }
    
    map_list ::= map_list(mut l) Comma expr(a) Colon expr(b) {
        l.push((Box::new(a),Box::new(b)));
        l
    }
    map_list ::= map_list(l) Comma {
        l
    }
    
    obj_list ::= Ident(a) Colon expr(b) {
        vec![(a,Box::new(b))]
    }
    obj_list ::= obj_list(mut l) Comma Ident(a) Colon expr(b) { 
        l.push((a,Box::new(b)));
        l
    }
    obj_list ::= obj_list(mut l) Comma { 
        l
    }
    expr_list ::= expr(e) { vec![Box::new(e)] }
    expr_list ::= expr_list(mut es) Comma expr(e) { es.push(Box::new(e)); es }
    hard_list ::= expr(e) Comma expr(x) { vec![Box::new(e),Box::new(x)]}
    hard_list ::= hard_list(mut es) Comma expr(e) { es.push(Box::new(e)); es }
}

pub use parser::*;