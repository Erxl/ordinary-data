/*
  Copyright (c) 2021 Erxl
  ordinary-data is licensed under Mulan PSL v2.
  You can use this software according to the terms and conditions of the Mulan PSL v2.
  You may obtain a copy of Mulan PSL v2 at:
  http://license.coscl.org.cn/MulanPSL2
  THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
  See the Mulan PSL v2 for more details.
*/
use super::*;

#[test]
fn test_ref_data_create_delet() {
    let mut c = Container::<i32, i128, f64>::new();
    assert_eq!(c.concepts_count(), 0);
    let c1 = c.create_concept_with_data(555);
    assert_eq!(c.concepts_count(), 1);

    //测试ConceptRef
    let c1_copy = c1;
    let c1_copy_copy = c1_copy;
    assert!(c1 == c1_copy_copy);
    assert_eq!(c.concepts_count(), 1);

    unsafe {
        //data测试
        assert_eq!(*c1.data(), 555);

        //移除测试
        c.delete_concept(c1);
        assert_eq!(c.concepts_count(), 0);
    }
}

#[test]
fn test_relations() {
    unsafe {
        let mut c = Container::<(), (), ()>::new();
        assert_eq!(c.concepts_count(), 0);
        assert_eq!(c.relations_count(), 0);
        assert_eq!(c.relation_types_count(), 0);
        let from = c.create_concept();
        let to = c.create_concept();
        let prop = c.create_relation_type();
        assert_eq!(c.concepts_count(), 2);
        assert_eq!(c.relations_count(), 0);
        assert_eq!(c.relation_types_count(), 1);
        let relation = c.create_relation(prop, from, [to].iter());
        assert_eq!(c.concepts_count(), 2);
        assert_eq!(c.relations_count(), 1);
        assert_eq!(c.relation_types_count(), 1);
        c.delete_relation(relation);
        assert_eq!(c.concepts_count(), 2);
        assert_eq!(c.relations_count(), 0);
        assert_eq!(c.relation_types_count(), 1);
        c.delete_concept(from);
        c.delete_concept(to);
        c.delete_relation_type(prop);
        assert_eq!(c.relation_types_count(), 0);
        assert_eq!(c.concepts_count(), 0);
        assert_eq!(c.relations_count(), 0);
    }
}

#[test]
fn test_contains() {
    unsafe {
        let mut c = Container::<i32, i32, i32>::new();
        let c1 = c.create_concept_with_data(111);
        let c2 = c.create_concept_with_data(222);
        let c2_key = c2.key();
        c.delete_concept(c2);

        assert!(c.contains_concept(c1));
        assert!(c.contains_concept_key(c1.key()));
        assert!(!c.contains_concept(c2));
        assert!(!c.contains_concept_key(c2_key));

        let from = c.create_concept_with_data(6);
        let to = c.create_concept_with_data(66);
        let to2 = c.create_concept_with_data(66);
        let kind = c.create_relation_type_with_data(666);

        let r = c.create_relation_with_data(kind, from, [to].iter(), 6666);
        let r2 = c.create_relation_with_data(kind, from, [to2].iter(), 6666);
        let r2_key = r2.key();
        c.delete_relation(r2);

        assert!(c.contains_relation(r));
        assert!(c.contains_relation_key(r.key()));
        assert!(!c.contains_relation(r2));
        assert!(!c.contains_relation_key(r2_key));
    }
}


#[test]
fn test_size() {
    const PTR_SIZE: usize = std::mem::size_of::<*const i32>();
    assert_eq!(std::mem::size_of::<Option<RelationPtr<i32, i32, i32>>>(), PTR_SIZE);
    assert_eq!(std::mem::size_of::<Option<ConceptPtr<i32, i32, i32>>>(), PTR_SIZE);
    assert_eq!(std::mem::size_of::<Option<RelationTypePtr<i32, i32, i32>>>(), PTR_SIZE);
}

#[test]
fn test_iter() {
    unsafe {
        let mut c = Container::<i32, f32, i32>::new();
        let from = c.create_concept_with_data(666); //Six means good luck in China, while five means crying in China
        let to = c.create_concept_with_data(6666);
        let kind = c.create_relation_type_with_data(66666);
        let relation = c.create_relation_with_data(kind, from, [to].iter(), 233.);
        assert!(c.concepts_iter().any(|x| *x.data() == 666 && x == from));
        assert!(c.concepts_iter().any(|x| *x.data() == 6666 && x == to));
        assert!(c.relations_iter().any(|x| *x.data() == 233. && x == relation));
        assert!(c.relation_types_iter().any(|x| *x.data() == 66666 && x == kind));
    }

// #[test]
// fn test_info() {
//     unsafe {
//         let mut c = StdContainer::<Option<Box<dyn std::any::Any>>, ()>::new();
//
//         //创建属性
//         let prop_people = c.create_concept();
//         let prop_age = c.create_concept();
//         let prop_name = c.create_concept();
//         let prop_interest = c.create_concept();
//         let prop_sex = c.create_concept();
//
//         //创建对象
//         let sys = c.create_concept();
//         let person_zhao_shan = c.create_concept();
//         let person_ji_zi_shan = c.create_concept();
//         let person_wang_yu_xuan = c.create_concept();
//         let sex_male = c.create_concept();
//         let sex_female = c.create_concept();
//         let age_16 = c.create_concept_with_data(Some(Box::new(16)));
//         let age_12 = c.create_concept_with_data(Some(Box::new(12)));
//         let age_18 = c.create_concept_with_data(Some(Box::new(15)));
//         let name_zhao_shan = c.create_concept_with_data(Some(Box::new(String::from("赵善"))));
//         let name_ji_zi_shan = c.create_concept_with_data(Some(Box::new(String::from("季子杉"))));
//         let name_wang_yu_xuan = c.create_concept_with_data(Some(Box::new(String::from("王宇轩"))));
//         let interest_rust_developing = c.create_concept_with_data(Some(Box::new(String::from("Rust开发"))));
//         let interest_game_developing = c.create_concept_with_data(Some(Box::new(String::from("游戏开发"))));
//         let interest_ui_disign = c.create_concept_with_data(Some(Box::new(String::from("UI设计"))));
//         let interest_csharp_developing = c.create_concept_with_data(Some(Box::new(String::from("C#开发"))));
//         let interest_minecraft = c.create_concept_with_data(Some(Box::new(String::from("玩Minecraft"))));
//         let interest_dlang_developing = c.create_concept_with_data(Some(Box::new(String::from("D语言开发"))));
//
//         //创建关系
//         c.create_relation(prop_people, sys, [
//             person_zhao_shan,
//             person_ji_zi_shan,
//             person_wang_yu_xuan].iter());
//
//         c.create_relation(prop_sex, person_zhao_shan, [sex_male].iter());
//         c.create_relation(prop_sex, person_wang_yu_xuan, [sex_female].iter());
//         c.create_relation(prop_sex, person_ji_zi_shan, [sex_male].iter());
//
//         c.create_relation(prop_age, person_zhao_shan, [age_12].iter());
//         c.create_relation(prop_age, person_wang_yu_xuan, [age_16].iter());
//         c.create_relation(prop_age, person_ji_zi_shan, [age_18].iter());
//
//         c.create_relation(prop_name, person_zhao_shan, [name_zhao_shan].iter());
//         c.create_relation(prop_name, person_wang_yu_xuan, [name_wang_yu_xuan].iter());
//         c.create_relation(prop_name, person_ji_zi_shan, [name_ji_zi_shan].iter());
//
//         c.create_relation(prop_interest, person_wang_yu_xuan, [
//             interest_csharp_developing,
//             interest_dlang_developing,
//             interest_minecraft].iter());
//         c.create_relation(prop_interest, person_zhao_shan, [
//             interest_rust_developing,
//             interest_game_developing,
//             interest_minecraft].iter());
//         let r = c.create_relation(prop_interest, person_ji_zi_shan, [
//             interest_rust_developing,
//             interest_ui_disign,
//             interest_minecraft].iter());
//
//         assert!(person_ji_zi_shan.relations_out().values().find(|x| x.kind() == prop_sex).unwrap().to().get(&sex_male.key()).unwrap().relations_in().values().find(|x| x.from() == person_ji_zi_shan).unwrap().from().relations_out().get(&r.key()).unwrap().to().values().map(|x| x.data().as_ref().unwrap().downcast_ref::<String>().unwrap()).any(|x| *x == "玩Minecraft"));
//
//     }
// }
//
// #[test]
// fn test_other() {
//     fn aaa(aaa: C) {}
//     fn aaaa(aaa: A) {}
//     struct A {
//         aa: Vec<Box<B>>
//     }
//     struct B {
//         a: i32
//     }
//     struct C<'a> {
//         a: &'a B
//     }
//     impl A {
//         fn get(&self) -> C {
//             unsafe { C { a: &self.aa.get(0).unwrap() } }
//         }
//     }
//     let mut v = Vec::new();
//     v.push(Box::new(B { a: 3434 }));
//     let a = A { aa: v };
//     let b = a.get();
//     //aaaa(a);
//     aaa(b);
//     //println!("{}", b.a.a)
// }

    #[test]
    fn test_test() {
    }
}