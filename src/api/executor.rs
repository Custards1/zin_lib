use crate::types;
//use crate::todostd;
use crate::error::*;
use crate::object::*;
use crate::types::*;
use super::ast::*;
use super::lexer::*;

#[derive(Debug)]
pub struct Frame(Vec<Value>,Vec<usize>);

impl Into<Vec<Value>> for Frame {
    #[inline]
    fn into(self) ->Vec<Value> {
        self.0
    }
}
impl Into<Vec<usize>> for Frame {
    #[inline]
    fn into(self) ->Vec<usize> {
        self.1
    }
}
impl Frame {

    #[inline]
    pub fn new(a:usize)->Self {
        Frame(Vec::with_capacity(a),Vec::with_capacity(1))
    }
    ///Attempts to get a function pointer from the stack
    #[inline]
    pub fn ptr(&mut self)->Result<function::Function> {
        Ok(function::Function::from(&self.obj()?))
    }
    #[inline]
    pub fn obj(&mut self)->Result<GcObject> {
        let c =self.pop_or_error()?;
        match &c {
            Value::Object(a)=>Ok(a.clone()), 
            Value::NULL=>Ok(GcObject::from(Object::new(access::ObjAccess::NONE))),
            _=>Err(Error::ExpectedFuncPtr(0))
        }
    }
    #[inline]
    pub fn pop_or_error(&mut self)->Result<Value> {
        match self.0.pop() {
            Some(a)=>Ok(a),
            _=>Err(Error::MalformedStack)
        }
    }
    #[inline]
    pub fn popn_or_error(&mut self,n:usize) ->Result<Vec<Value>> {
        let mut out = Vec::new();
        for i in 0..n {
            match self.0.pop() {
                Some(a)=>out.push(a),
                _=>return Err(Error::MalformedStack)
            }
        }
        Ok(out)
    }
    #[inline]
    pub fn pop_try_block(&mut self)->Option<usize> {
        match self.1.pop() {
            Some(a)=>Some(a),
            _=>None
        }
    }
    #[inline]
    pub fn peek_try_block(&self)->Option<usize> {
        Some(self.1.last()?.clone())
    }
    #[inline]
    pub fn push_try_block(&mut self,val:usize) {
        self.1.push(val);
    }
    
}
impl From<Vec<Value>> for Frame {
    fn from(v: Vec<Value>)->Frame {
        Frame(v,Vec::new())
    }
}
impl std::ops::Deref for Frame {
    type Target = Vec<Value>;
    #[inline]
    fn deref(&self)->&Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for Frame {
    #[inline]
    fn deref_mut(&mut self)->&mut Self::Target {
        &mut self.0
    }
}
impl From<Frame> for StackFrame {
    #[inline]
    fn from(a:Frame)->StackFrame{
        StackFrame(vec![a])
    }
}
impl From<Vec<Frame>> for StackFrame {
    #[inline]
    fn from(a:Vec<Frame>)->StackFrame{
        StackFrame(a)
    }
}
#[derive(Debug)]
#[repr(transparent)]
pub struct StackFrame(Vec<Frame>);
impl StackFrame {
    #[inline]
    pub fn new(a:usize)->StackFrame {
        StackFrame(vec![Frame::new(a)])
    }
}
impl std::ops::Deref for StackFrame {
    type Target = Vec<Frame>;
    #[inline]
    fn deref(&self)->&Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for StackFrame {
    #[inline]
    fn deref_mut(&mut self)->&mut Self::Target {
        &mut self.0
    }
}
impl Into<Vec<Frame>> for StackFrame {
    #[inline]
    fn into(self) ->Vec<Frame> {
        self.0
    }
}
impl StackFrame {
    #[inline]
    pub fn current_frame(&mut self)->&mut Frame {
        match self.len() {
            0=>{
                self.push(Frame::new(1));
                &mut self[0]
            }
            _=>{
                let a= self.len()-1;
                &mut self[a]
            }
        }
    }

    #[inline]
    pub fn stl(&mut self,ids:(usize,usize),c:Value)->Result<()> {
        let (func,idx) = ids;
        let func = self.len()-func-1;
        if func >= self.len() {
           
            Err((Error::MalformedStack))
        } else {
            if idx >= self[func].len() {
                if idx == self[func].len() {
                    self[func].push(c);
                    Ok(())
                } else if idx ==1 && self[func].len() ==0 {
                    self[func].push(c);
                    Ok(())
                   
                } else {
                    Err((Error::MalformedStack))
                }
                
            } else {
                self[func][idx] = c;
                Ok(())
            }
        }
    }
    #[inline]
    pub fn ldl(&mut self,ids:(usize,usize))->Result<()> {
        let local = self._ldl(ids)?;
        self.current_frame().push(local);
        Ok(())
    }
    #[inline]
    fn _ldl(&self,ids:(usize,usize))->Result<Value> {
        let (func,idx) = ids;
        let func = self.len()-func-1;
        if func >= self.len() {
            Err(Error::MalformedStack)
        } else {
            if idx >= self[func].len() {
                Err(Error::MalformedStack)
            } else {
                Ok(self[func][idx].clone())
            }
        }
    }
}

#[repr(transparent)]
#[derive(Clone,Copy,PartialEq)]
pub struct ProgramCounter(usize);
impl From<usize> for ProgramCounter {
    #[inline]
    fn from(idx: usize) -> ProgramCounter{
        ProgramCounter(idx)
    }
}
impl ProgramCounter{
    pub fn as_usize(self) -> usize{
        self.0
    }
    pub fn jump_forward(&mut self,x:usize)->Result<()> {
        let temp = self.0 + x;
        if temp < self.0 {
            return Err(Error::InvalidJump)
        }
        self.0 =temp;
        Ok(())
    }
    #[inline]
    pub fn jump(&mut self,x:i64)->Result<()> {
        if x == 0 {
            return Ok(())
        } else if x < 0 {
            let a = x.abs() as usize;
            if a > self.0 {
                return Err(Error::InvalidJump)
            }
            self.0 -= a;
            Ok(())
        } else {
            let temp = self.0 + x.abs() as usize;
            if temp < self.0 {
                return Err(Error::InvalidJump)
            }
            self.0 =temp;
            Ok(())
        }
    }
}
