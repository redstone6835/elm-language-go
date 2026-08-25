# 贡献指南

变更应保持 loader 语言无关、trusted-direct 为默认路径，并避免把完整 Go runtime 逻辑放入
`language-runtime`。新增依赖必须固定版本、说明许可证和供应链来源。

提交前运行：

```sh
cargo fmt --all -- --check
cargo test --workspace --all-features --locked
go test ./...
./toolchain/scripts/check-layout.sh
./toolchain/scripts/verify-toolchain.sh
git diff --check
```

目标端代码尚不可运行时，测试必须明确标记为结构或 host-side 测试，不得把占位摘要、未实现
绑定或宿主模拟声称为可装载 EKI。

提交消息沿用 Hitoshizuku 的 Conventional Commits 风格，主题使用中文，例如：

```text
feat: 增加 RISC-V 64 TamaGo 启动适配
fix: 修复卸载时的 goroutine 汇合顺序
docs: 补充直接 DMA 调用边界
```
