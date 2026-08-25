# 接口 Schema

接口 schema 由 EKI Profile 生成，是 Go 声明、汇编 trampoline 和 Rust carrier wrapper 的共同
输入。它必须描述固定宽度类型、布局、endianness、ownership、nullable、错误、symbol identity、
capability 和 Profile digest，而不只是一组 Rust 类型名字符串。

当前 `interface.managed.shape-only.invalid.json` 只展示 cargo-elm schema v2 的 managed 字段形状，
但故意保留空 type/symbol/operation 图，与 managed bridge 不一致，必须作为负面 fixture 被拒绝。
它不能描述 trusted-direct symbol carrier。direct 目标草案见
`manifests/DirectBindings.target.toml`；其正式 schema 必须由 cargo-elm 发布后才能生成 SDK。

生成文件放在 build 目录；仓库只提交 schema 版本和最小示例。任何手工修改生成结果都应在
下一次生成时失败。
