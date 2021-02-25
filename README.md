# ordinary-data 平凡的数据模型
网结构数据模型，用*BTreeMap*实现的，主要操作都优化成了O(Log n)

## 基本类型
- *Container* 容器
- *ConceptPtr* 概念
- *RelationPtr* 关系
- *RelationTypePtr* 关系类型
- 没了
  
## 为啥是Ptr
这是针对性能设计的库，Ptr后缀的都是对象的unsafe引用，方法也都有unsafe标注，请慎重使用

## 教程
自己看代码，也没几个类，这傻瓜式api，你肯定能看懂
（其实是懒得写教程，不过说实话你真能通过代码看懂api是怎么用的）
