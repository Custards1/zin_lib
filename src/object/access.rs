use bitflags::bitflags;
bitflags!{
    pub struct ObjAccess :u32 {
        const CONSTANT     = 0b00000001;
        const METHOD       = 0b00000010;
        const UNTARGETABLE = 0b00000100;
        const NONE = 0;
    }
}
impl ObjAccess {
    #[inline]
    pub fn clear(&mut self,b:Self) {
        self.bits &= !(b.bits);
    }
}
