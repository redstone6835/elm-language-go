# Hello ELM

首个可运行示例将验证完整生命周期而不是只打印字符串：

1. 依赖 `go.support` 并注册 carrier；
2. 通过 direct kernel symbol 输出一条日志；
3. 启动一个可取消 goroutine；
4. quiesce 后停止新工作并 join；
5. finalize 后不再访问代码、heap 或 callback；
6. 重新装载时旧 generation handle 被拒绝。
