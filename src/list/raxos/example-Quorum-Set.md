example-Read-Quorum-Set

例如, 假如 3 节点的系统 的`read_quorum_set`定义为: `{{1,2}, {2,3}, {1,3}}`,
那么 `read({1})`, 系统就不对它返回的结果提供任何保证.

![](quorum-majority-3.excalidraw.png)

一般一个集群节点固定的系统来说, 它的 `read_quorum_set`一般是不变的, 例如 classic-paxos 的 `read_quorum_set` 和 `write_quorum_set`总是多数派.
如果涉及到集群节点的变化, 它的 `read_quorum_set` 和 `write_quorum_set` 可能就会发生变化, 以满足 committed 的要求.


例如,
- 如果一个 3 节点集群的 `read_quorum_set` 是多数派, 即:
  ```
  {
   {1,2},
   {1,  3},
     {2,3}
  }
  ```
- 那么它的`write_quorum_set`就可以也是多数派.

也就是常说的多数派读写 quorum-rw 的配置. 这里也可以看出 quorum-rw 只是分布式一致性的一个组成部分.

![](quorum-majority-3.excalidraw.png)
