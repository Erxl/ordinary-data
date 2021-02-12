use std::cell::Cell;
use std::collections::btree_map::Values;
use std::collections::BTreeMap;
use std::iter::Map;
use std::ptr::NonNull;
#[macro_use]
use std::cmp;
#[macro_use]//todo 使用宏
use std::marker::*;

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
            data,
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
            relation.to.values().map(|x| x.get_mut().relations_in.remove(&key));

            self.relations.remove(&key);
        });
        c.relations_in.values().for_each(|x| {
            let relation = x.get();
            let key = relation.key;

            relation.kind.get_mut().relations_kind.remove(&key);
            relation.from.get_mut().relations_out.remove(&key);

            self.relations.remove(&key);
        });
        c.relations_kind.values().for_each(|x| {
            let relation = x.get();
            let key = relation.key;

            relation.to.values().map(|x| x.get_mut().relations_in.remove(&key));
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
    pub unsafe fn create_relation_with_data<'a, To: Clone + Iterator<Item=&'a ConceptRef<C, R>>>(
        &'a mut self,
        kind: ConceptRef<C, R>,
        from: ConceptRef<C, R>,
        to: To, data: R) -> RelationRef<C, R> {
        //获取key
        let key = self.relations_newest_key.get();
        *self.relations_newest_key.get_mut() += 1;

        //创建关系
        let relation = Box::new(Relation {
            key,
            data: data,
            kind: kind,
            from: from,
            to: to.clone().map(|x| (x.key(), *x)).collect::<_>(),
        });
        let relation_ref = RelationRef::new_from_ref(&*relation);

        //注册关系
        kind.get_mut().relations_kind.insert(key, relation_ref);
        from.get_mut().relations_out.insert(key, relation_ref);
        to.for_each(|x| { x.get_mut().relations_in.insert(key, relation_ref); });
        self.relations.insert(key, relation);
        //封装并返回
        relation_ref
    }
    #[inline]
    pub unsafe fn create_relation<'a, To: Clone + Iterator<Item=&'a ConceptRef<C, R>>>(
        &'a mut self,
        kind: ConceptRef<C, R>,
        from: ConceptRef<C, R>,
        to: To) ->
        RelationRef<C, R> where R: Default {
        self.create_relation_with_data(kind, from, to, Default::default())
    }

    pub unsafe fn delete_relation(&mut self, relation: RelationRef<C, R>) {
        let key = relation.key();
        let relation = relation.get();
        relation.kind.get_mut().relations_kind.remove(&key);
        relation.to.values().for_each(|x| { x.get_mut().relations_in.remove(&key); });
        relation.from.get_mut().relations_out.remove(&key);
        self.relations.remove(&key);
    }
    #[inline]
    pub fn relations_count(&self) -> usize { self.relations.len() }
    #[inline]
    pub fn concepts_count(&self) -> usize { self.concepts.len() }
    #[inline]
    pub fn concepts_iter<'a>(&'a self) -> ConceptsRefIter<'a, C, R> {
        ConceptsRefIter(self.concepts.values().map(|x| ConceptRef::new_from_ref(&**x)))
    }
    #[inline]
    pub fn relations_iter<'a>(&'a self) -> RelationsRefIter<'a, C, R> {
        RelationsRefIter(self.relations.values().map(|x| RelationRef::new_from_ref(&**x)))
    }
}

pub struct ConceptsRefIter<'a, C, R>(
    Map<Values<'a, u64, Box<Concept<C, R>>>, fn(&'a Box<Concept<C, R>>) -> ConceptRef<C, R>>);

impl<'a, C, R> Clone for ConceptsRefIter<'a, C, R> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub struct RelationsRefIter<'a, C, R>(
    Map<Values<'a, u64, Box<Relation<C, R>>>, fn(&'a Box<Relation<C, R>>) -> RelationRef<C, R>>);

impl<'a, C, R> Clone for RelationsRefIter<'a, C, R> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'a, C, R> Iterator for ConceptsRefIter<'a, C, R> {
    type Item = ConceptRef<C, R>;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a, C, R> Iterator for RelationsRefIter<'a, C, R> {
    type Item = RelationRef<C, R>;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
//todo 实现原始迭代器的所有功能

pub struct Relation<C, R> {
    key: u64,
    data: R,
    kind: ConceptRef<C, R>,
    from: ConceptRef<C, R>,
    to: BTreeMap<u64, ConceptRef<C, R>>,
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
    fn new_from_ref(relation: &Relation<C, R>) -> RelationRef<C, R> {
        unsafe { Self(NonNull::new_unchecked(relation as *const Relation<_, _> as _)) }
    }

    #[inline]
    pub fn kind(&self) -> ConceptRef<C, R> { unsafe { self.get() }.kind }
    #[inline]
    pub fn from(&self) -> ConceptRef<C, R> { unsafe { self.get() }.from }
    #[inline]
    pub fn to(&self) -> &BTreeMap<u64, ConceptRef<C, R>> { &unsafe { self.get() }.to }
}

pub struct ConceptRef<C, R> (NonNull<Concept<C, R>>);

impl<C, R> Clone for ConceptRef<C, R> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<C, R> Copy for ConceptRef<C, R> {}
impl<C, R> PartialEq for ConceptRef<C, R> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0==other.0
    }
}
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
    #[inline]
    pub unsafe fn relations_in(&self) -> &BTreeMap<u64, RelationRef<C, R>> {
        &self.get().relations_in
    }
    #[inline]
    pub unsafe fn relations_out(&self) -> &BTreeMap<u64, RelationRef<C, R>> {
        &self.get().relations_out
    }
    #[inline]
    pub unsafe fn relations_kind(&self) -> &BTreeMap<u64, RelationRef<C, R>> {
        &self.get().relations_kind
    }
}

pub struct Concept<C, R> {
    key: u64,
    data: C,
    relations_in: BTreeMap<u64, RelationRef<C, R>>,
    relations_out: BTreeMap<u64, RelationRef<C, R>>,
    relations_kind: BTreeMap<u64, RelationRef<C, R>>,
}
