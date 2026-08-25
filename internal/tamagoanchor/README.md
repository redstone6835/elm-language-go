# tamagoanchor

这个包是 TamaGo 依赖版本的仓库内锚点，便于生成器和校验工具引用一致的
模块名、框架版本与编译器标签。真正的依赖关系由根目录 `go.mod` 的
`require` 和 `tool` 指令建立；本包不使用旧式 `tools.go` 空导入技巧，也不
参与 ELM 的运行时调用路径。

修改 `pin.go` 时，必须同步更新：

- 根目录 `go.mod` 与 `go.sum`；
- `toolchain/tamago/toolchain.lock.toml`；
- `toolchain/scripts/verify-toolchain.sh` 中的预期版本。
