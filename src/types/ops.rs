use super::Value;
use crate::zstring::ZString;

use crate::types::number::Signed;
const OP_ARG_LBOUND:usize = 33;
#[derive(Debug,Clone,Copy,PartialEq,Hash,Eq,Ord,PartialOrd)]
pub enum RaiseErrKind {
    CurrentError,
    FromStack,
    FromStackWithWhy,
    FromStackObj,
}
impl RaiseErrKind {
    pub fn inst_name(&self) ->&'static str {
        match self{
            RaiseErrKind::CurrentError=> "cerr",
            RaiseErrKind::FromStack=> "last",
            RaiseErrKind::FromStackWithWhy=> "larg",
            RaiseErrKind::FromStackObj=> "obj",
        }
    }
}

#[derive(Debug,Clone,PartialEq,Hash,Eq,Ord,PartialOrd,gc::Trace,gc::Finalize)]
pub enum Instruction {
    Add,
    Sub,
    Div,
    Mul,
    Exp,
    Cl0,
    Cl1,
    Shl,
    Pop,
    Shr,
    Err /*pushes the current exception to the stack*/,
    Str,
    Bytes,
    ArrayAccess,
    ArraySet,
    Rot,
    Len,
    Fcb,
    Cmp,
    Negate,
    This,
    AAnd,
    And,
    AOr,
    Or,
    AXor,
    Xor,
    Less,
    Great,
    LessEq,
    GreatEq,
    Pg45Break, /*Will induce a no op, it is soley used to tag breakpoints,
    will not be seen in fully compiled object.*/
    Pg45Continue, /*Will induce a no op, it is soley used to tag continues,
    will not be seen in fully compiled object.*/
    Cld(Value),
    Import(ZString),

    Ldl(usize,usize),
    Stl(usize,usize),
    MkFunc(usize),
    MkArr(usize),
    MkMap(usize),
    Raise(#[unsafe_ignore_trace]
        RaiseErrKind),
    Slice(bool/*slice 2?*/),
    Dup,
    Ltb(usize),/*load try block */
    Ftb(usize),/*finish try block,pop try block from stack and jump pc by arg */
    ESwitch(usize),
    Formula(usize),
    NamedFormula(usize),
    
    Unpack(usize),
    Ldm(ZString),
    Stm(ZString),
    Ldup(usize),
    Stup(usize),
    Call(usize),
    Return(usize),
    Jmp(i64),
    Test(usize),
    Ktest(usize),
    Rtest(usize),
    Builtin(ZString),
    StoreBuiltin(ZString),
}
impl Instruction{
    pub fn into_loaded(self)->Self {
        match &self{
            Instruction::Ldl(a,b)=>Instruction::Stl(*a,*b),
            Instruction::Ldup(a)=>Instruction::Stup(*a),
            Instruction::Ldm(v)=>Instruction::Stm(v.clone()),
            Instruction::Builtin(a)=>Instruction::StoreBuiltin(a.clone()),
            _=>self
        }
    }
}

use crate::error::*;
pub type InstructionSlice = gc::GcCell<Vec<Instruction>>;





#[inline]
pub(crate) fn current_code(code:&InstructionSlice,pc:usize)->Result<Option<Instruction>> {
    if let Ok(a) = code.try_borrow() {
        return Ok(match a.get(pc){
            Some(a)=>Some(a.clone()),
            _=>None
        })
    }
    Err(Error::InvalidBorrow(file!(),line!()))
}
#[inline]
pub(crate) fn current_code_in_obj(code:&crate::object::GcObject,pc:usize)->Result<Option<Instruction>> {
    if let Ok(a) = code.try_borrow() {
        return match a.function() {
            Some(a)=>match &a.code {
                crate::object::ObjFunction::Zin(a)=>current_code(a,pc),
                _=>Ok(None)
            },
            None=>Ok(None)
        }
    }
    Err(Error::InvalidBorrow(file!(),line!()))
}
pub(crate) fn pop_ret(code:&mut InstructionSlice) {
    if let Ok(mut a) = code.try_borrow_mut() {
        if let Some(Instruction::Return(_)) = a.last() {
            a.pop();
        }
    }
}
pub(crate) fn append_code(code:&mut InstructionSlice,appendee:Vec<Instruction>) {
    if let Ok(mut a) = code.try_borrow_mut() {
        a.extend(appendee)
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Instruction::Import(a)=>write!(f, "import"),
            Instruction::Add=>write!(f, "add"),
            Instruction::Sub=>write!(f, "sub"),
            Instruction::Div=>write!(f, "div"),
            Instruction::Mul=>write!(f, "mul"),
            Instruction::Cl0=>write!(f, "cl0"),
            Instruction::Cl1=>write!(f, "cl1"),
            Instruction::Shl=>write!(f, "shl"),
            Instruction::Pop=>write!(f, "pop"),
            Instruction::Shr=>write!(f, "shr"),
            Instruction::Rot=>write!(f, "rot"),
            Instruction::Exp=>write!(f, "exp"),
            Instruction::Dup=>write!(f, "dup"),
            Instruction::Ltb(a)=>write!(f, "ltb {}",a),
            Instruction::Str=>write!(f, "str"),
            Instruction::Bytes=>write!(f, "bytes"),
            Instruction::Len=>write!(f, "len"),
            Instruction::This=>write!(f, "this"),
            Instruction::Negate=>write!(f, "negate"),
            Instruction::ESwitch(a)=>write!(f, "eswitch {}",a),
            Instruction::Raise(a)=>write!(f, "raise {}",a.inst_name()),
            Instruction::Cld(a)=>write!(f,"cld {}",a),
      
            Instruction::Ldl(a,b)=>write!(f,"ldl ({},{})",a,b),
            Instruction::Stl(a,b)=>write!(f,"stl ({},{})",a,b),
            Instruction::MkFunc(a)=>write!(f,"mkfn {}",a),
            Instruction::Builtin(a)=>write!(f,"builtin {}",a),
            Instruction::StoreBuiltin(a)=>write!(f,"sbuiltin {}",a),
            
            Instruction::MkArr(a) =>write!(f,"mkarr {}",a),
            Instruction::MkMap(a) =>write!(f,"mkmap {}",a),
            Instruction::Formula(a)=>write!(f,"formula {}",a),
            Instruction::Unpack(a)=>write!(f,"unpack {}",a),
            Instruction::ArrayAccess=>write!(f,"read"),
            Instruction::ArraySet=>write!(f,"write"),
            Instruction::Ftb(a)=>write!(f,"ftb {}",a),
            Instruction::Ldm(a)=>write!(f,"ldm {}",a.to_string()),
            Instruction::Stm(a)=>write!(f,"stm {}",a.to_string()),
            Instruction::Slice(a)=>match a {
                true=>write!(f,"slice2"),
                _=>write!(f,"slice3")
            }
            Instruction::Err=>write!(f,"err"),
            Instruction::Fcb=>write!(f,"fcb"),
            Instruction::Ldup(a)=>write!(f,"ldup {}",a),
            Instruction::Stup(a)=>write!(f,"stup {}",a),
            Instruction::Call(a)=>write!(f,"call {}",a),
            Instruction::Return(a)=>write!(f,"ret {}",a),
            Instruction::Jmp(a)=>write!(f,"jmp {}",a),
            Instruction::Cmp=>write!(f, "cmp"),
            Instruction::Test(a)=>write!(f,"test {}",a),
            Instruction::Ktest(a)=>write!(f,"ktest {}",a),
            Instruction::Rtest(a)=>write!(f,"rtest {}",a),
            Instruction::NamedFormula(a)=>write!(f,"nformula {}",a),
            Instruction::AAnd=>write!(f,"and"),
            Instruction::And=>write!(f,"bin and"),
            Instruction::AOr=>write!(f,"or"),
            Instruction::Or=>write!(f, "bin or"),
            Instruction::AXor=>write!(f, "xor"),
            Instruction::Xor=>write!(f, "bin xor"),
            Instruction::Less=>write!(f, "less"),
            Instruction::Great=>write!(f, "great"),
            Instruction::LessEq=>write!(f, "less-eq"),
            Instruction::GreatEq=>write!(f, "great-eq"),
            Instruction::Pg45Continue=>write!(f, "noop"),
            Instruction::Pg45Break=>write!(f, "noop"),

        }
    }
}