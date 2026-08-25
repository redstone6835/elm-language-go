# Kernel SDK

本目录规划 Go 风格的 direct kernel API：日志、时钟、同步、设备对象、队列和错误类型。生成器
从 EKI Profile 产生固定 ABI 声明和 symbol ID，架构 trampoline 连接到 Rust carrier。

API wrapper 不拥有 loader 权限；它只能调用 package 已声明、装载时已解析的符号。跨 ABI 数据
必须是固定宽度整数、固定布局结构、byte region 或 opaque handle。
