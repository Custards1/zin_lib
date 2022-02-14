use super::access::ObjAccess;
use super::interpreter::Executor;
use super::obj::*;
use crate::api::{ast::*, executor::*};
use crate::error::*;
use crate::types::ops::{Instruction, InstructionSlice};
use crate::types::{
    number::{Number, Signed, Unsigned},
    value::Value,
    ZBytes, ZString,
};
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstantString {
    ZString(ZString),
    ZBytes(ZBytes),
}
impl From<ZString> for ConstantString {
    fn from(value: ZString) -> ConstantString {
        ConstantString::ZString(value)
    }
}
impl From<ZBytes> for ConstantString {
    fn from(value: ZBytes) -> ConstantString {
        ConstantString::ZBytes(value)
    }
}

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct ConstantLib(Vec<ConstantString>);

impl ConstantLib {
    #[inline]
    pub fn new() -> Self {
        Self(Vec::new())
    }
    #[inline]
    pub fn constant_index<T: Into<ZString>>(&self, c: T) -> Option<(usize,ZString)> {
        self.constant_tindex(c.into())
    }
    #[inline]
    pub fn constant_bindex<T: Into<ZBytes>>(&self, c: T) -> Option<(usize,ZBytes)> {
        self._constant_bindex(c.into())
    }
    #[inline]
    pub fn get_or_create_constant<T: Into<ZString>>(&mut self, c: T) -> (usize, ZString) {
        self.get_or_create_tconstant(c.into())
    }
    #[inline]
    pub fn constant_tindex(&self, c: ZString) -> Option<(usize, ZString)> {
        let mut idx = 0;
        for i in &self.0 {
            match i {
                ConstantString::ZString(a) => {
                    if a == &c {
                        return Some((idx,a.clone()));
                    }
                }
                _ => {}
            }

            idx += 1;
        }
        None
    }
    #[inline]
    fn _constant_bindex(&self, c: ZBytes) -> Option<(usize, ZBytes)> {
        let mut idx = 0;
        for i in &self.0 {
            match i {
                ConstantString::ZBytes(a) => {
                    if a == &c {
                        return Some((idx,a.clone()));
                    }
                }
                _ => {}
            }

            idx += 1;
        }
        None
    }
    #[inline]
    pub fn get_or_create_tconstant(&mut self, c: ZString) -> (usize, ZString) {
        match self.constant_tindex(c.clone()) {
            Some(a) => a,
            _ => {
                self.push(c.clone().into());
                (self.len() - 1, c)
            }
        }
    }
    #[inline]
    pub fn get_or_create_bconstant(&mut self, c: ZBytes) -> (usize, ZBytes) {
        match self._constant_bindex(c.clone()) {
            Some(a) => a,
            _ => {
                self.push(c.clone().into());
                (self.len() - 1, c)
            }
        }
    }
}
impl std::ops::Deref for ConstantLib {
    type Target = Vec<ConstantString>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ConstantLib {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait Compileable {
    fn compile(
        self,
        builtin:&GcObject,
        lib: &mut Function,
        block: &mut crate::types::refrence::BlockChain,
        constants: &mut ConstantLib,
    ) -> Result<Vec<Instruction>>;
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Function {
    pub inner: GcObject,
}

impl std::ops::Deref for Function {
    type Target = GcObject;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for Function {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
pub mod options {
    use crate::types::*;
    #[macro_use]
    use crate::zstring::*;
    static_tstring!(__name);
    static_tstring!(__upvals);
    static_tstring!(__parent);
    static_tstring!(__nargs);
    static_tstring!(__nlocals);
    static_tstring!(import);
}
impl std::convert::From<Object> for Function {
    fn from(obj: Object) -> Self {
        Function {
            inner: GcObject::from(obj),
        }
    }
}

impl std::convert::From<&GcObject> for Function {
    fn from(obj: &GcObject) -> Self {
        Function { inner: obj.clone() }
    }
}
pub trait Argument {
    fn arg(self) -> Vec<Value>;
}

impl<T: Into<Value>> Argument for T {
    #[inline]
    fn arg(self) -> Vec<Value> {
        vec![self.into()]
    }
}

impl<T: Into<Value>> Argument for Vec<T> {
    #[inline]
    fn arg(self) -> Vec<Value> {
        let mut a = Vec::new();
        for i in self {
            a.push(i.into())
        }
        a
    }
}

impl Argument for Option<Value> {
    #[inline]
    fn arg(self) -> Vec<Value> {
        match self {
            Some(a) => a.arg(),
            None => vec![],
        }
    }
}

impl Function {
    #[inline]
    pub fn from_file<T: std::convert::AsRef<std::path::Path>>(
        builtin: &GcObject,
        file: T,
    ) -> Result<Self> {
        Self::parsed(builtin, std::fs::read_to_string(file)?)
    }
    #[inline]
    pub fn with_nargs<F: Into<Number>>(mut self, num: F) -> Self {
        self.set_nargs(num);
        self
    }
    #[inline]
    pub(super) fn consume_compiled_script<T: Compileable>(&mut self,builtin:&GcObject, script: T) -> Result<()> {
        let mut constants = ConstantLib::new();
        let mut blocks = crate::types::refrence::BlockChain::new();
        blocks.push(crate::types::refrence::WizardBlock::new());
        let new = script.compile(builtin,self,&mut blocks, &mut constants)?;
        let nlocal = match blocks.pop() {
            Some(a) => a.local.len(),
            _ => 0,
        };
        self.set_nlocals(nlocal);
        self.set_code(new, false);
        Ok(())
    }
    #[inline]
    pub(super) fn consume_script<T: Parseable>(&mut self, builtin:&GcObject,script: T) -> Result<()> {
        self.consume_compiled_script(builtin,*script.parse_zin()?.unwrap_or(Box::new(Statement::None)))?;
        Ok(())
    }
    #[inline]
    pub(super) fn compiled_script<T: Compileable>(builtin:&GcObject,script: T) -> Result<Self> {
        let mut func = Self::root(ObjAccess::NONE);
        func.consume_compiled_script(builtin,script)?;
        Ok(func)
    }
    #[inline]
    pub(super) fn parsed<T: Parseable>(builtin: &GcObject, script: T) -> Result<Self> {
        Self::compiled_script(builtin, *script.parse_zin()?.unwrap_or(Box::new(Statement::None)))
    }

    
    #[inline]
    pub(crate) fn child_callable(access: ObjAccess, nargs:usize,is_method: bool) -> Function {
        let mut a = Self {
            inner: GcObject::from(Object::map(access) ),
        }
        .with_code(Vec::new(), is_method);
        let _ = a.populate_function_vals();
        a.with_nargs(nargs)
    }
    #[inline]
    pub(crate) fn root(access: ObjAccess) -> Function {
        let mut a = Self {
            inner: GcObject::from(Object::map(access)),
        }
        .with_code(Vec::new(), false);
        let _ = a.populate_function_vals();
        a
    }
    
    #[inline]
    pub(crate) fn native_root(access: ObjAccess, f: NativeFunction) -> Function {
        let mut obj = Object::map(access);
        let mut a = Self {
            inner: GcObject::from(obj),
        }
        .with_code(f, false);
        let _ = a.populate_function_vals();
        a
    }
    #[inline]
    pub(crate) fn native_root_method(f: NativeMethod) -> Function {
        let mut obj = Object::map(ObjAccess::NONE);
        let mut a = Self {
            inner: GcObject::from(obj),
        }
        .with_code(f, true);
        let _ = a.populate_function_vals();
        a
    }
    pub fn envelop<T: Objectable>(obj: T) -> Result<Function> {
        let obj: GcObject = obj.object();
        let mut f: Function = Function { inner: obj };
        f.populate_function_vals()?;
        Ok(f)
    }
    #[inline]
    pub fn with_code<T: Into<ObjFunction>>(mut self, code: T, is_method: bool) -> Function {
        self.set_code(code, is_method);
        self
    }
    #[inline]
    pub fn set_code<T: Into<ObjFunction>>(&mut self, code: T, is_method: bool) -> Option<()> {
        match self.inner.try_borrow_mut() {
            Ok(mut a) => {
                a.is_method = is_method;
                a.vec = Some(InnerContainer::Function(InnerFunction {
                    code: code.into(),
                }));
                Some(())
            }
            _ => None,
        }
    }
    #[inline]
    pub fn set_gccode(&mut self, code: gc::GcCell<Vec<Instruction>>) -> Option<()> {
        match self.inner.try_borrow_mut() {
            Ok(mut a) => {
                a.vec = Some(InnerContainer::Function(InnerFunction {
                    code: ObjFunction::Zin(code),
                }));
                Some(())
            }
            _ => None,
        }
    }

    #[inline]
    pub fn inner_function(&self) -> Option<InnerFunction> {
        match self.inner.try_borrow() {
            Ok(a) => Some(a.function()?.clone()),
            _ => None,
        }
    }
    #[inline]
    pub fn try_code_once(&self) -> InstructionSlice {
        match self.inner_function() {
            Some(a) => match &a.code {
                ObjFunction::Zin(a) => a.clone(),
                _ => {
                    let code: InstructionSlice = InstructionSlice::new(Vec::new());
                    code
                }
            },
            _ => {
                let code: InstructionSlice = InstructionSlice::new(Vec::new());
                code
            }
        }
    }
    #[inline]
    pub fn with_upvals<T: Into<Value>>(mut self, code: T) -> Function {
        self.set_upvals(code);
        self
    }
    #[inline]
    pub fn set_upvals<T: Into<Value>>(&mut self, code: T) {
        self.set_member(options::__upvals(), code);
    }
    #[inline]
    pub fn module(&self)->Result<GcObject> {
        match self.member("__module") {
            Some(a)=>match a.try_into() {
                        Ok(a)=>Ok(a),
                        _=>Err(Error::UnknownVariable("__module".to_string()))
                    },

            _=>Err(Error::UnknownVariable("__module".to_string()))
        }
    }
    #[inline]
    pub fn set_module<T: Into<Value>>(&mut self, module:T) {
        self.set_member("__module", module);
    }
    #[inline]
    pub fn with_module<T: Into<Value>>(mut self, code: T) -> Function {
        self.set_module(code);
        self
    }
    #[inline]
    pub fn upvals(&mut self) -> Result<GcObject> {
        match self.member(options::__upvals()) {
            Some(a) => match &a {
                Value::Object(a) => Ok(a.clone()),
                Value::NULL => {
                    let func = GcObject::from(Object::vec(ObjAccess::NONE));
                    self.set_member(options::__upvals(), func.clone());
                    Ok(func)
                }
                _ => Err(Error::InvalidBorrow(file!(), line!())),
            },
            _ => Err(Error::InvalidBorrow(file!(), line!())),
        }
    }

    #[inline]
    pub fn set_nargs<T: Into<Number>>(&mut self, code: T) {
        self.set_member(options::__nargs(), Value::from(code.into()));
    }
    #[inline]
    pub fn nargs(&self) -> Result<Number> {
        match self.member(options::__nargs()) {
            Some(a) => match &a {
                Value::Num(a) => Ok(*a),
                Value::NULL => Ok(0u32.into()),
                Value::Object(a) => Err(Error::InvalidBorrow(file!(), line!())),
                _ => {
                    // println!("que? {}",a);
                    Err(Error::InvalidBorrow(file!(), line!()))
                }
            },
            _ => Ok(0u32.into()),
        }
    }
    #[inline]
    pub fn with_nlocals<T: Into<Number>>(mut self, code: T) -> Function {
        self.set_nlocals(code);
        self
    }
    #[inline]
    pub fn set_nlocals<T: Into<Number>>(&mut self, code: T) {
        self.set_member(options::__nlocals(), Value::from(code.into()));
    }
    #[inline]
    pub fn nlocals(&self) -> Result<Number> {
        match self.member(options::__nlocals()) {
            Some(a) => match a {
                Value::Num(a) => Ok(a),
                Value::NULL => Ok(0u32.into()),
                _ => Err(Error::InvalidBorrow(file!(), line!())),
            },
            _ => Ok(0u32.into()),
        }
    }

    #[inline]
    pub fn ensure_function_vals(&mut self) -> Result<()> {
        let mut ok = true;
        if self
            .ensure_value::<GcObject, ZString>(options::__upvals(), None, true)
            .is_err()
        {
            ok = false;
        }
        if self
            .ensure_value::<crate::types::Number, ZString>(options::__nargs(), None, true)
            .is_err()
        {
            ok = false;
        }
        if self
            .ensure_value::<crate::types::Number, ZString>(options::__nlocals(), None, true)
            .is_err()
        {
            ok = false;
        }
        match ok {
            true => Ok(()),
            _ => Err(Error::Unconvertable("object".into(), "function".into())),
        }
    }
    #[inline]
    pub fn populate_function_vals(&mut self) -> Result<()> {
        let mut ok = true;
        self.ensure_value::<GcObject, ZString>(options::__upvals(), None, true)?;
        self.ensure_value::<crate::types::Number, ZString>(
            options::__nargs(),
            Some(0u32.into()),
            true,
        )?;
        self.ensure_value::<crate::types::Number, ZString>(
            options::__nlocals(),
            Some(0u32.into()),
            true,
        )
    }

    #[inline]
    pub fn new_upval(&mut self, exec: &Executor, val: Value) -> Result<usize> {
        let upvals = self.upvals()?;
        let mut a = match upvals.try_borrow_mut() {
            Ok(a) => a,
            _ => return Err(Error::InvalidBorrow(file!(), line!())),
        };
        a.push(exec, val);
        Ok(a.vec_len() - 1)
    }
    pub fn nth_upval(&mut self, id: usize) -> Result<Option<Value>> {
        let upvals = self.upvals()?;
        let mut a = match upvals.try_borrow_mut() {
            Ok(a) => a,
            _ => return Err(Error::InvalidBorrow(file!(), line!())),
        };
        Ok(a.index(id))
    }
    pub fn set_nth_upval(&mut self, exec: &Executor, id: usize, val: Value) -> Result<()> {
        let upvals = self.upvals()?;
        let mut a = match upvals.try_borrow_mut() {
            Ok(a) => a,
            _ => return Err(Error::InvalidBorrow(file!(), line!())),
        };
        a.set_index(exec, id, val)
    }
    #[inline(always)]
    pub(super) fn current_code(obj: &GcObject, pc: &mut usize) -> Result<Option<Instruction>> {
        let a = match crate::types::ops::current_code_in_obj(obj, *pc)? {
            Some(a) => a,
            _ => return Ok(None),
        };
        *pc += 1;
        Ok(Some(a))
    }
    #[inline(always)]
    pub(super) fn blank_upval(upvals: Vec<Value>) -> GcObject {
        let mut obj = Object::with_vec(upvals, ObjAccess::NONE);
        GcObject::from(obj)
    }
}
impl Objectable for Function {
    fn object(self) -> GcObject {
        self.inner
    }
}
