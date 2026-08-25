# carrier

本 crate 定义 Go AOT 产物与 `go.support` Rust glue 之间的最小固定 C ABI。它保持
`no_std`、无分配、无内核依赖，便于在宿主机执行布局和保留字段测试。

默认 feature 是 `trusted-direct`。设备热路径由目标 EKI 生成的静态 Rust wrapper 直接进入
已审核的 kernel symbol，不经过本 crate 的通用 dispatcher。`managed` 必须显式启用，只提供
有界控制面 frame，不能承载 MMIO、DMA、IRQ、网络或块设备数据面。

生命周期表固定包含 `initialize`、`pause`、`resume`、`quiesce` 和 `finalize`。`pause` 与
`resume` 构成可逆停机边界；`quiesce` 是卸载前的终止性边界，成功后不得恢复。`finalize`
只允许在终止性边界内销毁 scheduler、GC、回调和 runtime 状态。

artifact 描述符中的入口先以 `u64` 地址值读取；结构校验只确认入口非零。loader 还必须验证
地址位于已重定位的可执行段，之后调用门才能将其转换成具体函数签名，避免在验证前构造无效
Rust 函数指针。
carrier 只定义契约，不自行实现 Go scheduler 或 GC。

源文件职责见 [src/README.md](src/README.md)。
