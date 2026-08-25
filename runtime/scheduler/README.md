# 调度器适配

Go runtime 的 M/P/G 调度模型不能直接假定宿主线程或 POSIX 信号。适配层将 Go worker 映射到
ELM owner 下的内核 worker，并提供：

- 启动与 join；
- monotonic timer、park/unpark 和抢占 safepoint；
- IRQ 顶半部到有界队列再到 goroutine worker 的交接；
- pause 后停止调度但允许 resume；quiesce 后永久禁止新 goroutine 和新回调；
- generation 变化后拒绝旧 timer、work item 与 callback。

首个版本可以限制为单 worker，但生命周期语义必须和未来多 worker 相同。
