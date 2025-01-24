def-Mergeable-History-2

### History 的兼容

如果 h1 和 h2 是兼容的, 那么 h1 和 h2 合并后, 任何一个 pTime 的 History 都不变.
或者说, 合并之后不产生不属于 h1 也不属于 h2 的新的边, 也就不会导致 History 变化.

所以, 读的正确性保证要求 read 返回的结果不能包含不兼容的 History.

例如, 如果 read_nodes 读到 2 个 History `{E1->E2}` 和 `{E1->E3}`, 且`E2->E3`,即 E2 的 pTime 小于 E3 的 pTime,
那么 read 函数不能同时包含这 2 个结果, 因为他们合并之后的结果`{E1->E2->E3}`不能是任何之前 write 写入的值.

因此如果 read 读到不兼容的 History,就必须舍弃其中一个. 如果舍弃大的, 那么系统则有可能永远无法提交更大的 History,
即如果`{E1->E2}`存在在一个存储节点上, 那么如果它就有可能被读到, 那么就没有办法保证其他任何更大的 History 能被读到了.
因此这里必须舍弃较小的不兼容 History,

这里的 History 都是指单顶点的`History({PTime})`, 一个 History 可以表示成 `History({pTime_1})` .. 的并集.
因此 read 函数就是把所有读到的单顶点 History 逐对比较, 舍弃小的.



单副本,或单机环境中, 对整个系统的读是一个简单的操作: `fn read(ptime: PTime) -> History`.
但是在多副本的分布式环境中, 读会读到多个副本, 读操作可以看做这样一个函数: `fn read(ptime: PTime, node_set: Vec<Node>) -> Vec<History>`:
它从多个节点`node_set`中读 History 副本, 并返回一个 History 的集合.


mergeable 比较方法:


给定的History, h 和 一个时刻T,

// TODO:
根据 [[def-Mergeable-History]]

对h中的每个待加入的时间Tᵢ,:
- 如果存在`Tᵢ < T`, 则h是不可写的, 因为一个read请求会选择包含T的history而舍弃Tᵢ
- 如果不存在`Tᵢ < T`, 则h是可写的,因为一个read操作会把h包含在自己的结果中.



