# TamaGo 接入

本项目采用 TamaGo 上游从 Go 1.24 起推荐的工具依赖方式：框架以 Go Module
依赖锁定，编译器启动器以 `tool` 指令声明，再通过 `go tool tamago` 调用。

```text
require github.com/usbarmory/tamago v1.27.0
tool github.com/usbarmory/tamago/cmd/tamago
```

TamaGo 启动器会从这项模块依赖推导出 `tamago-go1.27.0`，把对应修改版 Go
源码克隆并编译到用户缓存，然后将余下参数转发给该 Go 命令。仓库不会复制
上游源码，也不会维护一份容易漂移的 Git submodule。

## 基本调用

宿主机需要 Go 1.27 或兼容 `tool` 指令的更新版本。以下命令会在首次调用时
下载并编译工具链：

```sh
go tool tamago version
./toolchain/scripts/tamago-target.sh riscv64 env GOOS GOARCH GOOSPKG
```

第一条命令按宿主目标构建并验证官方 launcher。Go 会先编译 `go tool` 包，再启动它；
因此不能把 `GOOS=tamago` 直接设置在这条外层命令上，否则 launcher 自身也会被当成
裸机目标。第二条命令先用 `go tool -n tamago` 解析同一个固定版本的宿主 launcher，
再只向 launcher 传递目标环境。实际 `build`、`test` 和 `env` 也使用该脚本。

实际 ELM 产物还需要本仓库后续提供的 ELM 入口、链接布局和
`runtime/goos` 对接，不能把上面的版本探测命令误当成可装载模块构建。

## 版本更新

1. 检查 TamaGo 框架发布及其对应的 `tamago-go` 标签。
2. 更新根目录 `go.mod`，用 Go 命令重新生成 `go.sum`。
3. 解析 annotated tag 的 commit，更新 `toolchain.lock.toml`、版本锚点和校验脚本。
4. 执行 `../scripts/verify-toolchain.sh`，再运行仓库测试。
5. 在独立缓存中完成至少一次目标架构构建验证。

`patches/` 预留给确有必要的 ELM 专用编译器补丁。当前补丁集为空，默认及
受支持的路径始终是未经修改的上游标签。
