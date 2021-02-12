use std::collections::BTreeMap;
use std::cell::Cell;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConceptRef<D = ()> {
    ptr: *mut Concept<D>
}

impl<D> ConceptRef<D> {
    #[inline]
    pub unsafe fn data(&self) -> &D {
        &self.get().data
    }
    #[inline]
    pub unsafe fn data_mut(&mut self) -> &mut D {
        &mut (*self.ptr).data
    }
    #[inline]
    unsafe fn get_mut(&mut self) -> &mut Concept<D> {
        &mut *self.ptr
    }
    #[inline]
    unsafe fn get(&self) -> &Concept<D> {
        &*self.ptr
    }
    #[inline]
    pub unsafe fn key(&self) -> u64 { self.get().key }
}

struct Concept<D> {
    key: u64,
    data: D,

}

pub struct Container<D = ()> {
    newest_key: Cell<u64>,
    concepts: BTreeMap<u64, Box<Concept<D>>>,
}

impl Container {
    pub fn create_concept(&mut self) -> ConceptRef {
        self.create_concept_with_data(())
    }
}

impl<D> Container<D> {
    pub fn new() -> Self {
        Self {
            newest_key: Default::default(),
            concepts: Default::default(),
        }
    }
    #[inline]
    pub fn create_concept_with_data(&mut self, data: D) -> ConceptRef<D> {
        let c = Box::new(Concept {
            key: self.newest_key.get(),
            data: data,
        });
        let ptr = &*c as *const Concept<D> as *mut Concept<D>;
        self.concepts.insert(self.newest_key.get(), c);
        *self.newest_key.get_mut() += 1;
        ConceptRef::<D> {
            ptr
        }
    }
    #[inline]
    pub unsafe fn delete_concept(&mut self, concept: ConceptRef<D>) -> bool {
        self.concepts.remove(&concept.get().key).is_some()
    }
    #[inline]
    pub fn contains(&mut self, concept: ConceptRef<D>) -> bool {
        self.concepts.values().any(|x| &**x as *const Concept<_> == concept.ptr as _)
    }
    #[inline]
    pub fn contains_key(&mut self, key: u64) -> bool {
        self.concepts.contains_key(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ref_create_delete() {
        let mut c = Container::new();
        let c1 = c.create_concept();

        //测试ConceptRef
        let c1_copy = c1;
        let c1_copy_copy = c1_copy;
        assert!(c1 == c1_copy_copy);

        //移除测试
        unsafe {
            assert!(c.delete_concept(c1));
        }
    }

    #[test]
    fn test_contains() {
        unsafe {
            let mut c = Container::new();
            let c1 = c.create_concept();
            let c2 = c.create_concept();
            let c2_key = c2.key();
            assert!(c.delete_concept(c2));

            assert!(c.contains(c1));
            assert!(c.contains_key(c1.key()));
            assert!(!c.contains(c2));
            assert!(!c.contains_key(c2_key));
        }
    }
}