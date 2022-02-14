use logos::*;

use crate::types::{Number,Signed,Unsigned,ZString};
use super::node;
use crate::zstring::ZBytes;

pub (crate) fn signed_num_callback<U>(lex: &mut Lexer<Token>,end:usize)->Option<Number>
where U:Into<Signed>+std::str::FromStr
{
    let  a ={
        let mut a = lex.slice().to_string();
        a.retain(|c| !c.is_whitespace() && c!=':'); 
        a
    };
    let n: U = a[..a.len() - end].parse::<U>().ok()?; // skip 'k'
    let g:Signed = n.into();
    Some(Number::from(g))
}

pub (crate) fn unsigned_num_callback<U>(lex: &mut Lexer<Token>,end:usize)->Option<Number>
where U:Into<Unsigned>+std::str::FromStr
{
    let mut a ={
        let mut a = lex.slice().to_string();
        a.retain(|c| !c.is_whitespace() && c!=':'); 
        a
    };
    let n: U = a[..a.len() - end].parse::<U>().ok()?; // skip 'k'
    let g:Unsigned = n.into();
    Some(Number::from(g))
}



#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("=>")]
    #[token("->")]
    Arrow,
    #[token("in")]
    In,
    #[token("not")]
    #[token("!")]
    Not,
    #[token(";")]
    Semicolon,
    #[token(".")]
    Dot,
    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("/")]
    Div,
    #[token("*")]
    Mul,
    #[token("**")]
    Exp,
    #[token("@")]
    At,
    #[token("as")]
    As,
    #[token("and")]
    #[token("&&")]
    And,
    #[token("&")]
    BAnd,
    #[token("or")]
    #[token("||")]
    Or,
    #[token("defs")]
    OrDefault,
    #[token("|")]
    BOr,
    #[token("xor")]
    #[token("^^")]
    XOr,
    #[token("^")]
    BXor,
    #[token("str")]
    Str,
    #[token("bytearr")]
    BytesKey,
    #[token("=")]
    Eq,
    #[token("isnt")]
    #[token("!=")]
    Neq,
    #[token("shl")]
    #[token("<<")]
    Shl,
    #[token("shr")]
    #[token(">>")]
    Shr,
    #[token("len")]
    Len,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEq,
    #[token(">")]
    Great,
    #[token(">=")]
    GreatEq,
    #[token("$")]
    Dollar,
    #[token("is")]
    #[token("==")]
    Equals,
    #[token("try")]
    Try,
    #[token("catch")]
    Catch,
    #[token("finally")]
    Finally,
    #[token("null")]
    Null,
    #[token("(")]
    LParen,
    #[token(")")]
    RParan,
    #[token("ret")]
    Return,
    #[token("if")]
    If,
    #[token("for")]
    For,
    #[token("while")]
    While,
    #[token("else")]
    Else,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token(",")]
    Comma,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("induct")]
    Import,
    #[token("..")]
    Range,
    #[token("{")]
    LBrace,
    #[token(":=")]
    QuickAssign,
    #[token(":")]
    Colon,
    #[token("}")]
    RBrace,
    #[token("let")]
    Var,
    #[token("raise")]
    Raise,
    #[token("because")]
    Because,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("pub")]
    Extern,
    #[token("lift")]
    Lift,
    #[token("this")]
    This,
    #[regex("[0-9]+", |lex| match lex.slice().parse::<i64>() {
        Ok(a)=>Ok(Number::from(Signed::from(a))),
        Err(e)=>Err(e)
    })]
    #[regex("[0-9]+\\.[0-9]+", |lex| match lex.slice().parse::<f64>() {
        Ok(a)=>Ok(Number::from(Signed::from(a))),
        Err(e)=>Err(e)
    })]
    #[regex("[0-9]+:i8", |lex| signed_num_callback::<i8>(lex, 2))]
    #[regex("[0-9]+:i16", |lex| signed_num_callback::<i16>(lex, 3))]
    #[regex("[0-9]+:i32", |lex| signed_num_callback::<i32>(lex, 3))]
    #[regex("[0-9]+:f32", |lex| signed_num_callback::<f32>(lex, 3))]
    #[regex("[0-9]+:f64", |lex| signed_num_callback::<f64>(lex, 3))]
    #[regex("[0-9]+\\.[0-9]+:f32", |lex| signed_num_callback::<f32>(lex, 3))]
    #[regex("[0-9]+\\.[0-9]+:f64", |lex| signed_num_callback::<f64>(lex, 3))]
    #[regex("[0-9]+:i64", |lex| signed_num_callback::<i64>(lex, 3))]
    #[regex("[0-9]+:u8", |lex|  unsigned_num_callback::<u8>(lex, 2))]
    #[regex("[0-9]+:u16", |lex| unsigned_num_callback::<u16>(lex, 3))]
    #[regex("[0-9]+:u32", |lex| unsigned_num_callback::<u32>(lex, 3))]
    #[regex("[0-9]+:u64", |lex| unsigned_num_callback::<u64>(lex, 3))]
    Num(Number),

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*(\\.[_a-zA-Z][_a-zA-Z0-9]*)+", |lex|
    Some(lex.slice().split('.').map(|q| {ZString::from(q)}).collect()))]
    OAccess(Vec<ZString>),

    #[regex("[a-zA-Z_][_a-zA-Z0-9]*", |lex| Some(lex.slice().into()))]
    Word(ZString),
    #[regex("\"(?:[^\"\\\\]|\\\\.)*\"" , |lex|
    let a =lex.slice(); 
    let len = a.matches('\n').count();
    match crate::stringify::unescape(&a[1..a.len()-1]){
        Ok(a)=>Some((a.into(),len)),
        _=>None
    })]
    ZString((ZString,usize)),
    #[regex("'(?:[^'\\\\]|\\\\.)*'" , |lex|
    let a =lex.slice();
    let len = a.matches('\n').count();
    match crate::stringify::unescape(&a[1..a.len()-1]){
        Ok(a)=>Some((ZBytes::from(a.as_bytes()),len)),
        _=>None
    })]
    Bytes((ZBytes,usize)),
    #[regex(r"#.*\n")]
    #[token("\n")]
    Line,
    
    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    
    #[regex(r"[ \t\f\s]+", logos::skip)]
    Error,
}




impl From<Token> for node::Token {
    #[inline]
    fn from(a:Token)->node::Token {
        match a { 
            Token::Semicolon       =>node::Token::Semicolon,
            Token::Add             =>node::Token::Add,
            Token::Sub             =>node::Token::Sub,
            Token::Div             =>node::Token::Div,
            Token::Mul             =>node::Token::Mul,
            Token::Exp             =>node::Token::Exp,
            Token::Eq              =>node::Token::Eq,
            Token::Shl             =>node::Token::Shl,
            Token::Shr             =>node::Token::Shr,
            Token::And             => node::Token::And,
            Token::Or              => node::Token::Or,
            Token::XOr             => node::Token::Xor ,
            Token::BAnd            => node::Token::BAnd ,
            Token::BOr             => node::Token::BOr,
            Token::BXor            => node::Token::BXor,
            Token::Colon           =>node::Token::Colon,
            Token::Lift             =>node::Token::Lift ,
            Token::QuickAssign     =>node::Token::QuickAssign,
            Token::This             =>node::Token::This,
            Token::Less            =>node::Token::Less,
            Token::Great           =>node::Token::Greater,
            Token::LessEq          =>node::Token::LessEq,
            Token::GreatEq         =>node::Token::GreaterEq,
            Token::Dollar          =>node::Token::Dollar,
            Token::Var             =>node::Token::Var,
            Token::Equals          =>node::Token::Equal,
            Token::LParen          =>node::Token::LParen,
            Token::As=>node::Token::As,
            Token::RParan          =>node::Token::RParen,
            Token::Return          =>node::Token::Return,
            Token::If              =>node::Token::If,
            Token::Dot             =>node::Token::Dot,
            Token::In              =>node::Token::In,
            Token::Str             =>node::Token::Str,
            Token::For             =>node::Token::For,
            Token::Len             =>node::Token::Len,
            Token::Break           =>node::Token::Break,
            Token::Continue        =>node::Token::Continue,
            Token::Else            =>node::Token::Else,
            Token::LBracket        =>node::Token::LBracket,
            Token::RBracket        =>node::Token::RBracket,
            Token::Comma           =>node::Token::Comma ,
            Token::Import          =>node::Token::Import,
            Token::Neq             =>node::Token::Neq,
            Token::Not             =>node::Token::Not,
            Token::OrDefault       =>node::Token::OrDefault,
            Token::Range           =>node::Token::Range,
            Token::LBrace          =>node::Token::LBrace,
            Token::RBrace          =>node::Token::RBrace,
            Token::Num(a)          =>node::Token::Num(a),
            Token::Try             =>node::Token::Try,
            Token::Catch           =>node::Token::Catch,
            Token::Finally         =>node::Token::Finally,
            Token::Word(a)         =>node::Token::Ident(a),
            Token::ZString(a)      =>node::Token::String(a.0),
            Token::Raise=>node::Token::Raise,
            Token::Because             =>node::Token::Because,
            Token::Bytes(a)        =>{
                node::Token::ByteString(a.0)   
            },
            Token::BytesKey        =>node::Token::Bytes,
            Token::Arrow           =>node::Token::Arrow,
            Token::OAccess(a)      =>node::Token::Access(a),
            Token::True            =>node::Token::Bool(true),
            Token::False           =>node::Token::Bool(false),
            Token::While           =>node::Token::While,
            Token::Null            =>node::Token::Null,
            Token::Extern          =>node::Token::Extern,
            Token::At              =>node::Token::At,
           // Token::Error           =>node::Token::Error           ,
            _=>unimplemented!()
        }
    }
}
