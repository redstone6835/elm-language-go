# GC

Go ELM 可以保留 Go GC，但必须以 ELM 为资源和卸载边界：

- 每个 owner 使用独立、有上限的 heap 统计；
- roots、stack maps 与 write barrier 必须和目标 AOT 工具链一致；
- DMA 与跨 ABI 对象只能通过明确 pin/handle 生命周期暴露；
- pause 驱动全部 goroutine 到可恢复 safepoint，quiesce 进入不可恢复的卸载边界；
- finalize 前完成最后回收，之后不得扫描或执行旧 image；
- OOM 转为当前 ELM fault，不能无限扩张或拖垮全局 allocator。

carrier wire 中只允许固定布局值和 opaque handle，绝不传 Go heap pointer。
