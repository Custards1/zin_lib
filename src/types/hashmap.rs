use gc::custom_trace;
#[derive(Clone, Debug,Default)]
pub struct HashMap<T,K>(pub indexmap::IndexMap<T,K>);
impl<T,K> std::ops::Deref for HashMap<T,K> {
    type Target = indexmap::IndexMap<T,K>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T,K> std::ops::DerefMut for HashMap<T,K> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T,K> From<indexmap::IndexMap<T,K>> for HashMap<T,K> {
    fn from(w:indexmap::IndexMap<T,K>) -> Self {
        Self(w)
    }
}
impl<T:Eq+std::hash::Hash,K:PartialEq> PartialEq for HashMap<T,K> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl<T:Eq+std::hash::Hash,K:PartialEq> Eq for HashMap<T,K> {}


impl<T,K> HashMap<T,K> {
    #[inline]
    pub fn new()->Self {
        Self(indexmap::IndexMap::new())
    }
}
unsafe impl<T:gc::Trace,K:gc::Trace> gc::Trace for HashMap<T,K> {
    custom_trace!(this, {
        for (v,y) in this.iter() {
            mark(v);
            mark(y);
        }
    });
}
impl<T:gc::Finalize,K:gc::Finalize> gc::Finalize for HashMap<T,K> {}
