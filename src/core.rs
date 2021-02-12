use std::cell::Cell;
use std::collections::btree_map::Values;
use std::collections::BTreeMap;
use std::iter::Map;
use std::ptr::NonNull;

pub struct Container<C = (), R = ()> {
    concepts_newest_key: Cell<u64>,
    relations_newest_key: Cell<u64>,
    concepts: BTreeMap<u64, Box<Concept<C, R>>>,
    relations: BTreeMap<u64, Box<Relation<C, R>>>,

}


impl<C, R> Container<C, R> {
    pub fn new() -> Self {
        Self {
            concepts_newest_key: Default::default(),
            relations_newest_key: Default::default(),
            concepts: Default::default(),
            relations: Default::default(),
        }
    }
    #[inline]
    pub fn create_concept(&mut self) -> ConceptRef<C, R> where C: Default {
        self.create_concept_with_data(Default::default())
    }
    pub fn create_concept_with_data(&mut self, data: C) -> ConceptRef<C, R> {
        //获取key
        let key = self.concepts_newest_key.get();
        *self.concepts_newest_key.get_mut() += 1;

        let c = Box::new(Concept {
            key,
            data: data,
            relations_in: Default::default(),
            relations_out: Default::default(),
            relations_kind: Default::default(),
        });
        let ptr = &*c as *const Concept<C, R> as *mut Concept<C, R>;
        self.concepts.insert(key, c);
        ConceptRef::new_from_ptr(ptr)
    }

    pub unsafe fn delete_concept(&mut self, mut concept: ConceptRef<C, R>) {
        let c = concept.0.as_mut();
        let key = c.key;
        c.relations_out.values().for_each(|x| {
            let relation = x.get();
            let key = relation.key;

            relation.kind.get_mut().relations_kind.remove(&key);
            relation.to.get_mut().relations_in.remove(&key);
            //relation.from.get_mut().relations_out.remove(&key);
            self.relations.remove(&key);
        });
        c.relations_in.values().for_each(|x| {
            let relation = x.get();
            let key = relation.key;

            relation.kind.get_mut().relations_kind.remove(&key);
            //relation.to.get_mut().relations_in.remove(&key);
            relation.from.get_mut().relations_out.remove(&key);
            self.relations.remove(&key);
        });
        c.relations_kind.values().for_each(|x| {
            let relation = x.get();
            let key = relation.key;

            //relation.kind.get_mut().relations_kind.remove(&key);
            relation.to.get_mut().relations_in.remove(&key);
            relation.from.get_mut().relations_out.remove(&key);
            self.relations.remove(&key);
        });
        self.concepts.remove(&key).unwrap();
    }
    pub fn contains_concept(&mut self, concept: ConceptRef<C, R>) -> bool {
        self.concepts.values().any(|x| concept.as_ptr() == &**x as *const Concept<_, _> as _)
    }
    pub fn contains_concept_key(&mut self, key: u64) -> bool {
        self.concepts.contains_key(&key)
    }
    pub fn get_concept(&mut self, key: u64) -> Option<ConceptRef<C, R>> {
        self.concepts.get(&key).map(|x| ConceptRef::new_from_ref(x))
    }
    pub fn contains_relation(&mut self, relation: RelationRef<C, R>) -> bool {
        self.relations.values().any(|x| relation.as_ptr() == &**x as *const Relation<_, _> as _)
    }
    pub fn contains_relation_key(&mut self, key: u64) -> bool {
        self.relations.contains_key(&key)
    }
    pub fn get_relation(&mut self, key: u64) -> Option<RelationRef<C, R>> {
        self.relations.get(&key).map(|x| RelationRef::new_from_ref(x))
    }
    pub unsafe fn relate_with_data(&mut self, kind: ConceptRef<C, R>, from: ConceptRef<C, R>, to: ConceptRef<C, R>, data: R) -> RelationRef<C, R> {
        //获取key
        let key = self.relations_newest_key.get();
        *self.relations_newest_key.get_mut() += 1;

        //创建关系
        let relation = Box::new(Relation {
            key,
            data: data,
            kind: kind,
            from: from,
            to: to,
        });
        let ptr = &*relation as *const Relation<C, R> as *mut Relation<C, R>;

        let relation_ref = RelationRef(NonNull::new_unchecked(ptr));
        //注册关系
        kind.get_mut().relations_kind.insert(key, relation_ref);
        from.get_mut().relations_out.insert(key, relation_ref);
        to.get_mut().relations_in.insert(key, relation_ref);
        self.relations.insert(key, relation);

        //封装并返回
        relation_ref
    }
    #[inline]
    pub unsafe fn relate(&mut self, kind: ConceptRef<C, R>, from: ConceptRef<C, R>, to: ConceptRef<C, R>) ->
    RelationRef<C, R> where R: Default {
        self.relate_with_data(kind, from, to, Default::default())
    }

    pub unsafe fn disrelate(&mut self, relation: RelationRef<C, R>) {
        let key = relation.key();
        let relation = relation.get();
        relation.kind.get_mut().relations_kind.remove(&key);
        relation.to.get_mut().relations_in.remove(&key);
        relation.from.get_mut().relations_out.remove(&key);
        self.relations.remove(&key);
    }
    #[inline]
    pub fn relations_count(&self) -> usize { self.relations.len() }
    #[inline]
    pub fn concepts_count(&self) -> usize { self.concepts.len() }
    #[inline]
    pub fn iter<'a>(&'a self) -> Iter<'a, C, R> {
        Iter(self.concepts.values().map(|x| ConceptRef::new_from_ref(&**x)))
    }
}

pub struct Iter<'a, C, R>(Map<Values<'a, u64, Box<Concept<C, R>>>, fn(&'a Box<Concept<C, R>>) -> ConceptRef<C, R>>);

impl<'a, C, R> Iterator for Iter<'a, C, R> {
    type Item = ConceptRef<C, R>;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct Relation<C, R> {
    key: u64,
    data: R,
    kind: ConceptRef<C, R>,
    from: ConceptRef<C, R>,
    to: ConceptRef<C, R>,
}

#[derive(Eq, PartialEq)]
pub struct RelationRef<C, R> (NonNull<Relation<C, R>>);

impl<C, R> Clone for RelationRef<C, R> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<C, R> Copy for RelationRef<C, R> {}

impl<C, R> RelationRef<C, R> {
    #[inline]
    pub unsafe fn data(&self) -> &R {
        &self.0.as_ref().data
    }
    #[inline]
    pub unsafe fn data_mut(&self) -> &mut R {
        &mut (*self.as_ptr()).data
    }
    #[inline]
    pub unsafe fn key(&self) -> u64 { self.0.as_ref().key }
    #[inline]
    unsafe fn get_mut(&self) -> &mut Relation<C, R> {
        &mut *self.as_ptr()
    }
    #[inline]
    unsafe fn get(&self) -> &Relation<C, R> {
        self.0.as_ref()
    }
    #[inline]
    fn as_ptr(&self) -> *mut Relation<C, R> {
        self.0.as_ptr()
    }
    #[inline]
    fn new_from_ptr(ptr: *const Relation<C, R>) -> RelationRef<C, R> {
        unsafe { Self(NonNull::new_unchecked(ptr as _)) }
    }
    #[inline]
    fn new_from_ref(ptr: &Relation<C, R>) -> RelationRef<C, R> {
        unsafe { Self(NonNull::new_unchecked(ptr as *const Relation<_, _> as _)) }
    }
}

#[derive(Eq, PartialEq)]
pub struct ConceptRef<C, R> (NonNull<Concept<C, R>>);

impl<C, R> Clone for ConceptRef<C, R> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<C, R> Copy for ConceptRef<C, R> {}

impl<C, R> ConceptRef<C, R> {
    #[inline]
    pub unsafe fn data(&self) -> &C {
        &self.0.as_ref().data
    }
    #[inline]
    pub unsafe fn data_mut(&self) -> &mut C {
        &mut (*self.as_ptr()).data
    }
    #[inline]
    pub unsafe fn key(&self) -> u64 { self.0.as_ref().key }
    #[inline]
    unsafe fn get_mut(&self) -> &mut Concept<C, R> {
        &mut *self.as_ptr()
    }
    #[inline]
    unsafe fn get(&self) -> &Concept<C, R> {
        self.0.as_ref()
    }
    #[inline]
    fn as_ptr(&self) -> *mut Concept<C, R> {
        self.0.as_ptr()
    }
    #[inline]
    fn new_from_ptr(ptr: *const Concept<C, R>) -> ConceptRef<C, R> {
        unsafe { Self(NonNull::new_unchecked(ptr as _)) }
    }
    #[inline]
    fn new_from_ref(ptr: &Concept<C, R>) -> ConceptRef<C, R> {
        unsafe { Self(NonNull::new_unchecked(ptr as *const Concept<_, _> as _)) }
    }
}

pub struct Concept<C, R> {
    key: u64,
    data: C,
    relations_in: BTreeMap<u64, RelationRef<C, R>>,
    relations_out: BTreeMap<u64, RelationRef<C, R>>,
    relations_kind: BTreeMap<u64, RelationRef<C, R>>,
}
