
# 分布式多副本中的读写


分布式中的读和写都是基于我们单机的API读写来完成的 我们首先看分布式的写 分布式中的写真的比较简单 它只需要把一个要写的history 写到每一个节点上就行了 就像我们下面的代码所展示的那样 分布式里面的读相对会稍微复杂一点 你读的话也是从每个副本上读一个history过来 然后再通过这些读到的结果 然后返回给客户调用者一个结果

```rust
impl {
	fn write(&self, h: History({T})) {
		for node in self.get_write_quorum() {
			node.write(h);
		}
	}
}
```







写操作要写一个历史,即 `fn write(h: History({T}), nodes: &[Node])`, 对每个节点的写的内容都一样

直观上, 分布式环境中的读写变成了对多个节点的读写:
```rust
fn read_nodes(nodes: &[NodeId], t: Time) -> Vec<History>;
```

但我们要求 read 只能返回一个 History, 所以要将`Vec<History>` 合并为一个`History`
返回的结果