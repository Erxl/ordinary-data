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
use std::marker::PhantomData;
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
    #[inline]
    pub fn clear(&mut self) {
        self.0 = 0;
    }
}

pub struct Container<'a, ConceptData: 'a = (), RelationData: 'a = (), RelationTypeData: 'a = ()> {
    concepts_key_pool: KeyPool,
    relationtypes_key_pool: KeyPool,
    concepts: BTreeMap<u64, Box<Concept<'a, ConceptData, RelationData, RelationTypeData>>>,
    //todo 使用ordclct树，避免多次引用造成的性能损失
    relationtypes:
        BTreeMap<u64, Box<RelationType<'a, ConceptData, RelationData, RelationTypeData>>>,
}

pub struct Concept<'a, ConceptData, RelationData, RelationTypeData> {
    key: u64,
    data: ConceptData,
    relationtype_to_dst_relation:
        BTreeMap<u64, RelationPtr<'a, ConceptData, RelationData, RelationTypeData>>,
    src_to_relationtype_relation:
        BTreeMap<u64, BTreeMap<u64, RelationPtr<'a, ConceptData, RelationData, RelationTypeData>>>,
}

pub struct RelationType<'a, ConceptData, RelationData, RelationTypeData> {
    key: u64,
    data: RelationTypeData,
    relations: BTreeMap<u64, Box<Relation<'a, ConceptData, RelationData, RelationTypeData>>>,
    dst_to_relations:
        BTreeMap<u64, BTreeMap<u64, RelationPtr<'a, ConceptData, RelationData, RelationTypeData>>>,
    relations_key_pool: KeyPool,
}

pub struct Relation<'a, ConceptData, RelationData, RelationTypeData> {
    key: u64,
    data: RelationData,
    relationtype: RelationTypePtr<'a, ConceptData, RelationData, RelationTypeData>,
    src: ConceptPtr<'a, ConceptData, RelationData, RelationTypeData>,
    key_to_dst: BTreeMap<u64, ConceptPtr<'a, ConceptData, RelationData, RelationTypeData>>,
}

macro_rules! declare {
    ($ty_ptr:ident,$ty:ident,$ty_data:ident) => {
        #[derive(Debug)]
        pub struct $ty_ptr<'a, ConceptData: 'a, RelationData: 'a, RelationTypeData: 'a>(
            NonNull<$ty<'a, ConceptData, RelationData, RelationTypeData>>,
            PhantomData<&'a ()>,
        );
        impl<'a, ConceptData: 'a, RelationData: 'a, RelationTypeData: 'a> Clone
            for $ty_ptr<'a, ConceptData, RelationData, RelationTypeData>
        {
            #[inline]
            fn clone(&self) -> Self {
                Self(self.0, Default::default())
            }
        }
        impl<'a, ConceptData, RelationData, RelationTypeData> PartialEq
            for $ty_ptr<'a, ConceptData, RelationData, RelationTypeData>
        {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }
        impl<'a, ConceptData, RelationData, RelationTypeData> Eq
            for $ty_ptr<'a, ConceptData, RelationData, RelationTypeData>
        {
        }
        impl<'a, ConceptData, RelationData, RelationTypeData> PartialOrd
            for $ty_ptr<'a, ConceptData, RelationData, RelationTypeData>
        {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }
        impl<'a, ConceptData, RelationData, RelationTypeData> Ord
            for $ty_ptr<'a, ConceptData, RelationData, RelationTypeData>
        {
            #[inline]
            fn cmp(&self, other: &Self) -> Ordering {
                self.0.cmp(&other.0)
            }
        }
        impl<'a, ConceptData, RelationData, RelationTypeData> Copy
            for $ty_ptr<'a, ConceptData, RelationData, RelationTypeData>
        {
        }
        impl<'a, ConceptData, RelationData, RelationTypeData>
            $ty_ptr<'a, ConceptData, RelationData, RelationTypeData>
        {
            #[inline]
            pub unsafe fn data(&self) -> &'a $ty_data {
                &(*self.0.as_ptr()).data
            }
            #[inline]
            pub unsafe fn data_mut(&self) -> &'a mut $ty_data {
                &mut (*self.as_ptr()).data
            }
            #[inline]
            pub unsafe fn key(&self) -> u64 {
                self.0.as_ref().key
            }
            #[inline]
            #[allow(dead_code)]
            unsafe fn get_mut(
                &self,
            ) -> &'a mut $ty<'a, ConceptData, RelationData, RelationTypeData> {
                &mut *self.as_ptr()
            }
            #[inline]
            #[allow(dead_code)]
            unsafe fn get(&self) -> &'a $ty<'a, ConceptData, RelationData, RelationTypeData> {
                &(*self.0.as_ptr())
            }
            #[inline]
            #[allow(dead_code)]
            fn as_ptr(&self) -> *mut $ty<'a, ConceptData, RelationData, RelationTypeData> {
                self.0.as_ptr()
            }
            #[inline]
            #[allow(dead_code)]
            fn new_from_ptr(
                ptr: *const $ty<'a, ConceptData, RelationData, RelationTypeData>,
            ) -> $ty_ptr<'a, ConceptData, RelationData, RelationTypeData> {
                unsafe { Self(NonNull::new_unchecked(ptr as _), Default::default()) }
            }
            #[inline]
            #[allow(dead_code)]
            fn new_from_ref(
                x: &$ty<'a, ConceptData, RelationData, RelationTypeData>,
            ) -> $ty_ptr<'a, ConceptData, RelationData, RelationTypeData> {
                unsafe {
                    Self(
                        NonNull::new_unchecked(x as *const $ty<_, _, _> as _),
                        Default::default(),
                    )
                }
            }
        }
    };
}

declare!(ConceptPtr, Concept, ConceptData);
declare!(RelationPtr, Relation, RelationData);
declare!(RelationTypePtr, RelationType, RelationTypeData);

//——————————————————————————————————————————————————————实现—————————————————————————————————————————
impl<'a, ConceptData, RelationData, RelationTypeData>
    Container<'a, ConceptData, RelationData, RelationTypeData>
{
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
    //——————————————————————————————————————————————————————增删—————————————————————————————————————
    #[inline]
    pub fn create_concept(&mut self) -> ConceptPtr<'a, ConceptData, RelationData, RelationTypeData>
    where
        ConceptData: Default,
    {
        self.create_concept_with_data(Default::default())
    }
    pub fn create_concept_with_data(
        &mut self,
        data: ConceptData,
    ) -> ConceptPtr<'a, ConceptData, RelationData, RelationTypeData> {
        let key = self.concepts_key_pool.rent();

        ConceptPtr::new_from_ref(&**self.concepts.entry(key).or_insert(Box::new(Concept {
            key,
            data,
            relationtype_to_dst_relation: Default::default(),
            src_to_relationtype_relation: Default::default(),
        })))
    }

    #[inline]
    pub unsafe fn delete_concept(
        &mut self,
        concept: ConceptPtr<'a, ConceptData, RelationData, RelationTypeData>,
    ) {
        self.delete_concept_key(concept.key());
    }

    pub fn delete_concept_key(&mut self, key: u64) -> bool {
        match self.concepts.remove(&key) {
            Some(c) => {
                self.concepts_key_pool.ret(key);
                unsafe {
                    //todo 可优化;
                    c.relationtype_to_dst_relation //移除接出的关系
                        .values()
                        .collect::<Vec<_>>()
                        .into_iter()
                        .for_each(|x| x.delete());
                    c.src_to_relationtype_relation
                        .values()
                        .map(|x| x.values())
                        .flatten()
                        .collect::<Vec<_>>()
                        .into_iter()
                        .for_each(|x| x.delete());
                }
                true
            }
            None => false,
        };

        true
    }

    #[inline]
    pub fn create_relationtype(
        &mut self,
    ) -> RelationTypePtr<'a, ConceptData, RelationData, RelationTypeData>
    where
        RelationTypeData: Default,
    {
        self.create_relationtype_with_data(Default::default())
    }
    pub fn create_relationtype_with_data(
        &mut self,
        data: RelationTypeData,
    ) -> RelationTypePtr<'a, ConceptData, RelationData, RelationTypeData> {
        //申请key
        let key = self.relationtypes_key_pool.rent();

        //创建
        RelationTypePtr::new_from_ref(&**self.relationtypes.entry(key).or_insert(Box::new(
            RelationType {
                key,
                data,
                dst_to_relations: Default::default(),
                relations: Default::default(),
                relations_key_pool: Default::default(),
            },
        )))
    }

    pub unsafe fn delete_relationtype(
        &mut self,
        relationtype: RelationTypePtr<'a, ConceptData, RelationData, RelationTypeData>,
    ) {
        let relationtype_ref = relationtype.get_mut();
        let relationtype_key = relationtype_ref.key;
        //todo 可优化;
        relationtype_ref.relations.values_mut().for_each(
            #[inline]
            |y| y.delete(),
        );

        self.relationtypes.remove(&relationtype_ref.key);
        self.relationtypes_key_pool.ret(relationtype_key);
    }

    //——————————————————————————————————————————————————————检索—————————————————————————————————————
    #[inline]
    pub fn relations_count(&self) -> usize {
        self.relationtypes.values().map(|x| x.relations.len()).sum()
    }
    #[inline]
    pub fn concepts_count(&self) -> usize {
        self.concepts.len()
    }
    #[inline]
    pub fn relationtypes_count(&self) -> usize {
        self.relationtypes.len()
    }
    //
    #[inline]
    pub unsafe fn contains_concept(
        &mut self,
        concept: ConceptPtr<'a, ConceptData, RelationData, RelationTypeData>,
    ) -> bool {
        self.contains_concept_key(concept.key())
    }
    #[inline]
    pub unsafe fn contains_concept_key(&mut self, key: u64) -> bool {
        self.concepts.contains_key(&key)
    }
    #[inline]
    pub unsafe fn get_concept(
        &mut self,
        key: u64,
    ) -> Option<ConceptPtr<'a, ConceptData, RelationData, RelationTypeData>> {
        self.concepts.get(&key).map(
            #[inline]
            |x| ConceptPtr::new_from_ref(x),
        )
    }

    #[inline]
    pub unsafe fn contains_relationtype(
        &mut self,
        relationtype: RelationTypePtr<'a, ConceptData, RelationData, RelationTypeData>,
    ) -> bool {
        self.contains_relationtype_key(relationtype.key())
    }
    #[inline]
    pub unsafe fn contains_relationtype_key(&mut self, key: u64) -> bool {
        self.relationtypes.contains_key(&key)
    }
    #[inline]
    pub fn get_relationtype(
        &mut self,
        key: u64,
    ) -> Option<RelationTypePtr<'a, ConceptData, RelationData, RelationTypeData>> {
        self.relationtypes.get(&key).map(
            #[inline]
            |x| RelationTypePtr::new_from_ref(x),
        )
    }
    #[inline]
    pub fn concepts_iter(
        &self,
    ) -> impl Iterator<Item = ConceptPtr<'a, ConceptData, RelationData, RelationTypeData>> + '_
    {
        self.concepts.values().map(
            #[inline]
            |x| ConceptPtr::new_from_ref(x),
        )
    }
    #[inline]
    pub fn relations_iter(
        &self,
    ) -> impl Iterator<Item = RelationPtr<'a, ConceptData, RelationData, RelationTypeData>> + '_
    {
        self.relationtypes
            .values()
            .flat_map(|x| x.relations.values())
            .map(std::ops::Deref::deref)
            .map(RelationPtr::new_from_ref)
    }
    #[inline]
    pub fn relationtypes_iter(
        &self,
    ) -> impl Iterator<Item = RelationTypePtr<'a, ConceptData, RelationData, RelationTypeData>> + '_
    {
        self.relationtypes.values().map(
            #[inline]
            |x| RelationTypePtr::new_from_ref(x),
        )
    }

    pub fn clear(&mut self) {
        self.concepts.clear();
        self.concepts_key_pool.clear();
        self.relationtypes.clear();
        self.relationtypes_key_pool.clear();
    }
}

impl<'a, ConceptData, RelationData, RelationTypeData> Default
    for Container<'a, ConceptData, RelationData, RelationTypeData>
{
    fn default() -> Self {
        Self {
            concepts_key_pool: Default::default(),
            relationtypes_key_pool: Default::default(),
            concepts: Default::default(),
            relationtypes: Default::default(),
        }
    }
}

//todo 实现原始迭代器的所有功能
impl<'a, ConceptData: 'a, RelationData: 'a, RelationTypeData: 'a>
    ConceptPtr<'a, ConceptData, RelationData, RelationTypeData>
{
    #[inline]
    pub unsafe fn outgoing(
        self,
        relationtype: RelationTypePtr<'a, ConceptData, RelationData, RelationTypeData>,
    ) -> Option<RelationPtr<'a, ConceptData, RelationData, RelationTypeData>> {
        self.get()
            .relationtype_to_dst_relation
            .get(&relationtype.key())
            .cloned()
    }
    #[inline]
    pub unsafe fn outgoings(
        self,
    ) -> impl Iterator<Item = &'a RelationPtr<'a, ConceptData, RelationData, RelationTypeData>> + 'a
    {
        self.get().relationtype_to_dst_relation.values()
    }
    #[inline]
    pub unsafe fn incomings(
        self,
        relationtype: RelationTypePtr<ConceptData, RelationData, RelationTypeData>,
    ) -> Option<&'a BTreeMap<u64, RelationPtr<ConceptData, RelationData, RelationTypeData>>> {
        relationtype.get().dst_to_relations.get(&self.key()).map(
            #[inline]
            |x| &*(x as *const _),
        )
    }
    #[inline]
    pub unsafe fn incoming_relationtypes(
        self,
    ) -> impl Iterator<
        Item = &'a BTreeMap<u64, RelationPtr<'a, ConceptData, RelationData, RelationTypeData>>,
    > + 'a {
        self.get().src_to_relationtype_relation.values()
    }
    #[inline]
    pub unsafe fn incomings_all(
        self,
    ) -> impl Iterator<Item = &'a RelationPtr<'a, ConceptData, RelationData, RelationTypeData>> + 'a
    {
        self.incoming_relationtypes().map(|x| x.values()).flatten()
    }
    #[inline]
    pub unsafe fn relations_relationtype(
        self,
        dst: ConceptPtr<ConceptData, RelationData, RelationTypeData>,
    ) -> Option<&'a BTreeMap<u64, RelationPtr<ConceptData, RelationData, RelationTypeData>>> {
        dst.get().src_to_relationtype_relation.get(&self.key())
    }
    #[inline]
    pub unsafe fn relations(
        self,
        dst: ConceptPtr<'a, ConceptData, RelationData, RelationTypeData>,
    ) -> Option<impl Iterator<Item = RelationPtr<'a, ConceptData, RelationData, RelationTypeData>>>
    {
        self.relations_relationtype(dst)
            .map(|x| x.values().cloned())
    }
}

impl<'a, ConceptData, RelationData, RelationTypeData>
    RelationTypePtr<'a, ConceptData, RelationData, RelationTypeData>
{
    #[inline]
    pub unsafe fn relations(
        self,
    ) -> impl Iterator<Item = &'a RelationPtr<'a, ConceptData, RelationData, RelationTypeData>> + 'a
    {
        self.get().dst_to_relations.values().flat_map(
            #[inline]
            |x| x.values(),
        )
    }
    #[inline]
    pub unsafe fn contains_relation(
        self,
        relation: RelationPtr<ConceptData, RelationData, RelationTypeData>,
    ) -> bool {
        self.contains_relation_key(relation.key())
    }
    #[inline]
    pub unsafe fn contains_relation_key(self, key: u64) -> bool {
        self.get().relations.contains_key(&key)
    }
    #[inline]
    pub unsafe fn get_relation(
        self,
        key: u64,
    ) -> Option<RelationPtr<'a, ConceptData, RelationData, RelationTypeData>> {
        self.get().relations.get(&key).map(
            #[inline]
            |x| RelationPtr::new_from_ref(x),
        )
    }
    #[inline]
    pub unsafe fn relations_iter(
        self,
    ) -> impl Iterator<Item = RelationPtr<'a, ConceptData, RelationData, RelationTypeData>> + 'a
    {
        self.get().relations.values().map(
            #[inline]
            |x| RelationPtr::new_from_ref(x),
        )
    }
    pub unsafe fn create_relation_with_data(
        self,
        src: ConceptPtr<'a, ConceptData, RelationData, RelationTypeData>,
        data: RelationData,
    ) -> Result<
        RelationPtr<'a, ConceptData, RelationData, RelationTypeData>,
        (
            RelationPtr<'a, ConceptData, RelationData, RelationTypeData>,
            RelationData,
        ),
    > {
        let relationtype_ptr = self.get_mut();

        //申请key
        let key = relationtype_ptr.relations_key_pool.rent();

        //得到类型key
        let relationtype_key = relationtype_ptr.key;

        //判断此连接是否已存在
        match src
            .get_mut()
            .relationtype_to_dst_relation
            .entry(relationtype_key)
        {
            Entry::Vacant(entry) => {
                //创建关系
                let relation_ref =
                    RelationPtr::new_from_ref(&**relationtype_ptr.relations.entry(key).or_insert(
                        Box::new(Relation {
                            key,
                            data,
                            relationtype: self,
                            src: ConceptPtr::new_from_ref(src.get()),
                            key_to_dst: Default::default(),
                        }),
                    ));

                //注册关系
                entry.insert(relation_ref);

                //封装并返回
                return Ok(relation_ref);
            }
            Entry::Occupied(entry) => {
                //关系已存在，取消创建，归还key
                relationtype_ptr.relations_key_pool.ret(key);
                return Err((*entry.get(), data));
            }
        }
    }
    #[inline]
    pub unsafe fn create_relation(
        self,
        src: ConceptPtr<'a, ConceptData, RelationData, RelationTypeData>,
    ) -> Result<
        RelationPtr<'a, ConceptData, RelationData, RelationTypeData>,
        RelationPtr<'a, ConceptData, RelationData, RelationTypeData>,
    >
    where
        RelationData: Default + 'a,
        ConceptData: 'a,
        RelationTypeData: 'a,
    {
        self.create_relation_with_data(src, Default::default())
            .map_err(|(x, _)| x)
    }
}
impl<'a, ConceptData, RelationData, RelationTypeData>
    Relation<'a, ConceptData, RelationData, RelationTypeData>
{
    unsafe fn delete(&mut self) {
        let relation = self;
        let relation_key = relation.key;
        relation.key_to_dst.iter().for_each(
            #[inline]
            |(dst_key, dst)| {
                let dst_to_relations = &mut relation.relationtype.get_mut().dst_to_relations;
                let dst_to_relations_relations = &mut dst_to_relations.get_mut(&dst_key).unwrap();
                dst_to_relations_relations.remove(&relation_key);
                if dst_to_relations_relations.is_empty() {
                    dst_to_relations.remove(&dst_key);
                }

                dst.get_mut()
                    .src_to_relationtype_relation
                    .remove(&relation.src.key());
            },
        );
        relation
            .src
            .get_mut()
            .relationtype_to_dst_relation
            .remove(&relation.relationtype.key());
        let relationtype = relation.relationtype.get_mut();
        relationtype.relations.remove(&relation_key);
        relationtype.relations_key_pool.ret(relation_key);
    }
}

impl<'a, ConceptData, RelationData, RelationTypeData>
    RelationPtr<'a, ConceptData, RelationData, RelationTypeData>
{
    #[inline]
    pub unsafe fn delete(self) {
        self.get_mut().delete();
    }
    pub unsafe fn add_concept(
        self,
        dst: ConceptPtr<'a, ConceptData, RelationData, RelationTypeData>,
    ) -> bool {
        let rel = self.get_mut();
        let dst_key = dst.key();

        match rel.key_to_dst.entry(dst_key) {
            //可加入
            Entry::Vacant(entry) => {
                rel.relationtype
                    .get_mut()
                    .dst_to_relations
                    .entry(dst_key) //如果ty还没有给dst身上接入关联，就新建一组
                    .or_insert_with(BTreeMap::new)
                    .insert(rel.key, self);
                dst.get_mut()
                    .src_to_relationtype_relation
                    .entry(rel.src.key())
                    .or_insert_with(BTreeMap::new)
                    .insert(rel.relationtype.key(), self);
                entry.insert(dst);
                return true;
            }
            Entry::Occupied(_entry) => {
                return false;
            }
        }
    }
    #[inline]
    pub unsafe fn remove_concept(
        self,
        dst: ConceptPtr<'a, ConceptData, RelationData, RelationTypeData>,
    ) -> bool {
        self.remove_concept_key(dst.key())
    }
    pub fn remove_concept_key(self, dst_key: u64) -> bool {
        let rel = unsafe { self.get_mut() };
        match rel.key_to_dst.remove(&dst_key) {
            //查无此人，无法移除
            None => {
                return false;
            }
            Some(dst) => {
                let dst_key = unsafe { dst.key() };

                let dst_to_relations = unsafe { &mut rel.relationtype.get_mut().dst_to_relations };
                let relations = unsafe { dst_to_relations.get_mut(&dst_key).unwrap_unchecked() };
                if relations.len() == 1 {
                    //如果只剩一个，就带着树整个删了，节省一次删除
                    dst_to_relations.remove(&dst_key);
                } else {
                    relations.remove(&rel.key);
                }
                unsafe {
                    dst.get_mut()
                        .src_to_relationtype_relation
                        .remove(&rel.src.key());
                }
                return true;
            }
        }
    }
    #[inline]
    pub unsafe fn destinations(
        self,
    ) -> &'static BTreeMap<u64, ConceptPtr<'a, ConceptData, RelationData, RelationTypeData>> {
        &self.get().key_to_dst
    }
    #[inline]
    pub unsafe fn source(self) -> ConceptPtr<'a, ConceptData, RelationData, RelationTypeData> {
        self.get().src
    }
}

//todo 增加更多快捷函数，以快速组织网络
//todo trait化，为领域特定的系统提供优化空间，提供操作容器的包装等，以用于切片编程
//todo 检查btree中对象是否会移动，若移动则使用box封装
