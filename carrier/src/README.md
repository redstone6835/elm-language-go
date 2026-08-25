# carrier 源码

- `lib.rs`：固定 `repr(C)` 生命周期上下文、回调表、版本和布局校验；
- `managed.rs`：仅在启用 `managed` feature 时编译的有界控制面传输结构。

这里不得加入堆分配、内核 crate 依赖、动态符号查找或设备 API dispatcher。新增 ABI 字段时必须
消耗保留槽或提升 ABI 主版本，并同步补充尺寸、偏移和零保留字段测试。
