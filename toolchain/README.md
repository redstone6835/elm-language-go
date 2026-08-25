# 工具链

本目录管理可复现构建所需的工具链元数据、只读检查和官方 launcher 入口，不保存编译器源码、
二进制归档或构建缓存。

目录职责：

- `tamago/`：固定 TamaGo 框架、对应修改版 Go 编译器及可选发布包摘要；
- `scripts/`：提供无副作用的版本与摘要校验脚本。

正常入口是根模块声明的 `go tool tamago`。它按照 `go.mod` 中
`github.com/usbarmory/tamago` 的版本选择匹配的 `tamago-go` 标签，并在用户
缓存中构建工具链。首次执行会触发网络下载和本机编译，因此不应在纯校验
步骤中调用。

仓库明确不采用 Git submodule、`vendor/` 或提交预构建工具链的方式引入
TamaGo。Go Modules 的框架内容由 `go.mod`/`go.sum` 校验；修改版编译器的 tag、解析后的
commit 和可选发布包摘要记录在 `tamago/toolchain.lock.toml`。目标命令入口会核对官方 launcher
缓存 checkout 的 commit。无参元数据检查本身不声称验证尚未下载的编译器字节。
