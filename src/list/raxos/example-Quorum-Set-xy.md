example-Quorum-Set-xy

又如, 一个 4 节点的集群的`read_quorum_set` 如果是`{{1,2}, {3, 4}}`, 那么它的`write_quorum_set`就可以是`{{1,3}, {2,4}}`:

![](quorum-2x2.excalidraw.png)

如果一个`n*n`节点的集群的`read_quorum_set`定义是 n\*n 矩阵中任意一行, 那么它的`write_quorum_set`定义就可以是任意一列.

![](quorum-nxn.excalidraw.png)