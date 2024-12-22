## Read Quorum Set

对每个系统, 不论是单机的还是分布式的,
都显式的或隐含的定义了合法的 `read()` 可用的 `node_set` 有哪些, 系统只有用这样的`node_set` 读的时候才能提供 TODO:

- 例如单机系统, `read()` 可用的 `node_set` 就是唯一这个节点`{{1}}`,
  显然用一个空的 `node_set` 去读是没意义的.

- 一个简单 3 节点系统中, 如果不做任何限制,
  那么`read()`可用的`node_set`是所有非空节点集合: `{{1}, {2}, {3}, {1,2}, {2,3}, {1,3}, {1,2,3}}`
  但注意这样一个系统中`read()`得到的结果一般是没有任何高可用保证的, 因为它暗示了写操作必须写到每一个 node 上才能被合法的读读到.

- 一个多数派读写的 3 节点系统中(n=3, w=2, r=2),
  `read()`可用的`node_set`是至少包含 2 节点的集合: `{{1,2}, {2,3}, {1,3}, {1,2,3}}`,

如果一个 read 操作使用的 `node_set` 是这个系统定义的用于读的`node_set`,
那么认为这个 read 操作是合法的, 系统只给合法的读操作提供保证,
对于不合法的 read 操作,
系统对读取的结果不能提供任何保证(undefined behavior).

**def-Read-Quorum-Set** **def-Read-Quorum**:
在某个时刻`pt`, 可合法的用于`read()`的`node_set`的集合, 就是系统这个`pt`时刻的`read_quorum_set`,
`read_quorum_set` 中的一个元素是一个节点集合, 称之为一个`read_quorum`.
`read_quorum` 是一个节点集合`node_set`, `read_quorum_set` 是一个 节点集合的集合.