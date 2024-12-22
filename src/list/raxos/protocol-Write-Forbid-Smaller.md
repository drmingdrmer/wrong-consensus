protocol-Write-Forbid-Smaller
## Write 阻止更小的 History 被 Commit

所以 Writer 把 History 写到节点上前, 必须要求没有更小的 History 被 Commit.
在上面的例子中, W9 在写入 History{.., E4}前, 必须组织 W8 写.
这里的阻止是指, 不允许 W8 认为自己 Commit 成功了.

所以假设 Write 要写的 History 的大小是 T, 它首先要发一个消息, 给一个`read_quorum`, 要求这个`read_quorum`里的节点都不接受小于 T 的 write 消息.

之所以是要写到一个`read_quorum`, 是因为系统中任意一个 read_quorum 跟任意一个 write_quorum 有交集, (但是 write_quorum 之间, 或 read_quorum 之间没有必须有交集的约束),
这样 W8 在执行 write History{.., E3}时,
它的 write_quorum 里就至少有一个节点阻止它的写入, 避免 W8 误以为自己可以成功 Commit.