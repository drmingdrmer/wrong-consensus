# 读写API

这个由虚拟时间Time, 事件Event 和 History 的系统中, 读操作和写操作都定义为在一个指定时间上发生的行为:
```rust
impl Node {
	fn read(&self, t: Time) -> History({T});
	fn write(&mut self, h: History({T}));
}
```

其中, `read` 读系统中t时刻的历史, 即它应该返回所有t时刻之前发生的Event, read返回的History的最大时刻可能小于参数中的t.
`write`写一个某时刻t的历史.