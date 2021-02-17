/*
  Copyright (c) 2021 Erxl
  ordinary-data is licensed under Mulan PSL v2.
  You can use this software according to the terms and conditions of the Mulan PSL v2.
  You may obtain a copy of Mulan PSL v2 at:
  http://license.coscl.org.cn/MulanPSL2
  THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
  See the Mulan PSL v2 for more details.
*/
use std::cmp::Ordering;
use std::collections::btree_map::Entry;
use std::collections::*;
use std::ptr::NonNull;

// #[inline]
// unsafe fn staticize_ref<T>(x: &T) -> &'static T {
//     &*(x as *const T)
// }
// #[inline]
// unsafe fn staticize_mut<T>(x: &mut T) -> &'static mut T {
//     &mut *(x as *mut T)
// }
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
    pub fn ret(&mut self, _key: u64) {}
}

pub struct Container<ConceptData = (), RelationData = (), RelationTypeData = ()> {
    concepts_key_pool: KeyPool,
    relation_types_key_pool: KeyPool,
    concepts: BTreeMap<u64, args!(Concept)>,
    //todo 使用ordclct树，避免多次引用造成的性能损失
    relation_types: BTreeMap<u64, args!(RelationType)>,
}

pub struct Concept<ConceptData, RelationData, RelationTypeData> {
    key: u64,
    data: ConceptData,
    relation_type_to_dst_relation: BTreeMap<u64, args!(RelationPtr)>,
    src_to_relation: BTreeMap<u64, args!(RelationPtr)>,
}

pub struct RelationType<ConceptData, RelationData, RelationTypeData> {
    key: u64,
    data: RelationTypeData,
    relations: BTreeMap<u64, args!(Relation)>,
    dst_to_relations: BTreeMap<u64, BTreeMap<u64, args!(RelationPtr)>>,
    relations_key_pool: KeyPool,
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
        #[derive(Debug)]
        pub struct $ty_ptr<ConceptData, RelationData, RelationTypeData>(
            NonNull<$ty<ConceptData, RelationData, RelationTypeData>>,
        );
        impl<ConceptData, RelationData, RelationTypeData> Clone
            for $ty_ptr<ConceptData, RelationData, RelationTypeData>
        {
            #[inline]
            fn clone(&self) -> Self {
                Self(self.0)
            }
        }
        impl<ConceptData, RelationData, RelationTypeData> PartialEq
            for $ty_ptr<ConceptData, RelationData, RelationTypeData>
        {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }
        impl<ConceptData, RelationData, RelationTypeData> Eq
            for $ty_ptr<ConceptData, RelationData, RelationTypeData>
        {
        }
        impl<ConceptData, RelationData, RelationTypeData> PartialOrd
            for $ty_ptr<ConceptData, RelationData, RelationTypeData>
        {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }
        impl<ConceptData, RelationData, RelationTypeData> Ord
            for $ty_ptr<ConceptData, RelationData, RelationTypeData>
        {
            #[inline]
            fn cmp(&self, other: &Self) -> Ordering {
                self.0.cmp(&other.0)
            }
        }
        impl<ConceptData, RelationData, RelationTypeData> Copy
            for $ty_ptr<ConceptData, RelationData, RelationTypeData>
        {
        }
        impl<ConceptData, RelationData, RelationTypeData>
            $ty_ptr<ConceptData, RelationData, RelationTypeData>
        {
            #[inline]
            pub unsafe fn data(&self) -> &'static $ty_data {
                &(*self.0.as_ptr()).data
            }
            #[inline]
            pub unsafe fn data_mut(&self) -> &'static mut $ty_data {
                &mut (*self.as_ptr()).data
            }
            #[inline]
            pub unsafe fn key(&self) -> u64 {
                self.0.as_ref().key
            }
            #[inline]
            #[allow(dead_code)]
            unsafe fn get_mut(&self) -> &'static mut args!($ty) {
                &mut *self.as_ptr()
            }
            #[inline]
            #[allow(dead_code)]
            unsafe fn get(&self) -> &'static args!($ty) {
                &(*self.0.as_ptr())
            }
            #[inline]
            #[allow(dead_code)]
            fn as_ptr(&self) -> *mut args!($ty) {
                self.0.as_ptr()
            }
            #[inline]
            #[allow(dead_code)]
            fn new_from_ptr(
                ptr: *const args!($ty),
            ) -> $ty_ptr<ConceptData, RelationData, RelationTypeData> {
                unsafe { Self(NonNull::new_unchecked(ptr as _)) }
            }
            #[inline]
            #[allow(dead_code)]
            fn new_from_ref(
                x: &args!($ty),
            ) -> $ty_ptr<ConceptData, RelationData, RelationTypeData> {
                unsafe { Self(NonNull::new_unchecked(x as *const $ty<_, _, _> as _)) }
            }
        }
    };
}

declare!(ConceptPtr, Concept, ConceptData);
declare!(RelationPtr, Relation, RelationData);
declare!(RelationTypePtr, RelationType, RelationTypeData);

//——————————————————————————————————————————————————————实现—————————————————————————————————————————
impl<ConceptData: 'static, RelationData: 'static, RelationTypeData: 'static>
    Container<ConceptData, RelationData, RelationTypeData>
{
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
    //——————————————————————————————————————————————————————增删—————————————————————————————————————
    #[inline]
    pub fn create_concept(&mut self) -> args!(ConceptPtr)
    where
        ConceptData: Default,
    {
        self.create_concept_with_data(Default::default())
    }
    pub fn create_concept_with_data(&mut self, data: ConceptData) -> args!(ConceptPtr) {
        let key = self.concepts_key_pool.rent();

        ConceptPtr::new_from_ref(self.concepts.entry(key).or_insert(Concept {
            key,
            data,
            relation_type_to_dst_relation: Default::default(),
            src_to_relation: Default::default(),
        }))
    }

    pub unsafe fn delete_concept(&mut self, concept: args!(ConceptPtr)) {
        let c = concept.get_mut();
        let key = c.key;

        //todo 可优化;
        c.relation_type_to_dst_relation //移除接出的关系
            .values()
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|x| x.delete());
        c.src_to_relation
            .values()
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|x| x.delete());

        self.concepts.remove(&key).unwrap();
        self.concepts_key_pool.ret(key);
    }

    #[inline]
    pub fn create_relation_type(&mut self) -> args!(RelationTypePtr)
    where
        RelationTypeData: Default,
    {
        self.create_relation_type_with_data(Default::default())
    }
    pub fn create_relation_type_with_data(
        &mut self,
        data: RelationTypeData,
    ) -> args!(RelationTypePtr) {
        //申请key
        let key = self.relation_types_key_pool.rent();

        //创建
        RelationTypePtr::new_from_ref(self.relation_types.entry(key).or_insert(RelationType {
            key,
            data,
            dst_to_relations: Default::default(),
            relations: Default::default(),
            relations_key_pool: Default::default(),
        }))
    }

    pub unsafe fn delete_relation_type(&mut self, relation_type: args!(RelationTypePtr)) {
        let relation_type_ref = relation_type.get_mut();
        let relation_type_key = relation_type_ref.key;
        //todo 可优化;
        relation_type_ref
            .dst_to_relations
            .values()
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(
                #[inline]
                |x| {
                    x.values().for_each(
                        #[inline]
                        |y| y.delete(),
                    )
                },
            );

        self.relation_types.remove(&relation_type_ref.key);
        self.relation_types_key_pool.ret(relation_type_key);
    }

    //——————————————————————————————————————————————————————检索—————————————————————————————————————
    #[inline]
    pub fn relations_count(&self) -> usize {
        self.relation_types
            .values()
            .map(|x| x.relations.len())
            .sum()
    }
    #[inline]
    pub fn concepts_count(&self) -> usize {
        self.concepts.len()
    }
    #[inline]
    pub fn relation_types_count(&self) -> usize {
        self.relation_types.len()
    }
    //
    #[inline]
    pub unsafe fn contains_concept(&mut self, concept: args!(ConceptPtr)) -> bool {
        self.contains_concept_key(concept.key())
    }
    #[inline]
    pub unsafe fn contains_concept_key(&mut self, key: u64) -> bool {
        self.concepts.contains_key(&key)
    }
    #[inline]
    pub unsafe fn get_concept(&mut self, key: u64) -> Option<args!(ConceptPtr)> {
        self.concepts.get(&key).map(
            #[inline]
            |x| ConceptPtr::new_from_ref(x),
        )
    }

    #[inline]
    pub unsafe fn contains_relation_type(&mut self, relation_type: args!(RelationTypePtr)) -> bool {
        self.contains_relation_type_key(relation_type.key())
    }
    #[inline]
    pub unsafe fn contains_relation_type_key(&mut self, key: u64) -> bool {
        self.relation_types.contains_key(&key)
    }
    #[inline]
    pub fn get_relation_type(&mut self, key: u64) -> Option<args!(RelationTypePtr)> {
        self.relation_types.get(&key).map(
            #[inline]
            |x| RelationTypePtr::new_from_ref(x),
        )
    }
    #[inline]
    pub fn concepts_iter(&self) -> impl Iterator<Item = args!(ConceptPtr)> + '_ {
        self.concepts.values().map(
            #[inline]
            |x| ConceptPtr::new_from_ref(x),
        )
    }
    #[inline]
    pub fn relations_iter(&self) -> impl Iterator<Item = args!(RelationPtr)> + '_ {
        self.relation_types
            .values()
            .flat_map(|x| x.relations.values())
            .map(RelationPtr::new_from_ref)
    }
    #[inline]
    pub fn relation_types_iter(&self) -> impl Iterator<Item = args!(RelationTypePtr)> + '_ {
        self.relation_types.values().map(
            #[inline]
            |x| RelationTypePtr::new_from_ref(x),
        )
    }
}

impl<ConceptData: 'static, RelationData: 'static, RelationTypeData: 'static> Default
    for Container<ConceptData, RelationData, RelationTypeData>
{
    fn default() -> Self {
        Self {
            concepts_key_pool: Default::default(),
            relation_types_key_pool: Default::default(),
            concepts: Default::default(),
            relation_types: Default::default(),
        }
    }
}

//todo 实现原始迭代器的所有功能
impl<ConceptData: 'static, RelationData: 'static, RelationTypeData: 'static>
    ConceptPtr<ConceptData, RelationData, RelationTypeData>
{
    #[inline]
    pub unsafe fn outgoing(
        self,
        relation_type: args!(RelationTypePtr),
    ) -> Option<args!(RelationPtr)> {
        self.get()
            .relation_type_to_dst_relation
            .get(&relation_type.key())
            .cloned()
    }
    #[inline]
    pub unsafe fn outgoings(self) -> impl Iterator<Item = &'static args!(RelationPtr)> + 'static {
        self.get().relation_type_to_dst_relation.values() //todo 可能有重复
    }
    #[inline]
    pub unsafe fn incoming(
        self,
        relation_type: args!(RelationTypePtr),
    ) -> Option<&'static BTreeMap<u64, args!(RelationPtr)>> {
        relation_type.get().dst_to_relations.get(&self.key()).map(
            #[inline]
            |x| &*(x as *const _),
        )
    }
    #[inline]
    pub unsafe fn incomings(self) -> impl Iterator<Item = &'static args!(RelationPtr)> + 'static {
        self.get().src_to_relation.values()
    }

    #[inline]
    pub unsafe fn relation(self, dst: args!(ConceptPtr)) -> Option<args!(RelationPtr)> {
        RelationTypePtr::relation(self, dst)
    }
}

impl<ConceptData: 'static, RelationData: 'static, RelationTypeData: 'static>
    RelationTypePtr<ConceptData, RelationData, RelationTypeData>
{
    #[inline]
    pub unsafe fn relations(self) -> impl Iterator<Item = &'static args!(RelationPtr)> + 'static {
        self.get().dst_to_relations.values().flat_map(
            #[inline]
            |x| x.values(),
        )
    }
    #[inline]
    pub unsafe fn relation(
        src: args!(ConceptPtr),
        dst: args!(ConceptPtr),
    ) -> Option<args!(RelationPtr)> {
        dst.get().src_to_relation.get(&src.key()).cloned()
    }

    #[inline]
    pub unsafe fn contains_relation(self, relation: args!(RelationPtr)) -> bool {
        self.contains_relation_key(relation.key())
    }
    #[inline]
    pub unsafe fn contains_relation_key(self, key: u64) -> bool {
        self.get().relations.contains_key(&key)
    }
    #[inline]
    pub unsafe fn get_relation(self, key: u64) -> Option<args!(RelationPtr)> {
        self.get().relations.get(&key).map(
            #[inline]
            |x| RelationPtr::new_from_ref(x),
        )
    }
    #[inline]
    pub unsafe fn relations_iter(self) -> impl Iterator<Item = args!(RelationPtr)> + 'static {
        self.get().relations.values().map(
            #[inline]
            |x| RelationPtr::new_from_ref(x),
        )
    }
    pub unsafe fn create_relation_with_data<'a>(
        self,
        src: args!(ConceptPtr),
        dst_iter: impl Clone + Iterator<Item = &'a args!(ConceptPtr)>,
        data: RelationData,
    ) -> Result<args!(RelationPtr), (args!(RelationPtr), RelationData)>
    where
        RelationData: 'a,
        ConceptData: 'a,
        RelationTypeData: 'a,
    {
        let relation_type_ptr = self.get_mut();

        //申请key
        let key = relation_type_ptr.relations_key_pool.rent();

        //得到类型key
        let relation_type_key = relation_type_ptr.key;

        //判断此连接是否已存在
        match src
            .get_mut()
            .relation_type_to_dst_relation
            .entry(relation_type_key)
        {
            Entry::Vacant(entry) => {
                //创建关系
                let relation_ref = RelationPtr::new_from_ref(
                    relation_type_ptr.relations.entry(key).or_insert(Relation {
                        key,
                        data,
                        relation_type: self,
                        src: ConceptPtr::new_from_ref(src.get()),
                        key_to_dst: dst_iter.clone().map(|x| (x.key(), *x)).collect::<_>(),
                    }),
                );

                //注册关系
                src.get_mut()
                    .relation_type_to_dst_relation
                    .insert(relation_type_key, relation_ref);
                let relation_type_dst_to_relation_ref = &mut relation_type_ptr.dst_to_relations;
                dst_iter.for_each(|dst| {
                    relation_type_dst_to_relation_ref
                        .entry(dst.key())
                        .or_insert_with(BTreeMap::new)
                        .insert(key, relation_ref);
                    dst.get_mut()
                        .src_to_relation
                        .insert(src.key(), relation_ref);
                });

                //封装并返回
                return Ok(relation_ref);
            }
            Entry::Occupied(entry) => {
                //关系已存在，取消创建，归还key
                relation_type_ptr.relations_key_pool.ret(key);
                return Err((*entry.get(), data));
            }
        }
    }
    #[inline]
    pub unsafe fn create_relation<'a>(
        self,
        src: args!(ConceptPtr),
        dst_iter: impl Clone + Iterator<Item = &'a args!(ConceptPtr)>,
    ) -> Result<args!(RelationPtr), args!(RelationPtr)>
    where
        RelationData: Default + 'a,
        ConceptData: 'a,
        RelationTypeData: 'a,
    {
        self.create_relation_with_data(src, dst_iter, Default::default())
            .map_err(|(x, _)| x)
    }
}
impl<ConceptData: 'static, RelationData: 'static, RelationTypeData: 'static>
    RelationPtr<ConceptData, RelationData, RelationTypeData>
{
    pub unsafe fn delete(self) {
        let relation = self.get();
        let relation_key = relation.key;
        relation.key_to_dst.iter().for_each(
            #[inline]
            |(dst_key, dst)| {
                let dst_to_relations = &mut relation.relation_type.get_mut().dst_to_relations;
                let dst_to_relations_relations = &mut dst_to_relations.get_mut(&dst_key).unwrap();
                dst_to_relations_relations.remove(&relation_key);
                if dst_to_relations_relations.is_empty() {
                    dst_to_relations.remove(&dst_key);
                }

                dst.get_mut().src_to_relation.remove(&relation.src.key());
            },
        );
        relation
            .src
            .get_mut()
            .relation_type_to_dst_relation
            .remove(&relation.relation_type.key());
        let relation_type = relation.relation_type.get_mut();
        relation_type.relations.remove(&relation_key);
        relation_type.relations_key_pool.ret(relation_key);
    }
}
