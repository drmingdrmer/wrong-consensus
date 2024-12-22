def-2d-consensus-Apply

## Apply 阶段

分布式一致性算法的核心一个是 log 的提交(对应我们的 Event History),
另一个是状态机的实现, 也就是将 log(History) 以业务定义的方式进行解释.

我们上面提到, 系统的状态由 History 构成, 细心的你现在一定发现了一个问题,
就是对一个例如二维 Time 的 History 来说, apply 这些 Event 的顺序是不确定的.

如果我们有量子计算机,
那么一个可行的实现是将所有符合 DAG 顺序的顺序来 apply 一次 History,
这样所有的可能并存.

但是我们没有.

所以每个副本节点上, 我们仍然需要一个相互间一致的顺序来 apply 已 commit 的 History.

而对于二维 Time 来说, 就是怎么为`x₁ > x₂ && y₁ < y₂` 关系的 Time 确定一个顺序.

假设找到 2 个 T1, 和 T2 是相等的关系,

相对 T1 来说, T2 是`x2/x1, y2/y1`

commit 的定义: 总是能读到(因为: 可能脏读: 最大历史不在 quorum 里)
增加新历史前必须阻止其他写(阻止所有: 死锁, 2pc; 阻止旧的: raft, paxos)(为了满足 commit)
自己不是最大历史不能写: 因为增加新历史前不能覆盖已有的(为了满足 commit)
(一个简化的 raft: 可以 vote for 非最大日志的 candidate, 但是 candidate 自己不能继续写)
(paxos 的不同做法是: 把最大历史拿过来)

raft 接 append-entries 时不需要用(t2, a) 覆盖 (t2,b)