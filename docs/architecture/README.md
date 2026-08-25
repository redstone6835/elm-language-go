# 架构决策

当前已确定的决策：

1. 使用 TamaGo 和修改版 Go 工具链作为 freestanding AOT 基线；
2. 上游通过 Go Modules 与 `go tool` 引入，不使用 submodule；
3. Go ELM 默认走 trusted-direct，LR dispatcher 仅为可选 managed plane；
4. loader 不识别 Go，语言差异在 `go.support`、carrier 与 host 工具中解决；
5. 不依赖 cgo，跨语言边界由生成的 Go/汇编 stub 和固定 carrier ABI 完成；
6. 每个 Go EKI 拥有独立的 TamaGo runtime instance、owner、heap、goroutine 集合和卸载屏障；
   `go.support` 只共享实现和 ABI，不共享 heap。

改变这些决定前，需要给出 ABI 迁移、性能、安全和双架构影响分析。
