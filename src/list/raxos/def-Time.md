
## Time

![](history-generalized-time.excalidraw.png)

**def-Time**

现在，让我们将注意力从 Event 的具体内容转移到其在 History 中的位置。在 History 的图中，每一个 Event 的顶点 (vertex) 都代表一个**时间**。我们称之为**时间**是因为它表示了Event的顺序, 这跟我们常识中的wall clock 时间是相通的, 也是wall clock的超集.

对于一个给定的**时刻**, 由所有小于它的 Event，即所有与之**连通**的顶点和 Event 组成的集合，构成了这个**时间**的 History。

例如上图中, 
- `E3` 所在的时间`T3`对应的History 是 `(E1, E2) -> E3`.
- `E4` 所在的时间`T4`对应的History 是 `(E1, E2) -> E3 -> E4`

需要注意的是，这里的**时间**并非我们日常生活中所理解的线性时间(wall clock):
- 不加特殊说明, 本文中提及时间的概念都是这个时间DAG定义的时间, 
- 常识中的时间我们都称之为墙上时钟(wall clock time)的时间. 

本文中DAG定义的时间可以是离散的或连续的，并且满足偏序关系:

如果我们从表示时间的 DAG 中移除所有表示**传递性**的边(例如上图中`E1->E4`是由传递性`E1->E3->E4`定义的一条边)，我们就得到一个 transitive-reduction DAG (也称为 [Hasse diagram][])。Hasse diagram 和偏序集之间存在一一对应的关系, 因此我们说 **时间** 是一个偏序关系的值. 其中, 每一个时间点（即 DAG 中的一个顶点）可能存在或不存在一个 Event。

（作为对比，常识中的时间(wall clock)是线性的、连续的, 全序关系的值。）
