# go.support glue 源码

`lib.rs` 暂时只重导出 carrier ABI，并提供构建期/装载 glue 可复用的版本与描述符校验入口。
真实 ELM 接入后，本目录可以增加 artifact 绑定和生命周期适配模块，但不得加入设备热路径的
operation dispatcher，也不得在这里重新实现 `language.runtime` 的资源账本。

任何新入口都必须保持 `no_std`，并明确其 owner、generation、quiesce 和 finalize 行为。
