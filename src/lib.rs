#![feature(box_syntax)]

use counting_pointer::Sc;
use std::cell::{Cell, RefCell};
use std::alloc::System;
use std::collections::BTreeMap;

pub struct ConceptRef<K:Ord+Copy,D> {
    data: Sc<RefCell<Option<Box<Concept<K,D>>>>>
}
impl<K:Ord+Copy,D> ConceptRef<K,D> {
   pub fn Key(&self)->K{
       (*self.data).borrow().as_ref().unwrap().key
   }
}
struct Concept<K:Ord+Copy,D> {
    key:K,
    to_this:BTreeMap<K,ConceptRef<K,D>>,
    from_this:BTreeMap<K,ConceptRef<K,D>>,
    data:D,
}
impl<K:Ord+Copy,D> Concept<K,D> {
    fn new(key:K,data:D) -> ConceptRef<K,D>{
        ConceptRef{
            data:Sc::new(RefCell::new(Some(box Concept::<K,D>{
                key:key,
                to_this:Default::default(),
                from_this:Default::default(),
                data:data,
            },)),System)
        }
    }
}
struct Relation {

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::LinkedList;

    #[test]
    fn test_main() {
        let c=Concept::new(0,());
    }
}