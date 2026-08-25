# Go SDK

SDK 向 Go ELM 暴露符合 Go 习惯的 API，同时保持 EKI Profile 是唯一权限与 ABI 来源。

- [`kernel/`](kernel/README.md)：安全或类型化的 direct API；
- [`unsafeapi/`](unsafeapi/README.md)：MMIO、DMA、IRQ、裸地址等显式危险 API；
- [`managed/`](managed/README.md)：可选低频 managed 控制面。

SDK 的声明由 EKI schema 生成，手写层只提供命名、错误和生命周期包装。热路径不会调用
`language.runtime.resource` 或 `language.runtime.kernel.call`。
