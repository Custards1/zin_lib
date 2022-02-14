
use super::{HashMap,Instruction,ZString};
use crate::object::{GcObject,function};
#[derive(Debug)]
pub struct Block {
    pub in_scope:HashMap<ZString,usize>,
    sp:usize
}
impl Block {
    #[inline]
    pub fn new()->Block {
        Block{in_scope:HashMap::new(),sp:0}
    }
    #[inline]
    pub fn index_of<T:Into<ZString>>(&self,a:T)->Option<usize> {
        Some(*self.in_scope.get(&a.into())? )
    }
    #[inline]
    pub fn len(&self)->usize {
        self.sp
    }
    #[inline]
    pub fn try_create_index<T:Into<ZString>>(&mut self,name:T) ->Option<usize> {
       let name = name.into();
        match self.index_of(name.clone()) {
            Some(_)=>None,
            _=>{
                let b = self.sp;
                self.sp+=1;
                self.in_scope.insert(name.into(), b);
                Some(b)
            }
        }
    }
    #[inline]
    pub fn get_or_create_index<T:Into<ZString>>(&mut self,a:T) -> usize {
        let a = a.into();
        match self.index_of(a.clone()) {
            Some(c)=>c,
            _=>{
                let b = self.sp;
                self.sp+=1;
                self.in_scope.insert(a, b);
                b
            }
        }
        
    }
}
#[derive(Debug)]
pub struct UpvalBlock {
    pub in_scope:HashMap<ZString,(usize/*index into upval*/ ,(usize/*originating block*/,usize/*originating id*/,bool/* originates from rther upvals*/))>,
    sp:usize
}
impl UpvalBlock {
    #[inline]
    pub fn new()->UpvalBlock {
        UpvalBlock{in_scope:HashMap::new(),sp:0}
    }
    #[inline]
    pub fn index_of<T:Into<ZString>>(&self,a:T)->Option<usize> {
        let (id,(_,_,_)) = *self.in_scope.get(&a.into())?;
        Some(id)
    }
    #[inline]
    pub fn all_at_index(&self,a:usize) -> Option<(&ZString,&(usize,(usize,usize,bool)))> {
        self.in_scope.get_index(a)
    }
    #[inline]
    pub fn origin_of<T:Into<ZString>>(&self,a:T)->Option<(usize,usize)> {
        let (_,(a,b,_)) = *self.in_scope.get(&a.into())?;
        Some((a,b))
    }
    #[inline]
    pub fn origin_all_of<T:Into<ZString>>(&self,a:T)->Option<(usize,usize)> {
        let (_,(a,b,_)) = *self.in_scope.get(&a.into())?;
        Some((a,b))
    }
    #[inline]
    pub(crate) fn origin_at_index(&self,a:usize)->Option<(usize,usize)> {
        let (_,(_,(borigin,iorgin,_))) = self.in_scope.get_index(a)?;
        Some((*borigin,*iorgin))
    }
    #[inline]
    pub(crate) fn origin_all_at_index(&self,a:usize)->Option<(usize,usize,bool)> {
        let (_,(_,(borigin,iorgin,u))) = self.in_scope.get_index(a)?;
        Some((*borigin,*iorgin,*u))
    }
    #[inline]
    pub fn len(&self)->usize {
        self.sp
    }
    #[inline]
    pub fn try_create_index<T:Into<ZString>>(&mut self,name:T,orig:(usize,usize)) ->Option<usize> {
        let name = name.into();
        match self.index_of(name.clone()) {
            Some(_)=>None,
            _=>{
                let b = self.sp;
                self.sp+=1;
                self.in_scope.insert(name, (b,(match orig.0{
                    0=>0,
                    _=>orig.0-1

                },orig.1,false)));
                Some(b)
            }
        }
    }
    #[inline]
    pub fn try_create_orgin_index<T:Into<ZString>>(&mut self,name:T,orig:(usize,usize)) ->Option<usize> {
        let name = name.into();
        match self.index_of(name.clone()) {
            Some(_)=>None,
            _=>{
                let b = self.sp;
                self.sp+=1;
                self.in_scope.insert(name, (b,(match orig.0{
                    0=>0,
                    _=>orig.0-1

                },orig.1,true)));
                Some(b)
            }
        }
    }
    #[inline]
    pub fn get_or_create_index<T:Into<ZString>>(&mut self,a:T,orig:(usize,usize)) -> usize {
        let a = a.into();
        match self.index_of(a.clone()) {
            Some(c)=>c,
            _=>{
                let b = self.sp;
                self.sp+=1;
                self.in_scope.insert(a, (b,(match orig.0{
                    0=>0,
                    _=>orig.0-1

                },orig.1,false)));
                b
            }
        }
        
    }
}

#[derive(Debug)]
pub struct WizardBlock {
    pub upvals:UpvalBlock,
    pub local:Block
}
impl WizardBlock {
    #[inline]
    pub fn new() -> WizardBlock {
        Self{
            upvals:UpvalBlock::new(),
            local:Block::new()
        }
    }
    #[inline]
    pub fn index_of<T:Into<ZString>>(&self,a:T)->Option<usize> {
        Some(self.local.index_of(a)?)
    }

    #[inline]
    pub fn len(&self)->usize {
        self.local.len()
    }
    #[inline]
    pub fn try_create_index<T:Into<ZString>>(&mut self,name:T) ->Option<usize> {
        self.local.try_create_index(name)
    }
    #[inline]
    pub fn get_or_create_index<T:Into<ZString>>(&mut self,a:T) -> (bool/*if upval*/ ,usize) {
        let a = a.into();
        match self.upvals.index_of(a.clone()) {
            Some(a)=>(true,a),
            _=>(false,self.local.get_or_create_index(a))
        }
    }
    #[inline]
    pub fn upval_index_of<T:Into<ZString>>(&self,a:T)->Option<usize> {
        Some(self.upvals.index_of(a)?)
    }
    #[inline]
    pub fn upval_all_at_index(&self,a:usize)->Option<(&ZString,&(usize,(usize,usize,bool)))> {
        Some(self.upvals.all_at_index(a)?)
    }
    #[inline]
    pub fn upval_origin_of<T:Into<ZString>>(&self,a:T)->Option<(usize,usize)> {
        Some(self.upvals.origin_of(a)?)
    }
    #[inline]
    pub fn upval_len(&self)->usize {
        self.upvals.len()
    }
    #[inline]
    pub fn upval_try_create_index<T:Into<ZString>>(&mut self,name:T,orig:(usize,usize)) ->Option<usize> {
        self.upvals.try_create_index(name,orig)
    }
    #[inline]
    pub fn upval_get_or_create_index<T:Into<ZString>>(&mut self,a:T,orig:(usize,usize)) -> usize {
        self.upvals.get_or_create_index(a,orig)       
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct BlockChain(Vec<WizardBlock>);
impl std::ops::Deref for BlockChain {
    type Target = Vec<WizardBlock>;
    #[inline]
    fn deref(&self)->&Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for BlockChain {
    #[inline]
    fn deref_mut(&mut self)->&mut Self::Target {
        &mut self.0
    }
}

impl BlockChain {
    #[inline]
    pub fn new()->BlockChain {
        BlockChain(Vec::new())
    }
    #[inline]
    pub fn index_of<T:Into<ZString>>(&self,a:T)->Option<(bool,usize/*Block*/,usize/*Index*/)> {
        let mut block = 0;
        let a = a.into();
        for i in self.iter().rev(){
            match i.index_of(a.clone()) {
                Some(b)=>return Some((false,block,b)),
                _=> match i.upval_origin_of(a.clone()) {
                    Some((b,a))=>return Some((true,b,a)),
                    _=>{block+=1;}
                }
            }
        } 
        None
    }
    #[inline]
    pub fn upval_index_of<T:Into<ZString>>(&self,a:T)->Option<(usize/*Block*/,usize/*Index*/)> {
        let mut block = 0;
        let a = a.into();
        for i in self.iter().rev(){
            match i.index_of(a.clone()) {
                Some(b)=>return Some((block,b)),
                _=>{block+=1;}
            }
        } 
        None
    }
    #[inline]
    pub fn variable<T:Into<ZString>>(&mut self,b:T)->Option<Instruction>{
        self._variable(b)
    } 
    #[inline]
    fn _variable<T:Into<ZString>>(&mut self,b:T)->Option<Instruction>{
        let len = self.len();
        let b = b.into();
        if let Some((h,a,c)) = self.index_of(b.clone()) {
            return if a >= 1 {
               
                Some(Instruction::Ldup(self[len-1].upvals.get_or_create_index(b.clone(),(a,c))))
            } else {
                Some(match h {
                    false=>Instruction::Ldl(0,c),
                    true=> Instruction::Ldup(self[len-1].upvals.get_or_create_index(b.clone(),(a,c)))
                })
            };
        }
        if let Some(last) = self.last() {
            if let Some(a) = last.upvals.index_of(b) {
                return Some(Instruction::Ldup(a))
            }
        }
       None
       
    }
    #[inline]
    pub fn load_variable<T:Into<ZString>>(&mut self,b:T)->Option<Instruction>{
        Some(self.variable(b)?.into_loaded())
    }
    #[inline]
    pub fn compiled_var<T:Into<ZString>>(&mut self,b:T,builtin:&GcObject,lib:&mut function::Function,constants:&mut function::ConstantLib)->Option<Vec<Instruction>> {
        let b =  b.into();
        let c:Option<Instruction> = self.variable(b.clone());
        match c {
            Some(inst) =>{
                Some(vec![inst])
            },
            _=> {
                let mut val:Option<Vec<Instruction>> = None;
                let (_,xstring) = constants.get_or_create_constant(b);
                let string:super::Value = xstring.clone().into();
                let mut count = 0;
                lib.traverse(|x|->bool{
                    count+=1;
                    debugln!("traversing");
                    match x.member_unchecked(string.clone()) {
                        Some(_)=>{
                            debugln!("is some");
                            val = if count > 1 {
                                Some(vec![Instruction::Cld(x.clone().into()),Instruction::Ldm(xstring.clone())])
                            }  else {
                                Some(vec![Instruction::Cld(lib.clone().into()),Instruction::Ldm(xstring.clone())])
                            };
                            
                            true
                        },
                        None=> {debugln!("is none");false}
                    }
                });
                
                match val{
                    Some(a)=>Some(a),
                    _=>{
                        match builtin.member_unchecked(string.clone()) {
                            Some(_)=>{
                                Some(vec![Instruction::Builtin(xstring)])
                            },
                            None=>None
                        } 
                    }
                }
            }

        }
    }
    #[inline]
    pub fn load_compiled_var<T:Into<ZString>>(&mut self,b:T,builtin:&GcObject,lib:&mut function::Function,constants:&mut function::ConstantLib)->Option<Vec<Instruction>> {
        let mut temp = self.compiled_var(b,builtin,lib,constants)?;
        let inst =temp.pop().unwrap().into_loaded();
        temp.push(inst);
        Some(temp)
        
    }
    #[inline]
    pub fn get_or_create_index<T:Into<ZString>>(&mut self,a:T) -> (bool/* upval*/,usize/*Block*/,usize/*Index*/) {
        let a = a.into();
        match self.index_of(a.clone()) {
            Some((u,c,d))=>(u,c,d),
            _=>{
                let idx = match self.len() {
                    0=>{
                        self.push(WizardBlock::new());
                        self[0].get_or_create_index(a)
                    }
                    _=>{
                        let idx =self.len()-1; 
                        self[idx].get_or_create_index(a)
                    }
                };
                (idx.0,0,idx.1)
            }
        }
        
    }
    

}
