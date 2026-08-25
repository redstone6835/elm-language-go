# 路线图

## M0：仓库与契约

- 固定 TamaGo module/tool 版本和供应链来源；
- 建立 carrier ABI、`go.support` ELM 清单和目录检查；
- 定义 package/bridge/schema 模板；
- 明确 trusted-direct 与 managed 的边界。

## M1：单架构最小模块

- 选择 RISC-V 64 或 LoongArch 64 完成 TamaGo freestanding 端口；
- 生成 Go 声明、架构 trampoline 和 Rust carrier wrapper；
- 构建、签名、装载并卸载一个无 goroutine 泄漏的 `hello` ELM；
- 证明 loader 没有 Go 专用分支。

## M2：驱动基础

- 直接日志、时钟、分配和错误接口；
- MMIO、IRQ、DMA 与 buffer 的类型化 SDK；
- 中断顶半部和 goroutine worker 的队列协议；
- 故障、取消、超时与卸载回归。

## M3：Go 开发体验

- 有界 GC heap 与 safepoint；
- 常用标准库的 freestanding 兼容矩阵；
- 构建期保留的有限反射；
- `go vet`/analyzer 检查禁止的 syscall、plugin、Go 指针逃逸和 IRQ 分配；
- 调试符号、panic 报告和 profile 映射。

## M4：第二架构与发布

- 第二架构达到相同 ABI 与生命周期测试；
- 可复现构建、SBOM、工具链摘要和离线镜像；
- 性能基准证明 trusted-direct 热路径没有 LR dispatcher；
- 发布稳定 SDK、兼容政策与迁移指南。
