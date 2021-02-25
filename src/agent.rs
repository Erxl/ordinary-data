/*
  Copyright (c) 2021 Erxl
  ordinary-data is licensed under Mulan PSL v2.
  You can use this software according to the terms and conditions of the Mulan PSL v2.
  You may obtain a copy of Mulan PSL v2 at:
  http://license.coscl.org.cn/MulanPSL2
  THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
  See the Mulan PSL v2 for more details.
*/
use crate::*;
use std::cmp::Ordering;
use std::collections::*;
macro_rules! declare {
    ($ty_agent_ptr:ident,$ty_ptr:ident,$ty_data:ident) => {
        #[derive(Debug)]
        pub struct $ty_agent_ptr<'a, C: 'a, R: 'a, T: 'a>($ty_ptr<'a, C, R, T>);
        impl<'a, C, R, T> Clone for $ty_agent_ptr<'a, C, R, T> {
            #[inline]
            fn clone(&self) -> Self {
                Self(self.0)
            }
        }
        impl<'a, C, R, T> PartialEq for $ty_agent_ptr<'a, C, R, T> {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }
        impl<'a, C, R, T> Eq for $ty_agent_ptr<'a, C, R, T> {}
        impl<'a, C, R, T> PartialOrd for $ty_agent_ptr<'a, C, R, T> {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }
        impl<'a, C, R, T> Ord for $ty_agent_ptr<'a, C, R, T> {
            #[inline]
            fn cmp(&self, other: &Self) -> Ordering {
                self.0.cmp(&other.0)
            }
        }
        impl<'a, C, R, T> Copy for $ty_agent_ptr<'a, C, R, T> {}
        impl<'a, C, R, T> $ty_agent_ptr<'a, C, R, T> {}
    };
}

declare!(ConceptAgentPtr, ConceptPtr, C);
declare!(RelationAgentPtr, RelationPtr, R);
declare!(RelationTypeAgentPtr, RelationTypePtr, T);

trait ContainerAgent<'a, C, R, T> {
    fn create_concept(&mut self) -> Option<ConceptAgentPtr<'a, C, R, T>>;
    fn create_concept_with_data(&mut self, data: C) -> Option<ConceptAgentPtr<'a, C, R, T>>;
    unsafe fn delete_concept(&mut self, concept: ConceptAgentPtr<'a, C, R, T>) -> bool;
    fn create_relationtype(&mut self) -> Option<RelationTypeAgentPtr<'a, C, R, T>>;
    fn create_relationtype_with_data(
        &mut self,
        data: T,
    ) -> Option<RelationTypeAgentPtr<'a, C, R, T>>;
    unsafe fn delete_relationtype(
        &mut self,
        relationtype: RelationTypeAgentPtr<'a, C, R, T>,
    ) -> bool;
    unsafe fn create_relation(
        &mut self,
        relationtype: RelationTypeAgentPtr<'a, C, R, T>,
    ) -> Option<RelationAgentPtr<'a, C, R, T>>;
    unsafe fn create_relation_with_data(
        &mut self,
        relationtype: RelationTypeAgentPtr<'a, C, R, T>,
        data: R,
    ) -> Option<RelationAgentPtr<'a, C, R, T>>;
    unsafe fn delete_relation(&mut self, relation: RelationAgentPtr<'a, C, R, T>) -> bool;
    unsafe fn add_concept(
        &mut self,
        relation: RelationAgentPtr<'a, C, R, T>,
        concept: ConceptAgentPtr<'a, C, R, T>,
    ) -> bool;
    unsafe fn remove_concept(
        &mut self,
        relation: RelationAgentPtr<'a, C, R, T>,
        concept: ConceptAgentPtr<'a, C, R, T>,
    ) -> bool;
    fn concepts_iter(&self) -> dyn Iterator<Item = ConceptAgentPtr<'a, C, R, T>> + '_;

    fn relations_iter(&self) -> dyn Iterator<Item = RelationAgentPtr<'a, C, R, T>> + '_;
}

pub struct DefaultContainerAgent<'a, C, R, T>(&'a ConceptAgentPtr<'a, C, R, T>);
pub struct DefaultContainerAgentMut<'a, C, R, T>(&'a mut ConceptAgentPtr<'a, C, R, T>);
