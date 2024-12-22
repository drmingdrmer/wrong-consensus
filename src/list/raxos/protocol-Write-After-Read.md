protocol-Write-After-Read
## Write 要基于已经 Commit 的 History 追加

阻止了更小 History 的写入后, writer 就可以选择一个在 T 时间的 History 来写入(即 History 的最后一个 Event 的 Time 是 T),
但是写入的 History 仍然不能破坏 Commit 的约束, 不能覆盖已经可能 Commit 的 History.
因为这时如果直接写入一个很大的 History, 可能大于一个已经 Committed 的 History,
而如果写入的 History 不包含这个 H₀, 就会导致后面的 read_linear() 读不到 H₀,
从而导致 Committed 的数据丢失.

所以 writer 在执行写入之前, 还要联系一个`read_quorum`, 进行一次读操作,
通过 read_linear()读到当前 Committed 的 History,
再在这个读到的 History 上追加新的 Event, 再写入.

这次读操作, 可以选择不同的`read_quorum`;
如果看到更大的 History, 那表示无法写入, 要终止,
因为写入的 T 一定不会被 read_linear() 选择了.

P1.1 保证了小于 T 的无法 Commit,
P1.2 保证了写入 T 的 History 不会覆盖 Committed 的数据;

这时如果写入成功一个 `write_quorum`, 那么 T 之前的数据都包括在这次写入之内,
T 的 History 和 T 之前的 History 都保证了 Commit 的条件.