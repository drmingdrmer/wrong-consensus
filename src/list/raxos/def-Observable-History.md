**def-Observable**
## 对系统总是可见的 History

对于某个时刻`T`,
对一个 History, 如果这个时刻的 `read_quorum_set` 中每个`read_quorum` 都能读到这个 History,
那么就认为这个 History 是 **在`T`时刻可见** 的.

注意 **可见** 是跟系统的状态和 `read_quorum_set` 的定义相关的,
例如, 下图中对 `History = {E1->E2}`来说,
如果 `read_quorum_set` 是多数派读写的定义, 即 `{{1,2},{1,3},{2,3}}`,
那么它 **可见** 的:

![](history-visible-12.excalidraw.png)

但对 `History = {E1->E2->E3}`来说,
- 如果 `read_quorum_set` 仍然是多数派读写的定义, 即 `{{1,2},{1,3},{2,3}}`,
  那么它不是 **可见** 的, 因为通过`{2,3}`读不到`History = {E1,E2,E3}`,
- 但如果修改 `read_quorum_set` 为 `{{1,2},{1,3}}`, 那么即使系统状态不变,
  它也变成 **可见** 的了:

![](history-visible-123.excalidraw.png)