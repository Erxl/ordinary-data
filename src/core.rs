/*
  Copyright (c) 2021 Erxl
  ordinary-data is licensed under Mulan PSL v2.
  You can use this software according to the terms and conditions of the Mulan PSL v2.
  You may obtain a copy of Mulan PSL v2 at:
  http://license.coscl.org.cn/MulanPSL2
  THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
  See the Mulan PSL v2 for more details.
*/
use std::collections::*;
use std::ptr::NonNull;
use std::cmp::Ordering;

//——————————————————————————————————————————————————————结构—————————————————————————————————————————
macro_rules! args {($ty:ident) => {$ty<C, R, T>}}

#[derive(Default)]
struct KeyPool(u64);

impl KeyPool {
    #[inline]
    pub fn rent(&mut self) -> u64 {
        let key = self.0;
        self.0 += 1;
        key
    }
    #[inline]
    pub fn ret(&mut self, _key: u64) {}
}

pub struct Container<C = (), R = (), T = ()> {
    concepts_key_pool: KeyPool,
    relations_key_pool: KeyPool,
    relation_types_key_pool: KeyPool,
    concepts: BTreeMap<u64, Concept<C, R, T>>,
    //todo 使用ordclct树，避免多次引用造成的性能损失
    relations: BTreeMap<u64, Relation<C, R, T>>,
    relation_types: BTreeMap<u64, RelationType<C, R, T>>,
}

pub struct Concept<C, R, T> {
    key: u64,
    data: C,
    relation_type_to_relation: BTreeMap<u64, RelationPtr<C, R, T>>,
    src_to_relation: BTreeMap<u64, RelationPtr<C, R, T>>,
}

pub struct RelationType<C, R, T> {
    key: u64,
    data: T,
    dst_to_relations: BTreeMap<u64, BTreeMap<u64, RelationPtr<C, R, T>>>,
}

pub struct Relation<C, R, T> {
    key: u64,
    data: R,
    relation_type: RelationTypePtr<C, R, T>,
    src: ConceptPtr<C, R, T>,
    key_to_dst: BTreeMap<u64, ConceptPtr<C, R, T>>,
}

macro_rules! declare {
    ($ty_ptr:ident,$ty:ident,$ty_data:ident) => {
        pub struct $ty_ptr<C, R, T>
        (NonNull<$ty<C, R, T>>);
        impl<C, R, T> Clone for args!($ty_ptr) {
            #[inline] fn clone(&self) -> Self {Self(self.0)}
        }
        impl<C, R, T> PartialEq for args!($ty_ptr){
            #[inline] fn eq(&self, other: &Self) -> bool {self.0.eq(&other.0)}
        }
        impl <C, R, T> Eq for args!($ty_ptr){}
        impl <C, R, T> PartialOrd for args!($ty_ptr){
            #[inline] fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.0.partial_cmp(&other.0)}
        }
        impl <C, R, T> Ord for args!($ty_ptr){
            #[inline]fn cmp(&self, other: &Self) -> Ordering {self.0.cmp(&other.0)}
        }
        impl<C, R, T> Copy for args!($ty_ptr) {}
        impl<C, R, T> $ty_ptr<C, R, T> {
            #[inline] pub unsafe fn data(&self) -> &$ty_data { &self.0.as_ref().data }
            #[inline] pub unsafe fn data_mut(&self) -> &mut $ty_data { &mut (*self.as_ptr()).data }
            #[inline] pub unsafe fn key(&self) -> u64 { self.0.as_ref().key }
            #[inline] #[allow(dead_code)] unsafe fn get_mut(&self) -> &mut args!($ty) {&mut *self.as_ptr() }
            #[inline] #[allow(dead_code)] unsafe fn get(&self) -> &args!($ty) {self.0.as_ref() }
            #[inline] #[allow(dead_code)] fn as_ptr(&self) -> *mut args!($ty) { self.0.as_ptr() }
            #[inline] #[allow(dead_code)] fn new_from_ptr(ptr: *const args!($ty))-> args!($ty_ptr) {
                unsafe { Self(NonNull::new_unchecked(ptr as _)) }}
            #[inline] #[allow(dead_code)] fn new_from_ref(x: &args!($ty))-> args!($ty_ptr) {
                unsafe { Self(NonNull::new_unchecked(x as *const $ty<_, _, _> as _)) }
            }
        }
    }
}

declare!(ConceptPtr,Concept,C);
declare!(RelationPtr,Relation,R);
declare!(RelationTypePtr,RelationType,T);

//——————————————————————————————————————————————————————实现—————————————————————————————————————————
impl<C, R, T> Container<C,R,T> {
    pub fn new() -> Self {
        Self {
            concepts_key_pool: Default::default(),
            relations_key_pool: Default::default(),
            relation_types_key_pool: Default::default(),
            concepts: Default::default(),
            relations: Default::default(),
            relation_types: Default::default(),
        }
    }
    //——————————————————————————————————————————————————————增删—————————————————————————————————————
    #[inline]
    pub fn create_concept(&mut self) -> ConceptPtr<C, R, T> where C: Default {
        self.create_concept_with_data(Default::default())
    }
    pub fn create_concept_with_data(&mut self, data: C) -> ConceptPtr<C, R, T> {
        let key = self.concepts_key_pool.rent();

        ConceptPtr::new_from_ref(self.concepts.entry(key).or_insert(Concept {
            key,
            data,
            relation_type_to_relation: Default::default(),
            src_to_relation: Default::default(),
        }))
    }

    pub unsafe fn delete_concept(&mut self, concept: ConceptPtr<C, R, T>) {
        let c = concept.get_mut();
        let key = c.key;

        //todo 可优化;
        c.relation_type_to_relation.values()
            .collect::<Vec<_>>().into_iter().for_each(|x| self.delete_relation(*x));
        c.src_to_relation.values()
            .collect::<Vec<_>>().into_iter().for_each(|x| self.delete_relation(*x));

        self.concepts.remove(&key).unwrap();
        self.concepts_key_pool.ret(key);
    }

    pub unsafe fn create_relation_with_data<'a, DstConceptsIter: Clone + Iterator<Item=&'a ConceptPtr<C, R, T>>>(
        &'a mut self,
        relation_type: RelationTypePtr<C, R, T>,
        src: ConceptPtr<C, R, T>,
        dst_iter: DstConceptsIter,
        data: R) -> RelationPtr<C, R, T> {
        //申请key
        let key = self.relations_key_pool.rent();

        //创建关系
        let relation_ref = RelationPtr::new_from_ref(self.relations.entry(key).or_insert(
            Relation {
                key,
                data,
                relation_type: RelationTypePtr::new_from_ref(relation_type.get()),
                src: ConceptPtr::new_from_ref(src.get()),
                key_to_dst: dst_iter.clone().map(|x| (x.key(), *x)).collect::<_>(),
            }));

        //注册关系
        src.get_mut().relation_type_to_relation.insert(relation_type.key(), relation_ref);
        let relation_type_dst_to_relation_ref = &mut relation_type.get_mut().dst_to_relations;
        dst_iter.for_each(|dst| {
            relation_type_dst_to_relation_ref.entry(dst.key())
                .or_insert_with(BTreeMap::new).insert(key, relation_ref);
            dst.get_mut().src_to_relation.insert(src.key(), relation_ref);
        });

        //封装并返回
        relation_ref
    }
    #[inline]
    pub unsafe fn create_relation<'a, DstConceptsIter: Clone + Iterator<Item=&'a ConceptPtr<C, R, T>>>(
        &'a mut self,
        relation_type: RelationTypePtr<C, R, T>,
        src: ConceptPtr<C, R, T>,
        dst_iter: DstConceptsIter) -> RelationPtr<C, R, T> where R: Default {
        self.create_relation_with_data(relation_type, src, dst_iter, Default::default())
    }

    pub unsafe fn delete_relation(&mut self, relation: RelationPtr<C, R, T>) {
        let relation = relation.get();
        let relation_key = relation.key;
        relation.key_to_dst.iter().for_each(#[inline]|(dst_key, dst)| {
            let dst_to_relations = &mut relation.relation_type.get_mut().dst_to_relations;
            let dst_to_relations_relations = &mut dst_to_relations.get_mut(&dst_key).unwrap();
            dst_to_relations_relations.remove(&relation_key);
            if dst_to_relations_relations.is_empty() { dst_to_relations.remove(&dst_key); }

            dst.get_mut().src_to_relation.remove(&relation.src.key());
        });
        relation.src.get_mut().relation_type_to_relation.remove(&relation.relation_type.key());
        self.relations.remove(&relation_key);
        self.relations_key_pool.ret(relation_key);
    }
    #[inline]
    pub fn create_relation_type(&mut self) -> RelationTypePtr<C, R, T> where T: Default {
        self.create_relation_type_with_data(Default::default())
    }
    pub fn create_relation_type_with_data(&mut self, data: T) -> RelationTypePtr<C, R, T> {
        //申请key
        let key = self.relation_types_key_pool.rent();

        //创建
        RelationTypePtr::new_from_ref(self.relation_types.entry(key).or_insert(
            RelationType {
                key,
                data,
                dst_to_relations: Default::default(),
            }))
    }

    pub unsafe fn delete_relation_type(&mut self, relation_type: RelationTypePtr<C, R, T>) {
        let relation_type_ref = relation_type.get_mut();
        let relation_type_key = relation_type_ref.key;
        //todo 可优化;
        relation_type_ref.dst_to_relations.values().collect::<Vec<_>>().into_iter()
            .for_each(#[inline]|x| x.values().for_each(#[inline]|y| self.delete_relation(*y)));

        self.relation_types.remove(&relation_type_ref.key);
        self.relation_types_key_pool.ret(relation_type_key);
    }
    //——————————————————————————————————————————————————————检索—————————————————————————————————————
    #[inline]
    pub fn relations_count(&self) -> usize { self.relations.len() }
    #[inline]
    pub fn concepts_count(&self) -> usize { self.concepts.len() }
    #[inline]
    pub fn relation_types_count(&self) -> usize { self.relation_types.len() }
    //
    #[inline]
    pub fn contains_concept(&mut self, concept: ConceptPtr<C, R, T>) -> bool {
        self.concepts.values()
            .any(#[inline]|x| concept.as_ptr() == &*x as *const Concept<_, _, _> as _)
    }
    #[inline]
    pub fn contains_concept_key(&mut self, key: u64) -> bool {
        self.concepts.contains_key(&key)
    }
    #[inline]
    pub fn get_concept(&mut self, key: u64) -> Option<ConceptPtr<C, R, T>> {
        self.concepts.get(&key).map(#[inline]|x| ConceptPtr::new_from_ref(x))
    }
    #[inline]
    pub fn contains_relation(&mut self, relation: RelationPtr<C, R, T>) -> bool {
        self.relations.values()
            .any(#[inline]|x| relation.as_ptr() == &*x as *const Relation<_, _, _> as _)
    }
    #[inline]
    pub fn contains_relation_key(&mut self, key: u64) -> bool {
        self.relations.contains_key(&key)
    }
    #[inline]
    pub fn get_relation(&mut self, key: u64) -> Option<RelationPtr<C, R, T>> {
        self.relations.get(&key).map(#[inline]|x| RelationPtr::new_from_ref(x))
    }
    #[inline]
    pub fn contains_relation_type(&mut self, relation_type: RelationTypePtr<C, R, T>) -> bool {
        self.relation_types.values()
            .any(#[inline]|x| relation_type.as_ptr() == &*x as *const RelationType<_, _, _> as _)
    }
    #[inline]
    pub fn contains_relation_type_key(&mut self, key: u64) -> bool {
        self.relation_types.contains_key(&key)
    }
    #[inline]
    pub fn get_relation_type(&mut self, key: u64) -> Option<RelationTypePtr<C, R, T>> {
        self.relation_types.get(&key).map(#[inline]|x| RelationTypePtr::new_from_ref(x))
    }
    #[inline]
    pub fn concepts_iter(&self) -> impl Iterator<Item=ConceptPtr<C, R, T>> + '_ {
        self.concepts.values().map(#[inline]|x| ConceptPtr::new_from_ref(x))
    }
    #[inline]
    pub fn relations_iter(&self) -> impl Iterator<Item=RelationPtr<C, R, T>> + '_ {
        self.relations.values().map(#[inline]|x| RelationPtr::new_from_ref(x))
    }
    #[inline]
    pub fn relation_types_iter(&self) -> impl Iterator<Item=RelationTypePtr<C, R, T>> + '_ {
        self.relation_types.values().map(#[inline]|x| RelationTypePtr::new_from_ref(x))
    }
}

//todo 实现原始迭代器的所有功能
