# Go 端 Carrier ABI

本目录是 Rust `carrier/` 固定生命周期结构在 Go 端的镜像。这里只定义整数、固定数组和状态，
不保存 Rust/Go 指针，也不实现设备 API。

布局测试必须与 `carrier/src/lib.rs` 的 size、alignment 和 offset 断言同步。改变已有字段属于 ABI
破坏；兼容扩展使用新版本结构，不能复用保留字段而不提升版本。
