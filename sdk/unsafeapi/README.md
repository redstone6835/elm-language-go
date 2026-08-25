# 危险 API

内核驱动无法避免 MMIO、DMA、IRQ、端口 I/O 与裸地址。本目录不会把它们伪装成完全安全的
接口，而是以显式 `Unsafe` 命名、能力类型和生命周期约束暴露：

- 资源在 probe/admission 阶段获得并登记 owner；
- MMIO region 和 DMA buffer 在热路径直接访问；
- IRQ handler 只执行允许的顶半部操作；
- unload 前先静默设备，再注销 IRQ、停止 DMA、join worker；
- generation 变化后所有 wrapper 失效。

这些规则用于审计和回收，不意味着每次读写都经过 capability dispatcher。
