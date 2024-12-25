example-crdt-implementation

![](crdt-storage.excalidraw.png)
这是一个理论上的设计, 实现上:
- 在实现中一般我们是将无限个key分配到有限个bucket中, 即一个实现中的只提供有限个维度的CRDT.
而每个维度可能用一组专用服务器来存储(例如3副本).
- 这时, 每次写入一个操作(Event)时,它的quorum是 Hierarchical Quorum, 例如一个操作要写入的key分布在下面两组服务器中,那么这次写的quorum定义就是 TODO: read_quorum / write_quorum
- apply()时要跨多个组.