
### 过去未来在分布式中的例子

classic Paxos 中 的 phase-1 可以认为是一个 read 函数的调用,
而 classic Paxos 中的 PTime 是它的 ballot number(一般是递增变量和节点 id 组成的一个 tuple),
所以, 如果一个 paxos instance 没有 commit 一个值(即还没有将一个值写到多数派),
那么运行 classic Paxos 的 phase-1,可能会看到不同的值, 即现在系统存储着什么值,属于未来的,还没到来的一个状态.
而当一个值提交了, 那么后续所有的 phase-1(后续指用更大的 ballot number(对应我们的 PTime)), 都能读到这个已提交的值.
即这个系统的值已经属于**过去**了,不会再变了.

所以也可以说一个时刻的状态是否不变, 表示了过去和未来.

假设一个三节点的 classic paxos 系统,
发生的事件序列是:

P1 用 ballot number=1 通过 node-1 和 node-2 完成 phase1, 将值 x 写到 node-1;
P3 用 ballot number=3 通过 node-3 和 node-2 完成 phase1, 将值 z 写到 node-3;

这时 P4(ballot number=4)执行 phase-1, 如果
- P4 联系 node-1 和 node-2, 它读到的值是 x
- P4 联系 node-3 和 node-2, 它读到的值是 z.

读到不同的值表示系统的状态还没确定, 属于未来.

这时假设 P4 联系 node-1 和 node-2 完成 phase1, 再将读到的值 x 写回 node-1 和 node-2,
这时 node-1 和 node-2 的状态都是 ballot number=4,vballot =4, value =x.

而后续的 P5(ballot number=5), 不论通过哪 2 个节点完成 phase-1,都会看到 x(因为 x 的 vballot 比 z 的 vballot 大,z 会被忽略),
这时认为系统的状态已经确定了, 属于**过去**了.

![](past-future-paxos.excalidraw.png)

这也是我们对**过去/未来**定义的在分布式系统中最直接的一个反应.