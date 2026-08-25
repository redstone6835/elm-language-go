# Go 运行时适配

本目录承载 TamaGo runtime 到 Hitoshizuku ELM 生命周期的适配，不包含编译器源码。它负责
让 `go.support` 为每个 Go EKI 建立独立 runtime instance、heap、goroutine 调度、safepoint、
panic、TLS 与静态反射元数据；不同 EKI 不共享 Go heap。

- [`abi/`](abi/README.md)：Go 端固定生命周期 ABI 与 golden layout；
- [`pal/`](pal/README.md)：最小平台适配层；
- [`gc/`](gc/README.md)：GC heap、roots、pin 与卸载协议；
- [`scheduler/`](scheduler/README.md)：goroutine、worker 和 ELM quiesce；
- [`reflect/`](reflect/README.md)：AOT 静态反射范围。

这里的接口最终由 TamaGo fork/patch 或生成代码消费。任何 patch 都必须在
`toolchain/tamago/patches/` 登记，不在此复制上游源文件。
