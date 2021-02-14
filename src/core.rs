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
macro_rules! args {($ty:ident) => {$ty<ConceptData, RelationData, RelationTypeData>}}

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
    pub fn ret(&mut self, key: u64) {}
}

pub struct Container<ConceptData = (), RelationData = (), RelationTypeData = ()> {
    concepts_key_pool: KeyPool,
    relations_key_pool: KeyPool,
    relation_types_key_pool: KeyPool,
    concepts: BTreeMap<u64, args!(Concept)>,
    //todo 使用ordclct树，避免多次引用造成的性能损失
    relations: BTreeMap<u64, args!(Relation)>,
    relation_types: BTreeMap<u64, args!(RelationType)>,
}

pub struct Concept<ConceptData, RelationData, RelationTypeData> {
    key: u64,
    data: ConceptData,
    relation_type_to_relation: BTreeMap<u64, args!(RelationPtr)>,
    src_to_relation: BTreeMap<u64, args!(RelationPtr)>,
}

pub struct RelationType<ConceptData, RelationData, RelationTypeData> {
    key: u64,
    data: RelationTypeData,
    dst_to_relations: BTreeMap<u64, BTreeMap<u64, args!(RelationPtr)>>,
}

pub struct Relation<ConceptData, RelationData, RelationTypeData> {
    key: u64,
    data: RelationData,
    relation_type: args!(RelationTypePtr),
    src: args!(ConceptPtr),
    key_to_dst: BTreeMap<u64, args!(ConceptPtr)>,
}

macro_rules! declare {
    ($ty_ptr:ident,$ty:ident,$ty_data:ident) => {
        pub struct $ty_ptr<ConceptData, RelationData, RelationTypeData>
        (NonNull<$ty<ConceptData, RelationData, RelationTypeData>>);
        impl<ConceptData, RelationData, RelationTypeData> Clone for args!($ty_ptr) {
            #[inline] fn clone(&self) -> Self {Self(self.0)}
        }
        impl<ConceptData, RelationData, RelationTypeData> PartialEq for args!($ty_ptr){
            #[inline] fn eq(&self, other: &Self) -> bool {self.0.eq(&other.0)}
        }
        impl <ConceptData, RelationData, RelationTypeData> Eq for args!($ty_ptr){}
        impl <ConceptData, RelationData, RelationTypeData> PartialOrd for args!($ty_ptr){
            #[inline] fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.0.partial_cmp(&other.0)}
        }
        impl <ConceptData, RelationData, RelationTypeData> Ord for args!($ty_ptr){
            #[inline]fn cmp(&self, other: &Self) -> Ordering {self.0.cmp(&other.0)}
        }
        impl<ConceptData, RelationData, RelationTypeData> Copy for args!($ty_ptr) {}
        impl<ConceptData, RelationData, RelationTypeData> $ty_ptr<ConceptData, RelationData, RelationTypeData> {
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

declare!(ConceptPtr,Concept,ConceptData);
declare!(RelationPtr,Relation,RelationData);
declare!(RelationTypePtr,RelationType,RelationTypeData);

//——————————————————————————————————————————————————————实现—————————————————————————————————————————
impl<ConceptData, RelationData, RelationTypeData> args!(RelationPtr) {
    //todo 合理性分析
    // #[inline]
    // pub fn relation_type(&self) -> RelationPtr<ConceptData, RelationData, RelationTypeData> {
    //     unsafe { self.get() }.relation_type.clone()
    // }
    // #[inline]
    // pub fn relation_src(&self) -> ConceptPtr<ConceptData, RelationData, RelationTypeData> {
    //     unsafe { self.get() }.relation_src
    // }
    // #[inline]
    // pub fn key_to_relation_dst(&self) -> &BTreeMap<u64, ConceptPtr<ConceptData, RelationData, RelationTypeData>> {
    //     &unsafe { self.get() }.key_to_relation_dst
    // }
}
// impl<ConceptData, RelationData, RelationTypeData> args!(Concept) {
//     fn relations(&self)->BTreeSet<args!(RelationPtr)>{
//         //去重
//         let mut relations:BTreeSet<args!(RelationPtr)> = self.relation_type_to_relation.values().map(|x|*x).collect();
//         self.concept_src_to_relation.values().for_each(|x|{relations.insert(*x);});
//         relations
//     }
// }

impl<ConceptData, RelationData, RelationTypeData> args!(Container) {
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
    pub fn create_concept(&mut self) -> args!(ConceptPtr) where ConceptData: Default {
        self.create_concept_with_data(Default::default())
    }
    pub fn create_concept_with_data(&mut self, data: ConceptData) -> args!(ConceptPtr) {
        let key = self.concepts_key_pool.rent();

        ConceptPtr::new_from_ref(self.concepts.entry(key).or_insert(Concept {
            key,
            data,
            relation_type_to_relation: Default::default(),
            src_to_relation: Default::default(),
        }))
    }

    pub unsafe fn delete_concept(&mut self, concept: args!(ConceptPtr)) {
        let c = concept.get_mut();
        let key = c.key;

        //todo 可优化;
        c.relation_type_to_relation.values().collect::<Vec<_>>().into_iter().for_each(|x| self.delete_relation(*x));
        c.src_to_relation.values().collect::<Vec<_>>().into_iter().for_each(|x| self.delete_relation(*x));

        self.concepts.remove(&key).unwrap();
        self.concepts_key_pool.ret(key);
    }

    pub unsafe fn create_relation_with_data<'a, DstConceptsIter: Clone + Iterator<Item=&'a args!(ConceptPtr)>>(
        &'a mut self,
        relation_type: args!(RelationTypePtr),
        src: args!(ConceptPtr),
        dst_iter: DstConceptsIter,
        data: RelationData) -> args!(RelationPtr) {
        //申请key
        let key = self.relations_key_pool.rent();

        //创建关系
        let relation_ref = RelationPtr::new_from_ref(self.relations.entry(key).or_insert(Relation {
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
            relation_type_dst_to_relation_ref.entry(dst.key()).or_insert_with(BTreeMap::new).insert(key, relation_ref);
            dst.get_mut().src_to_relation.insert(src.key(), relation_ref);
        });

        //封装并返回
        relation_ref
    }
    #[inline]
    pub unsafe fn create_relation<'a, DstConceptsIter: Clone + Iterator<Item=&'a args!(ConceptPtr)>>(
        &'a mut self,
        relation_type: args!(RelationTypePtr),
        src: args!(ConceptPtr),
        dst_iter: DstConceptsIter) -> args!(RelationPtr) where RelationData: Default {
        self.create_relation_with_data(relation_type, src, dst_iter, Default::default())
    }

    pub unsafe fn delete_relation(&mut self, relation: args!(RelationPtr)) {
        let relation = relation.get();
        let relation_key = relation.key;
        relation.key_to_dst.iter().for_each(|(dst_key, dst)| {
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
    pub fn create_relation_type(&mut self) -> args!(RelationTypePtr) where RelationTypeData: Default {
        self.create_relation_type_with_data(Default::default())
    }
    pub fn create_relation_type_with_data(&mut self, data: RelationTypeData) -> args!(RelationTypePtr) {
        //申请key
        let key = self.relation_types_key_pool.rent();

        //创建
        RelationTypePtr::new_from_ref(self.relation_types.entry(key).or_insert(RelationType {
            key,
            data,
            dst_to_relations: Default::default(),
        }))
    }

    pub unsafe fn delete_relation_type(&mut self, relation_type: args!(RelationTypePtr)) {
        let relation_type_ref = relation_type.get_mut();
        let relation_type_key = relation_type_ref.key;
        //todo 可优化;
        relation_type_ref.dst_to_relations.values().collect::<Vec<_>>().into_iter()
            .for_each(|x| x.values().for_each(|y| self.delete_relation(*y)));

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
    pub fn contains_concept(&mut self, concept: args!(ConceptPtr)) -> bool {
        self.concepts.values().any(|x| concept.as_ptr() == &*x as *const Concept<_, _, _> as _)
    }
    #[inline]
    pub fn contains_concept_key(&mut self, key: u64) -> bool {
        self.concepts.contains_key(&key)
    }
    #[inline]
    pub fn get_concept(&mut self, key: u64) -> Option<args!(ConceptPtr)> {
        self.concepts.get(&key).map(|x| ConceptPtr::new_from_ref(x))
    }
    #[inline]
    pub fn contains_relation(&mut self, relation: args!(RelationPtr)) -> bool {
        self.relations.values().any(|x| relation.as_ptr() == &*x as *const Relation<_, _, _> as _)
    }
    #[inline]
    pub fn contains_relation_key(&mut self, key: u64) -> bool {
        self.relations.contains_key(&key)
    }
    #[inline]
    pub fn get_relation(&mut self, key: u64) -> Option<args!(RelationPtr)> {
        self.relations.get(&key).map(|x| RelationPtr::new_from_ref(x))
    }
    #[inline]
    pub fn contains_relation_type(&mut self, relation_type: args!(RelationTypePtr)) -> bool {
        self.relation_types.values().any(|x| relation_type.as_ptr() == &*x as *const RelationType<_, _, _> as _)
    }
    #[inline]
    pub fn contains_relation_type_key(&mut self, key: u64) -> bool {
        self.relation_types.contains_key(&key)
    }
    #[inline]
    pub fn get_relation_type(&mut self, key: u64) -> Option<args!(RelationTypePtr)> {
        self.relation_types.get(&key).map(|x| RelationTypePtr::new_from_ref(x))
    }
    #[inline]
    pub fn concepts_iter(&self) -> impl Iterator<Item=args!(ConceptPtr)> + '_ {
        self.concepts.values().map(|x| ConceptPtr::new_from_ref(x))
    }
    #[inline]
    pub fn relations_iter(&self) -> impl Iterator<Item=args!(RelationPtr)> + '_ {
        self.relations.values().map(|x| RelationPtr::new_from_ref(x))
    }
    #[inline]
    pub fn relation_types_iter(&self) -> impl Iterator<Item=args!(RelationTypePtr)> + '_ {
        self.relation_types.values().map(|x| RelationTypePtr::new_from_ref(x))
    }
}

//todo 实现原始迭代器的所有功能

