desc-Availability

## 系统可用性

可以看出, Committed 的是跟`read_quorum_set`相关的,
一般地, 直观来说,

- 如果 `read_quorum_set` 中的 `read_quorum` 变小, 系统读的可用性增加, 因为只需更少的节点就可以完成一次读; 但写的可用性降低, 因为要成功写入更多节点才能认为是成功, 即 Committed 达成的条件变得困难.
  例如, 5 节点系统, 如果 `read_quorum_set` 是任意 4 节点, 那么 `write_quorum_set` 可以是任意 2 节点;
  如果 `read_quorum_set` 改成任意 3 节点, 那么`write_quorum_set`要相应的变成任意 3 节点;
  即读成功更容易了, 写成功更困难了.

- 如果 `read_quorum_set` 中的 `read_quorum`数量变少, 系统的读可用性降低, 因为可用于读的节点组合变少了, 能容忍的节点宕机的组合变少了; 但写的可用性提高, 因为某些节点组合不需要去覆盖了, 即更容易 Commit 了.
  例如, 3 节点的多数派系统中, `read_quorum_set`是任意 2 节点: `{{1,2},{1,3},{2,3}}`, `write_quorum_set` 也是: `{{1,2},{1,3},{2,3}}`;
  如果 `read_quorum_set`中去掉 `{1,3}` 变成: `{{1,2},{2,3}}`, 那么 `write_quorum_set` 变成: `{{1,3},{2}}`;
  读可用性降低: 从容忍任一节点宕机变成不能容忍 node-2 宕机; 写的可用性提高: 从容忍任一节点宕机又增加了可以容忍 node 1,3 同时宕机.

再例如不做限制的 3 节点系统(`read_quorum_set={{1},{2},{3},{1,2},{1,3},{2,3},{1,2,3}}`),
要达成 Committed, 它的`read_quorum_set`是最大的, 就要求从任意节点都能读到这个 History,
读操作不能容忍任何节点故障.

如果将`read_quorum_set` 去掉几个元素, 例如去掉`{1}`,
即系统不再要求从 node-1 节点上能读到这个 History 了, 系统就容忍了一定的故障(`node-1`的故障),
也就是说可用性提高了.

所以可用性的就被`read_quorum_set`决定(因为`read_quorum_set`直接决定了`write_quorum_set`)
或者说, 这个`read()`操作容忍了哪些节点集的故障.