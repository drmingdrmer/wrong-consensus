example-Raft
# Raft

将其转变成 Raft 也很简单,
将 Event 设计成一个 sub-event 的容器,其中每个 sub-event 也分配一个时间`(T, index)`.
并要求在读 Committed 的阶段也考虑这个 sub-Time.

剩下的就是将整块的 History 的复制实现为分段传输的协议就好了.

是不是很简单?