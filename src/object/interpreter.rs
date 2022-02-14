use super::*;
use function::*;
use crate::types::{value::Value,number::{Signed,Unsigned,Number},ZString};
use crate::types::ops::{Instruction,InstructionSlice,RaiseErrKind};
use super::obj::*;
use crate::error::*;
use super::access::ObjAccess;
use crate::api::{ast::*,executor::*};
const EXEC_NAME: &'static str = "Executor";
const EXEC_DOC: &'static str = "contains the core zin executor";
use crate::object::zstd::builtin;
use std::convert::TryInto;
#[derive(Clone,gc::Trace,gc::Finalize)]
pub struct Executor {
    pub(crate) builtin:GcObject,
    pub(crate) system:GcObject,
    pub(crate) json:GcObject,
}
macro_rules! inline_call {
    ($sself:ident,$arg_len:ident,$current_function:ident,$callframe:ident,$pc:ident,$function:ident,$newframe:ident,$sframe:ident,$frame:ident,$cur_err:ident,$($main_loop:tt)*)=>{
        {
            let mut ok = true;
            let mut kval = 0;
            if $function.is_method() {
                
                kval=1;
                for _ in 0..$arg_len {
                    $newframe.push($sframe.pop_or_error()?);
                    
                }
                let _ =match $sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            $cur_err = Some(err.into());
                            unwind_zin_err!($frame,$current_function,$callframe,$pc,$cur_err);
                            
                            continue $($main_loop)+;
                        }
                    };
                if ok {
                    $newframe.push($function.parent().unwrap().into());
                    $newframe.reverse();
                }
                
            } else {
           
                for _ in 0..$arg_len {
                    $newframe.push($sframe.pop_or_error()?);
                }
                let _ =match $sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            $cur_err = Some(err.into());
                            unwind_zin_err!($frame,$current_function,$callframe,$pc,$cur_err);
                            continue $($main_loop)+;
                        }
                    }; 
                $newframe.reverse();
            }
           
           
            
            if let Some(native) = $function.native() {
                let ms = ($function.nargs()?).u64() as usize;
                let len = $newframe.len();
                if len <= ms {
                    for i in len..ms {
                        $newframe.push(Value::NULL);
                    }
                }
                $sframe.push(native($sself.clone(),$newframe.into())?);
                None
            } else {
                let aoo =$function.nargs()?.u64() + kval;
                let ms = ($function.nlocals()?.u64()+aoo) as usize;
                let len = $newframe.len();
                if len <= aoo as usize {
                    if ms > len {
                        for i in len..ms {
                            $newframe.push(Value::NULL);
                        }
                    }
                   
                    $callframe.push(($current_function,$pc));
                    $current_function = $function.object();
                    $pc = 0;
                    $frame.push($newframe.into());
                    Some(())
                } else {
                    let err :GcObject = Error::InvalidArgs(len,aoo as usize).into();
                    $cur_err = Some(err.into());
                    unwind_zin_err!($frame,$current_function,$callframe,$pc,$cur_err);
                    continue $($main_loop)+;
                }
               
            }
        }
       
    };
}
macro_rules! inline_call_value {
    ($sself:ident,$arg_len:ident,$current_function:ident,$callframe:ident,$pc:ident,$fval:ident,$newframe:ident,$sframe:ident,$frame:ident,$cur_err:ident,$($main_loop:tt)*)=>{
        match &$fval {
            Value::Object(function)=>{
                let function = Function{inner:function.clone()};
                inline_call!($sself,$arg_len,$current_function,$callframe,$pc,function,$newframe,$sframe,$frame,$cur_err,$($main_loop)+)
            }
            _=>{
                let err :GcObject = Error::InvalidCall(format!("{}",$fval)).into();
                $cur_err = Some(err.into());
                unwind_zin_err!($frame,$current_function,$callframe,$pc,$cur_err);
                continue $($main_loop)+;
            }
        }
    };
}

macro_rules! inline_call_builtin_member_with_arg {
    ($sself:ident,$current_function:ident, $callframe:ident, $frame:ident,$sframe:ident,$pc:ident,$one:ident,$two:ident,$meth:ident,$ok:ident,$cur_err:ident,$($main_loop:tt)*)=>{
        if let Value::Object($one) = &$one {
            let fval = match $one.member(super::zstd::builtin::property::accessors::$meth()){
                Some(a)=>a.clone(),
                _=>{
                    let err :GcObject = Error::FunctionDne(super::zstd::builtin::property::accessors::$meth().to_string()).into();
                    $cur_err = Some(err.into());
                    unwind_zin_err!($frame,$current_function,$callframe,$pc,$cur_err);
                    continue $($main_loop)+;
                }
            }; 
            $sframe.push(Value::NULL);
            $sframe.push($two);
            let mut newframe = Frame::new(2);
            let uui = 1;
            inline_call_value!($sself,uui,$current_function,$callframe,$pc,fval,newframe,$sframe,$frame,$cur_err,$($main_loop)+);
        } else if let Value::Object($two) = &$two {
            let fval = match $two.member(super::zstd::builtin::property::accessors::$meth()){
                Some(a)=>a.clone(),
                _=>{
                    let err :GcObject = Error::FunctionDne(super::zstd::builtin::property::accessors::$meth().to_string()).into();
                    $cur_err = Some(err.into());
                    unwind_zin_err!($frame,$current_function,$callframe,$pc,$cur_err);
                    continue $($main_loop)+;
                }
            }; 
            $sframe.push(Value::NULL);
            $sframe.push($one);
            
            let mut newframe = Frame::new(2);
            let uui = 1;
            inline_call_value!($sself,uui,$current_function,$callframe,$pc,fval,newframe,$sframe,$frame,$cur_err,$($main_loop)+);
        } else {
            $sframe.push($one.$ok($two))
        }
    };
}
macro_rules! inline_call_builtin_member {
    ($sself:ident,$current_function:ident, $callframe:ident, $frame:ident,$sframe:ident,$pc:ident,$one:ident,$meth:ident,$cur_err:ident,$($main_loop:tt)*)=>{
        if let Value::Object($one) = &$one {
            let fval = match $one.member(super::zstd::builtin::property::accessors::$meth()){
                Some(a)=>a.clone(),
                _=>{
                    let err :GcObject = Error::FunctionDne(super::zstd::builtin::property::accessors::$meth().to_string()).into();
                    $cur_err = Some(err.into());
                    unwind_zin_err!($frame,$current_function,$callframe,$pc,$cur_err);
                    continue $($main_loop)+;
                }
            }; 
            $sframe.push(Value::NULL);
            let mut newframe = Frame::new(1);
            let uui = 0;
            inline_call_value!($sself,uui,$current_function,$callframe,$pc,fval,newframe,$sframe,$frame,$cur_err,$($main_loop)+);
        }  else {
            $sframe.push(Value::NULL)
        }
    };
}
use std::convert::TryFrom;

macro_rules! unwind_zin_err {
    ($frame:ident,$current:ident,$callback:ident,$pc:ident,$cur_err:ident) => {
        
        {
            let sframe = $frame.current_frame();
            let mut z_has_catch = false;
            while let Some(catch_block) = sframe.pop_try_block() {
                $pc = catch_block;
                z_has_catch = true;
                break;
            }
            
            if !z_has_catch {
                match $frame.pop() {
                    None=>{
                        return Err(match &$cur_err {
                            Some(a)=>{
                                Error::from(a.clone())
                            }
                            _=>{
                                Error::InvalidUnwind
                            }
                        });
                    }
                    _=>{

                    }
                }
                $current = match $callback.pop() {
                    None=>{
                        return Err(match &$cur_err {
                            Some(a)=>{
                                Error::from(a.clone())
                            }
                            _=>{
                                Error::InvalidUnwind
                            }
                        })
                    }
                    Some(a)=>a.0
                };
                let mut sframe = $frame.current_frame(); 
                loop {
                    let mut z_has_catch = false;
                    while let Some(catch_block) = sframe.pop_try_block() {
                        $pc = catch_block;
                        z_has_catch = true;
                        break;
                    }
                    if z_has_catch {
                        break;
                    }
                    match $frame.pop() {
                        None=>{
                            return Err(match &$cur_err {
                                Some(a)=>{
                                    Error::from(a.clone())
                                }
                                _=>{
                                    Error::InvalidUnwind
                                }
                            })
                        }
                        _=>{

                        }
                    };
                    $current = match $callback.pop() {
                        None=>{
                            return Err(match &$cur_err {
                                Some(a)=>{
                                    Error::from(a.clone())
                                }
                                _=>{
                                    Error::InvalidUnwind
                                }
                            })
                        }
                        Some(a)=>a.0
                    };
                    sframe = $frame.current_frame();
                }
            }
        }
    }
}

macro_rules! _unwrap_result_if_err {
    ($op:expr,$frame:ident,$sframe:ident,$current:ident,$callback:ident,$pc:ident)=>{
        match $op {
            Ok(a)=>a,
            Err(e)=>{
                let e = Some(e);
                unwind_zin_err!($frame:ident,$sframe:ident,$current:ident,$callback:ident,$pc:ident,e);
                break
            }
        }
    }
}

impl Executor {
    #[inline]
    pub fn new(script_dir:ZString)-> Executor {
        
        let builtin =zstd::builtin::builtin();
        Self { 
            builtin:builtin.clone(),
            system:zstd::system::system(builtin,&script_dir),
            json:zstd::json::json(),
        }
    }
    #[inline]
    pub fn set_argv(&mut self,arg:Vec<String>) {
        let mut args:Vec<Value> = Vec::new();
        for arg in arg {
            args.push(super::zstd::builtin::string_from(arg).into());
        }
        let _ = self.system.set_member("argv",super::zstd::builtin::vec_from(args));
    }
    #[inline]
    pub fn script<Code:Parseable>(&self,code:Code) -> Result<GcObject> {
        Ok(Function::parsed(&self.builtin, code)?.inner)
    }
    #[inline]
    pub (crate) fn system_imports_append(&self,name:ZString,value:GcObject) {
        match &mut self.system.member(super::zstd::system::property::zin_imports()) {
            Some(Value::Object(a))=>{
                a.set_index(self, name, value.into());
            }
            _=>{

            }
        }
    }
    #[inline]
    pub (crate) fn system_builtin_spec(&self,name:ZString)->Option<GcObject> {
        match &mut self.system.member("preload") {
            Some(Value::Object(a))=>{
                let obj: GcObject= match a.index_access( name)?.try_into(){
                    Ok(a)=>a,
                    _=>return None
                };
                Some(obj)
            }
            _=>None
        } 
    }
    #[inline]
    pub (crate) fn system_imports_get(&self,name:ZString)->Option<GcObject> {
        match &mut self.system.member(super::zstd::system::property::zin_imports()) {
            Some(Value::Object(a))=>{
                let obj: GcObject= match a.index_access( name)?.try_into(){
                    Ok(a)=>a,
                    _=>return None
                };
                Some(obj)
            }
            _=>None
        }
    }
    #[inline]
    pub fn call_script<Code:Parseable,Arg:Argument>(&self,code:Code,args:Arg) -> Result<Value> {
        let inner = Function::parsed(&self.builtin, code)?.inner;
        self.call(inner, args.arg())
    }
    

    #[inline(always)]
    fn __call(&self,function:GcObject,mut frame:Frame,callframe:Vec<(GcObject,usize)>) -> Result<Value>{
        let function = Function{inner:function};
        let nargs = function.nargs()? + function.nlocals()?;
        let ak = nargs.u64() as usize;
        while frame.len() <  ak {
            frame.push(Value::NULL);
        }
        if let Some(native) = function.native() {
            return native(self.clone(),frame.into())
        }
        let mut frame = StackFrame::from(frame);
        let mut current_function = function.object();
        let mut callframe = callframe;
        let mut pc = 0;
        let mut cur_err:Option<GcObject> = None;
        debugln!("starting loop with current_function {:?}",current_function);
        'main_loop: while let Some(scode) = Function::current_code(&current_function,&mut pc)? {
            let scode = scode.clone();
            debugln!("Pc {}: depth:{} Calling {:?} with frame: {:?}",pc,callframe.len(),&scode,frame);
            match &scode {
                Instruction::Import(a)=>{
                
                    let fval:GcObject =match self.system.member(options::import()).and_then(|x|{if let Value::Object(o) = &x{
                        Some(o.clone())
                    }else{None}}) { 
                        Some(a)=>a,
                        _=>{
                            let mut err:GcObject = Error::NoImportSystem.into();
                            err.set_member("__why","Your zin system library was corrupted!");
                            cur_err = Some(err);
                            
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    let sframe = frame.current_frame();
                    sframe.push(Value::NULL);
                    sframe.push(a.clone().into());
                    let mut newframe = Frame::new(2);
                    let uui = 1;
                    let function = Function{inner:fval};
                    inline_call!(self,uui,current_function,callframe,pc,function,newframe,sframe,frame,cur_err,'main_loop);
                   // inline_call_value!(self,uui,current_function,callframe,pc,fval,newframe,sframe,frame);

                }
                Instruction::Builtin(name)=>{
                    let sframe = frame.current_frame();
                    sframe.push(self.builtin.member(name.clone()).unwrap_or(Value::NULL))
                }
                Instruction::StoreBuiltin(name)=>{
                    let sframe = frame.current_frame();
                    let one = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    let mut builtin = self.builtin.clone();
                    builtin.set_member(name.clone(),one);
                }
                Instruction::Cld(a)=>{
                    frame.current_frame().push(a.clone())
                }
                Instruction::Add=>{
                    let sframe = frame.current_frame();
                    let two = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    let one = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    inline_call_builtin_member_with_arg!(self,current_function,callframe,frame,sframe,pc,one,two,__add,add,cur_err,'main_loop);
                },
                Instruction::Sub=>{
                    let sframe = frame.current_frame();
                    let two = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    let one = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    inline_call_builtin_member_with_arg!(self,current_function,callframe,frame,sframe,pc,one,two,__sub,sub,cur_err,'main_loop);
                }
                Instruction::Div=>{
                    let sframe = frame.current_frame();
                    let two = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    let one = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    inline_call_builtin_member_with_arg!(self,current_function,callframe,frame,sframe,pc,one,two,__div,div,cur_err,'main_loop);
                },
                Instruction::Mul=>{
                    let sframe = frame.current_frame();
                    let two = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    let one = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    inline_call_builtin_member_with_arg!(self,current_function,callframe,frame,sframe,pc,one,two,__mul,mul,cur_err,'main_loop);
                },
                Instruction::Exp=>{
                   
                    let sframe = frame.current_frame();
                    let two = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    let one = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    inline_call_builtin_member_with_arg!(self,current_function,callframe,frame,sframe,pc,one,two,__exp,exp,cur_err,'main_loop);
                }
                Instruction::Shl=>{
                   
                    let sframe = frame.current_frame();
                    let two = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    let one = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    inline_call_builtin_member_with_arg!(self,current_function,callframe,frame,sframe,pc,one,two,__shl,shl,cur_err,'main_loop);
                }
                Instruction::Shr=>{
                    let sframe = frame.current_frame();
                    let two = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    let one = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    inline_call_builtin_member_with_arg!(self,current_function,callframe,frame,sframe,pc,one,two,__shr,shr,cur_err,'main_loop);
                }
                Instruction::Cl0=>{
                    frame.current_frame().push(Value::Bool(false))
                },
                Instruction::Cl1=>{
                    frame.current_frame().push(Value::Bool(true))
                },
                Instruction::Pop=>{
                    let _ = frame.current_frame().pop_or_error()?;
                }
               
                Instruction::Stl(a,b)=>{
                    let local = frame.current_frame().pop_or_error()?;
                    frame.stl((*a as usize,*b as usize),local)?;
                },
                Instruction::Ldl(a,b)=>{
                    frame.ldl((*a as usize,*b as usize))?;
                }
                
                Instruction::Ldm(v)=>{
                    let sframe = frame.current_frame();
                    let one = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    match &one {
                        Value::Object(obj)=> match obj.member(v.clone()){
                            Some(a)=>sframe.push(a.clone()),
                            None=>sframe.push(Value::NULL)
                        }
                        Value::String(a)=>{
                            let obj = builtin::string_from(a.as_ref());
                            match obj.member(v.clone()){
                                Some(a)=>sframe.push(a.clone()),
                                None=>sframe.push(Value::NULL)
                            }
                        }
                        _=>{
                            cur_err = Some(Error::Custom("TypeError".to_string(),"expected object".to_string()).into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    }
                   
                }
                Instruction::Stm(v)=>{
                    let sframe = frame.current_frame();
                    let mut one = match sframe.obj() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    let three = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    one.set_member(v.clone(), three);
                }
                Instruction::Stup(idx)=>{
                    let sframe = frame.current_frame();
                    let back = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    Function{inner:current_function.clone()}.set_nth_upval(self,*idx,back)?;
                }
                Instruction::Ldup(idx)=>{
                    let sframe = frame.current_frame();
                    sframe.push(Function{inner:current_function.clone()}.nth_upval(*idx)?.unwrap_or(Value::NULL));
                }
                Instruction::This=>{
                    let sframe = frame.current_frame();
                    sframe.push(current_function.clone().into())
                }
                Instruction::MkFunc(u)=> {
                    let sframe = frame.current_frame();
                    let mut upvals = Vec::new();
                    for _ in 0..*u {
                        upvals.push(sframe.pop_or_error()?);
                    }
                    upvals.reverse();
                    let obj:GcObject = match sframe.obj() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    }.hard_clone();
                    let mut obj:Function = Function{inner:obj};
                    obj.set_upvals(Function::blank_upval(upvals));
                    sframe.push(obj.into());
                }
                Instruction::Str=>{
                    let sframe = frame.current_frame();
                    let back = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    match &back {
                        Value::Object(a)=>{
                            match a.try_borrow() {
                                Ok(a)=>{
                                    match &a.contents.vec {
                                        Some(InnerContainer::String(a))=>{
                                            sframe.push(builtin::string_from(a.as_str() ).into())
                                        }
                                        Some(InnerContainer::Vector(InternedVector::ByteVector(a)))=>{
                                            
                                            sframe.push(builtin::string_from(std::str::from_utf8(&a).unwrap_or("")).into())
                                        }
                                        _=>{
                                            
                                            sframe.push(builtin::string_from(back.invoke_to_string(self)).into())
                                        }
                                    }
                                }
                                _=>{
                                    
                                    sframe.push(builtin::string_from(back.invoke_to_string(self)).into())
                                }
                            }
                        }
                        Value::Num(a)=>{
                            match std::char::from_u32(a.u32()){
                                Some(i) =>sframe.push(builtin::string_from(i).into()),
                                _=>sframe.push(builtin::string_from("").into())
                            }
                        }
                        Value::Char(a)=>{
                            sframe.push(builtin::string_from(*a).into())
                        }
                        Value::Bytes(a)=>{
                            sframe.push(builtin::string_from(std::str::from_utf8(&a).unwrap_or("")).into())
                        }
                        _=>{
                            sframe.push(builtin::string_from(back.invoke_to_string(self)).into())
                        }
                    }
                    
                }
                Instruction::Bytes=>{
                    let sframe = frame.current_frame();
                    let back = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    sframe.push(builtin::byte_vec_from(back.invoke_to_bytes(self).as_ref()).into())
                }
                Instruction::Call(a)=>{
                    let sframe = frame.current_frame();
                    let len =sframe.len(); 
                    let t = a +1;
                    if len == 0 {
                        cur_err = Some(Error::MalformedStack.into());
                        unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                        continue 'main_loop;
                    }
                    if t > len {
                        cur_err = Some(Error::InvalidJump.into());
                        unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                        continue 'main_loop;
                    }
                    let fval = sframe[len-t].clone();
                    let mut newframe = Frame::new(t as usize);
                    let uui = *a;
                    inline_call_value!(self,uui,current_function,callframe,pc,fval,newframe,sframe,frame,cur_err,'main_loop);
                },
                Instruction::Return(a)=>{
                    let sframe = frame.current_frame();
                    let mut newframe = Frame::new(*a as usize);
                    if *a == 0 {
                        newframe.push(Value::NULL);
                    } else {
                        for _ in 0..*a {
                            newframe.push(sframe.pop_or_error()?);
                        }
                        newframe.reverse();
                    }
                    match callframe.pop() {
                        Some(pcc)=>{
                            current_function = pcc.0;
                            pc = pcc.1;
                        }
                        None=>match newframe.len() {
                            0=>{
                                return Ok(Value::NULL)
                            }
                            _=>{
                                return Ok(newframe.pop_or_error()?)
                            }
                        }
                    }
                    match frame.pop() {
                        Some(_)=>{
                            let sframe =frame.current_frame();
                            let newframe:Vec<Value> = newframe.into();
                            sframe.extend(newframe);
                        },
                        None=>match newframe.len() {
                            0=>{
                                return Ok(Value::NULL)
                            }
                            _=>{
                                return Ok(newframe.pop_or_error()?)
                            }
                        }
                    }
                },
                Instruction::Jmp(a)=>{
                    let mut i = ProgramCounter::from(pc);
                    i.jump(*a)?;
                    pc = i.as_usize();
                },
                Instruction::Cmp=>{
                    let sframe = frame.current_frame();
                    let two = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    let one = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    if one == two {
                        sframe.push(true.into());
                    } else {
                        inline_call_builtin_member_with_arg!(self,current_function,callframe,frame,sframe,pc,one,two,__eq,eqval,cur_err,'main_loop);
                    }
                    
                }
                Instruction::Negate=>{
                    let sframe = frame.current_frame();
                    let one = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    let mut ok=true;
                    match &one {
                        Value::Bool(a)=>sframe.push((!a).into()),
                        Value::NULL => sframe.push(true.into()),
                        Value::Num(a)=>sframe.push((*a * Number::from(-1).mimic(*a)).into()),
                        _=>{
                            ok=false;
                        }
                    }
                    if !ok {
                        sframe.push(one);
                    }
                    
                },
                Instruction::MkArr(a)=>{
                    let mut arr = Vec::with_capacity(*a);
                    for _ in 0..*a {
                        arr.push(match frame.current_frame().pop_or_error() {
                            Ok(a)=>a,
                            Err(err)=>{
                                cur_err = Some(err.into());
                                unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                                continue 'main_loop;
                            }
                        });
                    }
                    arr.reverse();
                    frame.current_frame().push(super::zstd::builtin::vec_from(arr).into());
                }
                Instruction::Dup=>{
                    let sframe = frame.current_frame();
                    let val = match sframe.last() {
                        Some(a)=>a.clone(),
                        _=>Value::NULL
                    };
                    sframe.push(val);
                }
                Instruction::Ltb(jmp)=>{
                    let sframe = frame.current_frame();
                    let mut catch_pc = ProgramCounter::from(pc);
                    catch_pc.jump_forward(*jmp-1)?;
                    sframe.push_try_block(catch_pc.as_usize());
                }
                Instruction::Rot=>{
                    let sframe = frame.current_frame();
                    let two = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    let one = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    sframe.push(two);
                    sframe.push(one);
                }
                Instruction::MkMap(a)=>{
                    
                    let arr = {
                        let sframe = frame.current_frame();
                        let mut arr = PropertyMap::new();
                        for _ in 0..*a {
                            let j = match sframe.pop_or_error() {
                            Ok(a)=>a,
                            Err(err)=>{
                                cur_err = Some(err.into());
                                unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                                continue 'main_loop;
                            }
                        };
                            let i = match sframe.pop_or_error() {
                            Ok(a)=>a,
                            Err(err)=>{
                                cur_err = Some(err.into());
                                unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                                continue 'main_loop;
                            }
                        };
                            arr.insert(i, j);
                        }
                        arr
                    };
                    let sframe = frame.current_frame();
                    sframe.push(super::zstd::builtin::map_from(arr).into());

                }
                Instruction::Formula(n)=>{
                    let obj = {
                        let sframe = frame.current_frame();
                        let mut obj = GcObject::from(Object::new(access::ObjAccess::NONE));
                        for _ in 0..*n {
                            let mut j = match sframe.pop_or_error() {
                            Ok(a)=>a,
                            Err(err)=>{
                                cur_err = Some(err.into());
                                unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                                continue 'main_loop;
                            }
                        };
                            let i:ZString = ZString::try_from(sframe.pop_or_error()?)?;
                            if let Value::Object(maybe_func) = &mut j {
                                if maybe_func.is_method() {
                                    
                                    let _= GcObject::bind_method(maybe_func,obj.clone());
                                }
                            }
                            obj.set_member(i, j);
                        }
                        obj
                    };
                    let sframe = frame.current_frame();
                    sframe.push(obj.into());
                }
                Instruction::NamedFormula(n)=>{
                    let obj = {
                        let sframe = frame.current_frame();
                        let mut obj = GcObject::from(Object::new(access::ObjAccess::NONE));
                        let mut name = match sframe.pop_or_error() {
                            Ok(a)=>a,
                            Err(err)=>{
                                cur_err = Some(err.into());
                                unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                                continue 'main_loop;
                            }
                        };
                        obj.set_member("__name",name);
                        for _ in 0..*n {
                            let mut j = match sframe.pop_or_error() {
                                Ok(a)=>a,
                                Err(err)=>{
                                    cur_err = Some(err.into());
                                    unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                                    continue 'main_loop;
                                }
                            };
                            let i:ZString = ZString::try_from(sframe.pop_or_error()?)?;
                            if let Value::Object(maybe_func) = &mut j {
                                if maybe_func.is_method() {
                                    
                                    let _= GcObject::bind_method(maybe_func,obj.clone());
                                }
                            }
                            obj.set_member(i, j);
                        }
                        obj
                    };
                    let sframe = frame.current_frame();
                    sframe.push(obj.into());
                }
                Instruction::Raise(rval)=>match rval{
                    RaiseErrKind::CurrentError=>{
                        let tdo = match &cur_err {
                            Some(_)=>true,
                            _=>false
                        };
                        match tdo {
                            false=>{}
                            true=>{
                                unwind_zin_err!(frame,current_function,callframe,pc,cur_err)
                            }
                        };
                    }
                    RaiseErrKind::FromStack=> {
                        let sframe = frame.current_frame();
                        let mut one = (match sframe.pop_or_error() {
                            Ok(a)=>a,
                            Err(err)=>{
                                cur_err = Some(err.into());
                                unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                                continue 'main_loop;
                            }
                        }).invoke_to_string(self);
                        cur_err = Some(Error::CustomSingle(one.to_string()).into());
                        unwind_zin_err!(frame,current_function,callframe,pc,cur_err)
                    }
                    RaiseErrKind::FromStackWithWhy=>{
                        let sframe = frame.current_frame();
                        let mut two = (match sframe.pop_or_error() {
                            Ok(a)=>a,
                            Err(err)=>{
                                cur_err = Some(err.into());
                                unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                                continue 'main_loop;
                            }
                        }).invoke_to_string(self);
                        let mut one = (match sframe.pop_or_error() {
                            Ok(a)=>a,
                            Err(err)=>{
                                cur_err = Some(err.into());
                                unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                                continue 'main_loop;
                            }
                        }).invoke_to_string(self);
                        
                        cur_err = Some(Error::Custom(one.to_string(),two.to_string()).into());
                        unwind_zin_err!(frame,current_function,callframe,pc,cur_err)
                    }
                    RaiseErrKind::FromStackObj=>{
                        let sframe = frame.current_frame();
                        let mut one = match sframe.obj() {
                            Ok(a)=>a,
                            Err(err)=>{
                                cur_err = Some(err.into());
                                unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                                continue 'main_loop;
                            }
                        };
                        cur_err = Some(one);
                        unwind_zin_err!(frame,current_function,callframe,pc,cur_err)
                    }
                }
                Instruction::Fcb=>{
                    cur_err=None;
                }
                Instruction::Unpack(n)=> {
                    let arr = {
                        let sframe = frame.current_frame();
                        let mut arr = match sframe.obj() {
                            Ok(a)=>a,
                            Err(err)=>{
                                cur_err = Some(err.into());
                                unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                                continue 'main_loop;
                            }
                        };
                        
                        arr
                    };
                    let mut id = match n {
                        0=>0,
                        _=>*n-1
                    };
                    let sframe = frame.current_frame();
                    for _ in 0..*n {
                        sframe.push(arr.map_index(id).unwrap_or(Value::NULL));
                        id-=1;
                    }
                  
                }
                Instruction::Len=>{
                    let sframe = frame.current_frame();
                    let one = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    match &one {
                        Value::String(a)=>sframe.push(a.len().into()),
                        Value::Bytes(a)=>sframe.push(a.len().into()),
                        Value::Object(obj)=>{
                            match obj.try_borrow() {
                                Ok(a)=>{
                                    let ok = match &a.contents.vec {
                                        Some(a)=>match a{
                                            InnerContainer::String(a)=>{
                                                sframe.push(a.len().into());
                                                true
                                            }
                                            InnerContainer::Vector(a)=>{
                                                sframe.push(a.len().into());
                                                true
                                            }
                                            InnerContainer::Function(_)|InnerContainer::File(_)=>{
                                                false
                                            }
                                            InnerContainer::Map(a)=>{
                                                sframe.push(a.len().into());
                                                true
                                            }
                                        }
                                        _=>{
                                            false
                                        }
                                    };
                                    if !ok {
                                        let one = one.clone();
                                        let two = Value::NULL;
                                        inline_call_builtin_member!(self,current_function,callframe,frame,sframe,pc,one,__len,cur_err,'main_loop);
                                    }
                                }
                                _=>{
                                    sframe.push(0usize.into());
                                }
                            }
                        }
                        _=>sframe.push(0usize.into())
                    }
                }
                Instruction::ArrayAccess=> {
                    let sframe = frame.current_frame();
                    let two = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    let one = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    match &one {
                        Value::String(a)=>{
                            sframe.push(InternedString::Str(a.clone()).get(two).unwrap_or(Value::NULL))
                        }
                        Value::Bytes(a)=>{
                            match &two {
                                Value::Num(b)=>{
                                    todo!("make bytes []able");
                                }
                                _=>{
                                    sframe.push(Value::NULL);
                                }
                            }
                        }
                        Value::Object(one)=>{
                            sframe.push(Value::NULL);
                            sframe.push(two);
                            let fval = match one.member(super::zstd::builtin::property::accessors::__get()){
                                Some(a)=>a.clone(),
                                _=>{
                                    cur_err = Some(Error::FunctionDne(super::zstd::builtin::property::accessors::__get().to_string()).into());
                                    unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                                    continue 'main_loop;
                                }
                            };
                            let mut newframe = Frame::new(2);
                            let uui = 1;
                            inline_call_value!(self,uui,current_function,callframe,pc,fval,newframe,sframe,frame,cur_err,'main_loop);
                        }
                        _=>{
                            sframe.push(Value::NULL);
                        }
                    }
                   
                }
                Instruction::Slice(is_two)=> {
                    match is_two {
                        true => {
                            let sframe = frame.current_frame();
                            let two = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                            let one = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                            match &one {
                                Value::Object(one)=>{
                                    sframe.push(Value::NULL);
                                    sframe.push(two);
                                    sframe.push(std::usize::MAX.into());
                                    let fval = match one.member(super::zstd::builtin::property::accessors::__get()){
                                        Some(a)=>a.clone(),
                                        _=>{
                                            cur_err = Some(Error::FunctionDne(super::zstd::builtin::property::accessors::__get().to_string()).into());
                                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                                            continue 'main_loop;
                                        }
                                    };
                                    let mut newframe = Frame::new(3);
                                    let uui = 2;
                                    inline_call_value!(self,uui,current_function,callframe,pc,fval,newframe,sframe,frame,cur_err,'main_loop);
                                }
                                Value::String(a)=>{
                                    match &two {
                                        Value::Num(b)=>{
                                            sframe.push(InternedString::Str(a.clone()).get((b.usize(),std::usize::MAX)).unwrap_or(Value::NULL))
                                        }
                                        _=>sframe.push(Value::NULL)
                                    }
                                    
                                }
                                Value::Bytes(a)=>{
                                    todo!("make bytes [..]able");
                                }
                                _=>sframe.push(Value::NULL)
                            }
                            
                        }
                        _=>{
                            let sframe = frame.current_frame();
                            let three = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                            let two = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                            let one = sframe.obj()?;
                            sframe.push(Value::NULL);
                            sframe.push(two);
                            sframe.push(three);
                            let fval = match one.member(super::zstd::builtin::property::accessors::__get()){
                                Some(a)=>a.clone(),
                                _=>{
                                    cur_err = Some(Error::FunctionDne(super::zstd::builtin::property::accessors::__get().to_string()).into());
                                    unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                                    continue 'main_loop;
                                }
                            };
                            let mut newframe = Frame::new(3);
                            let uui = 2;
                            inline_call_value!(self,uui,current_function,callframe,pc,fval,newframe,sframe,frame,cur_err,'main_loop);
                        }
                    }
                   
                }
                Instruction::ArraySet=> {
                    let sframe = frame.current_frame();
                    let two = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    let one = sframe.obj()?;
                    let three = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    sframe.push(Value::NULL);
                    sframe.push(two);
                    sframe.push(three);
                    let fval = match one.member(super::zstd::builtin::property::accessors::__set()){
                        Some(a)=>a.clone(),
                        _=>{
                            cur_err = Some(Error::FunctionDne(super::zstd::builtin::property::accessors::__set().to_string()).into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    let mut newframe = Frame::new(3);
                    let uui = 2;
                    inline_call_value!(self,uui,current_function,callframe,pc,fval,newframe,sframe,frame,cur_err,'main_loop);
                }
                Instruction::Test(a)/*pops stack in hopes of a true boolean,if false,push true and jump*/=>{
                    let sframe = frame.current_frame();
                    let boolean = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    if !boolean.as_bool() {
                        sframe.push(Value::Bool(true));
                        match a {
                            0|1=>{},
                            _=>{
                                let mut i = ProgramCounter::from(pc);
                                i.jump_forward(a-1)?; 
                                pc = i.as_usize();
                            }
                        }
                    }
                },
                Instruction::Ftb(size)=>{
                    let sframe = frame.current_frame();
                    let _ = sframe.pop_try_block();
                    let mut i = ProgramCounter::from(pc);
                    i.jump_forward(*size)?;
                    pc = i.as_usize();
                }
                Instruction::Err=>{
                    match &cur_err {
                        Some(a)=>{
                            let sframe = frame.current_frame();
                            sframe.push(a.clone().into());
                        }
                        _=>{
                            cur_err = Some(Error::InvalidWind.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    }
                }
                Instruction::ESwitch(switch)=>{
                    let cases = match frame.current_frame().popn_or_error(*switch) {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    match &cur_err {
                        Some(a)=>{
                            match a.name() {
                                Some(name)=>{
                                    let mut ok=false;
                                    for case in cases {
                                        if name == case.invoke_to_string(self) {
                                            ok=true;
                                        }
                                    }
                                    if !ok {
                                        unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                                        continue 'main_loop;
                                    }
                                }
                                _=>{
                                    cur_err = Some(Error::InvalidErrorType.into());
                                    unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                                    continue 'main_loop;
                                }

                            };
                            let sframe = frame.current_frame();
                            sframe.push(a.clone().into());
                        }
                        _=>{
                            cur_err = Some(Error::InvalidWind.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    }
                }
                Instruction::Ktest(a)/*pops stack in hopes of a true boolean,if false, jumps*/=>{
                    let sframe = frame.current_frame();
                    let boolean = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    if !boolean.as_bool() {
                        match a {
                            0|1=>{},
                            _=>{
                                let mut i = ProgramCounter::from(pc);
                                i.jump_forward(a-1)?; 
                                pc = i.as_usize();
                            }
                        }
                    }
                }
                Instruction::Rtest(a)/*pops stack in hopes of a true boolean,if false, jumps*/=>{
                    let sframe = frame.current_frame();
                    let boolean = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    if !boolean.is_null() {
                        match a {
                            0|1=>{},
                            _=>{
                                let mut i = ProgramCounter::from(pc);
                                i.jump_forward(a-1)?; 
                                pc = i.as_usize();
                            }
                        }
                        sframe.push(boolean);
                    }
                }
            //  Instruction::Mtdom(a)=>,
            
                
                Instruction::AAnd=>{
                    let sframe = frame.current_frame();
                    let two = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    let one = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    sframe.push(if one.as_bool() && two.as_bool() {
                        Value::Bool(true)
                    } else {
                        Value::Bool(false)
                    })
                },
                Instruction::And=>todo!(),
                Instruction::AOr=>{
                    let sframe = frame.current_frame();
                    let two = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    let one = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    sframe.push(if one.as_bool() || two.as_bool() {
                        Value::Bool(true)
                    } else {
                        Value::Bool(false)
                    })
                },
                Instruction::Or=>todo!(),
                Instruction::AXor=>{
                    let sframe = frame.current_frame();
                    let two = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    let one = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    let one = one.as_bool();
                    let two = two.as_bool();
                    sframe.push(if one == two{
                        Value::Bool(false)
                    } else {
                        Value::Bool(true)
                    })
                },
                Instruction::Xor=>todo!(),
                Instruction::Less=>{
                    let sframe = frame.current_frame();
                    let two = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    let one = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                   
                    inline_call_builtin_member_with_arg!(self,current_function,callframe,frame,sframe,pc,one,two,__le,le,cur_err,'main_loop);
                },
                Instruction::Great=>{
                    let sframe = frame.current_frame();
                    let two = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    let one = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    inline_call_builtin_member_with_arg!(self,current_function,callframe,frame,sframe,pc,one,two,__gre,gre,cur_err,'main_loop);
                
                },
                Instruction::LessEq=>{
                    let sframe = frame.current_frame();
                    let two = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    let one = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    inline_call_builtin_member_with_arg!(self,current_function,callframe,frame,sframe,pc,one,two,__leq,leq,cur_err,'main_loop);
                
                },
                Instruction::GreatEq=>{
                    let sframe = frame.current_frame();
                    let two = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    let one = match sframe.pop_or_error() {
                        Ok(a)=>a,
                        Err(err)=>{
                            cur_err = Some(err.into());
                            unwind_zin_err!(frame,current_function,callframe,pc,cur_err);
                            continue 'main_loop;
                        }
                    };
                    inline_call_builtin_member_with_arg!(self,current_function,callframe,frame,sframe,pc,one,two,__greq,greq,cur_err,'main_loop);
                
                },
                Instruction::Pg45Break|Instruction::Pg45Continue=>{}
            }
        }
       Ok(Value::NULL)
    }
    #[inline]
    pub fn call(&self,function:GcObject,args:Vec<Value>)->Result<Value> {
        let args = args.arg();
        self.__call(function,Frame::from(args),Vec::new())
    }
    pub fn call_method(&self,function:GcObject,method:ZString,args:Vec<Value>) -> Result<Value> {
        let mut aargs = vec![Value::Object(function.clone())];
        aargs.extend(args.arg());
        let mem:Value =method.into();
        match function.member_unchecked(mem.clone()) {
            Some(a)=>match &a {
                Value::Object(x) => {
                    self.__call(x.clone(),Frame::from(aargs),Vec::new())
                }
                _=>Err(Error::InvalidCall(format!(".{}",mem))),
            }
            _=>Err(Error::InvalidCall(format!(".{}",mem)))
        }
    }
}