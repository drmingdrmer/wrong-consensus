
# 分布式多副本中的读写


写操作要写一个历史,即 `fn write(h: History({T}), nodes: Vec<Node>)`, 对每个节点的写的内容都一样

直观上, 分布式环境中的读写变成了对多个节点的读写:
```rust
fn read_nodes(nodes: &[NodeId], t: Time) -> Vec<History>;
```

但我们要求 read 只能返回一个 History, 所以要将`Vec<History>` 合并为一个`History`
返回的结果