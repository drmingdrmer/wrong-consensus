# 分布式多副本的 History

我们有了一个系统 State 的描述方式 History, 接下来在分布式环境中把它实现高可用,
即通过多个 History 的副本来实现高可用.


我们先来看 History 部分.

一个在分布式系统, 可以看作每个节点上存储的 History 的副本的集合:

```rust
System: BTreeMap<NodeId, History>
```