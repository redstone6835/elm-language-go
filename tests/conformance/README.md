# 一致性测试

一致性测试用于证明不同目标和 SDK 版本具有相同外部语义：

- 同一 schema 生成相同 operation/symbol 身份；
- direct 热路径不引用 LR managed dispatcher；
- manifest 声明与 EKI imports/capabilities 完全一致；
- RISC-V 64 与 LoongArch 64 的整数宽度、对齐和状态码一致；
- unsupported target 在构建前明确失败。
