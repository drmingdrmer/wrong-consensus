
### 读写函数在单机上的例子

例如, 在单机系统中的读操作因为使用 wall clock, 所以 ptime 就是当前时刻不需要指定. 所以是`fn read() -> History`.
但一般我们只取其中一个 key,所以是`fn read(key: &str) -> V`的形式.

而单机系统中的写也无需指定 wall clock, 但我们一般不是把系统的整个状态写进去, 而是用一个 command 来表示
这个 command 描述在系统现有基础上做出哪些修改, 例如 command 是`let x = 3` 或 `let x = x + y`,
这些 command 的默认前提都是**在系统现有的状态下**做一个更新, 也就是在当前的 wall clock 时刻之前的已有状态基础上更新,
也就对应了我们最原始的`fn write(h:History({ptime}))`的逻辑.

这 2 个读写接口很容易映射到一个单线程的系统上, 我们希望也用这个接口去定义一个分布式的系统.