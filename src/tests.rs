use super::*;

#[test]
fn test_ref_data_create_delet() {
    let mut c = Container::<i32, i32>::new();
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
        let mut c = Container::<(), ()>::new();
        assert_eq!(c.concepts_count(), 0);
        assert_eq!(c.relations_count(), 0);
        let from = c.create_concept();
        let to = c.create_concept();
        let kind = c.create_concept();
        assert_eq!(c.concepts_count(), 3);
        assert_eq!(c.relations_count(), 0);
        let relation = c.relate(kind, from, to);
        assert_eq!(c.concepts_count(), 3);
        assert_eq!(c.relations_count(), 1);
        c.disrelate(relation);
        assert_eq!(c.concepts_count(), 3);
        assert_eq!(c.relations_count(), 0);
        c.delete_concept(from);
        c.delete_concept(to);
        c.delete_concept(kind);
        assert_eq!(c.concepts_count(), 0);
        assert_eq!(c.relations_count(), 0);
    }
}

#[test]
fn test_contains() {
    unsafe {
        let mut c = Container::<i32, i32>::new();
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
        let kind = c.create_concept_with_data(666);

        let r = c.relate_with_data(kind, from, to, 6666);
        let r2 = c.relate_with_data(kind, from, to2, 6666);
        let r2_key = r2.key();
        c.disrelate(r2);

        assert!(c.contains_relation(r));
        assert!(c.contains_relation_key(r.key()));
        assert!(!c.contains_relation(r2));
        assert!(!c.contains_relation_key(r2_key));
    }
}

#[test]
fn test_size() {
    assert_eq!(std::mem::size_of::<Option<RelationRef<i32, i32>>>(), std::mem::size_of::<*const i32>());
    assert_eq!(std::mem::size_of::<Option<ConceptRef<i32, i32>>>(), std::mem::size_of::<*const i32>());
}

#[test]
fn test_iter() {
    let mut c = Container::<i32, f32>::new();
    let _ = c.create_concept_with_data(666);//Six means good luck in China, while five means crying in China
    let _ = c.create_concept_with_data(6666);
    let _ = c.create_concept_with_data(66666);

    //let aaa = Vec::<i32>::new().iter().
    //let _ = c.iter();
}


#[test]
fn test_other() {
    fn aaa(aaa: C) {}
    fn aaaa(aaa: A) {}
    struct A {
        aa: Vec<Box<B>>
    }
    struct B {
        a: i32
    }
    struct C<'a> {
        a: &'a B
    }
    impl A {
        fn get(&self) -> C {
            unsafe { C { a: &self.aa.get(0).unwrap() } }
        }
    }
    let mut v = Vec::new();
    v.push(Box::new(B { a: 3434 }));
    let a = A { aa: v };
    let b = a.get();
    //aaaa(a);
    aaa(b);
    //println!("{}", b.a.a)
}