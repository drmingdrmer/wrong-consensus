
## 过去 未来 Committed

这里我们要提出一个概念:

如果对一个时刻 `t`, `read(t: Time)` 的所有调用都返回同样的 History({t}),
那么这个`t`时刻是 **过去**, 即认为这个 History({t}) 是确定的了, 即 committed,
否则这个时刻在系统中属于是 **未来**, 这个时刻对应的状态还没有确定, 没有一个 committed 的 History.