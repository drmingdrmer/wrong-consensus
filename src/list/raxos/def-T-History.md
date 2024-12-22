
显然一个 History 有一个或多个 [Maximal][], 也就是**最大时刻**, 也就是时间DAG中没有出向边的顶点.
显然History 中,的多个**最打时刻** 之间是不可比较的. 

如果一个 History 中只有一个**最大时刻** [Maximal][], 那么把它写作 `History({T})` 的形式, 后面我们称之为T时刻的History.

> 我们现实世界中, wall clock是线性的时间的, 因此一个 History 一定是一个单向链, 只有一个 Maximal, 就是墙上时钟的当前时刻.