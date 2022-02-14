use float_eq::*;
#[derive(Copy,Clone,Debug,PartialOrd)]
pub enum Signed {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
} 
impl Ord for Signed {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Signed::I8(a)=>  a.cmp(&other.i8()),
            Signed::I16(a)=> a.cmp(&other.i16()),
            Signed::I32(a)=> a.cmp(&other.i32()),
            Signed::I64(a)=> a.cmp(&other.i64()),
            Signed::F32(a)=> {
                let x= other.f32();
                if float_eq!(*a, x, abs <= 0.000_1) {
                    std::cmp::Ordering::Equal
                } else if *a < x {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            },
            Signed::F64(a)=> {
                let x= other.f64();
                if float_eq!(*a, x, abs <= 0.000_1) {
                    std::cmp::Ordering::Equal
                } else if *a < x {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            },
        }
    }
}
impl Eq for Signed {}

impl std::hash::Hash for Signed {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Signed::I8(a)=>  a.hash(state),
            Signed::I16(a)=> a.hash(state),
            Signed::I32(a)=> a.hash(state),
            Signed::I64(a)=> a.hash(state),
            Signed::F32(a)=> a.to_bits().hash(state),
            Signed::F64(a)=> a.to_bits().hash(state),
        }
    }
}

impl PartialEq for Signed {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Signed::I8(a)=> *a == other.i8(),
            Signed::I16(a)=> *a == other.i16(),
            Signed::I32(a)=> *a == other.i32(),
            Signed::I64(a)=> *a == other.i64(),
            Signed::F32(a)=> {
                let x= other.f32();
                float_eq!(*a, x, abs <= 0.000_1)
            },
            Signed::F64(a)=>  {
                let x= other.f64();
                float_eq!(*a, x, abs <= 0.000_1)
            },
            
        }
    }
}
impl std::fmt::Display for Signed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Signed::I8(a)=>write!(f,"{}",a),
            Signed::I16(a)=>write!(f,"{}",a),
            Signed::I32(a)=>write!(f,"{}",a),
            Signed::I64(a)=>write!(f,"{}",a),
            Signed::F32(a)=>write!(f,"{}",a),
            Signed::F64(a)=>write!(f,"{}",a),
            
        }
        
    }
}
#[derive(Copy,Clone,Debug,PartialEq,Eq,Hash,PartialOrd)]
pub enum Unsigned {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64)
}
impl std::fmt::Display for Unsigned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unsigned::U8(a)=>write!(f,"{}",a),
            Unsigned::U16(a)=>write!(f,"{}",a),
            Unsigned::U32(a)=>write!(f,"{}",a),
            Unsigned::U64(a)=>write!(f,"{}",a),
            
        }
        
    }
}
impl Ord for Unsigned {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Unsigned::U8(a)=>  a.cmp(&other.u8()),
            Unsigned::U16(a)=> a.cmp(&other.u16()),
            Unsigned::U32(a)=> a.cmp(&other.u32()),
            Unsigned::U64(a)=> a.cmp(&other.u64()),
            
        }
    }
}
#[derive(Debug,Copy,Clone,Hash,Eq,PartialOrd)]
pub enum Number {
    SNum(Signed),
    UNum(Unsigned)
}
impl Ord for Number {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Number::SNum(a)=>a.cmp(&other.signed()),
            Number::UNum(a)=>a.cmp(&other.unsigned()),
            
            
        }
    }
}
impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number:: SNum(a)=>write!(f,"{}",a),
            Number::UNum(a)=>write!(f,"{}",a),
        }
        
    }
}

impl From<i8> for Signed {
    #[inline]
    fn from(i:i8)->Signed {
        Signed::I8(i)
    }
}
impl From<i16> for Signed {
    #[inline]
    fn from(i:i16)->Signed {
        Signed::I16(i)
    }
}

impl From<i32> for Signed {
    #[inline]
    fn from(i:i32)->Signed {
        Signed::I32(i)
    }
}
impl From<i64> for Signed {
    #[inline]
    fn from(i:i64)->Signed {
        Signed::I64(i)
    }
}
impl From<f32> for Signed {
    #[inline]
    fn from(i:f32)->Signed {
        Signed::F32(i)
    }
}
impl From<f64> for Signed {
    #[inline]
    fn from(i:f64)->Signed {
        Signed::F64(i)
    }
}



impl From<u8> for Unsigned {
    #[inline]
    fn from(i:u8)->Unsigned {
        Unsigned::U8(i)
    }
}
impl From<u16> for Unsigned {
    #[inline]
    fn from(i:u16)->Unsigned {
        Unsigned::U16(i)
    }
}

impl From<u32> for Unsigned {
    #[inline]
    fn from(i:u32)->Unsigned {
        Unsigned::U32(i)
    }
}
impl From<u64> for Unsigned {
    #[inline]
    fn from(i:u64)->Unsigned {
        Unsigned::U64(i)
    }
}

impl From<Signed> for Unsigned {
    #[inline]
    fn from(a:Signed)->Unsigned {
        match a {
            Signed::I8(i)  =>Unsigned::U8(i as u8),
            Signed::I16(i) =>Unsigned::U16(i as u16),
            Signed::I32(i) =>Unsigned::U32(i as u32),
            Signed::I64(i) =>Unsigned::U64(i as u64),
            Signed::F64(i) =>Unsigned::U64(i as u64),
            Signed::F32(i) =>Unsigned::U32(i as u32)
        }
    }
}

impl std::ops::Add for Signed {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        match self {
            Signed::I8(i)  =>Signed::I8(i+other.i8()),
            Signed::I16(i) =>Signed::I16(i+other.i16()),
            Signed::I32(i) =>Signed::I32(i+other.i32()),
            Signed::I64(i) =>Signed::I64(i+other.i64()),
            Signed::F64(i) =>Signed::F64(i+other.f64()),
            Signed::F32(i) =>Signed::F32(i+other.f32())
        }
    }
}
impl std::ops::Sub for Signed {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        match self {
            Signed::I8(i)  =>Signed::I8(i-other.i8()),
            Signed::I16(i) =>Signed::I16(i-other.i16()),
            Signed::I32(i) =>Signed::I32(i-other.i32()),
            Signed::I64(i) =>Signed::I64(i-other.i64()),
            Signed::F64(i) =>Signed::F64(i-other.f64()),
            Signed::F32(i) =>Signed::F32(i-other.f32())
        }
    }
}
impl std::ops::Shl for Signed {
    type Output = Self;
    #[inline]
    fn shl(self, other: Self) -> Self {
        match self {
            Signed::I8(i)  =>Signed::I8( i<<other.i8()),
            Signed::I16(i) =>Signed::I16(i<<other.i16()),
            Signed::I32(i) =>Signed::I32(i<<other.i32()),
            Signed::I64(i) =>Signed::I64(i<<other.i64()),
            Signed::F64(i) =>Signed::F64(i),
            Signed::F32(i) =>Signed::F32(i)
        }
    }
}
impl std::ops::Shr for Signed {
    type Output = Self;
    #[inline]
    fn shr(self, other: Self) -> Self {
        match self {
            Signed::I8(i)  =>Signed::I8( i>>other.i8()),
            Signed::I16(i) =>Signed::I16(i>>other.i16()),
            Signed::I32(i) =>Signed::I32(i>>other.i32()),
            Signed::I64(i) =>Signed::I64(i>>other.i64()),
            Signed::F64(i) =>Signed::F64(i),
            Signed::F32(i) =>Signed::F32(i)
        }
    }
}
impl std::ops::Mul for Signed {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        match self {
            Signed::I8(i)  =>Signed::I8(i*other.i8()),
            Signed::I16(i) =>Signed::I16(i*other.i16()),
            Signed::I32(i) =>Signed::I32(i*other.i32()),
            Signed::I64(i) =>Signed::I64(i*other.i64()),
            Signed::F64(i) =>Signed::F64(i*other.f64()),
            Signed::F32(i) =>Signed::F32(i*other.f32())
        }
    }
}
impl std::ops::Div for Signed {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        self.try_div(other).unwrap_or(Signed::I8(0).convert(other))
    }
}
const MAX_I8 :Signed = Signed::I64(std::i8::MAX as i64);
const MAX_I16 :Signed = Signed::I64(std::i16::MAX as i64);
const MAX_I32 :Signed = Signed::I64(std::i32::MAX as i64);
const MAX_I64 :Signed = Signed::I64(std::i64::MAX);
const MAX_F32 :Signed = Signed::F64(std::f32::MAX as f64);
const MAX_F64 :Signed = Signed::F64(std::f64::MAX as f64);

impl Signed{
    #[inline]
    pub fn is_zero(&self)->bool {
        match self {
            Signed::I8(i)  =>*i == 0,
            Signed::I16(i) =>*i == 0,
            Signed::I32(i) =>*i == 0,
            Signed::I64(i) =>*i == 0,
            Signed::F32(i) =>*i == 0f32,
            Signed::F64(i) =>*i == 0f64,
        }
    }
    #[inline]
    pub const fn get_type(&self)->&'static str {
        match self {
            Signed::I8(i)  =>"i8",
            Signed::I16(i) =>"i16",
            Signed::I32(i) =>"i32",
            Signed::I64(i) =>"i64",
            Signed::F32(i) =>"f32",
            Signed::F64(i) =>"f64",
        }
    }
    #[inline]
    pub fn is_one(&self)->bool {
        match self {
            Signed::I8(i)  =>*i == 1,
            Signed::I16(i) =>*i == 1,
            Signed::I32(i) =>*i == 1,
            Signed::I64(i) =>*i == 1,
            Signed::F32(i) =>*i == 1f32,
            Signed::F64(i) =>*i == 1f64,
        }
    }
    #[inline] 
    pub fn rectify(&self)->Signed {
        if self < &MAX_I8 {
            self.i8().into()
        } else if self < &MAX_I16 {
            self.i16().into()
        } else if self < &MAX_I32 {
            self.i32().into()
        } else {
            self.clone()
        }
        
    }
    #[inline] 
    pub fn compare_bytes(&self, b: &[u8])->bool {
        match self.rectify() {
            Signed::I8(a)=> a.to_ne_bytes() == b,
            Signed::I16(a)=>a.to_ne_bytes() == b,
            Signed::I32(a)=>a.to_ne_bytes() == b,
            Signed::I64(a)=>a.to_ne_bytes() == b,
            Signed::F32(a)=>a.to_ne_bytes() == b,
            Signed::F64(a)=>a.to_ne_bytes() == b,
        }
    }
    #[inline]
    pub fn size(&self)->u8{
        match self {
            Signed::I8(_)  =>8u8,
            Signed::I16(_) =>16u8,
            Signed::I32(_) =>32u8,
            Signed::I64(_) =>64u8,
            Signed::F32(_) =>32u8,
            Signed::F64(_) =>64u8
        }
    }
    #[inline]
    pub fn as_bool(&self)->bool {
        match self {
            Signed::I8(a)  =>*a >0,
            Signed::I16(a) =>*a >0,
            Signed::I32(a) =>*a >0,
            Signed::I64(a) =>*a >0,
            Signed::F32(a) =>*a >0f32,
            Signed::F64(a) =>*a >0f64
        }
    }
    #[inline]
    pub fn i8(self)->i8 {
        match self {
            Signed::I8(a)  => a,
            Signed::I16(b) => b as i8,
            Signed::I32(b) => b as i8,
            Signed::I64(b) => b as i8,
            Signed::F32(b) => b as i8,
            Signed::F64(b) => b as i8,
        }
    }
    #[inline]
    pub fn i16(self)->i16 {
        match self {
            Signed::I8(a)  => a as i16,
            Signed::I16(b) => b,
            Signed::I32(b) => b as i16,
            Signed::I64(b) => b as i16,
            Signed::F32(b) => b as i16,
            Signed::F64(b) => b as i16,
            
        }
    }
    #[inline]
    pub fn i32(self)->i32 {
        match self {
            Signed::I8(a)  => a as i32,
            Signed::I16(b) => b as i32,
            Signed::I32(b) => b,
            Signed::I64(b) => b as i32,
            Signed::F32(b) => b as i32,
            Signed::F64(b) => b as i32,
            
        }
    }
    #[inline]
    pub fn i64(self)->i64 {
        match self {
            Signed::I8(a)  => a as i64,
            Signed::I16(b) => b as i64,
            Signed::I32(b) => b as i64,
            Signed::I64(b) => b,
            Signed::F32(b) => b as i64,
            Signed::F64(b) => b as i64
        }
    }
    #[inline]
    pub fn f64(self)->f64 {
        match self {
            Signed::I8(a)  => a as f64,
            Signed::I16(b) => b as f64,
            Signed::I32(b) => b as f64,
            Signed::I64(b) => b as f64,
            Signed::F32(b) => b as f64,
            Signed::F64(b) => b
        }
    }
    #[inline]
    pub fn f32(self)->f32 {
        match self {
            Signed::I8(a)  => a as f32,
            Signed::I16(b) => b as f32,
            Signed::I32(b) => b as f32,
            Signed::I64(b) => b as f32,
            Signed::F32(b) => b,
            Signed::F64(b) => b as f32
        }
    }
    
    #[inline]
    pub fn isize(self)->isize {
        match self {
            Signed::I8(a)  => a as isize,
            Signed::I16(b) => b as isize,
            Signed::I32(b) => b as isize,
            Signed::I64(b) => b as isize,
            Signed::F32(b) => b as isize,
            Signed::F64(b) => b as isize
        }
    }
    
    #[inline]
    pub fn try_div(self, other:Self)->Option<Signed> {
        if other.is_zero() {
            return None;
        }
        Some(match self {
            Signed::I8(a)  => Signed::I8(a /other.i8()),
            Signed::I16(a) => Signed::I16(a /other.i16()),
            Signed::I32(a) => Signed::I32(a /other.i32()),
            Signed::I64(a) => Signed::I64(a /other.i64()),
            Signed::F32(a) => Signed::F32(a /other.f32()),
            Signed::F64(a) => Signed::F64(a /other.f64()),
        })
    }

    #[inline]
    pub fn convert(&self,b:Signed)->Signed {
        if self.size()>=b.size() {
            self.clone()
        } else {
            match b {
                Signed::I8(_)  =>  Signed::I8(self.i8() ),
                Signed::I16(_) =>  Signed::I16(self.i16()),
                Signed::I32(_) =>  Signed::I32(self.i32()),
                Signed::I64(_) =>  Signed::I64(self.i64()),
                Signed::F32(_) =>  Signed::F32(self.f32()),
                Signed::F64(_) =>  Signed::F64(self.f64()),
                
            }
        }
    }
    #[inline]
    pub fn exp(&self,other:Unsigned) ->Signed {
        match self {
            Signed::I8(i)  =>Signed::I8( i.pow(other.u32())),
            Signed::I16(i) =>Signed::I16(i.pow(other.u32())),
            Signed::I32(i) =>Signed::I32(i.pow(other.u32())),
            Signed::I64(i) =>Signed::I64(i.pow(other.u32())),
            Signed::F64(i) =>Signed::F64(i.powf(other.u64() as f64)),
            Signed::F32(i) =>Signed::F32(i.powf(other.u32() as f32))
        }
    }
}

const MAX_U8 :Unsigned =  Unsigned::U64(std::u8::MAX as u64);
const MAX_U16:Unsigned = Unsigned::U64(std::u16::MAX as u64);
const MAX_U32:Unsigned = Unsigned::U64(std::u32::MAX as u64);
const MAX_U64:Unsigned = Unsigned::U64(std::u64::MAX as u64);



impl Unsigned{
    #[inline]
    pub const fn is_zero(&self)->bool {
        match self {
            Unsigned::U8(i)  =>*i == 0,
            Unsigned::U16(i) =>*i == 0,
            Unsigned::U32(i) =>*i == 0,
            Unsigned::U64(i) =>*i == 0,
        }
    }
    #[inline]
    pub const fn is_one(&self)->bool {
        match self {
            Unsigned::U8(i)  =>*i == 1,
            Unsigned::U16(i) =>*i == 1,
            Unsigned::U32(i) =>*i == 1,
            Unsigned::U64(i) =>*i == 1,
        }
    }
    #[inline]
    pub const fn get_type(&self) ->&'static str {
        match self{
            Unsigned::U8(i)  =>"u8",
            Unsigned::U16(i) =>"u16",
            Unsigned::U32(i) =>"u32",
            Unsigned::U64(i) =>"u64",
        }
    }
    #[inline]
    pub const fn size(&self)->u8{
        match self {
            Unsigned::U8(_)  =>8u8,
            Unsigned::U16(_) =>16u8,
            Unsigned::U32(_) =>32u8,
            Unsigned::U64(_) =>64u8,
        }
    }
    #[inline] 
    pub fn rectify(&self)->Unsigned {
        if self < &MAX_U8 {
            self.u8().into()
        } else if self < &MAX_U16 {
            self.u16().into()
        } else if self < &MAX_U32 {
            self.u32().into()
        } else {
            self.clone()
        }
        
    }
    #[inline] 
    pub fn compare_bytes(&self, b: &[u8])->bool {
        match self.rectify() {
            Unsigned::U8(a)=>a.to_ne_bytes() == b,
            Unsigned::U16(a)=>a.to_ne_bytes() == b,
            Unsigned::U32(a)=>a.to_ne_bytes() == b,
            Unsigned::U64(a)=>a.to_ne_bytes() == b,
        }
    }
    #[inline]
    pub fn as_bool(&self)->bool {
        match self {
            Unsigned::U8(a)  =>*a >0,
            Unsigned::U16(a) =>*a >0,
            Unsigned::U32(a) =>*a >0,
            Unsigned::U64(a) =>*a >0
        }
    }
    #[inline]
    pub fn u8(self)->u8 {
        match self {
            Unsigned::U8(a)  => a,
            Unsigned::U16(b) => b as u8,
            Unsigned::U32(b) => b as u8,
            Unsigned::U64(b) => b as u8
        }
    }
    #[inline]
    pub fn u16(self)->u16 {
        match self {
            Unsigned::U8(a)  => a as u16,
            Unsigned::U16(b) => b,
            Unsigned::U32(b) => b as u16,
            Unsigned::U64(b) => b as u16
        }
    }
    #[inline]
    pub fn u32(self)->u32 {
        match self {
            Unsigned::U8(a)  => a as u32,
            Unsigned::U16(b) => b as u32,
            Unsigned::U32(b) => b,
            Unsigned::U64(b) => b as u32
        }
    }
    #[inline]
    pub fn u64(self)->u64 {
        match self {
            Unsigned::U8(a)  => a as u64,
            Unsigned::U16(b) => b as u64,
            Unsigned::U32(b) => b as u64,
            Unsigned::U64(b) => b
        }
    }
    #[inline]
    pub fn usize(self)->usize {
        match self {
            Unsigned::U8(a)  => a as usize,
            Unsigned::U16(b) => b as usize,
            Unsigned::U32(b) => b as usize,
            Unsigned::U64(b) => b as usize, 
        }
    }
    pub fn try_div(self, other:Self)->Option<Unsigned> {
        if other.is_zero() {
            return None;
        }
        Some(match self {
            Unsigned::U8(a)  => Unsigned::U8(a /other.u8()),
            Unsigned::U16(a) => Unsigned::U16(a /other.u16()),
            Unsigned::U32(a) => Unsigned::U32(a /other.u32()),
            Unsigned::U64(a) => Unsigned::U64(a /other.u64()),
        })
    }
    #[inline]
    pub fn convert(&self,b:Unsigned)->Unsigned {
        if self.size()>=b.size() {
            self.clone()
        } else {
            match b {
                Unsigned::U8(_)  => Unsigned::U8(self.u8()),
                Unsigned::U16(_) => Unsigned::U16(self.u16()),
                Unsigned::U32(_) => Unsigned::U32(self.u32()),
                Unsigned::U64(_) => Unsigned::U64(self.u64()),
                
            }
        }
    }
    
}

impl std::ops::Add for Unsigned {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        match self {
            Unsigned::U8(a)  => Unsigned::U8( a +other.u8()),
            Unsigned::U16(a) => Unsigned::U16(a +other.u16()),
            Unsigned::U32(a) => Unsigned::U32(a +other.u32()),
            Unsigned::U64(a) => Unsigned::U64(a +other.u64()),
        }
    }
}
impl std::ops::Sub for Unsigned {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        match self {
            Unsigned::U8(a)  => Unsigned::U8( a -other.u8()),
            Unsigned::U16(a) => Unsigned::U16(a -other.u16()),
            Unsigned::U32(a) => Unsigned::U32(a -other.u32()),
            Unsigned::U64(a) => Unsigned::U64(a -other.u64()),
        }
    }
}
impl std::ops::Mul for Unsigned {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        match self {
            Unsigned::U8(a)  => Unsigned::U8( a *other.u8()),
            Unsigned::U16(a) => Unsigned::U16(a *other.u16()),
            Unsigned::U32(a) => Unsigned::U32(a *other.u32()),
            Unsigned::U64(a) => Unsigned::U64(a *other.u64()),
        }
    }
}
impl std::ops::Shl for Unsigned {
    type Output = Self;
    #[inline]
    fn shl(self, other: Self) -> Self {
        match self {
            Unsigned::U8(a)  => Unsigned::U8( a << other.u8()),
            Unsigned::U16(a) => Unsigned::U16(a << other.u16()),
            Unsigned::U32(a) => Unsigned::U32(a << other.u32()),
            Unsigned::U64(a) => Unsigned::U64(a << other.u64()),
        }
    }
}
impl std::ops::Shr for Unsigned {
    type Output = Self;
    #[inline]
    fn shr(self, other: Self) -> Self {
        match self {
            Unsigned::U8(a)  => Unsigned::U8( a >> other.u8()),
            Unsigned::U16(a) => Unsigned::U16(a >> other.u16()),
            Unsigned::U32(a) => Unsigned::U32(a >> other.u32()),
            Unsigned::U64(a) => Unsigned::U64(a >> other.u64()),
        }
    }
}
impl std::ops::Div for Unsigned {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        self.try_div(other).unwrap_or(Unsigned::U8(0).convert(other))
    }
}


impl From<Unsigned> for Signed {
    #[inline]
    fn from(a:Unsigned)->Signed {
        match a {
            Unsigned::U8(i)  =>Signed::I8(i as i8),
            Unsigned::U16(i) =>Signed::I16(i as i16),
            Unsigned::U32(i) =>Signed::I32(i as i32),
            Unsigned::U64(i) =>Signed::I64(i as i64)
        }
    }
}

impl From<Signed> for Number {
    #[inline]
    fn from(a:Signed)->Number {
        Number::SNum(a)
    }
}
impl From<Unsigned> for Number {
    #[inline]
    fn from(a:Unsigned)->Number {
        Number::UNum(a)
    }
}
impl PartialEq for Number {
    fn eq(&self, other: &Number) -> bool {
        match self {
            Number::SNum(a)=>a.eq(&other.signed()),
            Number::UNum(a)=>a.eq(&other.unsigned()),
        }
    }
}
impl Number {
    #[inline]
    pub fn from_number(a:Number) -> Number {
        a
    }
    #[inline]
    pub fn u8(self)->u8{
        match self {
            Number::SNum(a)=> {
                Unsigned::from(a).u8()
            }
            Number::UNum(a)=> {
                a.u8()
            }
        }
    }
    #[inline] 
    pub fn compare_bytes(&self, b: &[u8])->bool {
        match self {
            Number::SNum(a)=> {
                a.compare_bytes(b)
            }
            Number::UNum(a)=> {
                a.compare_bytes(b)
            }
        }
    }
    #[inline]
    pub fn u16(self)->u16{
        match self {
            Number::SNum(a)=> {
                Unsigned::from(a).u16()
            }
            Number::UNum(a)=> {
                a.u16()
            }
        }
    }
    #[inline]
    pub fn u32(self)->u32{
        match self {
            Number::SNum(a)=> {
                Unsigned::from(a).u32()
            }
            Number::UNum(a)=> {
                a.u32()
            }
        }
    }
    #[inline]
    pub fn u64(self)->u64{
        match self {
            Number::SNum(a)=> {
                Unsigned::from(a).u64()
            }
            Number::UNum(a)=> {
                a.u64()
            }
        }
    }
    #[inline]
    pub fn usize(self)->usize {
        match self {
            Number::SNum(a)=> {
                Unsigned::from(a).usize()
            }
            Number::UNum(a)=> {
                a.usize()
            }
        }
    }
    #[inline]
    pub fn i8(self)->i8{
        match self {
            Number::UNum(a)=> {
                Signed::from(a).i8()
            }
            Number::SNum(a)=> {
                a.i8()
            }
        }
    }
    #[inline]
    pub fn i16(self)->i16{
        match self {
            Number::UNum(a)=> {
                Signed::from(a).i16()
            }
            Number::SNum(a)=> {
                a.i16()
            }
        }
    }
    #[inline]
    pub fn i32(self)->i32{
        match self {
            Number::UNum(a)=> {
                Signed::from(a).i32()
            }
            Number::SNum(a)=> {
                a.i32()
            }
        }
    }
    #[inline]
    pub fn i64(self)->i64{
        match self {
            Number::UNum(a)=> {
                Signed::from(a).i64()
            }
            Number::SNum(a)=> {
                a.i64()
            }
        }
    }
    #[inline]
    pub fn f32(self)->f32{
        match self {
            Number::UNum(a)=> {
                Signed::from(a).f32()
            }
            Number::SNum(a)=> {
                a.f32()
            }
        }
    }
    #[inline]
    pub fn f64(self)->f64{
        match self {
            Number::UNum(a)=> {
                Signed::from(a).f64()
            }
            Number::SNum(a)=> {
                a.f64()
            }
        }
    }
    pub const fn get_type(&self) -> &'static str {
        match self {
            Number::UNum(a)=>a.get_type(),
            Number::SNum(a)=>a.get_type()
        }
    }
    #[inline]
    pub fn isize(self)->isize{
        match self {
            Number::UNum(a)=> {
                Signed::from(a).isize()
            }
            Number::SNum(a)=> {
                a.isize()
            }
        }
    }
    pub fn is_one(&self)->bool {
        match self {
            Number::SNum(a)=>a.is_one(),
            Number::UNum(a)=>a.is_one(),
        }
    }
    pub fn is_zero(&self)->bool {
        match self {
            Number::UNum(a)=>a.is_zero(),
            Number::SNum(a)=>a.is_zero(),
        }
    }
    #[inline]
    pub fn signed(&self)->Signed {
        match self {
            Number::SNum(a)=>*a,
            Number::UNum(a)=>Signed::from(*a)
        }
    }
    #[inline]
    pub fn try_div(&self,b:Number)->Option<Number> {
        match self {
            Number::SNum(a)=>a.try_div(b.signed()).and_then(|x|{Some(x.into())}),
            Number::UNum(a)=>a.try_div(b.unsigned()).and_then(|x|{Some(x.into())})
        }
    }
    #[inline]
    pub fn exp(&self,other:Number)->Number {
        Number::SNum(self.signed().exp(other.unsigned()))
    }
    #[inline]
    pub fn is_signed(&self)->bool {
        match self {
            Number::SNum(_)=>true,
            _=>false
        }
    }
    #[inline]
    pub fn unsigned(&self)->Unsigned {
        match self {
            Number::SNum(a)=>Unsigned::from(*a),
            Number::UNum(a)=>*a
        }
    }
    #[inline]
    pub fn mimic(&self, p:Number) ->Number {
        match p {
            Number::SNum(_)=>Number::from(self.signed()),
            _=>Number::from(self.unsigned())
        }
    }
    #[inline]
    pub fn as_bool(&self)->bool {
        match self {
            Number::SNum(a)=>a.as_bool(),
            Number::UNum(a)=>a.as_bool()
        }
    }
}
impl std::ops::Mul for Number {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        let a = self.is_signed();
        let b = other.is_signed();
        if a == b {
            match self {
                Number::SNum(s)=>Number::SNum(s*other.signed()),
                Number::UNum(s)=>Number::UNum(s*other.unsigned()),
            }
        } else if a && !b {
            match self {
                Number::SNum(s)=>Number::SNum(s*other.signed()),
                Number::UNum(s)=>Number::UNum(s*other.unsigned()),//never executed
            }
        } else {
            match self {
                Number::SNum(s)=>Number::SNum(s*other.signed()),//never executed
                Number::UNum(s)=>Number::UNum(Unsigned::from(s)*other.unsigned()),
            }
        }
    }
}
impl std::ops::Shl for Number {
    type Output = Self;
    #[inline]
    fn shl(self, other: Self) -> Self {
        let a = self.is_signed();
        let b = other.is_signed();
        if a == b {
            match self {
                Number::SNum(s)=>Number::SNum(s<<other.signed()),
                Number::UNum(s)=>Number::UNum(s<<other.unsigned()),
            }
        } else if a && !b {
            match self {
                Number::SNum(s)=>Number::SNum(s<<other.signed()),
                Number::UNum(s)=>Number::UNum(s<<other.unsigned()),//never executed
            }
        } else {
            match self {
                Number::SNum(s)=>Number::SNum(s<<other.signed()),//never executed
                Number::UNum(s)=>Number::UNum(Unsigned::from(s)<<other.unsigned()),
            }
        }
    }
}
impl std::ops::Shr for Number {
    type Output = Self;
    #[inline]
    fn shr(self, other: Self) -> Self {
        let a = self.is_signed();
        let b = other.is_signed();
        if a == b {
            match self {
                Number::SNum(s)=>Number::SNum(s>>other.signed()),
                Number::UNum(s)=>Number::UNum(s>>other.unsigned()),
            }
        } else if a && !b {
            match self {
                Number::SNum(s)=>Number::SNum(s>>other.signed()),
                Number::UNum(s)=>Number::UNum(s>>other.unsigned()),//never executed
            }
        } else {
            match self {
                Number::SNum(s)=>Number::SNum(s>>other.signed()),//never executed
                Number::UNum(s)=>Number::UNum(Unsigned::from(s)>>other.unsigned()),
            }
        }
    }
}
impl std::ops::Div for Number {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        let a = self.is_signed();
        let b = other.is_signed();
        if a == b {
            match self {
                Number::SNum(s)=>Number::SNum(s/other.signed()),
                Number::UNum(s)=>Number::UNum(s/other.unsigned()),
            }
        } else if a && !b {
            match self {
                Number::SNum(s)=>Number::SNum(s/other.signed()),
                Number::UNum(s)=>Number::UNum(s/other.unsigned()),//never executed
            }
        } else {
            match self {
                Number::SNum(s)=>Number::SNum(s/other.signed()),//never executed
                Number::UNum(s)=>Number::UNum(Unsigned::from(s)/other.unsigned()),
            }
        }
    }
}
impl std::ops::Sub for Number {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        let a = self.is_signed();
        let b = other.is_signed();
        if a == b {
            match self {
                Number::SNum(s)=>Number::SNum(s-other.signed()),
                Number::UNum(s)=>Number::UNum(s-other.unsigned()),
            }
        } else if a && !b {
            match self {
                Number::SNum(s)=>Number::SNum(s-other.signed()),
                Number::UNum(s)=>Number::UNum(s-other.unsigned()),//never executed
            }
        } else {
            match self {
                Number::SNum(s)=>Number::SNum(s-other.signed()),//never executed
                Number::UNum(s)=>Number::UNum(Unsigned::from(s)-other.unsigned()),
            }
        }
    }
}
impl std::ops::Add for Number {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        let a = self.is_signed();
        let b = other.is_signed();
        if a == b {
            match self {
                Number::SNum(s)=>Number::SNum(s+other.signed()),
                Number::UNum(s)=>Number::UNum(s+other.unsigned()),
            }
        } else if a && !b {
            match self {
                Number::SNum(s)=>Number::SNum(s+other.signed()),
                Number::UNum(s)=>Number::UNum(s+other.unsigned()),//never executed
            }
        } else {
            match self {
                Number::SNum(s)=>Number::SNum(s+other.signed()),//never executed
                Number::UNum(s)=>Number::UNum(Unsigned::from(s)+other.unsigned()),
            }
        }
    }
}
#[macro_export]
macro_rules! impl_numbers_with_generics {
    ($trait:ident,$trait_func:ident,$sself:ident,$from_numbers:ident)=>{
        impl $trait<i8> for $sself {
            #[inline]
            fn $trait_func(a:i8)->$sself{
                $sself::$from_numbers(Number::from(Signed::from(a)))
            }
        } 
        impl $trait<i16> for $sself {
            #[inline]
            fn $trait_func(a:i16)->$sself{
                $sself::$from_numbers(Number::from(Signed::from(a)))
            }
        } 
        impl $trait<i32> for $sself {
            #[inline]
            fn $trait_func(a:i32)->$sself{
                $sself::$from_numbers(Number::from(Signed::from(a)))
            }
        } 
        impl $trait<i64> for $sself {
            #[inline]
            fn $trait_func(a:i64)->$sself{
                $sself::$from_numbers(Number::from(Signed::from(a)))
            }
        } 
        impl $trait<u8> for$sself {
            #[inline]
            fn $trait_func(a:u8)->$sself{
                $sself::$from_numbers(Number::from(Unsigned::from(a)))
            }
        } 
        impl $trait<u16> for $sself {
            #[inline]
            fn $trait_func(a:u16)->$sself{
                $sself::$from_numbers(Number::from(Unsigned::from(a)))
            }
        } 
        impl $trait<u32> for $sself {
            #[inline]
            fn $trait_func(a:u32)->$sself{
                $sself::$from_numbers(Number::from(Unsigned::from(a)))
            }
        } 
        impl $trait<u64> for $sself {
            #[inline]
            fn $trait_func(a:u64)->$sself{
                $sself::$from_numbers(Number::from(Unsigned::from(a)))
            }
        } 
    }
}
impl From<isize> for Signed {
    #[inline]
    fn from(s:isize) -> Signed{
        if cfg!(target_pointer_width = "32") {
            Signed::I32(s as i32)
        } else {
            Signed::I64(s as i64)
        }
    }
}
impl From<usize> for Unsigned {
    #[inline]
    fn from(s:usize) -> Unsigned{
        if cfg!(target_pointer_width = "32") {
            Unsigned::U32(s as u32)
        } else {
            Unsigned::U64(s as u64)
        }
    }
}
impl From<isize> for Number {
    #[inline]
    fn from(s:isize)->Number {
        Number::SNum(Signed::from(s))
    }
}
impl From<usize> for Number {
    #[inline]
    fn from(s:usize)->Number {
        Number::UNum(Unsigned::from(s))
    }
}
impl From<f32> for Number {
    #[inline]
    fn from(s:f32)->Number {
        Number::SNum(Signed::F32(s))
    }
}
impl From<f64> for Number {
    #[inline]
    fn from(s:f64)->Number {
        Number::SNum(Signed::F64(s))
    }
}
impl_numbers_with_generics!(From,from,Number,from_number);