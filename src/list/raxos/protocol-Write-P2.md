protocol-Write-P2
## 执行写入!

假设追加的操作是在 T 追加 Event, 那么写入时只能写入 **小于 T**的 History,
因为在 P1.1 阶段只保证了小于 T 可以安全 Committed, 其他的 History 如果写入,
有可能造成已 Committed 的数据的丢失

TODO: 例子

执行具体的 write 过程相对简单, 直接将 History 复制到一个`write_quorum`, 完成 Commit.