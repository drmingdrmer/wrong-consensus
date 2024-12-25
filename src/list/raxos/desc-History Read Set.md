## History Read Set

对于
`fn read(pt: PTime, nodes: Vec<Node>) -> History`,
对读到的 History 的任一子集 H, 我们可以称这个`node_set` 是这个`History`的一个 ReadSet. 表示这个`History` 可以通过这个`node_set`读到.

对系统的某个特定的状态, History 的 ReadSet 定义为可以读到这个 History 的一个节点的集合.

![](history-read-set.excalidraw.png)

例如, 在下面,

- `read({1})` 返回: `[{E1->E2->E3}]`,
- `read({1, 2})` 返回: `[{E1->E2->E3}, {E1->E2}]`,
- `read({3})` 返回空 `[]`.

例如, 在下面这个 3 节点的系统中, `{E1->E2->E3}`的 ReadSet, 即能读到它的节点的集合, 有 4 个, 是所有包括 node-1 节点的节点集合:

`{1}, {1,2}, {1,2,3}, {1,3}`

例如`read({1})` 会返回`{E1->E2->E3}`在结果里,
`read({1,3})` 也会返回`{E1->E2->E3}`在结果里.

但是`read({3})` 不会返回`{E1->E2->E3}`, 所以 `{3}` 不是`{E1->E2->E3}` 的一个 ReadSet

而`{E1->E2}`的 ReadSet 有 5 个, 除了`{3}` 之外的所有非空节点集合都是它的 ReadSet:
`{1}, {1,2}, {1,2,3}, {1,3}, {2,3}`,

例如`read({2,3})` 会返回`{E1->E2}`在结果里.


对于返回的结果, 我们也可以将 `Vec<History>`里的元素做一个并集来简化表示,
例如, 上图中, `read({1, 2})` 可以看做返回了一个 History: `History{E1, E2, E3}`

而在下图中, 我们可以认为`read({1, 2})` 返回了一个树形的 History:

```
 .->E4
E1->E2->E3
```

![](history-read-set-union.excalidraw.png)