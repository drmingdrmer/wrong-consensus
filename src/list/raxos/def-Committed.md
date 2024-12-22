
### Committed 和 数据丢失

直观上来说, 一个写操作达成 Committed 的状态就是说它总是能被读到了,
在单机上来说一个写操作只要完成就是 Committed,


对一个 History, 从系统的某个时刻`pt`开始, 对所有`pt' > pt`, 都是 **可见** 的,
那么就认为它是 Committed. 注意这里如果有集群成员变化, 相应的 `read_quorum_set` 会发生变化.


因为 **可见** 的定义是跟节点上存储的数据的状态和`read_quorum_set`决定的,
所以数据丢失一般常见的就是这两种:
- 一种是节点上 History 副本状态回退, 例如磁盘损坏等;

  这时如果 E3 从 1 节点上删除了, 在这个系统状态的变化中, 就导致了`History{E1, E2, E3}`的丢失,
  因为 `read_quorum` `{1, 2}` 无法读到 `History{E1, E2, E3}`.

  但是`History{E1, E2}` 没有发生数据丢失, 因为从任意一个`read_quorum`
  都可以读到它.

- 或者是系统的`read_quorum_set` 发生变化, 例如 raft 的单步成员变更算法的 bug(已解决), 实际上就是 read_quorum_set 变化产生的. 下图中对应`read_quorum_set` 增加了`{2}` 的情况.

TODO: 更新连接
https://blog.openacid.com/distributed/raft-bug/#raft-%E5%8D%95%E6%AD%A5%E5%8F%98%E6%9B%B4%E7%9A%84bug

![](history-data-loss.excalidraw.png)