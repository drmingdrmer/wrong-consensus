protocol-All
## 协议描述

为了简化设计, 我们假设每个 writer 新增

TODO: 图:

所以, 最终我们得到的分布式一致性算法为:

每个 Node 存储的数据为: 一个线性的 History 路径, 和一个最小可 Commit 的时间

```rust
struct Node {
    least_commit_time: Time,
    history: BTreeMap<Time, Event>,
}
```

Phase-1

Phase-2