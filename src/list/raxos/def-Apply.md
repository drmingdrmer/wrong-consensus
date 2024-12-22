def-Apply

如果 History 的定义是一个简单的线性的 Event 的 log 数组, 那么`apply()` 也是一个简单的实现,
即逐个应用 log.
如果 History 是一个非线性的结构, 例如是 DAG 关系组织的 Event 的图,
那么`apply()`就要包含更多的内容, 后面说

`apply()` 方法可以认为是确定的, 因此我们可以后面都用 History 来表示系统的状态(State)