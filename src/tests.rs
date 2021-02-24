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
use std::any::Any;

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
        assert_eq!(c.relationtypes_count(), 0);
        let from = c.create_concept();
        let to = c.create_concept();
        let prop = c.create_relationtype();
        assert_eq!(c.concepts_count(), 2);
        assert_eq!(c.relations_count(), 0);
        assert_eq!(c.relationtypes_count(), 1);
        let relation = prop.create_relation(from).unwrap();
        assert!(relation.add_concept(to));
        assert_eq!(c.concepts_count(), 2);
        assert_eq!(c.relations_count(), 1);
        assert_eq!(c.relationtypes_count(), 1);
        relation.delete();
        assert_eq!(c.concepts_count(), 2);
        assert_eq!(c.relations_count(), 0);
        assert_eq!(c.relationtypes_count(), 1);
        c.delete_concept(from);
        c.delete_concept(to);
        c.delete_relationtype(prop);
        assert_eq!(c.relationtypes_count(), 0);
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
        //let to2 = c.create_concept_with_data(66);
        let kind = c.create_relationtype_with_data(666);

        let r = kind.create_relation_with_data(from, 6666).unwrap();
        assert!(r.add_concept(to));
        //c
        //  .create_relation_with_data(kind, from, [to2].iter(), 6666)
        //  .unwrap_unchecked();
        //let r2_key = r2.key();
        //c.delete_relation(r2);
        //
        assert!(kind.contains_relation(r));
        assert!(kind.contains_relation_key(r.key()));
        //// assert!(!kind.contains_relation(r2));
        //assert!(!kind.contains_relation_key(r2_key));
    }
}

#[test]
fn test_size() {
    const PTR_SIZE: usize = std::mem::size_of::<*const i32>();
    assert_eq!(
        std::mem::size_of::<Option<RelationPtr<i32, i32, i32>>>(),
        PTR_SIZE
    );
    assert_eq!(
        std::mem::size_of::<Option<ConceptPtr<i32, i32, i32>>>(),
        PTR_SIZE
    );
    assert_eq!(
        std::mem::size_of::<Option<RelationTypePtr<i32, i32, i32>>>(),
        PTR_SIZE
    );
}

#[test]
fn test_iter() {
    unsafe {
        let mut c = Container::<i32, f32, i32>::new();
        let from = c.create_concept_with_data(666); //Six means good luck in China, while five means crying in China
        let to = c.create_concept_with_data(6666);
        let kind = c.create_relationtype_with_data(66666);
        let relation = kind.create_relation_with_data(from, 233.).unwrap();
        assert!(relation.add_concept(to));
        assert!(c.concepts_iter().any(|x| *x.data() == 666 && x == from));
        assert!(c.concepts_iter().any(|x| *x.data() == 6666 && x == to));
        assert!(c
            .relations_iter()
            .any(|x| *x.data() == 233. && x == relation));
        assert!(c
            .relationtypes_iter()
            .any(|x| *x.data() == 66666 && x == kind));
    }
}

#[test]
fn test_accessing() {
    use std::rc::*;

    let names = unsafe {
        //内存泄漏检测
        let fr_name = Rc::new("fr");
        let ty_name = Rc::new("ty");
        let ty2_name = Rc::new("ty2");
        let to_name = Rc::new("to");
        let rl_name = Rc::new("fr->rl(ty)->[to]");
        let rl2_name = Rc::new("fr->rl2(ty2)->[to]");
        let rl_inv_name = Rc::new(666);
        let rl2_inv_name = Rc::new(666);
        let fr2_name = Rc::new(666);
        let to2_name = Rc::new(666);
        let del_name = Rc::new(666);
        let del1_name = Rc::new(666);
        let del2_name = Rc::new(666);
        let del3_name = Rc::new("666".to_string());
        let del4_name = Rc::new(666);
        let del5_name = Rc::new(666);
        let fr2_rl_name = Rc::new(666);

        let mut c =
            Container::<Option<Rc<dyn Any>>, Option<Rc<dyn Any>>, Option<Rc<dyn Any>>>::new();
        let fr = c.create_concept_with_data(Some(fr_name.clone()));
        let ty = c.create_relationtype_with_data(Some(ty_name.clone()));
        let ty2 = c.create_relationtype_with_data(Some(ty2_name.clone()));
        let to = c.create_concept_with_data(Some(to_name.clone()));
        //正向连接
        let rl = ty
            .create_relation_with_data(fr, Some(rl_name.clone()))
            .unwrap();
        assert!(rl.add_concept(to));
        assert!(ty.create_relation(fr).unwrap_err() == rl);
        let rl2 = ty2
            .create_relation_with_data(fr, Some(rl2_name.clone()))
            .unwrap();
        assert!(rl2.add_concept(to));
        assert!(ty2.create_relation(fr,).unwrap_err() == rl2);

        //反向连接
        let rl_inv = ty
            .create_relation_with_data(to, Some(rl_inv_name.clone()))
            .unwrap();
        assert!(rl_inv.add_concept(fr));
        assert!(ty.create_relation(to).unwrap_err() == rl_inv);
        let rl2_inv = ty2
            .create_relation_with_data(to, Some(rl2_inv_name.clone()))
            .unwrap();
        assert!(rl2_inv.add_concept(fr));
        assert!(ty2.create_relation(to).unwrap_err() == rl2_inv);
        assert!(!rl2_inv.add_concept(fr));

        //多连接测试
        let to2 = c.create_concept_with_data(Some(fr2_name.clone()));
        let fr2 = c.create_concept_with_data(Some(to2_name.clone()));
        assert!(rl.add_concept(to2));
        assert!(!rl.add_concept(to2));
        let fr2_rl = ty
            .create_relation_with_data(fr2, Some(fr2_rl_name.clone()))
            .unwrap();
        assert!(fr2_rl.add_concept(to));
        assert!(!fr2_rl.add_concept(to));
        assert_eq!(*rl.destinations().get(&to2.key()).unwrap(), to2);
        assert_eq!(rl.destinations().len(), 2);
        assert_eq!(rl.source(), fr);
        assert_eq!(*fr2_rl.destinations().get(&to.key()).unwrap(), to);
        assert_eq!(fr2_rl.destinations().len(), 1);
        assert_eq!(fr2_rl.source(), fr2);

        //删除测试
        assert!(rl.remove_concept(to2));
        // let del0 = c.create_concept_with_data(Some(del1_name.clone()));
        // let del1 = c.create_concept_with_data(Some(del2_name.clone()));
        // let del2 = c.create_relationtype_with_data(Some(del4_name.clone()));
        // let del4=del3.create_relation_with_data(del1,Some(del_name.clone());

        //这属于在fr的ty连接上再增加一个连接，一个源概念只能有一个同种类型的连接，这里不能用create方法
        let err = ty
            .create_relation_with_data(fr, Some(del5_name.clone()))
            .unwrap_err();
        assert!(rl.add_concept(to2));
        assert!(!rl.add_concept(to2));

        //取数据测试
        assert!(err.0 == rl);
        assert!(err.1.as_ref().unwrap().downcast_ref::<i32>().unwrap() == &666);

        //let rl3 =
        //let rl4 = c.create_relation_with_data(ty, fr2, [to].iter(), Some(Box::new(2))).unwrap_unchecked();

        //正连接测试
        assert!(fr.outgoing(ty).unwrap() == rl);
        assert!(fr.outgoing(ty2).unwrap() == rl2);
        assert_eq!(fr.outgoings().count(), 2);
        assert_eq!(fr.outgoings().filter(|x| **x == rl).count(), 1);
        assert_eq!(fr.outgoings().filter(|x| **x == rl2).count(), 1);
        assert_eq!(to.incomings(ty).unwrap().len(), 2);
        assert!(*to.incomings(ty).unwrap().values().next().unwrap() == rl);
        assert_eq!(to.incomings(ty2).unwrap().len(), 1);
        assert!(*to.incomings(ty2).unwrap().values().next().unwrap() == rl2);
        assert_eq!(to.incomings_all().count(), 3);
        assert_eq!(to.incomings_all().filter(|x| **x == rl).count(), 1);
        assert_eq!(to.incomings_all().filter(|x| **x == rl2).count(), 1);
        assert_eq!(fr.relations(to).unwrap().count(), 2);
        assert!(fr.relations(to).unwrap().next().unwrap() == rl);

        //反连接测试
        assert!(to.outgoing(ty).unwrap() == rl_inv);
        assert!(to.outgoing(ty2).unwrap() == rl2_inv);
        assert_eq!(to.outgoings().count(), 2);
        assert_eq!(to.outgoings().filter(|x| **x == rl_inv).count(), 1);
        assert_eq!(to.outgoings().filter(|x| **x == rl2_inv).count(), 1);
        assert_eq!(fr.incomings(ty).unwrap().len(), 1);
        assert!(*fr.incomings(ty).unwrap().values().next().unwrap() == rl_inv);
        assert_eq!(fr.incomings(ty2).unwrap().len(), 1);
        assert!(*fr.incomings(ty2).unwrap().values().next().unwrap() == rl2_inv);
        assert_eq!(fr.incomings_all().count(), 2);
        assert_eq!(fr.incomings_all().filter(|x| **x == rl_inv).count(), 1);
        assert_eq!(fr.incomings_all().filter(|x| **x == rl2_inv).count(), 1);
        assert_eq!(to.relations(fr).unwrap().count(), 2);
        assert!(to.relations(fr).unwrap().next().unwrap() == rl_inv);

        let names: [Rc<dyn Any>; 17] = [
            fr_name,
            ty_name,
            ty2_name,
            to_name,
            rl_name,
            rl2_name,
            rl_inv_name,
            rl2_inv_name,
            fr2_name,
            to2_name,
            del_name,
            del1_name,
            del2_name,
            del3_name,
            del4_name,
            del5_name,
            fr2_rl_name,
        ];
        names.iter().map(|x| Rc::downgrade(x)).collect::<Vec<_>>()
    };
    names.iter().for_each(|x| assert!(x.strong_count() == 0));
}

#[test]
fn test_moves() {
    //失败则证明对象在BTree的Node上面是移动的，故应将对象以Box形式存储在Node上，此错误为我这个小废物对rust标准库的不熟悉导致
    //使用Box存储后果然不报错了
    unsafe {
        let mut c = Container::<i32, (), ()>::new();
        let fr = c.create_concept();
        let mut tos = (0..100)
            .map(|i| (i, c.create_concept_with_data(i)))
            .collect::<Vec<_>>();
        (50..70).for_each(|i| {
            c.delete_concept(tos.remove(i).1);
            //c.delete_concept_key(i);
        });
        let kd = c.create_relationtype();
        let _rl = kd.create_relation(fr).unwrap();
        tos.iter().for_each(|(i, x)| {
            assert!(x.data() == i);
        })
    }
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
    //BTreeMap::entry(&mut self, key).or_default()
    println!("{}", 33);
}
