
use crate::types::*;
use crate::object::*;
use crate::error::*;
use std::borrow::Borrow;
use function::Function;
pub mod property {
    use crate::zstring::*;
    static_tstring!(__builtin);
    static_tstring!(print);
    static_tstring!(assert);
    static_tstring!(fassert);
    static_tstring!(input);
    static_tstring!(append);
    static_tstring!(pop);
    static_tstring!(push);
    static_tstring!(insert);
    static_tstring!(remove);
    static_tstring!(empty);
    static_tstring!(join);
    static_tstring!(map);
    static_tstring!(props);
    static_tstring!(get_index);
    static_tstring!(iter);
    static_tstring!(range);
    static_tstring!(skip);
    static_tstring!(collect);
    static_tstring!(starts_with);
    static_tstring!(ends_with);
    
    static_tstring!(count);
    static_tstring!(matches);
    static_tstring!(split);
    
    
    static_tstring!(is_callable);
    static_tstring!(is_bytevec);
    static_tstring!(is_vec);
    static_tstring!(is_anyvec);
    static_tstring!(is_map);
    static_tstring!(is_native);
    static_tstring!(is_zin_code);
    static_tstring!(is_string);
    static_tstring!(is_object);
    static_tstring!(is_str);
    static_tstring!(is_bytes);
    static_tstring!(is_bool);
    static_tstring!(is_null);
    static_tstring!(is_method);
    static_tstring!(is_container);
    static_tstring!(is_number);
    static_tstring!(is_signed);
    static_tstring!(is_unsigned);
    static_tstring!(is_i8);
    static_tstring!(is_i16);
    static_tstring!(is_i32);
    static_tstring!(is_i64);
    static_tstring!(is_f32);
    static_tstring!(is_f64);
    static_tstring!(is_isize);
    static_tstring!(is_usize);
    static_tstring!(is_u8);
    static_tstring!(is_u16);
    static_tstring!(is_u32);
    static_tstring!(is_u64);
    static_named_tstring!("type",type_of);
    static_tstring!(__formula);
    pub mod accessors {
        use super::*;
        static_tstring!(__str);
        static_tstring!(__bytes);
        static_tstring!(__set);
        static_tstring!(__get);
        static_tstring!(__eq);
        static_tstring!(__neq);
        static_tstring!(__add);
        static_tstring!(__slice);
        static_tstring!(__sub);
        static_tstring!(__div);
        static_tstring!(__mul);
        static_tstring!(__exp);
        static_tstring!(__shl);
        static_tstring!(__shr);
        static_tstring!(__le);
        static_tstring!(__leq);
        static_tstring!(__gre);
        static_tstring!(__greq);
        static_tstring!(__doc);
        static_tstring!(__len);
    }
}
macro_rules! get_self {
    ($args:ident)=>{
        {
            let mut hashed_iter_ggvcc = $args.iter();
            match hashed_iter_ggvcc.next() {
                Some(a)=>match a {
                    crate::types::Value::Object(b)=>Ok((b.clone(),hashed_iter_ggvcc)),
                    _=>Err(crate::error::Error::InvalidSelf(format!("{} ",a)))
                },
                _=>Err(crate::error::Error::InvalidSelf(format!("None")))
            }
        }
    }
}

pub mod zstring {
    use super::*;


    use crate::object::interpreter::Executor;
    #[inline]
    pub fn invoke_to_string(exec:&Executor,obj:&GcObject)->Option<ZString> {
        if let Ok(b) = exec.call_method(obj.clone(),property::accessors::__str(),Vec::with_capacity(0)) {
                match &b{
                    Value::String(a)=>{
                        Some(a.clone())
                    }
                    Value::Bytes(a)=>match std::str::from_utf8(&a) {
                        Ok(a)=>Some(ZString::from(a)),
                        _=>None
                    },
                    _=>Some(ZString::from(format!("{}",b).as_str()))
                }
        } else if let Ok(b) =  exec.call_method(obj.clone(),property::accessors::__bytes(),Vec::with_capacity(0)) {
            match &b{
                Value::String(a)=>{
                    Some(a.clone())
                }
                Value::Bytes(a)=>match std::str::from_utf8(&a) {
                    Ok(a)=>Some(ZString::from(a)),
                    _=>None
                },
                _=>Some(ZString::from(format!("{}",b).as_str()))
            }
        } else {
            None
        }
    }
    #[inline]
    pub fn invoke_to_bytes(exec:&Executor,obj:&GcObject)->ZBytes {
        if let Ok(b) = exec.call_method(obj.clone(),property::accessors::__bytes(),Vec::with_capacity(0)) {
                match &b{
                    Value::String(a)=>{
                        ZBytes::from(a.as_bytes())
                    }
                    Value::Bytes(a)=>a.clone(),
                    _=>ZBytes::from(Vec::new())
                }
        } else if let Ok(b) =  exec.call_method(obj.clone(),property::accessors::__str(),Vec::with_capacity(0)) {
            match &b{
                Value::String(a)=>{
                    ZBytes::from(a.as_bytes())
                }
                Value::Bytes(a)=>a.clone(),
                _=>ZBytes::from(Vec::new())
            }
        } else {
            ZBytes::from(Vec::new())
        }
    }
}

pub mod implication {
    use crate::types::*;
    use crate::object::*;
    use crate::error::*;
    use function::Function;
    use interpreter::Executor;
    use std::convert::TryFrom;

    pub fn push(e:Executor,mut args:Vec<Value>)->Result<Value>{
        let (mut mself,iter) = get_self!(args)?;
        for arg in iter {
            mself.push(&e,arg.clone())?;
        }
        Ok(Value::NULL)
    }
    pub fn add(e:Executor,mut args:Vec<Value>)->Result<Value>{
        let (mself,iter) = get_self!(args)?;
        let mut begin = match mself.try_borrow() {
            Ok(a)=>match &a.contents.vec {
                Some(InnerContainer::String(a))=>{
                    String::from(a.as_str())
                }
                _=>{
                    String::new()
                }
            }
            _=>String::new()
        };
        for arg in iter {
            begin.push_str(&arg.invoke_to_string(&e));
        }
        Ok(begin.into())
    }
    pub (crate) fn gc_contains(this:&[GcObject],obj:&GcObject)->bool {
        for i in this {
            if i.eq(obj) {
                return true;
            }
        }
        false
    }
    pub fn starts_with(e:Executor,mut args:Vec<Value>) -> Result<Value> {
        let (mut mself,_) = get_self!(args)?;
        match args.len() {
            0=>Err(Error::InvalidArgs(2,1)),
            _=>{
                let string = args[0].invoke_to_string(&e);
                match mself.try_borrow() {
                    Ok(a)=>match &a.contents.vec{
                        Some(InnerContainer::String(s))=>{Ok(s.as_str().starts_with(string.as_ref()).into() )}
                        _=>Err(Error::Custom("TypeError".to_string(),"Expected string".to_string()))
                    },
                    Err(e)=>Err(Error::InvalidBorrow(file!(),line!()))
                }
            }
        }
    }
    pub fn ends_with(e:Executor,mut args:Vec<Value>) -> Result<Value> {
        let (mut mself,_) = get_self!(args)?;
        match args.len() {
            0=>Err(Error::InvalidArgs(2,1)),
            _=>{
                let string = args[0].invoke_to_string(&e);
                match mself.try_borrow() {
                    Ok(a)=>match &a.contents.vec{
                        Some(InnerContainer::String(s))=>{Ok(s.as_str().ends_with(string.as_ref()).into() ) }
                        _=>Err(Error::Custom("TypeError".to_string(),"Expected string".to_string()))
                    },
                    Err(e)=>Err(Error::InvalidBorrow(file!(),line!()))
                }
            }
        }
    }
    pub fn contains(e:Executor,mut args:Vec<Value>) -> Result<Value> {
        let (mut mself,_) = get_self!(args)?;
        match args.len() {
            0=>Err(Error::InvalidArgs(2,1)),
            _=>{
                let string = args[0].invoke_to_string(&e);
                match mself.try_borrow() {
                    Ok(a)=>match &a.contents.vec{
                        Some(InnerContainer::String(s))=>{Ok(s.as_str().contains(string.as_ref()).into() ) }
                        _=>Err(Error::Custom("TypeError".to_string(),"Expected string".to_string()))
                    },
                    Err(e)=>Err(Error::InvalidBorrow(file!(),line!()))
                }
            }
        }
    }
    pub fn count_occurrences(e:Executor,mut args:Vec<Value>) -> Result<Value> {
        let (mut mself,_) = get_self!(args)?;
        match args.len() {
            0=>Err(Error::InvalidArgs(2,1)),
            _=>{
                let string = args[0].invoke_to_string(&e);
                match mself.try_borrow() {
                    Ok(a)=>match &a.contents.vec{
                        Some(InnerContainer::String(s))=>{Ok(s.as_str().matches(string.as_ref()).count().into() ) }
                        _=>Err(Error::Custom("TypeError".to_string(),"Expected string".to_string()))
                    },
                    Err(e)=>Err(Error::InvalidBorrow(file!(),line!()))
                }
            }
        }
    }
    pub fn matches(e:Executor,mut args:Vec<Value>) -> Result<Value> {
        let (mut mself,_) = get_self!(args)?;
        let len = args.len(); 
        match args.len() {
            0=>Err(Error::InvalidArgs(2,1)),
            _=>{
                let string = args[0].invoke_to_string(&e);
                match mself.try_borrow() {
                    Ok(a)=>match &a.contents.vec{
                        Some(InnerContainer::String(s))=>{
                            let m = s.as_str().matches(string.as_ref()).map(|x|{ZString::from(x).into()});
                            
                            let m:Vec<Value> = match len{
                                1=>m.collect(),
                                _=>{
                                    match Number::try_from(args[1].clone()){
                                        Ok(a)=>m.take(a.usize()).collect(),
                                        _=>m.collect(),
                                    }
                                }
                            };
                            if m.len()==0 {
                                return Ok(Value::NULL)
                            }
                            Ok(super::vec_from(m).into() )
                        }
                        _=>Err(Error::Custom("TypeError".to_string(),"Expected string".to_string()))
                    },
                    Err(e)=>Err(Error::InvalidBorrow(file!(),line!()))
                }
            }
        }
    }
    pub fn split(e:Executor,mut args:Vec<Value>) -> Result<Value> {
        let (mut mself,_) = get_self!(args)?;
        let len = args.len(); 
        match args.len() {
            0=>Err(Error::InvalidArgs(2,1)),
            _=>{
                let string = args[0].invoke_to_string(&e);
                match mself.try_borrow() {
                    Ok(a)=>match &a.contents.vec{
                        Some(InnerContainer::String(s))=>{
                            let m = s.as_str().split(string.as_ref()).map(|x|{ZString::from(x).into()});
                            
                            let m:Vec<Value> = match len{
                                1=>m.collect(),
                                _=>{
                                    let num = Number::try_from(args[1].clone())?.usize();
                                    m.take(num).collect()
                                }
                            };
                            if m.len()==0 {
                                return Ok(Value::NULL)
                            }
                            Ok(super::vec_from(m).into() )
                        }
                        _=>Err(Error::Custom("TypeError".to_string(),"Expected string".to_string()))
                    },
                    Err(e)=>Err(Error::InvalidBorrow(file!(),line!()))
                }
            }
        }
    }
    pub fn _str(e:Executor,mut args:Vec<Value>)->Result<Value>{
        let (mut mself,_) = get_self!(args)?;
        Ok(mself.__str(&e).unwrap_or("0xDEADBEEF".into()))
    }
    pub fn pop(_:Executor,args:Vec<Value>)->Result<Value>{
        let (mut mself,_) = get_self!(args)?;
        mself.pop()
    }
    pub fn zstr_eq(e:Executor,args:Vec<Value>)->Result<Value>{
        let (mut mself,mut iter) = get_self!(args)?;
        let a:Vec<&Value> = iter.collect();
        Ok(mself.zstr_eq(&e,&a).into())
    }
    
    pub fn len(_:Executor,args:Vec<Value>)->Result<Value>{
        let (mut mself,_) = get_self!(args)?;
        Ok(mself.vec_len()?.into())
    }
    pub fn join(e:Executor,args:Vec<Value>)->Result<Value>{
        let (mself,mut iter) = get_self!(args)?;
        let iter:Vec<&Value> = iter.collect();
        Ok(mself.vec_join(&e,&iter)?)
    }
    pub fn is_empty(_:Executor,args:Vec<Value>)->Result<Value>{
        let (mut mself,_) = get_self!(args)?;
        Ok((mself.vec_len()? == 0 ).into())
    }
    pub fn remove(_:Executor,args:Vec<Value>)->Result<Value>{
        let (mut mself,mut iter) = get_self!(args)?;
        let args:Vec<&Value> = iter.collect(); 
        match args.len() {
            0=>{
                Ok(Value::NULL)
            }
            1=>{
                let num:Number = Number::try_from(args[0].clone())?;
                mself.vec_remove(num.usize())
            }
            _=>{
                let num:Number = Number::try_from(args[0].clone())?;
                let numt:Number = Number::try_from(args[1].clone())?;
                mself.vec_remove(num.usize()..numt.usize())
            }
        }
    }
    
    pub fn get_vec(_:Executor,args:Vec<Value>)->Result<Value>{
        let (mut mself,mut iter) = get_self!(args)?;
        let args:Vec<&Value> = iter.collect(); 
        match args.len() {
            0=>{
                Ok(Value::NULL)
            }
            1=>{
                let num:Number = Number::try_from(args[0].clone())?;
                mself.index(num.usize())
            }
            _=>{
                let num:Number = Number::try_from(args[0].clone())?;
                let numt:Number = Number::try_from(args[1].clone())?;
                mself.index(num.usize()..numt.usize())
            }
        }
    }
    pub fn get_map(_:Executor,args:Vec<Value>)->Result<Value>{
        let (mut mself,mut iter) = get_self!(args)?;
        let args:Vec<&Value> = iter.collect(); 
        match args.len() {
            0=>{
                Ok(Value::NULL)
            }
            1=>{
                mself.index(args[0].clone())
            }
            _=>{
                let num:Number = Number::try_from(args[0].clone())?;
                let numt:Number = Number::try_from(args[1].clone())?;
                mself.index(num.usize()..numt.usize())
            }
        }
    }
    pub fn get_map_index(_:Executor,args:Vec<Value>)->Result<Value>{
        let (mut mself,mut iter) = get_self!(args)?;
        let args:Vec<&Value> = iter.collect(); 
        match args.len() {
            0=>{
                Ok(Value::NULL)
            }
            1=>{
                mself.map_index(args[0].clone())
            }
            _=>{
                let num:Number = Number::try_from(args[0].clone())?;
                let numt:Number = Number::try_from(args[1].clone())?;
                mself.index(num.usize()..numt.usize())
            }
        }
    }
    pub fn set_vec(e:Executor,args:Vec<Value>)->Result<Value>{
        let (mut mself,mut iter) = get_self!(args)?;
        let args:Vec<&Value> = iter.collect(); 
        match args.len() {
            0|1=>{
                Ok(Value::NULL)
            }
            2=>{
                let num = Number::try_from(args[0].clone())?.usize();
                mself.set_index(&e,num,args[1].clone())?;
                Ok(Value::NULL)
            }
            _=>{
                let num:Number = Number::try_from(args[0].clone())?;
                let numt:Number = Number::try_from(args[1].clone())?;
                mself.set_index(&e,num.usize()..numt.usize(),args[2].clone())?;
                Ok(Value::NULL)
            }
        }
    }
    pub fn set_map(e:Executor,args:Vec<Value>)->Result<Value>{
        let (mut mself,mut iter) = get_self!(args)?;
        let args:Vec<&Value> = iter.collect(); 
        match args.len() {
            0|1=>{
                Ok(Value::NULL)
            }
            _=>{
                mself.set_index(&e,args[0].clone(),args[1].clone())?;
                Ok(Value::NULL)
            }
           
        }
    }
}



use property::accessors;
#[inline]
fn map_accessors(obj : GcObject)->GcObject {
    obj.with_method(accessors::__get(), Function::native_root_method(implication::get_map as NativeMethod).with_nargs(2))
    .with_method(accessors::__set(), Function::native_root_method(implication::set_map as NativeMethod).with_nargs(2))
    .with_method(property::get_index(), Function::native_root_method(implication::get_map_index as NativeMethod).with_nargs(2))
}
#[inline]
fn vec_accessors(obj : GcObject)->GcObject {
    immutable_vec_accessors(obj)
    .with_method(property::push(),  Function::native_root_method(implication::push as NativeMethod).with_nargs(2))
    .with_method(property::pop(),   Function::native_root_method(implication::pop as NativeMethod).with_nargs(1))
}
#[inline]
fn immutable_vec_accessors(obj : GcObject)->GcObject {
    obj.with_method(accessors::__get(), Function::native_root_method(implication::get_vec as NativeMethod).with_nargs(2))
    .with_method(accessors::__set(), Function::native_root_method(implication::set_vec as NativeMethod).with_nargs(2))
    .with_method(property::get_index(), Function::native_root_method(implication::get_vec as NativeMethod).with_nargs(2))
    .with_method(property::join(),   Function::native_root_method(implication::join as NativeMethod).with_nargs(2))
}
#[inline]
fn core(obj:Object)->GcObject {
    core_immutable(obj)
   .with_method(property::remove(),   Function::native_root_method(implication::remove as NativeMethod).with_nargs(2))
}
#[inline]
fn core_immutable(obj:Object)->GcObject {
    GcObject::from(obj)
    .with_method(accessors::__len(),   Function::native_root_method(implication::len as NativeMethod).with_nargs(1))
    .with_method(accessors::__str(),   Function::native_root_method(implication::_str as NativeMethod).with_nargs(1))
    .with_method(property::empty(),  Function::native_root_method(implication::is_empty as NativeMethod).with_nargs(1))
}
#[inline]
fn base_vec(obj:Object)->GcObject {
    vec_accessors(core(obj))
}
#[inline]
fn base_deck(obj:Object)->GcObject {
    immutable_vec_accessors(core_immutable(obj))
}
#[inline]
fn base_map(obj:Object)->GcObject {
    map_accessors(core(obj))
}


#[inline]
pub fn vec_from<T:Into<InternedVector>>(args:T)->GcObject {
    let obj =base_vec(Object::with_vec(args,access::ObjAccess::NONE));
    obj
}
#[inline]
pub fn byte_vec_from(args:&[u8])->GcObject {
    let obj =base_vec(Object::with_vec(Vec::from(args),access::ObjAccess::NONE));
    obj
}

#[inline]
pub fn string_from<T:Into<InternedString>>(arg:T)->GcObject {
    let obj =base_vec(Object::with_string(arg,access::ObjAccess::NONE))
    .with_method(accessors::__eq(),   Function::native_root_method(implication::zstr_eq as NativeMethod).with_nargs(2))
    .with_method(accessors::__add(),   Function::native_root_method(implication::add as NativeMethod).with_nargs(1))
    .with_method(property::starts_with(),Function::native_root_method(implication::starts_with).with_nargs(1))
    .with_method(property::ends_with(),Function::native_root_method(implication::ends_with).with_nargs(1))
    .with_method(property::count(),Function::native_root_method(implication::count_occurrences).with_nargs(1))
    .with_method(property::matches(),Function::native_root_method(implication::matches).with_nargs(1))
    .with_method(property::split(),Function::native_root_method(implication::split).with_nargs(1));
    obj
}
#[inline]
pub fn map_from(arg:PropertyMap)->GcObject {
    let obj =base_map(Object::with_map(arg,access::ObjAccess::NONE));
    obj
}

pub mod glob_implication {
    use super::property;
    use crate::types::*;
    use crate::object::*;
    use crate::error::*;
    use interpreter::Executor;
    use function::Function;
    use std::convert::TryFrom;
    use std::borrow::Borrow;
    #[inline]
    pub fn vec_iter(obj:GcObject,len:usize) ->GcObject {
        let code = vec![
            Instruction::Ldup(0),
            Instruction::Ldup(1),
            Instruction::Less,
            Instruction::Ktest(14),
            Instruction::Ldup(2),
            Instruction::Ldm(property::get_index().into()),
            Instruction::Ldup(0),
            Instruction::Call(1),
            Instruction::Stl(0,0),
            Instruction::Ldup(0),
            Instruction::Cld(1.into()),
            Instruction::Add,
            Instruction::Stup(0),
            Instruction::Ldl(0,0),
            Instruction::Cl1,
            Instruction::MkArr(2),
            Instruction::Return(1),
            Instruction::Cld(Value::NULL),
            Instruction::Cl0,
            Instruction::MkArr(2),
            Instruction::Return(1)
         ];
        Function::root(access::ObjAccess::NONE).with_code(code, false).with_nargs(0).with_nlocals(1).with_upvals(super::vec_from(vec![0usize.into(),Value::Num(len.into()),obj.into()])).object()
    }
    #[inline]
    pub fn num_iter<Begin:Into<Number>,End:Into<Number>> (beg: Begin,len:End,inc:Option<Number>) ->GcObject {
        /*
        0:      ldup 0
        1:      ldup 1
        2:      cmp
        3:      ktest 5 (8)
        4:      ldup 0
        5:      cl0
        6:      mkarr 2
        7:      ret 1
        8:      ldup 0
        9:      stl (0,0)
        10:     ldup 0
        11:     ldup 2
        12:     add
        13:     stup 0
        14:     ldl (0,0)
        15:     cl1
        16:     mkarr 2
        17:     ret 1 
        */
        let inc = match inc {
            Some(i) =>i,
            _=>1usize.into()
        };
        let beg:Number = beg.into();
        let len:Number= len.into();
        let code = vec![
            Instruction::Ldup(0),
            Instruction::Cld(len.into()),
            Instruction::Cmp,
            Instruction::Ktest(5),
            Instruction::Ldup(0),
            Instruction::Cl0,
            Instruction::MkArr(2),
            Instruction::Return(1),
            Instruction::Ldup(0),
            Instruction::Stl(0,0),
            Instruction::Ldup(0),
            Instruction::Cld(inc.into()),
            Instruction::Add,
            Instruction::Stup(0),
            Instruction::Ldl(0,0),
            Instruction::Cl1,
            Instruction::MkArr(2),
            Instruction::Return(1)
         ];
        Function::root(access::ObjAccess::NONE).with_code(code, false).with_nargs(0).with_nlocals(1).with_upvals(super::vec_from(vec![Value::Num(beg)])).object()
    }
    #[inline]
    pub fn map_code()->Vec<Instruction> {
        vec![
            Instruction::Ldup(0),
            Instruction::Call(0),
            Instruction::Unpack(2),
            Instruction::Stl(0,0),
            Instruction::Stl(0,1),
            Instruction::Ldl(0,1),
            Instruction::Ktest(7),
            Instruction::Ldup(1),
            Instruction::Ldl(0,0),
            Instruction::Call(1),
            Instruction::Cl1,
            Instruction::MkArr(2),
            Instruction::Return(1),
            Instruction::Ldl(0,0),
            Instruction::Ldl(0,1),
            Instruction::MkArr(2),
            Instruction::Return(1),
        ]
    }
    #[inline]
    pub fn skip_code()->Vec<Instruction> {
        vec![
        Instruction::Cld(0.into()),
        Instruction::Stl(0,2),
        Instruction::Ldl(0,1),
        Instruction::Cld(0.into()),
        Instruction::Great,
        Instruction::Ktest(22),
        Instruction::Ldl(0,0),
        Instruction::Dup,
        Instruction::Call(0),
        Instruction::Unpack(2),
        Instruction::Rot,
        Instruction::Ktest(12),
        Instruction::Stl(0,3),
        Instruction::Ldl(0,2),
        Instruction::Cld(1.into()),
        Instruction::Add,
        Instruction::Stl(0,2),
        Instruction::Ldl(0,2),
        Instruction::Ldl(0,1),
        Instruction::GreatEq,
        Instruction::Ktest(2 ),
        Instruction::Jmp(1),
        Instruction::Jmp(-16),
        Instruction::Pop,
        Instruction::Pop,
        Instruction::Ldl(0,0),
        Instruction::Return(1),
        Instruction::Ldl(0,0),
        Instruction::Return(1),
        ]
    }
    #[inline]
    pub fn collect_code()->Vec<Instruction> {
        vec![
            Instruction::MkArr(0),
            Instruction::Stl(0,1),
            Instruction::Ldl(0,0),
            Instruction::Dup,
            Instruction::Call(0),
            Instruction::Unpack(2),
            Instruction::Rot,
            Instruction::Ktest(8),
            Instruction::Stl(0,2),
            Instruction::Ldl(0,1),
            Instruction::Ldm(property::push().into()),
            Instruction::Ldl(0,2),
            Instruction::Call(1),
            Instruction::Pop,
            Instruction::Jmp(-12),
            Instruction::Pop,
            Instruction::Pop,
            Instruction::Ldl(0,1),
            Instruction::Return(1)
        ]
    }
    #[inline]
    pub fn skip_fn()->Function {
        Function::root(access::ObjAccess::NONE).with_code(skip_code(), false).with_nlocals(4).with_nargs(2)
    }
    #[inline]
    pub fn map_fn(iter:GcObject,lambda:GcObject)->Function {
        let iter:Value = iter.into();
        let lambda:Value = lambda.into();
        Function::root(access::ObjAccess::NONE).with_code(map_code(), false).with_nlocals(2).with_nargs(0)
        .with_upvals(super::vec_from(vec![iter,lambda]))
    }
    #[inline]
    pub fn collect_fn()->Function {
        Function::root(access::ObjAccess::NONE).with_code(collect_code(), false).with_nlocals(3).with_nargs(1)
    }
    #[inline]
    pub fn map(_:Executor,args:Vec<Value>)->Result<Value> {
        match args.len() {
            0=>Ok(Value::NULL),
            1=>Ok(args[0].clone()),
            _=>{
                let iter = GcObject::try_from(args[0].clone())?;
                let lambda = GcObject::try_from(args[1].clone())?;
                Ok(map_fn(iter,lambda).into())
            }
        }
    }
    #[inline]
    pub fn range(_:Executor,args:Vec<Value>)->Result<Value> {
        match args.len() {
            0=> Ok(num_iter(0,0,None).into()),
            1=> {
                let num = Number::try_from(args[0].clone())?;
                if num < 0.into() {
                    Ok(num_iter(0,num,Some((-1).into())).into())
                } else {
                    Ok(num_iter(0,num,None).into())
                }
            }
            2=> {
                let num = Number::try_from(args[0].clone())?;
                let end = Number::try_from(args[1].clone())?;

                if num > end {
                    Ok(num_iter(num,end,Some((-1).into())).into())
                } else {
                    Ok(num_iter(num,end,None).into())
                }
            }
            _=>{
                let num = Number::try_from(args[0].clone())?;
                let end = Number::try_from(args[1].clone())?;
                let amt = Number::try_from(args[2].clone())?;
                Ok(num_iter(num,end,Some(amt)).into())
            }
        }
       
    }
    #[inline]
    pub fn iter(_:Executor,args:Vec<Value>)->Result<Value> {
       
        match get_self!(args) {
            Ok((a,_))=>{
              Ok(vec_iter(a.clone(),a.vec_len()?).into())
            }
            _=>Ok(Value::NULL)
        }
       
    }
    pub fn props(_:Executor,args:Vec<Value>)->Result<Value>{
        match args.len() {
            0=>Ok(Value::NULL),
            _=>{
                match &args[0] {
                    Value::Object(o)=>{
                        match o.inner.try_borrow() {
                            Ok(a)=>match &a.contents.property {
                                Some(a)=>Ok(super::map_from(a.clone()).into() ),
                                _=>Ok(Value::NULL),
                            }
                            _=>Ok(args[0].clone())
                        }
                    }
                    _=>Ok(args[0].clone())

                }
            }
        }
    }
    pub fn print(exec:Executor,a:Vec<Value>)->Result<Value> {
        for i in a {
            print!("{} ",i.invoke_to_string(&exec))
        }
        println!();
        Ok(Value::NULL)
    }
    pub fn assert(e:Executor,a:Vec<Value>)->Result<Value> {
        match a.len() {
            0=>Ok(Value::NULL),
            1=>match a[0].as_bool() {
                true=>Ok(Value::NULL),
                _=>Err(Error::AssertionFailed("".to_string()))
            }
            _=>match a[0].as_bool() {
                true=>Ok(Value::NULL),
                _=>Err(Error::AssertionFailed(a[1].invoke_to_string(&e).to_string()))
            }
        }
        
    }
    pub fn fassert(e:Executor,a:Vec<Value>)->Result<Value> {
        match a.len() {
            0=>Ok(Value::NULL),
            1=>match a[0].as_bool() {
                false=>Ok(Value::NULL),
                _=>Err(Error::AssertionFailed("".to_string()))
            }
            _=>match a[0].as_bool() {
                false=>Ok(Value::NULL),
                _=>Err(Error::AssertionFailed(a[1].invoke_to_string(&e).to_string()))
            }
        }
        
    }
    #[inline]
    pub(crate) fn _inspect_code(_:Executor,args:Vec<Value>)->Result<Value> {
        match get_self!(args) {
            Ok((mut mself,_))=>{
                
                    let mut out = match mself.member(crate::object::function::options::__nlocals()) {
                        Some(a)=>format!(".locals {}\n", a.as_string()),
                        _=>String::from(".locals 0\n")
                    };
                    match mself.member(crate::object::function::options::__nargs()) {
                        Some(a)=>out.push_str(&format!(".nargs {}\n",a.as_string())),
                        _=>out.push_str(&format!(".nargs 0\n"))
                    };
                    match mself.member(crate::object::function::options::__upvals()) {
                        Some(a)=>out.push_str(&format!(".upvals {}\n",a.as_string())),
                        _=>out.push_str(&format!(".upvals 0\n"))
                    };

                    match mself.native() {
                    Some(_)=>{
                        out.push_str("native");
                        Ok(ZString::from(out).into())
                    },
                    _=>match mself.code() {
                        Some(a)=>{
                            if let ObjFunction::Zin(native) = &a {
                                match native.try_borrow() {
                                    Ok(a)=>{
                                        let a:&Vec<Instruction> =a.as_ref();
                                        let mut idx = 0;
                                        for i in a {
                                            match &i {
                                                Instruction::Jmp(a)=>out.push_str(&format!("{}:\tjmp {} ({})\n",idx,a,idx+a)),
                                                Instruction::Ktest(a)=>out.push_str(&format!("{}:\tktest {} ({})\n",idx,a,(idx as usize)+a)),
                                                Instruction::Test(a)=>out.push_str(&format!("{}:\ttest {} ({})\n",idx,a,(idx as usize)+a)),
                                                _=>out.push_str(&format!("{}:\t{}\n",idx,i))
                                            }
                                            idx+=1;
                                        }
                                        out.pop();
                                    }
                                    _=>{}
                                }   
                            }                   
                            Ok(out.into())
                        }
                        _=>Ok(Value::NULL)
                    }
                }
            }
            _=>Ok(Value::NULL)
        }
    }
    #[inline]
    pub fn is_method(_:Executor,args:Vec<Value>)->Result<Value>{
        match args.len() {
            0=>Ok(false.into()),
            _=>{
                for arg in args {
                    match &arg {
                        Value::Object(a)=>match a.is_method() {
                            true=>{},
                            _=>return Ok(false.into())
                        }
                        _=>return Ok(false.into())
                    }
                }
                Ok(true.into())
            }
        }
    }
    #[inline]
    pub fn type_of(e:Executor,args:Vec<Value>)->Result<Value>{
        match args.len() {
            0=>Err(Error::InvalidArgs(1,0)),
            _=>{
                return Ok(args[0].ztype().into())
            }
        }
    }
    #[inline]
    pub fn is_map(_:Executor,args:Vec<Value>)->Result<Value> {
        for arg in args {
            match &arg {
                Value::Object(a)=>match a.inner.try_borrow() {
                    Ok(a)=>match &a.vec{
                        Some(InnerContainer::Map(_))=>{}
                        _=>return Ok(false.into())
                    },
                    _=>return Ok(false.into())
                }
                _=>return Ok(false.into())
            }
        }
        Ok(true.into())
    }
    #[inline]
    pub fn is_vec(_:Executor,args:Vec<Value>)->Result<Value> {
        for arg in args {
            match &arg {
                Value::Object(a)=>match a.inner.try_borrow() {
                    Ok(a)=>match &a.vec{
                        Some(InnerContainer::Vector(InternedVector::Vector(_)))=>{}
                        _=>return Ok(false.into())
                    },
                    _=>return Ok(false.into())
                }
                _=>return Ok(false.into())
            }
        }
        Ok(true.into())
    }
    #[inline]
    pub fn is_anyvec(_:Executor,args:Vec<Value>)->Result<Value> {
        for arg in args {
            match &arg {
                Value::Object(a)=>match a.inner.try_borrow() {
                    Ok(a)=>match &a.vec{
                        Some(a)=>match a{
                            InnerContainer::Map(_)=>return Ok(false.into()),
                            _=>{}
                        }
                        _=>return Ok(false.into())
                    },
                    _=>return Ok(false.into())
                }
                _=>return Ok(false.into())
            }
        }
        Ok(true.into())
    }
    #[inline]
    pub fn is_container(_:Executor,args:Vec<Value>)->Result<Value> {
        for arg in args {
            match &arg {
                Value::Object(a)=>match a.inner.try_borrow() {
                    Ok(a)=>match &a.vec{
                        Some(a)=>{}
                        _=>return Ok(false.into())
                    },
                    _=>return Ok(false.into())
                }
                _=>return Ok(false.into())
            }
        }
        Ok(true.into())
    }
    #[inline]
    pub fn is_string(_:Executor,args:Vec<Value>)->Result<Value> {
        for arg in args {
            match &arg {
                Value::Object(a)=>match a.inner.try_borrow() {
                    Ok(a)=>match &a.vec{
                        Some(InnerContainer::String(_))=>{}
                        _=>return Ok(false.into())
                    },
                    _=>return Ok(false.into())
                }
                _=>return Ok(false.into())
            }
        }
        Ok(true.into())
    }
    #[inline]
    pub fn is_bytevec(_:Executor,args:Vec<Value>)->Result<Value> {
        for arg in args {
            match &arg {
                Value::Object(a)=>match a.inner.try_borrow() {
                    Ok(a)=>match &a.vec{
                        Some(InnerContainer::Vector(InternedVector::ByteVector(_)))=>{}
                        _=>return Ok(false.into())
                    },
                    _=>return Ok(false.into())
                }
                _=>return Ok(false.into())
            }
        }
        Ok(true.into())
    }
    #[inline]
    pub fn is_callable(_:Executor,args:Vec<Value>)->Result<Value>{
        match args.len() {
            0=>Ok(false.into()),
            _=>{
                for arg in args {
                    match &arg {
                        Value::Object(a)=>match a.is_callable(){
                            true=>{}
                            false=>return Ok(false.into())
                        },
                        _=>return Ok(false.into())
                    }
                }
                Ok(true.into())
            }
        }
    }
    #[inline]
    pub fn is_native(_:Executor,args:Vec<Value>)->Result<Value>{
        match args.len() {
            0=>Ok(false.into()),
            _=>{
                for arg in args {
                    match &arg {
                        Value::Object(a)=>match a.code() {
                            Some(ObjFunction::Native(a))=>{}
                            _=>return Ok(false.into())
                        }
                        _=>return Ok(false.into())
                    }
                }
                Ok(true.into())
            }
        }
    }
    #[inline]
    pub fn is_zin_code(_:Executor,args:Vec<Value>)->Result<Value>{
        match args.len() {
            0=>Ok(false.into()),
            _=>{
                for arg in args {
                    match &arg {
                        Value::Object(a)=>match &a.code() {
                            Some(ObjFunction::Zin(a))=>{}
                            _=>return Ok(false.into())
                        }
                        _=>return Ok(false.into())
                    }
                }
                Ok(true.into())
            }
        }
    }
    #[inline]
    pub fn is_object(_:Executor,args:Vec<Value>)->Result<Value>{
        match args.len() {
            0=>Ok(false.into()),
            _=>{
                for arg in args {
                    match &arg {
                        Value::Object(_)=>{}
                        _=>return Ok(false.into())
                    }
                }
                Ok(true.into())
            }
        }
    }
    #[inline]
    pub fn is_str(_:Executor,args:Vec<Value>)->Result<Value>{
        match args.len() {
            0=>Ok(false.into()),
            _=>{
                for arg in args {
                    match &arg {
                        Value::String(_)=>{

                        }
                        _=>return Ok(false.into())
                    }
                }
                Ok(true.into())
            }
        }
    }
    #[inline]
    pub fn is_bytes(_:Executor,args:Vec<Value>)->Result<Value>{
        match args.len() {
            0=>Ok(false.into()),
            _=>{
                for arg in args {
                    match &arg {
                        Value::Bytes(_)=>{

                        }
                        _=>return Ok(false.into())
                    }
                }
                Ok(true.into())
            }
        }
    }
    #[inline]
    pub fn is_bool(_:Executor,args:Vec<Value>)->Result<Value>{
        match args.len() {
            0=>Ok(false.into()),
            _=>{
                for arg in args {
                    match &arg {
                        Value::Bool(_)=>{

                        }
                        _=>return Ok(false.into())
                    }
                }
                Ok(true.into())
            }
        }
    }
    #[inline]
    pub fn is_null(_:Executor,args:Vec<Value>)->Result<Value>{
        match args.len() {
            0=>Ok(false.into()),
            _=>{
                for arg in args {
                    match &arg {
                        Value::NULL=>{

                        }
                        _=>return Ok(false.into())
                    }
                }
                Ok(true.into())
            }
        }
    }
    #[inline]
    pub fn is_number(_:Executor,args:Vec<Value>)->Result<Value>{
        match args.len() {
            0=>Ok(false.into()),
            _=>{
                for arg in args {
                    match &arg {
                        Value::Num(_)=>{

                        }
                        _=>return Ok(false.into())
                    }
                }
                Ok(true.into())
            }
        }
    }
    #[inline]
    pub fn is_i8(_:Executor,args:Vec<Value>)->Result<Value>{
        match args.len() {
            0=>Ok(false.into()),
            _=>{
                for arg in args {
                    match &arg{
                        Value::Num(Number::SNum(Signed::I8(a)))=>{

                        }
                        _=>return Ok(false.into())
                    }
                }
                Ok(true.into())
            }
        }
    }
    
    #[inline]
    pub fn is_i16(_:Executor,args:Vec<Value>)->Result<Value>{
        match args.len() {
            0=>Ok(false.into()),
            _=>{
                for arg in args {
                    match &arg{
                        Value::Num(Number::SNum(Signed::I16(a)))=>{

                        }
                        _=>return Ok(false.into())
                    }
                }
                Ok(true.into())
            }
        }
    }
    
    #[inline]
    pub fn is_i32(_:Executor,args:Vec<Value>)->Result<Value>{
        match args.len() {
            0=>Ok(false.into()),
            _=>{
                for arg in args {
                    match &arg{
                        Value::Num(Number::SNum(Signed::I32(a)))=>{

                        }
                        _=>return Ok(false.into())
                    }
                }
                Ok(true.into())
            }
        }
    }
    
    #[inline]
    pub fn is_i64(_:Executor,args:Vec<Value>)->Result<Value>{
        match args.len() {
            0=>Ok(false.into()),
            _=>{
                for arg in args {
                    match &arg{
                        Value::Num(Number::SNum(Signed::I64(a)))=>{

                        }
                        _=>return Ok(false.into())
                    }
                }
                Ok(true.into())
            }
        }
    }
    
    #[inline]
    pub fn is_f32(_:Executor,args:Vec<Value>)->Result<Value>{
        match args.len() {
            0=>Ok(false.into()),
            _=>{
                for arg in args {
                    match &arg{
                        Value::Num(Number::SNum(Signed::F32(a)))=>{

                        }
                        _=>return Ok(false.into())
                    }
                }
                Ok(true.into())
            }
        }
    }
    
    #[inline]
    pub fn is_f64(_:Executor,args:Vec<Value>)->Result<Value>{
        match args.len() {
            0=>Ok(false.into()),
            _=>{
                for arg in args {
                    match &arg{
                        Value::Num(Number::SNum(Signed::F64(a)))=>{

                        }
                        _=>return Ok(false.into())
                    }
                }
                Ok(true.into())
            }
        }
    }
    
    #[inline]
    pub fn is_isize(e:Executor,args:Vec<Value>)->Result<Value>{
        if cfg!(target_pointer_width = "32") {
            is_i32(e,args)
        } else {
            is_i64(e,args)
        }
    }
    
    #[inline]
    pub fn is_usize(e:Executor,args:Vec<Value>)->Result<Value>{
        if cfg!(target_pointer_width = "32") {
            is_u32(e,args)
        } else {
            is_u64(e,args)
        }
    }
    
    #[inline]
    pub fn is_u8(_:Executor,args:Vec<Value>)->Result<Value>{
        match args.len() {
            0=>Ok(false.into()),
            _=>{
                for arg in args {
                    match &arg{
                        Value::Num(Number::UNum(Unsigned::U8(a)))=>{

                        }
                        _=>return Ok(false.into())
                    }
                }
                Ok(true.into())
            }
        }
    }
    
    #[inline]
    pub fn is_u16(_:Executor,args:Vec<Value>)->Result<Value>{
        match args.len() {
            0=>Ok(false.into()),
            _=>{
                for arg in args {
                    match &arg{
                        Value::Num(Number::UNum(Unsigned::U16(a)))=>{

                        }
                        _=>return Ok(false.into())
                    }
                }
                Ok(true.into())
            }
        }
    }
    
    #[inline]
    pub fn is_u32(_:Executor,args:Vec<Value>)->Result<Value>{
        match args.len() {
            0=>Ok(false.into()),
            _=>{
                for arg in args {
                    match &arg{
                        Value::Num(Number::UNum(Unsigned::U32(a)))=>{

                        }
                        _=>return Ok(false.into())
                    }
                }
                Ok(true.into())
            }
        }
    }
    
    #[inline]
    pub fn is_u64(_:Executor,args:Vec<Value>)->Result<Value>{
        match args.len() {
            0=>Ok(false.into()),
            _=>{
                for arg in args {
                    match &arg{
                        Value::Num(Number::UNum(Unsigned::U64(a)))=>{

                        }
                        _=>return Ok(false.into())
                    }
                }
                Ok(true.into())
            }
        }
    }
    #[inline]
    pub fn is_signed(_:Executor,args:Vec<Value>)->Result<Value>{
        match args.len() {
            0=>Ok(false.into()),
            _=>{
                for arg in args {
                    match &arg{
                        Value::Num(Number::SNum(_))=>{

                        }
                        _=>return Ok(false.into())
                    }
                }
                Ok(true.into())
            }
        }
    }
    #[inline]
    pub fn is_unsigned(_:Executor,args:Vec<Value>)->Result<Value>{
        match args.len() {
            0=>Ok(false.into()),
            _=>{
                for arg in args {
                    match &arg{
                        Value::Num(Number::UNum(_))=>{

                        }
                        _=>return Ok(false.into())
                    }
                }
                Ok(true.into())
            }
        }
    }
    

}
pub mod encapsulation {
    use crate::types::*;
    use crate::object::*;
    use function::*;
    use access::*;
    use crate::error::*;

    macro_rules! encapsulate {
        ($($t:tt)*)=>{
            pub fn $($t)*(root:&mut Function) {
                let a = Function::native_root(ObjAccess::NONE,super::glob_implication::$($t)*);
                root.set_member(super::property::$($t)*(),a);
            }

        }
    }
    encapsulate!(print);
    encapsulate!(assert);
    encapsulate!(fassert);
    encapsulate!(is_callable);
    encapsulate!(is_native);
    encapsulate!(is_zin_code);
    encapsulate!(is_method);
    encapsulate!(is_vec);
    encapsulate!(is_anyvec);
    encapsulate!(is_bytevec);
    encapsulate!(is_container);
    encapsulate!(is_string);
    encapsulate!(is_map);
    encapsulate!(is_object);
    encapsulate!(is_str);
    encapsulate!(is_bytes);
    encapsulate!(is_bool);
    encapsulate!(is_null);
    encapsulate!(is_number);
    encapsulate!(is_i8);
    encapsulate!(is_i16);
    encapsulate!(is_i32);
    encapsulate!(is_i64);
    encapsulate!(is_f32);
    encapsulate!(is_f64);
    encapsulate!(is_u8);
    encapsulate!(is_u16);
    encapsulate!(is_u32);
    encapsulate!(is_u64);
    encapsulate!(is_unsigned);
    encapsulate!(is_signed);
    encapsulate!(iter);
    encapsulate!(range);
    encapsulate!(map);
    encapsulate!(props);
    encapsulate!(type_of);
    
}
#[inline]
pub fn builtin()->GcObject {
    let obj = GcObject::from(Object::new(access::ObjAccess::NONE)
    .with_name(property::__builtin())
    .with_doc("zin builtin library. \
    This is included automatically in any zin script and \
    you should never have to import this library unless you have \
    conflicting names and still want to use the builtin lib"));
    let mut obj = Function::from(&obj);
    encapsulation::print(&mut obj);
    encapsulation::assert(&mut obj);
    encapsulation::fassert(&mut obj);
    encapsulation::is_callable(&mut obj);
    encapsulation::is_native(&mut obj);
    encapsulation::is_zin_code(&mut obj);
    encapsulation::is_method(&mut obj);
    encapsulation::is_vec(&mut obj);
    encapsulation::is_anyvec(&mut obj);
    encapsulation::is_bytevec(&mut obj);
    encapsulation::is_container(&mut obj);
    encapsulation::is_string(&mut obj);
    encapsulation::is_map(&mut obj);
    encapsulation::is_object(&mut obj);
    encapsulation::is_str(&mut obj);
    encapsulation::is_bytes(&mut obj);
    encapsulation::is_bool(&mut obj);
    encapsulation::is_null(&mut obj);
    encapsulation::is_number(&mut obj);
    encapsulation::is_i8(&mut obj);
    encapsulation::is_i16(&mut obj);
    encapsulation::is_i32(&mut obj);
    encapsulation::is_i64(&mut obj);
    encapsulation::is_f32(&mut obj);
    encapsulation::is_f64(&mut obj);
    encapsulation::is_u8(&mut obj);
    encapsulation::is_u16(&mut obj);
    encapsulation::is_u32(&mut obj);
    encapsulation::is_u64(&mut obj);
    encapsulation::is_signed(&mut obj);
    encapsulation::is_unsigned(&mut obj);
    encapsulation::iter(&mut obj);
    encapsulation::range(&mut obj);
    encapsulation::map(&mut obj);
    encapsulation::props(&mut obj);
    encapsulation::type_of(&mut obj);
    obj.set_member(property::skip(), glob_implication::skip_fn());
    obj.set_member(property::collect(), glob_implication::collect_fn());
    obj.inner
}
