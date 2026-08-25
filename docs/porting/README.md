# 目标移植

新增目标按以下顺序推进：

1. 在未修改内核 loader 的前提下确认 Go 后端能输出目标 object；
2. 实现 TamaGo `GOOS=tamago` 的启动、时钟、原子、TLS、异常入口和链接布局；
3. 对接 `go.support` 的 heap、调度器、日志、panic 与生命周期；
4. 生成 EKI Profile 对应的 Go 声明、汇编 trampoline 和 Rust carrier；
5. 验证 relocation、W^X、栈、GC roots、unwind/panic 和调试符号；
6. 运行 ABI golden、启动、取消、IRQ/DMA、quiesce、卸载和 stale-generation 测试；
7. 加入可复现构建和工具链摘要后才能标记为支持。

RISC-V 64 与 LoongArch 64 必须分别通过，不能用“Go 编译器存在该 CPU 后端”代替裸机运行时
与 ELM 装载验证。
