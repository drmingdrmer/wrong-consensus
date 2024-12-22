TODO: remove concept History quorum

[[desc-Time-in-Distributed]]

[[example-T-in-single-threaded]]

[[def-API-read-write]]

[[def-RW-Necessity]]

[[example-RW-single-threaded]]

[[def-Past-Future]]

[[example-Past-Future-single-threaded]]

[[example-Past-Future-distributed]]

[[def-Distributed-HA]]

[[def-Apply]]

[[def-Distributed-Copies]]

[[def-RW-Distributed]]

[[def-Mergeable-History-2]]

[[prop-Merge-Read]]

[[prop-Discard-Smaller]]

[[prop-Write-Forbid-Smaller]]



## History Read Set

对于
`fn read(pt: PTime, nodes: Vec<Node>) -> History`,
对读到的 History 的任一子集 H, 我们可以称这个`node_set` 是这个`History`的一个 ReadSet. 表示这个`History` 可以通过这个`node_set`读到.

对系统的某个特定的状态, Histroy 的 ReadSet 定义为可以读到这个 History 的一个节点的集合.

例如, 在下面,

- `read({1})` 返回: `[History{E1, E2, E3}]`,
- `read({1, 2})` 返回: `[History{E1, E2, E3}, History{E1, E2}]`,
- `read({3})` 返回空 `ø`.

例如, 在下面这个 3 节点的系统中, `History{E1,E2,E3}`的 ReadSet, 即能读到它的节点的集合, 有 4 个, 是所有包括 node-1 节点的节点集合:

`{1}, {1,2}, {1,2,3}, {1,3}`

例如`read({1})` 会返回`History{E1,E2,E3}`在结果里,
`read({1,3})` 也会返回`History{E1,E2,E3}`在结果里.

但是`read({3})` 不会返回`History{E1,E2,E3}`, 所以 `{3}` 不是`History{E1,E2,E3}` 的一个 ReadSet

而`History{E1,E2}`的 ReadSet 有 5 个, 除了`{3}` 之外的所有非空节点集合都是它的 ReadSet:
`{1}, {1,2}, {1,2,3}, {1,3}, {2,3}`,

例如`read({2,3})` 会返回`History{E1,E2}`在结果里.

![](history-read-set.excalidraw.png)

对于返回的结果, 我们也可以将 `Vec<History>`里的元素做一个并集来简化表示,
例如, 上图中, `read({1, 2})` 可以看做返回了一个 History: `History{E1, E2, E3}`

而在下图中, 我们可以认为`read({1, 2})` 返回了一个树形的 History:

```
 .->E4
E1->E2->E3
```

![](history-read-set-union.excalidraw.png)


[[def-Read-Quorum-Set]]

[[example-Quorum-Set]]

[[def-Write-Quorum-Set]]

[[example-Quorum-Set-xy]]

[[def-Observable-History]]

[[def-T-Committed]]

[[def-Committed]]

[[desc-Availability]]

## 多重宇宙里的分布式一致性

如果允许 History 是树状的或图样的(到目前为止我们都没有要求 History 是线性的),
那么以上就是分布式高可用一致性算法的实现:

向一个`write_quorum` 添加(不能是替换)一个 History 分支,
那么一定可以被一个`read_quorum` 读到, 那么这次写入就可以认为是 Committed.

说它是多重宇宙是说, History 可以有多个分支, 所有的历史路径都存在, 都是现实存在的.

因为任意 2 个图是可以取并集的, 也没有冲突, 读写都是自然是完整的,
所以这里只需我们平时说的 quorum-rw 就可以实现可靠的分布式读写了,
即只需保证读写的 quorum 有交集.

到这里我们都没有提到时间的概念,
在这个多重宇宙里虽然每个分支都是一系列有先后顺序的 Event,
但这个先后顺序还不足以称之为时间
可以看出, 我们现在没有引入时间的概念,
如果允许多重历史的存在, 那么就不存在时间的概念.

因为 History 分支之间没有关联, 所以一个 write 操作可以读任意一个 History,
加入新的 Event 构造一个新的 History, 再将其写入到一个`write_quorum`, 即完成 Commit.
当然新加入的 Event 可以依赖于已有的多个 Event. 这样 History 会形成一个 Event 的 DAG.
否则 History 是一个树.
这里我们假设一个 Event 不会被多次加入到 History 里, 所以这个图是无环的.

例如: 下图中, 假设系统的 quorum set 配置是多数派的 read_quorum 和 write_quorum.

- 这个分布式系统的用户先通过一个 read_quorum 1, 2, 读到 2 个 History,
  然后把它 union 成一个完整的带分支的 History H₁
- 2, 在读到的值上添加新的数据 E5, E6, 其中 E5 和 E2, E4 有先后关系,
  跟 E3 的先后关系任意.
- 3, 将这个新的 History H₂ 写回系统, 使用 write_quorum 2, 3

![](multiverse-rw.excalidraw.png)

构建这样一个支持多分支历史的储存系统非常简单,
但是现实中, 多重历史的存储系统并不太好用,
例如上面如果 E2 的内容是将变量 x 设置为 3, E4 的内容是将变量的 x 设置为 4,
那么对于这样一个历史, 我们的状态机应用所有 log(Event)后, 没有明确定义 x 到底是多少.
虽然遇事不决量子力学, 你可以说这时 x 的值是 3, 4 的叠加态,
这样的设计在某些场景下也有应用的价值,
但是大部分场景中我们(的产品经理)仍然需要状态机中每个变量的值都是确定的.

现在我们如果要求历史必须是线性的(即我们的肉体不能同时存在于多于一个历史时间线,
只能选择一个),
这也是我们常识认知中的现实.
那么就会引入时间的概念.

接下来进入本文的第二部分

## 限制单一历史

但是我们现在希望得到一个线性 History,
对于 write(), 我们只需保证不在读到的 History 增加非线性的分支,
而对于 read, 因为它返回的是多节点的并集, 所以仍然可能读到多分支的 History.

即, 我们需要另一个 read 函数
`read_linear() -> Vec<Event>` ,只返回一个线性的 History,
也就是一个`Vec<Event>`, 而不是 DAG 的 History 了.

但是现在的`read()`返回的一个 DAG, 它是所有节点上读到的 History 的并集,
所以在实现上, `read_linear()` 通过 `read()` 得到一个 DAG, 从中选择一个分支, 作为读到的 History 的结果.

而且对于一个 Committed 的 History, 多个`read_linear()`
总是选择这个 Committed 的 History.

例如 分支 E4 和分支 E3 同时被读到, 那么`read_linear()`就总是选择 E4(或总是选 E3)

![](multiverse-read-linear.excalidraw.png)

这说明这个系统必须保证: 每个被读到的分支之间都必须有一个二元关系:
例如上面, 如果 E4 和 E3 都被读到时, 总是选择 E4, 那么就说 E4 大于 E3,
这也是一个全序关系. 当然 read_linear 也可以在 E3 E4 之间总是选择 E3,
这时我们就说 E3 大于 E4.

### 虚拟时间的诞生

为表示这个全序关系, 就说明每个 History 分支都有一个全序关系的属性 T,
因为每个 History 的节点对应一个分支,
这也就是说 History 上每个 Event 都对应一个全序关系的属性 T.
这个属性 T 的大小用来决定`read_linear()`读到多个分支时选择哪个.

如下图, `read_linear()`
根据每个 Event 节点对应的 T 来决定每个分支的大小并选择最大的哪个分支,
从而保证 **读的一致性**.

![](multiverse-read-by-t.excalidraw.png)

可以看到, 每次读到的 History 上的每个 Event 节点都有一个顺次递增的 T 属性,
我们把这个 T 属性看作是这个系统的 **虚拟时间**.
为什么我们把它称作时间呢?

- 不能回退,
- 每个 Event 对应一个 T
- 决定了 Event 的顺序.

到此, 我们看到, 为了将多分支的历史筛选成单分支的线性历史, 必须引入时间的概念.

后面我们会看到这个 **虚拟时间**
在分布式一致性协议里的角色跟我们现实中的墙上时钟几乎是完全一样的.

<!--
   - 而根据 Committed 的定义, 它必须总是能被读到,
   - 因此这个系统必须满足 Committed 的 History 分支, 在写入完成时, 必须(在任一 read_quorum 中)是最大的.
   -
   - 每次 Commit 一个 Event, 它必须具有全局中最大的 T.
   -->

## Commit 在线性 History 约束中的定义

这里 Commit 的定义没有变, 即对一个 History 永远能读到
但是我们现在用 read_linear() 替换了 read(),
它会抛弃一些 History 分支,
所以要保证 Commit, 一次写入不仅要保证 read() 总是可见,
还要保证 read_linear()总是可见.
对一个 History, 如果

- 1 读操作 read_linear()可以通过系统中`read_quorum_set`中任意一个 read_quorum 读到它,
- 2 且总是能被选择, 即它有最大的 T, 或是一个最大 T 的 History 的一部分(这样它也能被选择).

它就是 Committed.

# Write 约束

## Write 只追加 History

write 会有一些改变: 通过 read_linear()读到的 History 后,
不能覆盖已有 History, 只能追加.
在其上添加一个 Event 时, 也必须附带一个比 History 的最大 T 更大的 T,
并且只能在最后一个 Event 节点上添加新的 Event.

即保证, 随着 History 中 Event 的增加, T 是单调递增的, 永远不会回退.

现在 write 流程调整为如下图, 注意这里写回 node-1 的时候,
node-1 本地将已有 History 跟新写入的 History 合并了, 本地存储了一个多分支的 History,
这种情况是允许的, 因为 read_linear() 保证会将**旧的**History 分支忽略掉.
在实现中, 因为 read_linear()一定同时读到一个存储节点上的多个分支,
所以一定会舍弃掉较小的分支, 所以在实现上一般可以直接删掉较小的分支,
例如 raft 的 truncate log 就是在添加新的分支前删掉较小分支.
但我仍然在这个文章里保留这个分支, 以展示分布式一致性协议的真实的样子.

![](linear-rw.excalidraw.png)

## Write Prepare

现在 write 保证了不覆盖已有的 History, 且写到了`write_quorum_set`,
能保证`read()` 一定读到它, 但还没能保证 read_linear()一定选择它.

所以可能产生一个问题, 写入完成时, 可能因为系统中有更大的 History,
导致 read_linear()忽略了自己写`write_quorum_set`的 History:

例如可能有 2 个 Writer W8 和 W9,

- W8 写了 node-1,node-2,
- W9 写了 node-3.

我们假设系统的`read_quorum_set`是多数派模式,
那么 W8 写的 History{..,E3} 虽然能被任意一个 read()读到,但不一定被 read_linear()
读到:

- 如果 read_linear()操作选择了 read_quorum {1,2}, 那么它会选择 W8 写入的 History{..,E3}, 得到预期的结果
- 但是如果 read_linear()操作选择了 read_quorum {2,3}, 那么它会选择 W9 写入的更大的 History{..,E4},

这违背了 Commit 的原则, 没有达到**总是能被读到**的要求.

![](history-dirty-write.excalidraw.png)


[[protocol-Write-Forbid-Smaller]]

[[protocol-Write-After-Read]]

[[protocol-Write-P2]]

[[protocol-All]]

[[example-Classic-Paxos]]

[[example-Raft]]

[[2d-consensus]]

[[example-2d-consensus]]

## 二维 Time 的应用

有一个常见的Raft使用常见是用Raft做一个授时服务, 集群选出Leader, Leader处理申请一个时间戳的请求, Leader 提交一个日志记录已经分配的时间, 然后返回这个时间戳. 这里我们假设本地时间不会回退. 

但是当Leader切换时, 新的Leader可能本地时间略小于上一个Leader, 导致新分配的时间回退.

这个问题的解决并不困难, 在生成时间戳的逻辑中判断禁止生成回退的时间.

但问题的根本原因是, Raft本身在使用一个虚拟时间(`term`)来实现全局的虚拟时间永远递增, 但是它没有考虑墙上时钟, 分布式本身就是为了建立一个单调递增的系统, 所以最根本的解决方法是把墙上时钟的值也包括到Raft的虚拟时间中:


然后
- 把term 替换为`Time: [term, timestamp]`,  这个二维的向量是使用我们上面定义的偏序关系, 2个分量都大于等于时认为向量大于等于.
- 再把`LogId = (term, index)` 替换为 `([term, timestamp], index)`
来构建一个分布式系统.

这样我们只改了数据结构, 协议还是跟原来一样的. 这也说明Raft只是偏序时间的一致性算法的一个特例.

这时我们会得到一个改进版的 Raft: 在每个成员有时钟飘逸的情况下,
新的 Leader 也总是有更大的 unix-timestamp, 如果一个Candidate的本地时间较小, 那么在RequestVote阶段会失败: 
**注意,这里RequestVote要比较的是最新的时间戳...**

这个分布式系统可以用来构建分布式授时服务, 优雅的解决 Leader 切换时的时间可能回退问题.

我举这个例子是为了说明, Time 的每个维度之间的类型不能假设是一样的,
即不支持加法, 但可以支持乘法. 这将会引出后面的有趣的结论.


# 多维偏序时间应用例子

假设有一个 key value 的存储系统，在这个系统里面，每个 key 都代表一个不同的维度。然后我们的时间，就是一个多维的向量时间内的比较，就是多为向量的比较。

我们先从一个简单的场景来看, 假设这个系统里只有2个key:
x 和 y,
这个系统的虚拟时间`T`定义为: 一个操作涉及的key的对应的维度上值为i的向量.
例如, `let x = 2` 的T为`{x:i}`,
`let y = 3`的T为`{y:i}`;
`let x = x + y`的T为`{x:i, y:i}`
T的比较关系定义为公共分量的大小比较:
例如, `{x:1} < {x:2,y:2} < {y:3}`, 但`{x:1}` `{y:2}` 之间没有大小关系, 因为它们没有公共分量.
这个虚拟时间定义了哪些Event可以互补影响的执行, 哪些之间必须有确定的顺序, 以及这些顺序是什么(有`i` 定义).


![](crdt.excalidraw.png)

应用我们的Abstract Paxos到这个系统, 它将构建一个冲突自适应的Multi Master一致性协议.

例如

注意, 我们时间是对应到DAG的结构的, `{x:2,y:2} < {y:3}` 中的 `{y:3}` 顶点和 一个独立的 `{y:3}` 顶点代表的不是一个时间.

如果T不是**传递的**, 那么第一阶段必须在一个read-quorum-set上阻止要写入的History上所有的T, 而不是只有最大的一个T.因为不能通过T来防止之前的顶点被写入.
而在第二阶段, 也必须逐个判断要写入的History的每个T是否小于已经被阻止的T, 而不仅仅是要写入的History的Maximal T.


[[def-2d-consensus-Apply]]



[Hasse diagram]: https://xxx
[Maximal]: foo
[Time, Clocks and the Ordering of Events in a Distributed System]: https://lamport.azurewebsites.net/pubs/time-clocks.pdf
[Paxos Made Simple]: https://lamport.azurewebsites.net/pubs/paxos-simple.pdf
[Raft]: https://raft.github.io/raft.pdf
[Generalized Consensus and Paxos]: https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/tr-2005-33.pdf
[CRDT]: https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type
[CURP]: https://www.usenix.org/system/files/nsdi19-park.pdf