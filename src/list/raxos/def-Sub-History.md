
我们也可以定义一个History子集的关系: 它跟History对应的 Hasse diagram 的子集定义一致:
如果h1的Hasse diagram是h2 的 Hasse diagram 的子集, 那么h1 是 h2的子集, 或者说h1是h2的Sub-History

注意, 这里比较的不是完整的DAG, 而是将传递性得到的边都去掉后的Hasse diagram.
所以, `E1->E2` 是 `E1->E2->E3` 的Sub-History;
但`E1->E3` 不是`E1->E2->E3` 的Sub-History, 因为根据定义, `E1`到`E3`的边,在前者中是非传递边, 在后者是传递边. 

也可以更直观的理解为: `E1->E3` 之间插入一个`E2`后, 对整个系统的修改方式变了, 所以不能认为是History的子集.
例如:
```
E1 : let x = 3;
E2 : let x *= 2;
E3 : let x += 1;
```
显然，apply(E1->E3) 的结果是 x = 4，而 apply(E1->E2->E3) 的结果是 x = 7。虽然 E1 和 E3 在两个 History 中都存在，但由于 E2 的插入改变了最终的状态，因此 E1->E3 不能被认为是 E1->E2->E3 的 Sub-History。

TODO: 图