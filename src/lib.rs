#![feature(box_syntax)]
#![feature(allocator_api)]

use counting_pointer::Sc;
use std::cell::{Cell, RefCell, Ref, BorrowError, BorrowMutError, RefMut};
use std::alloc::{Global, GlobalAlloc};
use std::collections::BTreeMap;

pub struct ConceptRef<K: Ord + Copy, D> {
    data: Sc<RefCell<Option<Box<Concept<K, D>>>>>
}

impl<K: Ord + Copy, D> ConceptRef<K, D> {
    fn inner(&self) -> Result<Ref<'_, Option<Box<Concept<K, D>, Global>>>, BorrowError> {
        self.data.try_borrow()
    }
    fn inner_mut(&mut self) -> Result<RefMut<'_, Option<Box<Concept<K, D>, Global>>>, BorrowMutError> {
        self.data.try_borrow_mut()
    }

    pub fn key(&self) -> K {
        self.inner().as_ref().unwrap().as_ref().unwrap().key
    }
    pub fn relate(&mut self,to: &mut ConceptRef<K, D>,) {

    }
}

struct Concept<K: Ord + Copy, D, > {
    key: K,
    to_this: BTreeMap<K, RelationIn<K, D>>,
    from_this: BTreeMap<K, RelationOut<K, D>>,
    data: D,
}

impl<K: Ord + Copy, D> Concept<K, D> {
    fn new(key: K, data: D) -> ConceptRef<K, D> {
        ConceptRef {
            data: Sc::new(RefCell::new(Some(box Concept::<K, D> {
                key: key,
                to_this: Default::default(),
                from_this: Default::default(),
                data: data,
            })), Default::default())
        }
    }
}
struct Relation<K: Ord + Copy, D> {
    from: ConceptRef<K, D>,
    to: ConceptRef<K, D>,
    kind:ConceptRef<K, D>,
}
struct RelationIn<K: Ord + Copy, D> {
    from: ConceptRef<K, D>,
    kind: ConceptRef<K, D>,
}

struct RelationOut<K: Ord + Copy, D> {
    to: ConceptRef<K, D>,
    kind: ConceptRef<K, D>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn test_main() {
        let c = Concept::new(0, ());
    }
}