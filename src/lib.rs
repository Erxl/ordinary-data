use std::collections::BTreeMap;

#[derive(Copy,Clone)]
pub struct ConceptRef<D=()> {
    ptr: *mut Concept<D>
}

impl<D> ConceptRef<D> {
    pub unsafe fn data(&self) -> &D {
        &(*self.ptr).data
    }
    pub unsafe fn data_mut(&mut self) -> &mut D {
        &mut (*self.ptr).data
    }
}

struct Concept<D> {
    key: u64,
    data: D,

}

pub struct Container<D=()> {
    newest_key: Cell<u64>,
    concepts: Vec<Box<Concept<D>>>,
}
impl Container {
    pub fn create_concept(&mut self) -> ConceptRef {
        self.create_concept_data(())
    }
}

impl<D> Container<D> {
    pub fn new() -> Self {
        Self {
            newest_key: Default::default(),
            concepts: Default::default(),
        }
    }

    pub fn create_concept_data(&mut self, data: D) -> ConceptRef<D> {
        let c = Box::new(Concept {
            key: self.newest_key.get(),
            data: data,
        });
        let ptr = &*c as *const Concept<D> as *mut Concept<D>;
        self.concepts.push(c);
        *self.newest_key.get_mut() += 1;
        ConceptRef::<D> {
            ptr
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn test_main() {
        let mut c = Container::new();
        let c1 = c.create_concept();
        unsafe{
        let a=c1.data();
        }
    }
}