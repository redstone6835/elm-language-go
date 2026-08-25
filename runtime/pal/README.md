# PAL

PAL 是 TamaGo runtime 与 Hitoshizuku 之间的最小平台接口，预计包括：

- page/heap reserve、commit 和 release；
- 单调时钟、deadline、entropy、日志与 abort-current；
- TLS attach/detach、原子操作、park/unpark；
- ELM owner/generation 和生命周期 phase 查询。

设备 MMIO、DMA、IRQ、PCI 和网络 API 不属于 PAL。具体 Go ELM 通过生成的 direct SDK 调用
它们，避免把语言 runtime 变成设备 API dispatcher。
