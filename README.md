# ordinary-data 平凡的数据模型
网结构数据模型，用*BTreeMap*实现的，主要操作都优化成了O(Log n)

## 基本类型
- *Container* 容器
- *ConceptPtr* 概念
- *RelationPtr* 关系
- *RelationTypePtr* 关系类型
- 没了
  
## 为啥是Ptr
库的部分内容针对性能设计，不会对引用的访问进行检查，请慎重使用

## 教程
自己看代码，也没几个类，这傻瓜式api，你肯定能看懂
（其实是懒得写教程，不过说实话你真能通过代码看懂api是怎么用的）

```rust
#[test]
fn test_basicdemo() {
    let mut container = Container::<&str, (), &str>::new();

    //创建“朋友名单”
    let friendslist = container.create_concept_with_data("朋友名单");

    //创建“内容”属性
    let contents = container.create_relationtype_with_data("内容");

    //为“朋友名单”添加“内容”属性
    let friendslist_contents = unsafe { contents.create_relation(friendslist).unwrap() };

    //这些人是我的朋友（虚构）
    let jizishan = container.create_concept_with_data("季子杉");
    let wangyuxuan = container.create_concept_with_data("王宇轩");

    //把我的朋友们记录到“朋友名单”的“内容”属性上
    unsafe {
        friendslist_contents.add_concept(jizishan);
        friendslist_contents.add_concept(wangyuxuan);

        //"王宇轩"不幸去世了，把他从名单移除
        friendslist_contents.remove_concept(wangyuxuan);
    }
}
```