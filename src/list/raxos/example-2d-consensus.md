example-2d-consensus

## 二维 Time 的工作流程

例如, 现在系统的状态是有一个 E1,
E2 和 E3 的 Proposer 同时开始执行, 同时 commit 了 E2 和 E3,
E4 只能看到 E2, 也 Commit 了,
E5 看到了所有, 最后 Commit.

TODO: image